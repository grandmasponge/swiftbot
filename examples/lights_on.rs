use std::{thread::sleep, time::Duration};

use swiftbot::SwiftBot;

fn main() {
    let mut swift_bot = SwiftBot::init().unwrap();
    for _ in 0..10 {
        swift_bot.controls.leds.turn_on_all().unwrap();

        sleep(Duration::from_secs(3));

        swift_bot.controls.leds.turn_off_all().unwrap()
    }
    swift_bot.cleanup().unwrap();
}
