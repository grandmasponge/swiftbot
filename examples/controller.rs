use core::panic;
use futures::StreamExt;
use gilrs::GilrsBuilder;
use gilrs::{
    ev::{filter::Repeat, Code},
    Button, Event, EventType, Filter, GamepadId, Gilrs,
};
use std::{thread::sleep, time::Duration};
use swiftbot::SwiftBot;

use btleplug::api::CentralEvent;
use btleplug::api::{
    bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};

struct SwiftVelocity {
    speed_l: f64,
    speed_r: f64,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut swiftbot = SwiftBot::init("/dev/video0".to_string()).unwrap();
    swiftbot.controls.wheels.enable_wheels(true);

    let manager = Manager::new().await.unwrap();

    let adapter = manager.adapters().await.unwrap();
    //just pick the first adapter
    let controller = adapter.iter().nth(0).expect("no ble");
    controller.start_scan(ScanFilter::default()).await.unwrap();

    let mut events = controller.events().await.unwrap();

    let p = loop {
        if let Some(perp) = events.next().await {
            match perp {
                CentralEvent::DeviceDiscovered(id) => {
                    let peripheral = controller.peripheral(&id).await.unwrap();
                    if let Some(name) = peripheral.properties().await.unwrap().unwrap().local_name {
                        if name.trim() == "Xbox Wireless Controller" {
                            break Some(peripheral); // Return the peripheral
                        }
                    }
                }
                _ => {}
            }
        } else {
            break None; // End of events, no matching device found
        }
    };

    let p = p.unwrap();
    p.connect().await.unwrap();

    println!("connecting controller to gilrs");

    let mut gilrs = match GilrsBuilder::new().set_update_state(false).build() {
        Ok(g) => g,
        Err(gilrs::Error::NotImplemented(g)) => {
            eprintln!("Current platform is not supported");
            panic!("im panacing");
        }
        Err(e) => {
            eprintln!("Failed to create gilrs context: {}", e);
            panic!("im panacing");
        }
    };

    let filter = Repeat::new();

    let mut vel = 50.;

    //simple poll of controller events
    loop {
        println!("oh god im looping");
        while let Some(event) = gilrs
            .next_event_blocking(None)
            .filter_ev(&filter, &mut gilrs)
        {
            match event.event {
                EventType::ButtonChanged(button, eh, code) => {}
                EventType::ButtonPressed(button, code) => {
                    button_pressed(&mut swiftbot, vel, button, code);
                }
                EventType::ButtonReleased(button, code) => {}
                EventType::Connected => println!("new controller connected"),
                EventType::Disconnected => println!("Controller disconnected"),
                _ => {
                    println!("{event:?}")
                } //ignore event
            }
        }
    }
}

fn button_pressed(bot: &mut SwiftBot, vel: f64, button: Button, code: Code) {
    match button {
        Button::DPadUp => {
            bot.controls.wheels.move_forward(100.).expect("nahh cant");
        }
        Button::DPadLeft => {
            bot.controls.wheels.set_motor_left(25.).unwrap();
            bot.controls.wheels.set_motor_right(100.).unwrap();
        }
        Button::DPadRight => {
            bot.controls.wheels.set_motor_left(100.).unwrap();
            bot.controls.wheels.set_motor_right(25.).unwrap();
        }
        Button::DPadDown => {
            bot.controls.wheels.move_forward(-100.).unwrap();
        }
        Button::East => {
            bot.controls.wheels.stop().unwrap();
        }

        _ => {} //ignore event
    }
}
