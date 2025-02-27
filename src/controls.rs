use std::{
    io::Error,
    thread::sleep,
    time::{Duration, Instant},
};

use btleplug::api::BDAddr;
use rppal::gpio::{Gpio, InputPin, OutputPin};
use tokio::sync::mpsc::Receiver;

use crate::SwiftBotError;

//Buttons
const A_BUTTON_PIN: u8 = 0x5;
const B_BUTTON_PIN: u8 = 0x6;
const X_BUTTON_PIN: u8 = 0x10;
const Y_BUTTON_PIN: u8 = 0x18;
//Leds
const A_BUTTON_LED: u8 = 0x17;
const B_BUTTON_LED: u8 = 0x16;
const X_BUTTON_LED: u8 = 0x11;
const Y_BUTTON_LED: u8 = 0x1B;
//Wheels
const ENABLE_WHEELS: u8 = 0x1a;
const MOTOR_LEFT_P: u8 = 0x08;
const MOTOR_LEFT_N: u8 = 0x0B;
const MOTOR_RIGHT_P: u8 = 0x0A;
const MOTOR_RIGHT_N: u8 = 0x09;

pub struct Buttons {
    a: InputPin,
    b: InputPin,
    x: InputPin,
    y: InputPin,
}
#[derive(PartialEq, Debug)]
pub enum Button {
    A,
    B,
    X,
    Y,
    None,
}

pub struct ButtonStream {
    pub rx: tokio::sync::mpsc::Receiver<Event>,
    pub tx: tokio::sync::mpsc::Sender<Event>,
    buttons: Buttons,
}

impl ButtonStream {
    pub fn new(buttons: Buttons) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(10);
        Self {
            tx,
            rx,
            buttons
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Event {
    pub button: Button,
}

impl Buttons {
    fn setup_gpio(gpio: &Gpio) -> SwiftBotError<Self> {
        Ok(Self {
            a: gpio.get(A_BUTTON_PIN)?.into_input(),
            b: gpio.get(B_BUTTON_PIN)?.into_input(),
            x: gpio.get(X_BUTTON_PIN)?.into_input(),
            y: gpio.get(Y_BUTTON_PIN)?.into_input(),
        })
    }

    pub fn block_button_duration(&self, duration: Duration) -> Event {
        let now = Instant::now();
        let mut end = Instant::now();
        loop {
            if self.a.is_low() {
                loop {
                    if self.a.is_high() {
                        break;
                    };
                }
                return Event { button: Button::A };
            }
            if self.b.is_low() {
                loop {
                    if self.b.is_high() {
                        break;
                    };
                }
                return Event { button: Button::B };
            }
            if self.x.is_low() {
                loop {
                    if self.x.is_high() {
                        break;
                    };
                }
                return Event { button: Button::X };
            }
            if self.y.is_low() {
                loop {
                    if self.y.is_high() {
                        break;
                    };
                }
                return Event { button: Button::Y };
            }

            end = Instant::now();
            let dur = end.duration_since(now);
            if duration < dur {
                return Event {
                    button: Button::None,
                };
            }
        }
    }

    pub fn block_button(&self) -> Event {
        loop {
            if self.a.is_low() {
                loop {
                    if self.a.is_high() {
                        break;
                    };
                }
                return Event { button: Button::A };
            }
            if self.b.is_low() {
                loop {
                    if self.b.is_high() {
                        break;
                    };
                }
                return Event { button: Button::B };
            }
            if self.x.is_low() {
                loop {
                    if self.x.is_high() {
                        break;
                    };
                }
                return Event { button: Button::X };
            }
            if self.y.is_low() {
                loop {
                    if self.y.is_high() {
                        break;
                    };
                }
                return Event { button: Button::Y };
            }
        }
    }
}

pub struct LEDs {
    pub a: OutputPin,
    pub b: OutputPin,
    pub x: OutputPin,
    pub y: OutputPin,
}

impl LEDs {
    fn setup_gpio(gpio: &Gpio) -> SwiftBotError<Self> {
        Ok(Self {
            a: gpio.get(A_BUTTON_LED)?.into_output(),
            b: gpio.get(B_BUTTON_LED)?.into_output(),
            x: gpio.get(X_BUTTON_LED)?.into_output(),
            y: gpio.get(Y_BUTTON_LED)?.into_output(),
        })
    }
    pub fn turn_on_led(&mut self, led: Button) -> SwiftBotError<()> {
        match led {
            Button::A => self.a.set_high(),
            Button::B => self.b.set_high(),
            Button::X => self.x.set_high(),
            Button::Y => self.y.set_high(),
            _ => {
                // while i havnt implimented my own proper errors i will just panic
                panic!()
            }
        }
        Ok(())
    }

    pub fn turn_on_all(&mut self) -> SwiftBotError<()> {
        self.a.set_high();
        self.b.set_high();
        self.x.set_high();
        self.y.set_high();
        Ok(())
    }

    pub fn turn_off_all(&mut self) -> SwiftBotError<()> {
        self.a.is_set_low();
        self.b.set_low();
        self.x.set_low();
        self.y.set_low();
        Ok(())
    }

    pub fn turn_off_led(&mut self, led: Button) -> SwiftBotError<()> {
        match led {
            Button::A => self.a.set_low(),
            Button::B => self.b.set_low(),
            Button::X => self.x.set_low(),
            Button::Y => self.y.set_low(),
            _ => {
                // while i havnt implimented my own proper errors i will just panic
                panic!()
            }
        }

        Ok(())
    }
}

pub struct Wheels {
    enable: OutputPin,
    right_motor_p: OutputPin,
    right_motor_n: OutputPin,
    left_motor_p: OutputPin,
    left_motor_n: OutputPin,
    correction_r: f64,
    correction_l: f64,
}

pub enum Motor {
    Left,
    Right,
}

pub enum Direction {
    Halt,
    Forward,
    Backward,
}

impl Wheels {
    fn setup_gpio(gpio: &Gpio) -> SwiftBotError<Self> {
        Ok(Self {
            correction_l: 0.0,
            correction_r: 2.0,
            enable: gpio.get(ENABLE_WHEELS)?.into_output(),
            right_motor_n: gpio.get(MOTOR_RIGHT_N)?.into_output(),
            right_motor_p: gpio.get(MOTOR_RIGHT_P)?.into_output(),

            //there reversed i have no idea why but they are
            left_motor_n: gpio.get(MOTOR_LEFT_P)?.into_output(),
            left_motor_p: gpio.get(MOTOR_LEFT_N)?.into_output(),
        })
    }

    pub fn set_correction(&mut self, motor: Motor, correction: f64) {
        match motor {
            Motor::Right => self.correction_r = correction,
            Motor::Left => self.correction_l = correction,
        }
    }

    pub fn enable_wheels(&mut self, on: bool) {
        if on {
            self.enable.set_high();
        } else {
            self.enable.set_low();
        }
    }

    pub fn set_motor_speed(&mut self, motor: Motor, speed: f64) -> SwiftBotError<()> {
        match motor {
            Motor::Left => self.set_motor_left(speed)?,
            Motor::Right => self.set_motor_right(speed)?,
        }

        Ok(())
    }

    pub fn move_forward(&mut self, speed: f64) -> SwiftBotError<()> {
        self.set_motor_speed(Motor::Left, speed)?;
        self.set_motor_speed(Motor::Right, speed)?;

        Ok(())
    }

    pub fn stop(&mut self) -> SwiftBotError<()> {
        self.set_motor_speed(Motor::Left, 0.)?;
        self.set_motor_speed(Motor::Right, 0.)?;

        Ok(())
    }

    pub fn move_left(&mut self) -> SwiftBotError<()> {
        self.stop()?;
        self.set_motor_speed(Motor::Left, 100.)?;
        self.set_motor_speed(Motor::Right, 50.)?;
        Ok(())
    }

    pub fn move_right(&mut self) -> SwiftBotError<()> {
        self.stop()?;
        self.set_motor_speed(Motor::Left, 50.)?;
        self.set_motor_speed(Motor::Right, 100.)?;
        Ok(())
    }

    pub fn set_motor_left(&mut self, speed: f64) -> SwiftBotError<()> {
        if 100. >= speed && speed >= 0. {
            self.left_motor_p
                .set_pwm_frequency(100., speed.abs() / 100.)?;
            self.left_motor_n.set_pwm_frequency(100., 0.)?
        } else if -100. <= speed && speed <= 0. {
            self.left_motor_n
                .set_pwm_frequency(100., speed.abs() / 100.)?;
            self.left_motor_p.set_pwm_frequency(100., 0.)?;
        } else {
            // error handling will be done later for now set to 1
            panic!("noo")
        }
        Ok(())
    }

    pub fn set_motor_right(&mut self, speed: f64) -> SwiftBotError<()> {
        if 100. >= speed && speed >= 0. {
            self.right_motor_p
                .set_pwm_frequency(100., speed.abs() / 100.)?;
            self.right_motor_n.set_pwm_frequency(100., 0.)?
        } else if -100. <= speed && speed <= 0. {
            self.right_motor_n
                .set_pwm_frequency(100., speed.abs() / 100.)?;
            self.right_motor_p.set_pwm_frequency(100., 0.)?;
        } else {
            // error handling will be done later for now set to 1
            panic!("noo")
        }
        Ok(())
    }
}

pub struct Controls {
    pub wheels: Wheels,
    pub buttons: Buttons,
    pub leds: LEDs,
}

impl Controls {
    pub fn init(gpio: Gpio) -> SwiftBotError<Self> {
        Ok(Self {
            wheels: Wheels::setup_gpio(&gpio)?,
            buttons: Buttons::setup_gpio(&gpio)?,
            leds: LEDs::setup_gpio(&gpio)?,
        })
    }
}
