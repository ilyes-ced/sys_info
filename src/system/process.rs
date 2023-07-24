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
    name: String,
    cmd: Vec<String>,
    exe: PathBuf,
    pid: u64,
    parent: Option<u64>,
}