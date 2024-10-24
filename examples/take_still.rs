use std::io::{stdin, stdout, Write};

use swiftbot::SwiftBot;

fn main() {

    let mut buf = String::new();
    println!("path of cam?");
    stdout().flush().unwrap();
    stdin().read_line(&mut buf).unwrap();
    let mut swiftbot =  SwiftBot::init(buf)
    .unwrap();

    swiftbot.camera.still("./still.png").unwrap();

    swiftbot.cleanup().unwrap();
}