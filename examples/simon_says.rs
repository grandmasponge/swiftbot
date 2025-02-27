use std::error::Error;
use std::{thread::sleep, time::Duration};

use rand::Rng;
use swiftbot::{controls::Button, SwiftBot};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let mut score: usize = 0;
    let mut swiftbot = SwiftBot::init(None)?;
    let mut leds = &mut swiftbot.controls.leds;
    leds.turn_off_all();
    let mut buttons = &mut swiftbot.controls.buttons;

    loop {
        let light = rng.gen_range(0..4);
        println!("random_num  = {light}");
        let mut correct = Button::None;
        match light {
            0 => {
                correct = Button::A;
                leds.a.set_high();
            }
            1 => {
                correct = Button::B;
                leds.b.set_high();
            }
            2 => {
                correct = Button::X;
                leds.x.set_high();
            }
            3 => {
                correct = Button::Y;
                leds.y.set_high();
            }
            _ => panic!("not supposed to happen"),
        }
        println!("button to be pressed  = {correct:?}");
        //check for button presses
        let event = buttons.block_button_duration(Duration::from_secs(1));
        if &event.button == &Button::None {
            println!("failed to press button in time");
            break;
        }
        sleep(Duration::from_millis(500));

        let pressed = event.button;
        if correct == pressed {
            println!("Score: {score} + 1");
            score += 1;
            //turn off led
            leds.turn_off_led(pressed).unwrap();
        } else {
            break;
        }
    }

    println!("game ended Score: {score}");

    swiftbot.cleanup();

    Ok(())
}
