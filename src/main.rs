use macroquad::{color::*, input::TouchPhase, input::*, shapes::*, window::*};

use rand::{thread_rng, Rng};

#[macroquad::main("Game")]
async fn main() {
    let mut input_x: f32 = -999.0;
    const NUM_LOCATIONS: i32 = 3;
    let mut rng = thread_rng();
    let mut current_location: i32 = rng.gen_range(1..=NUM_LOCATIONS);

    loop {
        clear_background(DARKGRAY);
        draw_rectangle(
            screen_width() * (current_location as f32 - 1.0) / NUM_LOCATIONS as f32,
            0.0,
            screen_width() / NUM_LOCATIONS as f32,
            screen_height(),
            YELLOW,
        );

        for touch in touches() {
            if touch.phase == TouchPhase::Started {
                input_x = touch.position.x;
            }
        }

        let (mouse_x, _) = mouse_position();
        if is_mouse_button_pressed(MouseButton::Left) {
            input_x = mouse_x;
        }

        if input_x > screen_width() * (current_location as f32 - 1.0) / NUM_LOCATIONS as f32
            && input_x < screen_width() * current_location as f32 / NUM_LOCATIONS as f32
        {
            let prev_location = current_location;
            while current_location == prev_location {
                current_location = rng.gen_range(1..=NUM_LOCATIONS);
            }
            // TODO: Reward dog via serial commands
            // TODO: Keep track of dog's skill level in persistent storage and reward based on skill
            // TODO: Add option to switch between dog profiles and to manually adjust skill level in-game
        }

        input_x = -999.0;
        next_frame().await;
    }
}
