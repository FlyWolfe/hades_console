use std::error::Error;
use std::thread;
use std::time::Duration;
use rppal::gpio::{Gpio, OutputPin};
use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_MOTOR: u8 = 23;

pub struct TreatDispenser {
    motor_gpio: u8,
    motor_pin: OutputPin,
}

impl TreatDispenser {
    pub async fn new() -> Result<TreatDispenser, Box<dyn Error>> {
        println!("Connecting Motor on a {}.", DeviceInfo::new()?.model());
        let motor_pin = Gpio::new()?.get(GPIO_MOTOR)?.into_output();

        Ok(
            TreatDispenser {
                motor_gpio: GPIO_MOTOR,
                motor_pin: motor_pin,
            }
        )
    }

    pub async fn reward(&mut self) {
        // Reward via motor pin
        self.motor_pin.set_high();
        thread::sleep(Duration::from_millis(100));
        self.motor_pin.set_low();
    }
}
