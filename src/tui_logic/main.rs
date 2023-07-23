use super::crossterm::run;

use std::{error::Error, time::Duration};


pub fn main() -> Result<(), Box<dyn Error>> {
    let tick_rate = Duration::from_millis(1);
    run(tick_rate, true)?;
    Ok(())
}
