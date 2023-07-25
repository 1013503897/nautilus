use crate::config::Config;
use memmap::Mmap;
use std::fs::OpenOptions;
use std::process::{Command, Stdio};

pub struct CoverageInfo {
    pub lines_coverage: f32,
    pub func_coverage: f32,
    shm_lines: Mmap,
    shm_func: Mmap,
}

impl CoverageInfo {
    pub fn new() -> Self {
        let lines_path = "/dev/shm/lines_cov".to_string();
        let func_path = "/dev/shm/func_cov".to_string();

        let lines_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(lines_path)
            .expect("Failed to open shared memory file");
        lines_file
            .set_len(1024)
            .expect("Failed to set shared memory file size");

        let shm_lines =
            unsafe { Mmap::map(&lines_file).expect("Failed to create shared memory mapping") };

        let function_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(func_path)
            .expect("Failed to open shared memory file");
        function_file
            .set_len(1024)
            .expect("Failed to set shared memory file size");

        let shm_func =
            unsafe { Mmap::map(&function_file).expect("Failed to create shared memory mapping") };
        CoverageInfo {
            lines_coverage: 0.0,
            func_coverage: 0.0,
            shm_lines,
            shm_func,
        }
    }

    pub fn get_coverage(&mut self) {
        let lines_coverage = std::str::from_utf8(&self.shm_lines)
            .unwrap()
            .to_owned()
            .trim_end_matches('\0')
            .parse::<f32>()
            .unwrap_or(0.0);

        self.lines_coverage = self.lines_coverage.max(lines_coverage);

        let func_coverage = std::str::from_utf8(&self.shm_func)
            .unwrap()
            .to_owned()
            .trim_end_matches('\0')
            .parse::<f32>()
            .unwrap_or(0.0);

        self.func_coverage = self.func_coverage.max(func_coverage);
    }

    pub fn start_hermit_cov(&mut self, config: &Config) {
        let directory = config.path_to_workdir.clone();
        let executable = format!("{} 'AFL_FILE'", config.path_to_bin_target_with_cov.clone());
        let lua_src = config.path_to_src.clone();

        Command::new("hermit-cov")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("--live")
            .arg("-O")
            .arg("-d")
            .arg(directory)
            .arg("-e")
            .arg(executable)
            .arg("-c")
            .arg(lua_src)
            .spawn()
            .expect("Failed to start hermit-cov");
    }
}
