use super::utils::read_file;
use std::{path::Path};

#[derive(Debug)]
pub struct Memory {
    mem_total: u64,
    mem_free: u64,
    mem_available: u64,
    mem_buffers: u64,
    mem_page_cache: u64,
    mem_shmem: u64,
    mem_slab_reclaimable: u64,
    swap_total: u64,
    swap_free: u64,
}

pub fn new() -> Memory {
    Memory {
        mem_total: 0,
        mem_free: 0,
        mem_available: 0,
        mem_buffers: 0,
        mem_page_cache: 0,
        mem_shmem: 0,
        mem_slab_reclaimable: 0,
        swap_total: 0,
        swap_free: 0,
    }
}

impl Memory {
    pub fn set_memory_data(&mut self) {
        let memory = match read_file(Path::new("/proc/meminfo")) {
            Ok(result) => result,
            Err(_) => {
                println!("could not read the file /proc/meminfo");
                std::process::exit(1)
            }
        };

        let lines = memory.split("\n");

        for line in lines {
            if line == "" {
                break;
            }

            let info: Vec<&str> = line.split(":").collect();
            match info[0] {
                "MemTotal" => self.mem_total = size_string(info[1]),
                "MemFree" => self.mem_free = size_string(info[1]),
                "MemAvailable" => self.mem_available = size_string(info[1]),
                "Buffers" => self.mem_buffers = size_string(info[1]),
                "Cached" => self.mem_page_cache = size_string(info[1]),
                "Shmem" => self.mem_shmem = size_string(info[1]),
                "SReclaimable" => self.mem_slab_reclaimable = size_string(info[1]),
                "SwapTotal" => self.swap_total = size_string(info[1]),
                "SwapFree" => self.swap_free = size_string(info[1]),
                _ => {}
            }
        }
    }
}

fn size_string(str: &str) -> u64 {
    let string: Vec<&str> = str.trim().split(" ").collect();
    string[0].parse::<u64>().unwrap()
}
