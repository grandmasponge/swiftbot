use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::PathBuf;

use image::ImageBuffer;
use image::Rgba;
use v4l::context;
use v4l::io::traits::CaptureStream;
use v4l::prelude::*;
use v4l::FourCC;
use v4l::{video::Capture, Device};

use crate::SwiftBotError;

pub struct Camera<'a> {
    stream: MmapStream<'a>,
    dev: Device,
}

impl Camera<'_> {
    pub fn camera_list() {
        let context = context::enum_devices();
        for dev in context {
            println!("name: {}, path: {}",dev.name().unwrap(), dev.path().to_str().unwrap())
        }
    }

    pub fn setup(cam_path: Option<PathBuf>) -> SwiftBotError<Self> {
        let dev = if let Some(path) = cam_path {
            Device::with_path(path).expect("device dose not exist")
        } else {
            Device::with_path("/dev/video0").expect("device not found")
        };

           // Set the format to YUYV 4:2:2
         let format = v4l::Format::new(1280, 720, FourCC::new(b"YUYV"));
        dev.set_format(&format).expect("Failed to set format");

        let stream = MmapStream::with_buffers(&dev, v4l::buffer::Type::VideoCapture, 10)
            .expect("failed to make a stream");
      
       
        // warm up the camera

        println!("camera setup");

        Ok(Self { stream, dev })
    }

    pub fn still<T>(&mut self, path: T) -> SwiftBotError<()>
    where
        T: ToString,
    {
        let path = PathBuf::from(path.to_string());

        let (buf, meta) = self.stream.next()?;
        
        let rgb_image: ImageBuffer<Rgba<u8>, &[u8]> = ImageBuffer::from_raw(1270, 720, buf)
            .unwrap_or_else(|| {
                // error handling comes later
                panic!("oops")
            });
        rgb_image.save_with_format(path, image::ImageFormat::Png)?;

        Ok(())
    }
}
