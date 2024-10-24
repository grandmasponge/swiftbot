use btleplug::api::{bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};

pub struct Bluetooth {
    adapter: Adapter
}

pub struct BleInstance {
    peripheral: Peripheral
}

impl Bluetooth {
    pub async fn init() -> Self {
        let manager = Manager::new()
        .await
        .unwrap();

        let adapter = manager.adapters().await.unwrap();
        //just pick the first adapter
        let controller = adapter.get(0).unwrap().clone();
        
        Self {
            adapter
        }
    }

    pub async fn create_instance(&self, devname: String) -> Option<BleInstance> {
        self.adapter.start_scan(ScanFilter::default()).await.unwrap();
        for devices in self.adapter.peripherals().await.unwrap() {
            if devices.properties()
            .await
            .unwrap() 
            .unwrap()
            .local_name.unwrap() == devname {
                let conn = devices.connect().await.unwrap();
                return Some(BleInstance {
                    peripheral
                })
            }
        }
        None
    }
}


impl BleInstance {
    pub fn write() {

    }
    pub fn receive() {
        
    }
}