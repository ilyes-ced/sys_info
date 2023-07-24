use std::{collections::HashMap, path::Path};
use super::utils::read_file;



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
    pub fn get_memory_data() -> Result<HashMap<String, String>, ()> {
        let memory = match read_file(Path::new("/proc/meminfo")) {
            Ok(result) => result,
            Err(_) => return Err(()),
        };  
    
        let lines = memory.split("\n");
        let mut mem_info: HashMap<String, String> = HashMap::new();
    
        for line in lines {
            if line == "" {
                break
            }
            let info: Vec<&str> = line.split(":").collect();
            mem_info.insert(
                String::from(info[0].trim()), 
                String::from(info[1].trim())
            );
        }
    
        Ok(mem_info)
    }
}

