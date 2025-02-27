use swiftbot::net::ble::Bluetooth;

#[tokio::main]
async fn main() {
    println!("Bluetooth scan");
    let ble = Bluetooth::init().await;
    ble.scan().await;
}
