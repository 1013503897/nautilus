use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub number_of_threads: u8,
    pub thread_size: usize,
    pub number_of_generate_inputs: u16,
    pub number_of_deterministic_mutations: usize,
    pub max_tree_size: usize,
    pub bitmap_size: usize,
    pub timeout_in_millis: u64,
    pub path_to_bin_target: String,
    pub path_to_bin_target_with_cov: String,
    pub path_to_grammar: String,
    pub path_to_workdir: String,
    pub path_to_src: String,
    pub arguments: Vec<String>,
    pub hide_output: bool,
    pub show_coverage: bool,
}
