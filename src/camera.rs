use std::sync::Arc;
use opencv::prelude::*;
use opencv::videoio;
use tokio::sync::Mutex;
use opencv::core::Mat;
use opencv::core::Vector;
use opencv::imgcodecs;



//opencv camera
pub struct SwiftBotCamera {
   cam: videoio::VideoCapture,
}


impl SwiftBotCamera {
    pub fn new() -> Self {
      let cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap();

      if cam.is_opened().unwrap() == false {
            panic!("need camera");
      }

      Self {
         cam
      }
    }

    pub async fn decode_qr(&mut self) -> String
     {
        let mut mat = opencv::core::Mat::default();
        let mut qr_decoder = opencv::objdetect::QRCodeDetector::default().unwrap();
        self.cam.read(&mut mat).unwrap();

        let raw_str: Vec<u8> = qr_decoder.detect_and_decode_def(&mut mat).unwrap();

        String::from_utf8(raw_str).unwrap()
    }

    pub async fn save_pic<T>(&mut self, path: T)  
    where T: ToString
    {
        let mut mat = opencv::core::Mat::default();
        self.cam.read(&mut mat).unwrap();
        let params = opencv::core::Vector::from_slice(&[
            opencv::imgcodecs::IMWRITE_JPEG_QUALITY,
            90, // Quality from 0-100
        ]);
        
        match imgcodecs::imwrite(&path.to_string(), &mat, &opencv::core::Vector::new()) {
            Ok(_) => println!("Successfully saved image"),
            Err(e) => println!("Error saving image: {}", e)
        }
    }
}
