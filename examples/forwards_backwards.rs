use std::{thread::sleep, time::Duration};

use swiftbot::controls::Motor;
use swiftbot::SwiftBot;

fn main() {
    let mut swiftbot = SwiftBot::init().unwrap();
    swiftbot.controls.wheels.enable_wheels(true);
    for _ in 0..10 {
        //move forward
        swiftbot
            .controls
            .wheels
            .set_motor_speed(Motor::Left, 50.)
            .unwrap();
        swiftbot
            .controls
            .wheels
            .set_motor_speed(Motor::Right, 50.)
            .unwrap();
        sleep(Duration::from_secs(3));

        // move backwards

        swiftbot
            .controls
            .wheels
            .set_motor_speed(Motor::Left, -50.)
            .unwrap();
        swiftbot
            .controls
            .wheels
            .set_motor_speed(Motor::Right, -50.)
            .unwrap();

        sleep(Duration::from_secs(3));
    }
    swiftbot.cleanup().unwrap();
}
