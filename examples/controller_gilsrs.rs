use gilrs::{ev::Code, Button, Event, EventType, Gilrs};
use swiftbot::SwiftBot;

#[tokio::main]
async fn main() {

    let swiftbot =  SwiftBot::init("/dev/video0".to_string()).unwrap();
    let ble = swiftbot::net::ble::Bluetooth::init().await;

    let _inst = ble.create_instance("Xbox Wireless Controller".to_string()).await.unwrap();

    let mut gilrs = Gilrs::new().unwrap(); 

    for (_id, gamepad) in gilrs.gamepads() {
        println!("Connected to these current Controller: Name: {}, Battery: {}", gamepad.name(), gamepad.power_info())
    }

    //simple poll of controller events

   while let Some(event) = gilrs.next_event() {
        match event {
            EventType::ButtonPressed(button, code) => button_pressed(),
            EventType::ButtonReleased(button, code ) => {}
            EventType::Connected => println!("new controller connected"),
            EventType::Disconnected => println!("Controller disconnected"),
            _ => {} //ignore event
        }
   }

}

fn button_pressed(button: Button, code: Code) {
    match button {
        Button::DPadUp => println!("dpup pressed"),
        Button::DPadLeft => println!("dpleft pressed"),
        Button::DPadRight => println!("dpright pressed"),
        Button::DPadDown => println!("dpdown pressed"),

        _ => {} //ignore event
    }
}

fn button_released(button: Button, code: Code) {

}