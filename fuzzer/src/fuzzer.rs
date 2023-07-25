use crate::message_post;
use crate::sample::{Sample, SampleType};
use crate::shared_state::GlobalSharedState;
use chrono::Local;
use forksrv::exitreason::ExitReason;
use forksrv::newtypes::SubprocessError;
use forksrv::ForkServer;
use grammartec::context::Context;
use grammartec::tree::TreeLike;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::io::stdout;
use std::io::Write;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use tokio::runtime::Runtime;

#[derive(Debug, Clone, Copy)]
pub enum ExecutionReason {
    Havoc,
    HavocRec,
    Min,
    MinRec,
    Splice,
    Det,
    Gen,
}

pub struct Fuzzer {
    forksrv: ForkServer,
    last_tried_inputs: HashSet<Vec<u8>>,
    last_inputs_ring_buffer: VecDeque<Vec<u8>>,
    pub global_state: Arc<Mutex<GlobalSharedState>>,
    pub target_path: String,
    pub target_args: Vec<String>,
    pub execution_count: u64,
    pub average_executions_per_sec: f32,
    pub bits_found_by_havoc: u64,
    pub bits_found_by_havoc_rec: u64,
    pub bits_found_by_min: u64,
    pub bits_found_by_min_rec: u64,
    pub bits_found_by_splice: u64,
    pub bits_found_by_det: u64,
    pub bits_found_by_det_afl: u64,
    pub bits_found_by_gen: u64,
    pub asan_found_by_havoc: u64,
    pub asan_found_by_havoc_rec: u64,
    pub asan_found_by_min: u64,
    pub asan_found_by_min_rec: u64,
    pub asan_found_by_splice: u64,
    pub asan_found_by_det: u64,
    pub asan_found_by_det_afl: u64,
    pub asan_found_by_gen: u64,
    pub map_density: f32,
    pub addr: String,
    pub samples_vec: Vec<Sample>,
    pub container_id: String,
    pub work_dir: String,
    pub src_dir: String,
    pub target_cov_path: String,
}

impl Fuzzer {
    pub fn new(
        path: String,
        args: Vec<String>,
        global_state: Arc<Mutex<GlobalSharedState>>,
        hide_output: bool,
        timeout_in_millis: u64,
        bitmap_size: usize,
        addr: String,
        work_dir: String,
        src_dir: String,
        target_cov_path: String,
    ) -> Self {
        let fs = ForkServer::new(
            path.clone(),
            args.clone(),
            hide_output,
            timeout_in_millis,
            bitmap_size,
        );
        let container_id = std::env::var("HOSTNAME").unwrap_or("unknown".to_string());
        info!("container_id: {}", container_id);
        Fuzzer {
            forksrv: fs,
            last_tried_inputs: HashSet::new(),
            last_inputs_ring_buffer: VecDeque::new(),
            global_state,
            target_path: path,
            target_args: args,
            execution_count: 0,
            average_executions_per_sec: 0.0,
            bits_found_by_havoc: 0,
            bits_found_by_havoc_rec: 0,
            bits_found_by_min: 0,
            bits_found_by_min_rec: 0,
            bits_found_by_splice: 0,
            bits_found_by_det: 0,
            bits_found_by_det_afl: 0,
            bits_found_by_gen: 0,
            asan_found_by_havoc: 0,
            asan_found_by_havoc_rec: 0,
            asan_found_by_min: 0,
            asan_found_by_min_rec: 0,
            asan_found_by_splice: 0,
            asan_found_by_det: 0,
            asan_found_by_det_afl: 0,
            asan_found_by_gen: 0,
            map_density: 0.0,
            samples_vec: vec![],
            addr,
            container_id,
            work_dir,
            src_dir,
            target_cov_path,
        }
    }

    pub fn run_on_with_dedup<T: TreeLike>(
        &mut self,
        tree: &T,
        exec_reason: ExecutionReason,
        ctx: &Context,
    ) -> Result<bool, SubprocessError> {
        let code = tree.unparse_to_vec(ctx);
        if self.input_is_known(&code) {
            return Ok(false);
        }
        self.run_on(&code, tree, exec_reason, ctx)?;
        Ok(true)
    }

    pub fn run_on_without_dedup<T: TreeLike>(
        &mut self,
        tree: &T,
        exec_reason: ExecutionReason,
        ctx: &Context,
    ) -> Result<(), SubprocessError> {
        let code = tree.unparse_to_vec(ctx);
        self.run_on(&code, tree, exec_reason, ctx)
    }

    fn run_on<T: TreeLike>(
        &mut self,
        code: &[u8],
        tree: &T,
        exec_reason: ExecutionReason,
        ctx: &Context,
    ) -> Result<(), SubprocessError> {
        let (new_bits, term_sig) = self.exec(code, tree, ctx)?;
        match term_sig {
            ExitReason::Normal(223) => {
                if new_bits.is_some() {
                    //ASAN
                    self.global_state
                        .lock()
                        .expect("RAND_3390206382")
                        .total_found_asan += 1;
                    self.global_state
                        .lock()
                        .expect("RAND_202860771")
                        .last_found_asan = Local::now().format("[%Y-%m-%d] %H:%M:%S").to_string();
                    let file_path = format!(
                        "{}/outputs/signaled/ASAN_{:09}_{}",
                        self.work_dir,
                        self.execution_count,
                        thread::current().name().expect("RAND_4086695190")
                    );
                    let mut file = fs::File::create(&file_path).expect("RAND_3096222153");
                    tree.unparse_to(ctx, &mut file);
                    match Sample::new(&self.container_id, &file_path, SampleType::Normal, 0.0, 0.0)
                    {
                        Ok(s) => {
                            Runtime::new().unwrap().block_on(async {
                                let _res = message_post::send_sample(&self.addr, &s).await;
                            });
                        }
                        Err(e) => {
                            warn!("Sample::new err: {}", e);
                        }
                    }
                }
            }
            ExitReason::Normal(_) => {
                if new_bits.is_some() {
                    match exec_reason {
                        ExecutionReason::Havoc => {
                            self.bits_found_by_havoc += 1;
                        }
                        ExecutionReason::HavocRec => {
                            self.bits_found_by_havoc_rec += 1;
                        }
                        ExecutionReason::Min => {
                            self.bits_found_by_min += 1;
                        }
                        ExecutionReason::MinRec => {
                            self.bits_found_by_min_rec += 1;
                        }
                        ExecutionReason::Splice => {
                            self.bits_found_by_splice += 1;
                        }
                        ExecutionReason::Det => {
                            self.bits_found_by_det += 1;
                        }
                        ExecutionReason::Gen => {
                            self.bits_found_by_gen += 1;
                        }
                    }
                }
            }
            ExitReason::Timeouted => {
                self.global_state
                    .lock()
                    .expect("RAND_1706238230")
                    .last_timeout = Local::now().format("[%Y-%m-%d] %H:%M:%S").to_string();
                let file_path = format!(
                    "{}/outputs/timeout/{:09}",
                    self.work_dir, self.execution_count
                );
                let mut file = fs::File::create(&file_path).expect("RAND_452993103");
                tree.unparse_to(ctx, &mut file);
                match Sample::new(&self.container_id, &file_path, SampleType::Normal, 0.0, 0.0) {
                    Ok(s) => {
                        Runtime::new().unwrap().block_on(async {
                            let _res = message_post::send_sample(&self.addr, &s).await;
                        });
                    }
                    Err(e) => {
                        warn!("Sample::new err: {}", e);
                    }
                }
            }
            ExitReason::Signaled(sig) => {
                if new_bits.is_some() {
                    self.global_state
                        .lock()
                        .expect("RAND_1858328446")
                        .total_found_sig += 1;
                    self.global_state
                        .lock()
                        .expect("RAND_4287051369")
                        .last_found_sig = Local::now().format("[%Y-%m-%d] %H:%M:%S").to_string();

                    let file_path = format!(
                        "{}/outputs/signaled/{sig:?}_{:09}",
                        self.work_dir, self.execution_count
                    );
                    let mut file = fs::File::create(&file_path).expect("RAND_3690294970");
                    tree.unparse_to(ctx, &mut file);
                    match Sample::new(&self.container_id, &file_path, SampleType::Normal, 0.0, 0.0)
                    {
                        Ok(s) => {
                            Runtime::new().unwrap().block_on(async {
                                let _res = message_post::send_sample(&self.addr, &s).await;
                            });
                        }
                        Err(e) => {
                            warn!("Sample::new err: {}", e);
                        }
                    }
                }
            }
            ExitReason::Stopped(_sig) => {}
        }
        stdout().flush().expect("RAND_2937475131");
        Ok(())
    }

    pub fn has_bits<T: TreeLike>(
        &mut self,
        tree: &T,
        bits: &HashSet<usize>,
        exec_reason: ExecutionReason,
        ctx: &Context,
    ) -> Result<bool, SubprocessError> {
        self.run_on_without_dedup(tree, exec_reason, ctx)?;
        let run_bitmap = self.forksrv.get_shared();
        let mut found_all = true;
        for bit in bits.iter() {
            if run_bitmap[*bit] == 0 {
                //TODO: handle edge counts properly
                found_all = false;
            }
        }
        Ok(found_all)
    }

    pub fn exec_raw(&mut self, code: &[u8]) -> Result<(ExitReason, u32), SubprocessError> {
        self.execution_count += 1;
        let start = Instant::now();
        let exitreason = self.forksrv.run(code)?;
        let execution_time = start.elapsed().subsec_nanos();
        self.average_executions_per_sec = self.average_executions_per_sec * 0.9
            + ((1.0 / (execution_time as f32)) * 1_000_000_000.0) * 0.1;
        Ok((exitreason, execution_time))
    }

    fn input_is_known(&mut self, code: &[u8]) -> bool {
        if self.last_tried_inputs.contains(code) {
            true
        } else {
            self.last_tried_inputs.insert(code.to_vec());
            if self.last_inputs_ring_buffer.len() == 10000 {
                self.last_tried_inputs.remove(
                    &self
                        .last_inputs_ring_buffer
                        .pop_back()
                        .expect("No entry in last_inputs_ringbuffer"),
                );
            }
            self.last_inputs_ring_buffer.push_front(code.to_vec());
            false
        }
    }

    fn exec<T: TreeLike>(
        &mut self,
        code: &[u8],
        tree_like: &T,
        ctx: &Context,
    ) -> Result<(Option<Vec<usize>>, ExitReason), SubprocessError> {
        let (exitreason, execution_time) = self.exec_raw(code)?;

        let is_crash = matches!(
            exitreason,
            ExitReason::Normal(223) | ExitReason::Signaled(_)
        );

        let mut final_bits = None;
        if let Some(mut new_bits) = self.has_new_bits(is_crash) {
            //Only if not Timeout
            if exitreason != ExitReason::Timeouted {
                //Check for non deterministic bits
                let old_bitmap: Vec<u8> = self.forksrv.get_shared().to_vec();
                self.check_deterministic_behaviour(&old_bitmap, &mut new_bits, code)?;
                if !new_bits.is_empty() {
                    final_bits = Some(new_bits);
                    let tree = tree_like.to_tree(ctx);
                    let file_path = self
                        .global_state
                        .lock()
                        .expect("RAND_2835014626")
                        .queue
                        .add(tree, old_bitmap, exitreason, ctx, execution_time);

                    if !is_file_empty(&file_path) {
                        // get sample coverage
                        let coverage = match self.calc_sample_coverage(&file_path) {
                            Ok(cov) => cov,
                            Err(_e) => (0.0, 0.0),
                        };
                        if coverage != (0.0, 0.0) {
                            // send samples to server
                            match Sample::new(
                                &self.container_id,
                                &file_path,
                                SampleType::Normal,
                                coverage.1,
                                coverage.0,
                            ) {
                                Ok(s) => {
                                    info!(
                                        "sample {} func_cov: {}%; line_cov: {}%",
                                        &file_path, coverage.0, coverage.1
                                    );
                                    self.samples_vec.push(s);
                                    // upload every 10 samples
                                    if self.samples_vec.len() >= 10 {
                                        let rt = Runtime::new().unwrap();
                                        rt.block_on(async {
                                            let _res = message_post::send_samples(
                                                &self.addr,
                                                &self.samples_vec,
                                            )
                                            .await;
                                        });
                                        self.samples_vec.clear();
                                    }
                                }
                                Err(e) => {
                                    warn!("Sample::new err: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok((final_bits, exitreason))
    }

    fn check_deterministic_behaviour(
        &mut self,
        old_bitmap: &[u8],
        new_bits: &mut Vec<usize>,
        code: &[u8],
    ) -> Result<(), SubprocessError> {
        for _ in 0..5 {
            let (_, _) = self.exec_raw(code)?;
            let run_bitmap = self.forksrv.get_shared();
            for (i, &v) in old_bitmap.iter().enumerate() {
                if run_bitmap[i] != v {
                    //  println!("found fucky bit {i}");
                }
            }
            new_bits.retain(|&i| run_bitmap[i] != 0);
        }
        Ok(())
    }

    pub fn has_new_bits(&mut self, is_crash: bool) -> Option<Vec<usize>> {
        let mut res = vec![];
        let run_bitmap = self.forksrv.get_shared();
        let mut gstate_lock = self.global_state.lock().expect("get global state err");
        let shared_bitmap = gstate_lock
            .bitmaps
            .get_mut(&is_crash)
            .expect("Bitmap missing! Maybe shared state was not initialized correctly?");

        let mut density = 0;
        for (i, elem) in shared_bitmap.iter_mut().enumerate() {
            if *elem != 0 {
                density += 1;
            }
            if (run_bitmap[i] != 0) && (*elem == 0) {
                *elem |= run_bitmap[i];
                res.push(i);
            }
        }

        self.map_density = density as f32 / shared_bitmap.len() as f32;
        if !res.is_empty() {
            return Some(res);
        }
        None
    }
    pub fn calc_sample_coverage(&mut self, file_path: &str) -> Result<(f64, f64), String> {
        let output = Command::new(&self.target_cov_path)
            .arg(&file_path)
            .output()
            .map_err(|_| "Failed to execute the target binary".to_string())?;

        if !output.status.success() {
            return Err("The target binary returned a non-zero exit code".to_string());
        }

        let lcov_output = Command::new("lcov")
            .arg("--capture")
            .arg("--directory")
            .arg(&self.src_dir)
            .arg("--output-file")
            .arg("coverage.info")
            .output()
            .map_err(|_| "Failed to execute the lcov command".to_string())?;

        if !lcov_output.status.success() {
            return Err("The lcov command returned a non-zero exit code".to_string());
        }

        let lcov_summary_output = Command::new("lcov")
            .arg("--summary")
            .arg("coverage.info")
            .output()
            .map_err(|_| "Failed to execute the lcov command for summary".to_string())?;

        let (lines_coverage, func_coverage) =
            extract_coverage_from_summary_output(&lcov_summary_output.stdout)?;
        Ok((lines_coverage, func_coverage))
    }
}

fn extract_coverage_from_summary_output(output: &[u8]) -> Result<(f64, f64), String> {
    let output_str =
        std::str::from_utf8(output).map_err(|_| "Failed to parse lcov summary output")?;
    let func_coverage_line = output_str
        .lines()
        .find(|line| line.contains("functions..:")) // 找到以"functions..:"开头的行
        .ok_or("Coverage summary line not found")?;

    let func_coverage_percentage_str = func_coverage_line
        .split_whitespace()
        .nth(1)
        .ok_or("Failed to extract coverage percentage")?;

    let func_coverage_percentage = func_coverage_percentage_str
        .trim_end_matches('%')
        .parse::<f64>()
        .map_err(|_| "Failed to parse coverage percentage")?;

    let lines_coverage_line = output_str
        .lines()
        .find(|line| line.contains("lines......:")) // 找到以"functions..:"开头的行
        .ok_or("Coverage summary line not found")?;

    let lines_coverage_percentage_str = lines_coverage_line
        .split_whitespace()
        .nth(1)
        .ok_or("Failed to extract coverage percentage")?;

    let lines_coverage_percentage = lines_coverage_percentage_str
        .trim_end_matches('%')
        .parse::<f64>()
        .map_err(|_| "Failed to parse coverage percentage")?;

    Ok((func_coverage_percentage, lines_coverage_percentage))
}

fn is_file_empty(file_path: &str) -> bool {
    fs::metadata(file_path)
        .map(|m| m.len() == 0)
        .unwrap_or(true)
}
