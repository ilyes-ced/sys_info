use std::time::Instant;

pub struct GlobalCpu {
    global_cpu: Cpu,
    cpus: Vec<Cpu>,
    need_cpus_update: bool,
    got_cpu_frequency: bool,
    update_timeout: Option<Instant>,
}

pub struct Cpu {
    old_values: u64,
    new_values: u64,
    name: String,
    cpu_usage: f32,
    total_time: u64,
    old_total_time: u64,
    frequency: u64,
    vendor_id: String,
    brand: String,
}
