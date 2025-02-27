use std::thread::sleep;
use std::time::Duration;

use swiftbot::sn3218a::{Sn3218a, UnderLights};

fn main() {
    let sn3218a = Sn3218a::init();

    sn3218a.enable();

    sn3218a.enable_some_led(0b111111111111111111);

    let mut red: u8 = 255;
    let mut blue: u8 = 0;
    let mut green: u8 = 0;
    let mut red_time = true;
    let mut green_time = false;
    let mut blue_time = false;

    loop {
        UnderLights::turn_on_all_underlight(&sn3218a, red, green, blue);
        if red == 255 {
            green_time = true;
            red_time = false;
        }
        if green == 255 {
            blue_time = true;
            green_time = false;
        }
        if blue == 255 {
            red_time = true;
            blue_time = false;
        }

        if red_time {
            red += 1
        } else if !red_time {
            red -= 1;
        }

        if green_time {
            green += 1
        } else if !green_time {
            green -= 1;
        }

        if blue_time {
            blue += 1
        } else if !blue_time {
            blue -= 1;
        }

        sleep(Duration::from_millis(20));
    }
}
