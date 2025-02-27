use std::{default, io::ErrorKind, path::PathBuf, thread::sleep, time::{Duration, Instant}, vec};
use rand::{rngs::ThreadRng, Rng};
use rppal::gpio::Mode;
use swiftbot::{camera::SwiftBotCamera, controls::{Button, Controls}, sensors::{get_distance, SensorResult, Sensors}, sn3218a::{Sn3218a, UnderLights}, SwiftBot};


type DetectResult = (bool, f32);

enum Modes {
    None,
    Scardey,
    Curious,
    Dubious
}


struct State {
    mode: Modes,
    time_started: Instant,
    running: bool
}

impl Default for State {
    fn default() -> Self {
        Self {
            mode: Modes::None,
            time_started: Instant::now(),
            running: true
        }
    }
}

trait Reactions {
    fn wander(&self, controls: &mut Controls, led_controller: &mut Sn3218a);
    async fn detect(&self, sensor: &mut Sensors) -> DetectResult;
}

impl Reactions for Modes {
    fn wander(&self, controls: &mut Controls, led_controller: &mut Sn3218a) {
        match self {
            Modes::Scardey => {
                
            },
            Modes::Curious => {
                UnderLights::turn_on_all_underlight(led_controller, 0, 255, 0);
                let res = controls.wheels.move_forward(40.);
                match res {
                    Ok(a) => res.unwrap(),
                    Err(_) => panic!("wheels arent working"),
                }
            },
            _ => unreachable!()
        }
    }


    async fn detect(&self, sensor: &mut Sensors) -> DetectResult {
        let dist = get_distance(sensor).await
        .unwrap()
        .round();
        if dist <= 50. {
            return (true, dist);
        }
        (false, dist)
    }

}


pub fn generate_uuid(rand: &mut ThreadRng) -> String {
    let mut temp = vec![];
    (0..16)
    .into_iter()
    .map(|_x| {
        let rand_num  = rand.random_range(0..256);

        temp.push(rand_num.to_string());
    });
    temp.push(".png".to_string());
    temp.join("")
}


#[tokio::main]
async fn main() -> Result<(), ErrorKind> {
    let mut swiftbot = SwiftBot::init(None).unwrap();
    let mut state = State::default();
    let mut rng: ThreadRng = rand::rng();
    let mut file_paths: Vec<String> = vec![];
    //for this we need three parts of the swiftbot the swiftbot
    // camera, wheels, sensor
    //Wheels
    let mut controls = swiftbot.controls;
    //turn on wheels
    controls.wheels.enable_wheels(true);
    let mut led_controller = Sn3218a::init();
    //Sensor
    let mut sensor = swiftbot.sensors;
    //Camera 
    let mut camera = SwiftBotCamera::new();
    //decode qr code and match it to our modes
    println!("Show your QR code to the camera");
    let mode = camera.decode_qr().await;
    
    state.mode = match mode.as_str() {
        "curious" => Modes::Curious,
        "scardey" => Modes::Scardey,
        "dubious" => {
            let rand_mode = if (rng.random::<f32>() * 100.) % 2. == 0. {
                Modes::Curious
            } else {
                Modes::Scardey
            };
            rand_mode
        },
        _ => panic!("we dont care for now")
    };

    while state.running {
        //wander
        state.mode.wander(&mut controls, &mut led_controller);
        //detect
        let is_there = state.mode.detect(&mut sensor).await;
        let dist = is_there.1;

        if is_there.0 {
            //logic related to swiftbot being there for each mode
            match state.mode {
                Modes::Scardey => {
                    UnderLights::turn_on_all_underlight(&led_controller, 200, 0, 0);
                    let path = generate_uuid(&mut rng.clone());
                    camera.save_pic(path.clone()).await;
                    file_paths.push(path);
                    controls.wheels.move_left()
                    .unwrap();
                    sleep(Duration::from_millis(1000));
                    //TODO FIX REPEAT
                    controls.wheels.stop().unwrap();
                    controls.wheels.move_forward(100.).unwrap();
                    sleep(Duration::from_millis(5000));
                    controls.wheels.stop().unwrap();
                },
                Modes::Curious => {
                    //TODO IMPLIMENTATION
                    todo!()
                },
               _=> unreachable!()
            }
        }

       if controls.buttons.block_button_duration(Duration::from_millis(100)).button == Button::X {
            state.running = false
       }
    }

    println!("Do you want to see the Execution Log\n press Y for yes and anything else for no");
    if controls.buttons.block_button().button == Button::Y {
        print_execution_log();
    }


    
    Ok(())
}

fn print_execution_log() {
    //TODO PRINT EXECUTION
}