use std::error::Error;
use std::thread;
use std::time::Duration;

use macroquad::{color::*, input::TouchPhase, input::*, shapes::*, window::*};

use rand::{thread_rng, Rng};
use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_MOTOR: u8 = 23;

#[macroquad::main("Game")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut input_x: f32 = -999.0;
    const NUM_LOCATIONS: i32 = 2;
    let mut rng = thread_rng();
    let mut current_location: i32 = rng.gen_range(1..=NUM_LOCATIONS);

    println!("Connecting Motor on a {}.", DeviceInfo::new()?.model());
    let mut motor_pin = Gpio::new()?.get(GPIO_MOTOR)?.into_output();

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
            while NUM_LOCATIONS != 1 && current_location == prev_location {
                current_location = rng.gen_range(1..=NUM_LOCATIONS);
            }
            // Reward via motor pin
            motor_pin.set_high();
            thread::sleep(Duration::from_millis(100));
            motor_pin.set_low();

            // TODO: Keep track of dog's skill level in persistent storage and reward based on skill
            // TODO: Add option to switch between dog profiles and to manually adjust skill level in-game
        }

        input_x = -999.0;
        next_frame().await;
    }

    //Ok(())
}
