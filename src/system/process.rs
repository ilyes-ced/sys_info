use std::path::PathBuf;

pub enum ProcessStatus {
    Idle,
    Run,
    Sleep,
    Stop,
    Zombie,
    Tracing,
    Dead,
    Wakekill,
    Waking,
    LockBlocked,
    UninterruptibleDiskSleep,
    /// Unknown.
    Unknown(u32),
}

pub struct Process {
    pid: u64,
    name: String,
    cmd: Vec<String>,
    exe: PathBuf,
    parent: Option<u64>,
}
