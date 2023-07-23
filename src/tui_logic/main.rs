use super::crossterm::run;

use argh::FromArgs;
use std::{error::Error, time::Duration};

/// Demo
//#[derive(Debug, FromArgs)]
//struct Cli {
//    /// time in ms between two ticks.
//    #[argh(option, default = "250")]
//    tick_rate: u64,
//    /// whether unicode symbols are used to improve the overall look of the app
//    #[argh(option, default = "true")]
//    enhanced_graphics: bool,
//}

pub fn main() -> Result<(), Box<dyn Error>> {
    //let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(200 /* cli.tick_rate */);
    run(tick_rate, false /* cli.enhanced_graphics */)?;
    Ok(())
}
