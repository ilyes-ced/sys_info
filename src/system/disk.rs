use std::{ffi::OsString, path::PathBuf};


pub enum DiskType {
    HDD,
    SSD,
    NVME,
    PCIe,
    Unknown,
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
