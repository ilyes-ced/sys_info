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

    let mut m = system::memory::new();
    println!("{:#?}", m);
    m.get_memory_data();
    println!("{:#?}", m);
}
