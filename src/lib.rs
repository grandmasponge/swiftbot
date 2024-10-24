use std::{error::Error, path::PathBuf};

mod api;
pub mod camera;
pub mod controls;
pub mod sensors;
pub mod net;

use camera::Camera;
use controls::Controls;
use rppal::gpio::Gpio;
use sensors::Sensors;

type SwiftBotError<T> = Result<T, Box<dyn Error>>;

pub struct SwiftBot<'a> {
    gpio: Gpio,
    pub controls: Controls,
    pub camera: Camera<'a>,
    pub sensors: Sensors,
}

impl SwiftBot<'_> {
    pub fn init(cam_path: String) -> SwiftBotError<Self> {
        let gpio = Gpio::new()?;
        let controls = Controls::init(gpio.clone())?;
        let sensors = Sensors::setup_gpio(gpio.clone())?;
        let camera = Camera::setup(Some(PathBuf::from(cam_path))).expect("Camera is brokie");

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
