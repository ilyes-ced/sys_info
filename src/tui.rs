mod system;
mod tui_logic;

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

    // todo: match
    //let result = system::memory::get_memory_data().unwrap();
    //println!("{:#?}", result);



    // memory
    //let mut mem = system::memory::new();
    //println!("{:#?}", mem);
    //mem.get_memory_data();
    //println!("{:#?}", mem);



    // users
    //let mut users = system::user::get_users();

    
    // disks
    let mut disks = system::disk::get_disks();

}