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
    let mut dispenser = if dispenser_active {Some(TreatDispenser::new().await?)} else {None};

    //let mut boxes_minigame = minigames::boxes::BoxesMinigame::new();
    let mut advanced_boxes_minigame = minigames::advanced_boxes::AdvancedBoxesMinigame::new();

    loop {
        //boxes_minigame.run(&mut dispenser).await;
        advanced_boxes_minigame.run(&mut dispenser).await;
    }
}
