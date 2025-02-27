use std::{error::Error, path::PathBuf};

mod api;
pub mod camera;
pub mod controls;
pub mod net;
pub mod sensors;
pub mod sn3218a;

use controls::Controls;
use rppal::gpio::Gpio;
use sensors::Sensors;

type SwiftBotError<T> = Result<T, Box<dyn Error>>;

pub struct SwiftBot {
    gpio: Gpio,
    pub controls: Controls,
    pub camera: i32,
    pub sensors: Sensors,
}

impl SwiftBot {
    pub fn init(cam_path: Option<String>) -> SwiftBotError<Self> {
        let gpio = Gpio::new()?;
        let controls = Controls::init(gpio.clone())?;
        let sensors = Sensors::setup_gpio(gpio.clone())?;
        let camera = 10;

        Ok(Self {
            gpio,
            controls,
            camera,
            sensors,
        })
    }

    pub fn cleanup(&mut self) -> SwiftBotError<()> {
        //turn wheels off
        self.controls.wheels.enable_wheels(false);
        //turn of all lights
        self.controls.leds.turn_off_all()?;

        Ok(())
    }
}
