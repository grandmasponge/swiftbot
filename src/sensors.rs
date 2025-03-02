use std::{
    error::Error,
    future::Future,
    task::Poll,
    thread::sleep,
    time::{Duration, Instant},
};

use rppal::gpio::{Gpio, InputPin, OutputPin};

const ULTRA_TRIG_PIN: u8 = 0x0D;
const ULTRA_ECHO_PIN: u8 = 0x19;

pub type SensorResult<T> = Result<T, Box<dyn Error>>;

pub enum SensorStatus {
    Idle,
    SentSig,
    RecivedSig,
}

pub struct Sensors {
    state: SensorStatus,
    trig_pin: OutputPin,
    echo_pin: InputPin,
    time_started: Instant,
    time_ended: Instant,
}

impl Future for Sensors {
    type Output = SensorResult<f32>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match self.state {
            SensorStatus::Idle => {
                self.trig_pin.set_high();
                sleep(Duration::from_micros(10));
                self.trig_pin.set_low();
                self.state = SensorStatus::SentSig;

                Poll::Pending
            }
            SensorStatus::SentSig => {
                if self.echo_pin.is_high() {
                    self.time_started = Instant::now();
                    self.state = SensorStatus::RecivedSig;
                }
                Poll::Pending
            }
            SensorStatus::RecivedSig => {
                if self.echo_pin.is_low() {
                    self.time_ended = Instant::now();
                    let duration: f32 = self
                        .time_ended
                        .duration_since(self.time_started)
                        .as_micros() as f32;

                    let distance = (duration * 0.034) / 2.;

                    self.state = SensorStatus::Idle; // Reset state for next measurement
                    return Poll::Ready(Ok(distance));
                }
                Poll::Pending
            }
        }
    }
}

impl Sensors {
    pub fn setup_gpio(gpio: Gpio) -> SensorResult<Self> {
        Ok(Self {
            time_started: Instant::now(),
            time_ended: Instant::now(),
            state: SensorStatus::Idle,
            trig_pin: gpio.get(ULTRA_TRIG_PIN)?.into_output(),
            echo_pin: gpio.get(ULTRA_ECHO_PIN)?.into_input(),
        })
    }

    pub fn scan_distance(&mut self) {
        self.state = SensorStatus::Idle; // Reset state to start a new measurement
    }
}

pub async fn get_distance(sensors: &mut Sensors) -> SensorResult<f32> {
    // Start the measurement
    sensors.scan_distance();
    
    // Await the future to get the result
    let distance = sensors.await?;
    
    Ok(distance)
}