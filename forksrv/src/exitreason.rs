use nix::sys::wait::WaitStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExitReason {
    Normal(i32),
    Timeouted,
    Signaled(i32),
    Stopped(i32),
}

impl ExitReason {
    #[must_use]
    pub fn from_wait_status(status: WaitStatus) -> ExitReason {
        match status {
            WaitStatus::Exited(_, return_value) => ExitReason::Normal(return_value),
            WaitStatus::Signaled(_, signal, _) => ExitReason::Signaled(signal as i32),
            WaitStatus::Stopped(_, signal) => ExitReason::Stopped(signal as i32),
            _ => panic!("{}", "Unknown WaitStatus: {status:?}"),
        }
    }
}
