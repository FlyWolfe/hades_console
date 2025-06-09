use macroquad::{color::*, shapes::*, window::*};
use rand::{rng, Rng};


use crate::{input, treatdispenser};
use treatdispenser::TreatDispenser;

pub async fn run(mut dispenser: Option<TreatDispenser>) {
    const NUM_LOCATIONS: i32 = 2;
    let mut rng = rng();
    let mut current_location: i32 = rng.random_range(1..=NUM_LOCATIONS);

    loop {
        clear_background(DARKGRAY);
        draw_rectangle(
            screen_width() * (current_location as f32 - 1.0) / NUM_LOCATIONS as f32,
            0.0,
            screen_width() / NUM_LOCATIONS as f32,
            screen_height(),
            YELLOW,
        );

        if input::get_input_pos().is_some() {
            let input_x = input::get_input_pos().unwrap().x;

            if input_x > screen_width() * (current_location as f32 - 1.0) / NUM_LOCATIONS as f32
                && input_x < screen_width() * current_location as f32 / NUM_LOCATIONS as f32
            {
                let prev_location = current_location;
                while NUM_LOCATIONS != 1 && current_location == prev_location {
                    current_location = rng.random_range(1..=NUM_LOCATIONS);
                }
                
                if let Some(ref mut disp) = dispenser {disp.reward().await;}

                // TODO: Keep track of dog's skill level in persistent storage and reward based on skill
                // TODO: Add option to switch between dog profiles and to manually adjust skill level in-game
            }
        }
        next_frame().await;
    }
}
