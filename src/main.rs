use std::error::Error;
use std::env;
use macroquad::{color::*, shapes::*, window::*};
use rand::{rng, Rng};

use treatdispenser::TreatDispenser;

mod input;
mod minigames;
mod treatdispenser;

#[macroquad::main("Game")]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let dispenser_active = !args.contains(&"no_dispenser".to_string());
    let dispenser = if dispenser_active {Some(TreatDispenser::new().await?)} else {None};

    //minigames::boxes::run(dispenser).await;
    minigames::advanced_boxes::run(dispenser).await;

    Ok(())
}
