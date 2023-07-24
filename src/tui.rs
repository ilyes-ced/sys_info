mod tui_logic;
mod utils;


#[cfg(windows)]
mod windows {
    // include Windows-specific file here
}
#[cfg(unix)]
mod system::linux;


fn main() {
    //let output = Command::new("neofetch").output();
    //match output {
    //    Ok(result) => {
    //        println!("{}", String::from_utf8_lossy(&result.stdout));
    //    }
    //    Err(_) => {
    //        println!("neofetch is required for this");
    //    }
    //}

    //let gg = tui_logic::main::main().unwrap();





        	
    if cfg!(target_os = "linux") {
        mod system;
	} else if cfg!(target_os = "windows") {
		mod system::linux;
	} else if cfg!(target_os = "macos") {
		mod system::linux;
	} else {
		mod system::linux;
	}





}
