extern crate chrono;
extern crate clap;
extern crate fern;
extern crate forksrv;
extern crate grammartec;
extern crate log;
extern crate memmap;
extern crate pyo3;
extern crate redis;
extern crate reqwest;
extern crate ron;
extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate sha2;
extern crate tokio;

mod config;
mod coverage;
mod fuzzer;
mod message_post;
mod python_grammar_loader;
mod queue;
mod sample;
mod shared_state;
mod state;
mod status;

use clap::{Arg, Command};
use config::Config;
use coverage::CoverageInfo;

use forksrv::newtypes::SubprocessError;
use fuzzer::Fuzzer;
use grammartec::chunkstore::ChunkStoreWrapper;
use grammartec::context::Context;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use queue::{InputState, QueueItem};
use shared_state::GlobalSharedState;
use state::FuzzingState;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{thread, time};
use tokio::runtime::Runtime;

use crate::message_post::send_status_param;

// mutate the input tree
fn process_input(
    state: &mut FuzzingState,
    inp: &mut QueueItem,
    config: &Config,
) -> Result<(), SubprocessError> {
    match inp.state {
        InputState::Init(start_index) => {
            let end_index = start_index + 200;

            if state.minimize(inp, start_index, end_index)? {
                inp.state = InputState::Det((0, 0));
            } else {
                inp.state = InputState::Init(end_index);
            }
        }
        InputState::Det((cycle, start_index)) => {
            let end_index = start_index + 1;
            if state.deterministic_tree_mutation(inp, start_index, end_index)? {
                if cycle == config.number_of_deterministic_mutations {
                    inp.state = InputState::Random;
                } else {
                    inp.state = InputState::Det((cycle + 1, 0));
                }
            } else {
                inp.state = InputState::Det((cycle, end_index));
            }
            state.splice(inp)?;
            state.havoc(inp)?;
            state.havoc_recursion(inp)?;
        }
        InputState::Random => {
            state.splice(inp)?;
            state.havoc(inp)?;
            state.havoc_recursion(inp)?;
        }
    }
    Ok(())
}

fn setup_logger(log_path: &str) -> Result<(), fern::InitError> {
    if log_path.is_empty() {
        return Ok(());
    }

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file(log_path)?)
        .apply()?;

    Ok(())
}

fn coverage_thread(global_state: &Arc<Mutex<GlobalSharedState>>, config: &Config) {
    let mut coverage_info = CoverageInfo::new();
    coverage_info.start_hermit_cov(config);
    loop {
        thread::sleep(time::Duration::from_secs(2));
        let mut stats = global_state.lock().expect("get global_state error!");
        coverage_info.get_coverage();
        if coverage_info.lines_coverage == 0.0 || coverage_info.func_coverage == 0.0 {
            continue;
        }
        stats.func_coverage = coverage_info.func_coverage;
        stats.lines_coverage = coverage_info.lines_coverage;
    }
}

fn fuzzing_thread(
    global_state: &Arc<Mutex<GlobalSharedState>>,
    config: &Config,
    ctx: &Context,
    cks: &Arc<ChunkStoreWrapper>,
) {
    let path_to_bin_target = config.path_to_bin_target.clone();
    let args = config.arguments.clone();
    let fuzzer = Fuzzer::new(
        path_to_bin_target.clone(),
        args,
        global_state.clone(),
        config.hide_output,
        config.timeout_in_millis,
        config.bitmap_size,
        config.server_addr.clone(),
        config.path_to_workdir.clone(),
        config.path_to_src.clone(),
        config.path_to_bin_target_with_cov.clone(),
    );
    let mut state = FuzzingState::new(fuzzer, config.clone(), cks.clone());
    state.ctx = ctx.clone();
    let mut old_execution_count = 0;
    let mut old_executions_per_sec = 0;
    //Normal mode
    loop {
        let inp = global_state.lock().expect("cann't get queue!").queue.pop();
        if let Some(mut inp) = inp {
            // process input from queue
            // If subprocess died restart forkserver
            if process_input(&mut state, &mut inp, config).is_err() {
                let args = vec![];
                let fuzzer = Fuzzer::new(
                    path_to_bin_target.clone(),
                    args,
                    global_state.clone(),
                    config.hide_output,
                    config.timeout_in_millis,
                    config.bitmap_size,
                    config.server_addr.clone(),
                    config.path_to_workdir.clone(),
                    config.path_to_src.clone(),
                    config.path_to_bin_target_with_cov.clone(),
                );
                state = FuzzingState::new(fuzzer, config.clone(), cks.clone());
                state.ctx = ctx.clone();
                old_execution_count = 0;
                old_executions_per_sec = 0;
            }
            /*global_state
            .lock()
            .expect("RAND_788470278")
            .queue
            .finished(inp);*/
        } else {
            // Generate mode cause queue is empty
            // default generate 100 inputs
            for _ in 0..config.number_of_generate_inputs {
                // If subprocess dies restart forkserver
                if state.generate_random().is_err() {
                    let args = vec![];
                    let fuzzer = Fuzzer::new(
                        path_to_bin_target.clone(),
                        args,
                        global_state.clone(),
                        config.hide_output,
                        config.timeout_in_millis,
                        config.bitmap_size,
                        config.server_addr.clone(),
                        config.path_to_workdir.clone(),
                        config.path_to_src.clone(),
                        config.path_to_bin_target_with_cov.clone(),
                    );
                    state = FuzzingState::new(fuzzer, config.clone(), cks.clone());
                    state.ctx = ctx.clone();
                    old_execution_count = 0;
                    old_executions_per_sec = 0;
                }
            }
            //global_state
            //    .lock()
            //    .expect("RAND_2035137253")
            //    .queue
            //    .new_round();
        }
        let mut stats = global_state.lock().expect("RAND_2403514078");
        stats.execution_count += state.fuzzer.execution_count - old_execution_count;
        old_execution_count = state.fuzzer.execution_count;
        stats.average_executions_per_sec += state.fuzzer.average_executions_per_sec;
        stats.average_executions_per_sec -= old_executions_per_sec as f32;
        old_executions_per_sec = state.fuzzer.average_executions_per_sec as u64;
        stats.map_density = state.fuzzer.map_density;
        if state.fuzzer.bits_found_by_havoc > 0 {
            stats.bits_found_by_havoc += state.fuzzer.bits_found_by_havoc;
            state.fuzzer.bits_found_by_havoc = 0;
        }
        if state.fuzzer.bits_found_by_gen > 0 {
            stats.bits_found_by_gen += state.fuzzer.bits_found_by_gen;
            state.fuzzer.bits_found_by_gen = 0;
        }
        if state.fuzzer.bits_found_by_min > 0 {
            stats.bits_found_by_min += state.fuzzer.bits_found_by_min;
            state.fuzzer.bits_found_by_min = 0;
        }
        if state.fuzzer.bits_found_by_det > 0 {
            stats.bits_found_by_det += state.fuzzer.bits_found_by_det;
            state.fuzzer.bits_found_by_det = 0;
        }
        if state.fuzzer.bits_found_by_splice > 0 {
            stats.bits_found_by_splice += state.fuzzer.bits_found_by_splice;
            state.fuzzer.bits_found_by_splice = 0;
        }
        if state.fuzzer.bits_found_by_havoc_rec > 0 {
            stats.bits_found_by_havoc_rec += state.fuzzer.bits_found_by_havoc_rec;
            state.fuzzer.bits_found_by_havoc_rec = 0;
        }
        if state.fuzzer.bits_found_by_min_rec > 0 {
            stats.bits_found_by_min_rec += state.fuzzer.bits_found_by_min_rec;
            state.fuzzer.bits_found_by_min_rec = 0;
        }
    }
}

fn main() {
    //Parse parameters
    let matches = Command::new("hermitcrab")
        .about("Grammar fuzzer")
        .arg(
            Arg::new("config")
                .short('c')
                .value_name("CONFIG")
                .action(clap::ArgAction::Set)
                .help("Path to configuration file")
                .default_value("config.ron"),
        )
        .arg(
            Arg::new("grammar")
                .short('g')
                .action(clap::ArgAction::Set)
                .help("Overwrite the grammar file specified in the CONFIG"),
        )
        .arg(
            Arg::new("workdir")
                .short('o')
                .action(clap::ArgAction::Set)
                .help("Overwrite the workdir specified in the CONFIG"),
        )
        .arg(
            Arg::new("cmdline")
                .action(clap::ArgAction::Append)
                .trailing_var_arg(true),
        )
        .get_matches();

    let config_file_path = matches
        .get_one::<String>("config")
        .expect("the path to the configuration file has a default value");

    info!("Starting Fuzzing...");

    //Set Config
    let mut config_file = File::open(config_file_path).expect("cannot read config file");
    let mut config_file_contents = String::new();
    config_file
        .read_to_string(&mut config_file_contents)
        .expect("RAND_1413661228");
    let mut config: Config =
        ron::de::from_str(&config_file_contents).expect("Failed to deserialize");

    setup_logger(&config.path_to_log).expect("error setup_logger!");
    let workdir = matches
        .get_one("workdir")
        .unwrap_or(&config.path_to_workdir)
        .to_string();
    config.path_to_workdir = workdir;

    //Check if specified workdir exists:
    assert!(
        Path::new(&config.path_to_workdir).exists(),
        "Specified working directory does not exist!\nGiven path: {}",
        config.path_to_workdir
    );

    if let Some(mut cmdline) = matches.get_many::<String>("cmdline") {
        if cmdline.len() > 0 {
            config.path_to_bin_target = cmdline.next().unwrap().to_string();
            config.arguments = cmdline.map(std::string::ToString::to_string).collect();
        }
    }
    //Check if target binary exists:
    assert!(
        Path::new(&config.path_to_bin_target).exists(),
        "Target binary does not exist!\nGiven path: {}",
        config.path_to_bin_target
    );

    let shared: Arc<Mutex<GlobalSharedState>> = Arc::new(Mutex::new(GlobalSharedState::new(
        config.path_to_workdir.clone(),
        config.redis_addr.clone(),
        config.bitmap_size,
    )));
    let shared_chunkstore = Arc::new(ChunkStoreWrapper::new(config.path_to_workdir.clone()));

    let grammar_path = matches
        .get_one::<String>("grammar")
        .unwrap_or(&config.path_to_grammar)
        .to_owned();

    //Check if grammar file exists:
    if !Path::new(&grammar_path).exists() {
        panic!("{}", "Grammar does not exist!\nGiven path: {grammar_path}");
    }

    //Generate rules using a grammar
    let mut my_context = Context::new();
    if grammar_path.ends_with(".json") {
        let gf = File::open(grammar_path).expect("cannot read grammar file");
        let rules: Vec<Vec<String>> =
            serde_json::from_reader(&gf).expect("cannot parse grammar file");
        let root = "{".to_string() + &rules[0][0] + "}";
        my_context.add_rule("START", root.as_bytes());
        for rule in rules {
            my_context.add_rule(&rule[0], rule[1].as_bytes());
        }
    } else if grammar_path.ends_with(".py") {
        my_context = python_grammar_loader::load_python_grammar(&grammar_path);
    } else {
        panic!("Unknown grammar type");
    }

    my_context.initialize(config.max_tree_size);

    //Create output folder
    let folders = [
        "/outputs/signaled",
        "/outputs/queue",
        "/outputs/timeout",
        "/outputs/chunks",
    ];
    for f in &folders {
        fs::create_dir_all(format!("{}/{f}", config.path_to_workdir))
            .expect("Could not create folder in workdir");
    }

    //Start fuzzing threads
    let mut thread_number = 0;
    let threads = (0..config.number_of_threads).map(|_| {
        let state = shared.clone();
        let config = config.clone();
        let ctx = my_context.clone();
        let cks = shared_chunkstore.clone();
        thread_number += 1;
        thread::Builder::new()
            .name(format!("fuzzer_{thread_number}"))
            .stack_size(config.thread_size)
            .spawn(move || fuzzing_thread(&state, &config, &ctx, &cks))
    });

    // start coverage thread
    if config.show_coverage {
        let state = shared.clone();
        let config = config.clone();
        thread::Builder::new()
            .name("coverage_thread".to_string())
            .spawn(move || coverage_thread(&state, &config))
            .expect("coverage_thread failed to start");
    }
    //Start status thread
    let status_thread = {
        let global_state = shared.clone();
        let shared_cks = shared_chunkstore.clone();
        let work_dir = config.path_to_workdir.clone();
        let bin_target = config.path_to_bin_target.clone();
        let show_coverage = config.show_coverage;
        let server_addr = config.server_addr.clone();
        let container_id = std::env::var("HOSTNAME").unwrap_or("unknown".to_string());
        thread::Builder::new()
            .name("status_thread".to_string())
            .spawn(move || {
                let start_time = Instant::now();
                thread::sleep(time::Duration::from_secs(1));
                print!("{}[2J", 27 as char);
                print!("{}[H", 27 as char);
                loop {
                    let execution_count;
                    let average_executions_per_sec;
                    let bits_found_by_gen;
                    let bits_found_by_min;
                    let bits_found_by_min_rec;
                    let bits_found_by_det;
                    let bits_found_by_splice;
                    let bits_found_by_havoc;
                    let bits_found_by_havoc_rec;
                    let last_found_asan;
                    let last_found_sig;
                    let last_timeout;
                    let total_found_asan;
                    let total_found_sig;
                    let map_density;
                    let func_coverage;
                    let lines_coverage;
                    {
                        let shared_state = global_state.lock().expect("RAND_597319831");
                        execution_count = shared_state.execution_count;
                        average_executions_per_sec = shared_state.average_executions_per_sec;
                        bits_found_by_gen = shared_state.bits_found_by_gen;
                        bits_found_by_min = shared_state.bits_found_by_min;
                        bits_found_by_min_rec = shared_state.bits_found_by_min_rec;
                        bits_found_by_det = shared_state.bits_found_by_det;
                        bits_found_by_splice = shared_state.bits_found_by_splice;
                        bits_found_by_havoc = shared_state.bits_found_by_havoc;
                        bits_found_by_havoc_rec = shared_state.bits_found_by_havoc_rec;
                        last_found_asan = shared_state.last_found_asan.clone();
                        last_found_sig = shared_state.last_found_sig.clone();
                        last_timeout = shared_state.last_timeout.clone();
                        total_found_asan = shared_state.total_found_asan;
                        total_found_sig = shared_state.total_found_sig;
                        map_density = shared_state.map_density;
                        func_coverage = shared_state.func_coverage.clone();
                        lines_coverage = shared_state.lines_coverage.clone();
                    }
                    let secs = start_time.elapsed().as_secs();
                    let minutes = secs / 60;
                    let hours = minutes / 60;
                    let days = hours / 24;

                    let status = status::Status {
                        container_id: container_id.clone(),
                        // container_id: format!("0b9795c16f22"),
                        line_coverage: format!("{}%", lines_coverage),
                        function_coverage: format!("{}%", func_coverage),
                        last_crash_time: last_found_sig.clone(),
                        last_timeout_time: last_timeout.clone(),
                        map_density,
                        sample_count: execution_count,
                        crash_count: total_found_sig,
                        sample_run_rate: average_executions_per_sec,
                    };
                    Runtime::new().unwrap().block_on(async {
                        let _res = send_status_param(&server_addr, &status).await;
                    });
                    print!("{}[H", 27 as char);
                    println!(" _   _                     _ _    ____           _     ");
                    println!("| | | | ___ _ __ _ __ ___ (_) |_ / ___|_ __ __ _| |__  ");
                    println!("| |_| |/ _ \\ '__| '_ ` _ \\| | __| |   | '__/ _` | '_ \\ ");
                    println!("|  _  |  __/ |  | | | | | | | |_| |___| | | (_| | |_) |");
                    println!("|_| |_|\\___|_|  |_| |_| |_|_|\\__|\\____|_|  \\__,_|_.__/ ");
                    println!("                                                        ");
                    println!("------------------------------------------------------    ");
                    println!(
                        "Run Time: {} days, {} hours, {} minutes, {} seconds       ",
                        days,
                        hours % 24,
                        minutes % 60,
                        secs % 60
                    );
                    println!(
                        "Execution Count:          {}                              ",
                        execution_count
                    );
                    println!(
                        "Executions per Sec:       {}                              ",
                        average_executions_per_sec
                    );
                    let now = Instant::now();
                    while shared_cks.is_locked.load(Ordering::SeqCst) {
                        if now.elapsed().as_secs() > 30 {
                            panic!("Printing thread starved!");
                        }
                    }
                    println!(
                        "Trees in Chunkstore:      {}                              ",
                        shared_cks
                            .chunkstore
                            .read()
                            .expect("RAND_351823021")
                            .trees()
                    );
                    println!("------------------------------------------------------    ");
                    println!(
                        "Last ASAN crash:          {}                              ",
                        last_found_asan
                    );
                    println!(
                        "Last SIG crash:           {}                              ",
                        last_found_sig
                    );
                    println!(
                        "Last Timeout:             {}                              ",
                        last_timeout
                    );
                    println!(
                        "Total ASAN crashes:       {}                              ",
                        total_found_asan
                    );
                    println!(
                        "Total SIG crashes:        {}                              ",
                        total_found_sig
                    );
                    println!("------------------------------------------------------    ");
                    println!(
                        "New paths found by Gen:          {}                       ",
                        bits_found_by_gen
                    );
                    println!(
                        "New paths found by Min:          {}                       ",
                        bits_found_by_min
                    );
                    println!(
                        "New paths found by Min Rec:      {}                       ",
                        bits_found_by_min_rec
                    );
                    println!(
                        "New paths found by Det:          {}                       ",
                        bits_found_by_det
                    );
                    println!(
                        "New paths found by Splice:       {}                       ",
                        bits_found_by_splice
                    );
                    println!(
                        "New paths found by Havoc:        {}                       ",
                        bits_found_by_havoc
                    );
                    println!(
                        "New paths found by Havoc Rec:    {}                       ",
                        bits_found_by_havoc_rec
                    );
                    println!("------------------------------------------------------    ");
                    println!("Working dir:       {}                          ", work_dir);
                    println!("Target bin:        {}                       ", bin_target);
                    println!(
                        "Map density:       {}%                         ",
                        map_density * 100.0
                    );
                    if show_coverage {
                        println!(
                            "Lines coverage:    {}%                         ",
                            lines_coverage
                        );
                        println!(
                            "Function coverage: {}%                         ",
                            func_coverage
                        );
                    }
                    thread::sleep(time::Duration::from_secs(1));
                }
            })
            .expect("status_thread failed to start")
    };

    for t in threads.collect::<Vec<_>>() {
        t.expect("fuzzing_thread error")
            .join()
            .expect("fuzzing_thread error");
    }
    status_thread.join().expect("status_thread error");
    warn!("exit??");
}
