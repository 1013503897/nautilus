extern crate byteorder;
extern crate nix;
extern crate serde;
extern crate snafu;
extern crate tempfile;
extern crate timeout_readwrite;

pub mod exitreason;
pub mod newtypes;

use nix::errno::errno;
use nix::fcntl;
use nix::libc::{shmat, shmctl, shmget, strerror, IPC_CREAT, IPC_EXCL, IPC_PRIVATE, IPC_RMID};
use nix::sys::signal::{self, Signal};
use nix::sys::stat;
use nix::sys::wait::WaitStatus;
use nix::unistd;
use nix::unistd::Pid;
use nix::unistd::{fork, ForkResult};
use std::ffi::CString;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;

use std::io::BufReader;
use std::ptr;
use std::time::Duration;
use timeout_readwrite::TimeoutReader;

use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::os::unix::io::FromRawFd;

use exitreason::ExitReason;
use newtypes::{QemuRunIOSnafu, QemuRunNixSnafu, SubprocessError};
use snafu::ResultExt;

pub struct ForkServer {
    inp_file: File,
    ctl_in: File,
    shared_data: *mut [u8],
    st_out: std::io::BufReader<TimeoutReader<File>>,
}

impl ForkServer {
    #[must_use]
    pub fn new(
        path: String,
        args: Vec<String>,
        hide_output: bool,
        timeout_in_millis: u64,
        bitmap_size: usize,
    ) -> Self {
        let inp_file = tempfile::NamedTempFile::new().expect("couldn't create temp file");
        let (inp_file, in_path) = inp_file
            .keep()
            .expect("couldn't persists temp file for input");
        let inp_file_path = in_path
            .to_str()
            .expect("temp path should be unicode!")
            .to_string();
        let args = Some(path.clone())
            .into_iter()
            .chain(args.into_iter())
            .map(|s| if s == "@@" { inp_file_path.clone() } else { s });
        let (ctl_out, ctl_in) = nix::unistd::pipe().expect("failed to create ctl_pipe");
        let (st_out, st_in) = nix::unistd::pipe().expect("failed to create st_pipe");
        let (shm_file, shared_data) = ForkServer::create_shm(bitmap_size);

        match unsafe { fork() }.expect("couldn't fork") {
            // Parent returns
            ForkResult::Parent { child: _, .. } => {
                unistd::close(ctl_out).expect("coulnd't close ctl_out");
                unistd::close(st_in).expect("coulnd't close st_out");
                let mut st_out = BufReader::new(TimeoutReader::new(
                    unsafe { File::from_raw_fd(st_out) },
                    Duration::from_millis(timeout_in_millis),
                ));
                st_out
                    .read_u32::<LittleEndian>()
                    .expect("couldn't read child hello");
                Self {
                    inp_file,
                    ctl_in: unsafe { File::from_raw_fd(ctl_in) },
                    shared_data,
                    st_out,
                }
            }
            //Child does complex stuff
            ForkResult::Child => {
                let forkserver_fd = 198; // from AFL config.h
                unistd::dup2(ctl_out, forkserver_fd as RawFd)
                    .expect("couldn't dup2 ctl_out to FROKSRV_FD");
                unistd::dup2(st_in, (forkserver_fd + 1) as RawFd)
                    .expect("couldn't dup2 st_in to FROKSRV_FD+1");

                unistd::dup2(inp_file.as_raw_fd(), 0).expect("couldn't dup2 input file to stdin");

                unistd::close(inp_file.as_raw_fd()).expect("couldn't close input file");
                unistd::close(ctl_in).expect("couldn't close ctl_in");
                unistd::close(ctl_out).expect("couldn't close ctl_out");
                unistd::close(st_in).expect("couldn't close st_out");
                unistd::close(st_out).expect("couldn't close st_out");

                let path = CString::new(path).expect("binary path must not contain zero");
                let args = args
                    .into_iter()
                    .map(|s| CString::new(s).expect("args must not contain zero"))
                    .collect::<Vec<_>>();

                let shm_id = CString::new(format!("__AFL_SHM_ID={shm_file}")).unwrap();

                //Asan options: set asan SIG to 223 and disable leak detection
                let asan_settings = CString::new("abort_on_erro=true,detect_leaks=0,symbolize=0")
                    .expect("RAND_2089158993");

                let map_size = CString::new(format!("AFL_MAP_SIZE=256000 ")).unwrap();
                let env = vec![shm_id, asan_settings, map_size];

                if hide_output {
                    let null = fcntl::open("/dev/null", fcntl::OFlag::O_RDWR, stat::Mode::empty())
                        .expect("couldn't open /dev/null");
                    unistd::dup2(null, 1 as RawFd).expect("couldn't dup2 /dev/null to stdout");
                    unistd::dup2(null, 2 as RawFd).expect("couldn't dup2 /dev/null to stderr");
                    unistd::close(null).expect("couldn't close /dev/null");
                }
                println!("EXECVE {path:?} {args:?} {env:?}");
                unistd::execve(&path, &args, &env).expect("EXECVE ERROR");
                unreachable!();
            }
        }
    }

    pub fn run(&mut self, data: &[u8]) -> Result<ExitReason, SubprocessError> {
        for i in self.get_shared_mut().iter_mut() {
            *i = 0;
        }
        unistd::ftruncate(self.inp_file.as_raw_fd(), 0).context(QemuRunNixSnafu {
            task: "Couldn't truncate inp_file",
        })?;
        unistd::lseek(self.inp_file.as_raw_fd(), 0, unistd::Whence::SeekSet).context(
            QemuRunNixSnafu {
                task: "Couldn't seek inp_file",
            },
        )?;
        unistd::write(self.inp_file.as_raw_fd(), data).context(QemuRunNixSnafu {
            task: "Couldn't write data to inp_file",
        })?;
        unistd::lseek(self.inp_file.as_raw_fd(), 0, unistd::Whence::SeekSet).context(
            QemuRunNixSnafu {
                task: "Couldn't seek inp_file",
            },
        )?;

        unistd::write(self.ctl_in.as_raw_fd(), &[0, 0, 0, 0]).context(QemuRunNixSnafu {
            task: "Couldn't send start command",
        })?;

        let pid = Pid::from_raw(self.st_out.read_i32::<LittleEndian>().context(
            QemuRunIOSnafu {
                task: "Couldn't read target pid",
            },
        )?);

        if let Ok(status) = self.st_out.read_i32::<LittleEndian>() {
            return Ok(ExitReason::from_wait_status(
                WaitStatus::from_raw(pid, status).expect("402104968"),
            ));
        }
        signal::kill(pid, Signal::SIGKILL).context(QemuRunNixSnafu {
            task: "Couldn't kill timed out process",
        })?;
        self.st_out
            .read_u32::<LittleEndian>()
            .context(QemuRunIOSnafu {
                task: "couldn't read timeout exitcode",
            })?;
        Ok(ExitReason::Timeouted)
    }

    pub fn get_shared_mut(&mut self) -> &mut [u8] {
        unsafe { &mut *self.shared_data }
    }
    #[must_use]
    pub fn get_shared(&self) -> &[u8] {
        unsafe { &*self.shared_data }
    }

    fn create_shm(bitmap_size: usize) -> (i32, *mut [u8]) {
        unsafe {
            let shm_id = shmget(IPC_PRIVATE, bitmap_size, IPC_CREAT | IPC_EXCL | 0o600);
            assert!(
                shm_id >= 0,
                "shm_id {:?}",
                CString::from_raw(strerror(errno()))
            );

            let trace_bits = shmat(shm_id, ptr::null(), 0);
            assert!(
                (trace_bits as isize) >= 0,
                "shmat {:?}",
                CString::from_raw(strerror(errno()))
            );

            let res = shmctl(
                shm_id,
                IPC_RMID,
                std::ptr::null_mut::<nix::libc::shmid_ds>(),
            );
            assert!(
                res >= 0,
                "shmclt {:?}",
                CString::from_raw(strerror(errno()))
            );

            let ptr: *mut u8 = trace_bits.cast();
            (shm_id, std::slice::from_raw_parts_mut(ptr, bitmap_size))
        }
    }
}
