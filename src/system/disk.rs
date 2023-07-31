use super::utils::read_file;
use std::path::Path;
use std::{ffi::OsString, path::PathBuf};

pub enum DiskType {
    HDD,
    SSD,
    UNKNOWN,
}

pub struct Disk {
    type_: DiskType,
    device_name: OsString,
    file_system: Vec<u8>,
    mount_point: PathBuf,
    total_space: u64,
    available_space: u64,
    is_removable: bool,
}

pub fn get_disks() -> Result<Vec<Disk>, ()> {
    let _gg = DiskType::HDD;
    let _gg = DiskType::SSD;
    let _gg = DiskType::UNKNOWN;

    //let dirs = match read_dir("/dev/disk/by-id/"){
    //    Ok(dirs) => dirs,
    //    Err(_) => return Err(())
    //};
    //println!("{:#?}", dirs);

    let disks = match read_file(Path::new("/proc/mounts")) {
        Ok(result) => result,
        Err(_) => {
            println!("could not read the file /proc/mounts");
            std::process::exit(1)
        }
    };

    let lines = disks.split("\n");

    for line in lines {
        if line == "" {
            break;
        }
        println!("{}", line);
    }

    Ok(Vec::new())
}
