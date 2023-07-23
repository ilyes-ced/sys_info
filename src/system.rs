pub struct System {
    process_list: Process,
    mem_total: u64,
    mem_free: u64,
    mem_available: u64,
    mem_buffers: u64,
    mem_page_cache: u64,
    mem_shmem: u64,
    mem_slab_reclaimable: u64,
    swap_total: u64,
    swap_free: u64,
    components: Vec<Component>,
    disks: Vec<Disk>,
    networks: Networks,
    users: Vec<User>,
    info: SystemInfo,
    cpus: CpusWrapper,
}

pub struct User {
    uid: Uid,
    gid: Gid,
    name: String,
    groups: Vec<String>,
}

struct SystemInfo {
    page_size_kb: u64,
    clock_cycle: u64,
    boot_time: u64,
}

pub struct Cpu {
    old_values: CpuValues,
    new_values: CpuValues,
    name: String,
    cpu_usage: f32,
    total_time: u64,
    old_total_time: u64,
    frequency: u64,
    vendor_id: String,
    brand: String,
}

struct CpuValues {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
    guest: u64,
    guest_nice: u64,
}