use std::{collections::HashMap, path::Path};


pub fn get_memory_data() -> Result<String, ()> {
    let memory = match utils::read_file(Path::new("/proc/meminfo")) {
        Ok(_) => {},
        Err(_) => {},
    };  

    let lines = memory.split("\n");
    let mut mem_info: HashMap<&str, &str> = HashMap::new();

    for line in lines {
        if line == "" {
            break
        }
        let info: Vec<&str> = line.split(":").collect();
        mem_info.insert(info[0].trim(), info[1].trim());
        drop(info)
    }

    Ok(mem_info)
}