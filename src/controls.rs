use std::io::Error;

use rppal::gpio::{Gpio, InputPin, OutputPin};

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

pub enum Button {
    A,
    B,
    X,
    Y,
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
}

pub struct LEDs {
    a: OutputPin,
    b: OutputPin,
    x: OutputPin,
    y: OutputPin,
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
    pub fn turn_on_led(&mut self, led: char) -> SwiftBotError<()> {
        match led {
            'a' => self.a.set_high(),
            'b' => self.b.set_high(),
            'x' => self.x.set_high(),
            'y' => self.y.set_high(),
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

    pub fn turn_off_led(&mut self, led: char) -> SwiftBotError<()> {
        match led {
            'a' => self.a.set_low(),
            'b' => self.b.set_low(),
            'x' => self.x.set_low(),
            'y' => self.y.set_low(),
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
            correction_r: 0.0,
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

    pub fn set_motor_left(&mut self, speed: f64) -> SwiftBotError<()> {
        if 100. >= speed && speed >= 0. {
            self.left_motor_p
                .set_pwm_frequency(100., speed.abs() as f64 / 100.)?;
            self.left_motor_n.set_pwm_frequency(100., 0.)?
        } else if -100. <= speed && speed <= 0. {
            self.left_motor_n
                .set_pwm_frequency(100., speed.abs() as f64 / 100.)?;
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
                .set_pwm_frequency(100., speed.abs() - self.correction_r as f64 / 100.)?;
            self.right_motor_n.set_pwm_frequency(100., 0.)?
        } else if -100. <= speed && speed <= 0. {
            self.right_motor_n
                .set_pwm_frequency(100., speed.abs() - self.correction_l as f64 / 100.)?;
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
