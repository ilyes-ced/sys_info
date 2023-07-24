pub mod cpu;
pub mod disk;
pub mod memory;
pub mod network;
pub mod process;
pub mod user;
pub mod utils;

use cpu::Cpu;
use disk::Disk;
use memory::Memory;
use network::Networks;
use process::Process;
use user::User;

// not needed probably
enum ComponentType {
    CPU,
    GPU,
    RAM,
    DISK,
}

pub struct Component {
    r#type: ComponentType,
    name: String,
    brand: String,
    temperature: Option<f32>,
}

struct SystemInfo {
    page_size_kb: u64,
    clock_cycle: u64,
    boot_time: u64,
}

pub struct System {
    info: SystemInfo,
    process_list: Vec<Process>,
    memory: Memory,
    components: Vec<Component>,
    disks: Vec<Disk>,
    networks: Networks,
    users: Vec<User>,
    cpus: Cpu,
}
