use btleplug::api::CentralEvent;
use btleplug::api::{
    bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;

pub struct Bluetooth {
    adapter: Adapter,
}

pub struct BleInstance {
    peripheral: Peripheral,
}

impl Bluetooth {
    pub async fn init() -> Self {
        let manager = Manager::new().await.unwrap();

        let adapter = manager.adapters().await.unwrap();
        //just pick the first adapter
        let controller = adapter.iter().nth(0).expect("no ble");

        Self {
            adapter: controller.clone(),
        }
    }

    pub async fn scan(&self) {
        let mut events = self.adapter.events().await.unwrap();
        self.adapter
            .start_scan(ScanFilter::default())
            .await
            .unwrap();

        while let Some(event) = events.next().await {
            match event {
                CentralEvent::DeviceDiscovered(id) => {
                    let dev = self.adapter.peripheral(&id).await.unwrap();
                    let prop = dev
                        .properties()
                        .await
                        .unwrap()
                        .expect("couldnt get device peritherals");

                    println!("DeviceDiscovered: {:?} properties: {:?}", dev, prop);
                }
                _ => {}
            }
        }
    }

    pub async fn create_instance(&self, devname: String) -> Option<BleInstance> {
        let mut events = self.adapter.events().await.unwrap();
        self.adapter
            .start_scan(ScanFilter::default())
            .await
            .unwrap();
        while let Some(event) = events.next().await {
            match event {
                CentralEvent::DeviceDiscovered(id) => {
                    let p = self.adapter.peripheral(&id).await.unwrap();
                    println!("found Peripheral: {:?}", p);
                    if let Some(name) = p.properties().await.unwrap().unwrap().local_name {
                        println!("Found device: {}", name);
                        if name.trim() == devname.trim() {
                            return Some(BleInstance { peripheral: p });
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }
}

impl BleInstance {
    pub async fn connect(&self) {
        self.peripheral.connect().await.expect("failed to connect")
    }

    pub fn write(&self) {}
    pub fn receive() {}
}
