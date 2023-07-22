//#[cfg(any(target_os = "linux"))]



fn main() {
	
	let machine_kind = if cfg!(target_os = "linux") {
		"linux"
	} else if cfg!(target_os = "windows") {
		"windows"
	} else if cfg!(target_os = "macos") {
		"macos"
	} else {
		"unknown"
	};
	
	println!("machine type  : {}", machine_kind);
	
}