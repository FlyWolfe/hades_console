use macroquad::{color::*, math::Vec2, shapes::*, window::*};
use rand::{rng, rngs::ThreadRng, Rng};


use crate::{input, treatdispenser};
use treatdispenser::TreatDispenser;

const BOX_SIZE: f32 = 100.;

pub struct AdvancedBoxesMinigame {
    rng: ThreadRng,
    box_location: Vec2,
}

impl AdvancedBoxesMinigame {
    pub fn new() -> AdvancedBoxesMinigame {
        let mut rng = rng();
        let box_location = Vec2::new(rng.random_range(0.0..=screen_width() - BOX_SIZE), rng.random_range(0.0..=screen_height() - BOX_SIZE));

        AdvancedBoxesMinigame {
            rng,
            box_location
        }
    }

    pub async fn run(&mut self, dispenser: &mut Option<TreatDispenser>) {
        clear_background(DARKGRAY);
        draw_rectangle(
            self.box_location.x,
            self.box_location.y,
            BOX_SIZE,
            BOX_SIZE,
            YELLOW,
        );

        if input::get_input_pos().is_some() {
            let input = input::get_input_pos().unwrap();

            if Self::check_box_touched(input, self.box_location, BOX_SIZE) {
                self.box_location = Vec2::new(self.rng.random_range(BOX_SIZE..=screen_width() - BOX_SIZE), self.rng.random_range(BOX_SIZE..=screen_height() - BOX_SIZE));
                
                if let Some(ref mut disp) = dispenser {disp.reward().await;}

                // TODO: Keep track of dog's skill level in persistent storage and reward based on skill
                // TODO: Add option to switch between dog profiles and to manually adjust skill level in-game
            }
        }
        next_frame().await;
    }

    fn check_box_touched(touch_position: Vec2, box_pos: Vec2, box_size: f32) -> bool {
        if touch_position.x > box_pos.x &&
            touch_position.x < box_pos.x + box_size &&
            touch_position.y > box_pos.y &&
            touch_position.y < box_pos.y + box_size
        {
            return true;
        }

        false
    }
}
