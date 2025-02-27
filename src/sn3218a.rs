//LEDS

use rppal::i2c::{self, I2c};

const I2C_ADDR: u8 = 0x54; //i2c adress for the rasberry pi 4
const CMD_ENABLE_OUTPUT: u8 = 0x00;
const CMD_SET_PWM: u8 = 0x01;
const CMD_UPDATE: u8 = 0x16;
const CMD_ENABLE_LED: u8 = 0x13;
const CMD_RESET: u8 = 0x17;

pub enum UnderLights {
    U1, //starting point is 0 to enable would be 0b11100000000000000
    U2, // starting point is 3 to enable would be 0b0001110000000000
    U3, // starting point is 6 to enable would be 0b0000001110000000
    U4, // starting point is 9 to enable would be 0b0000000001110000
    U5, // starting point is 12 to enable would be0b00000000000011100
    U6, // starting point is 15 to enable would be 0b00000000000000111
}

impl UnderLights {
    pub fn turn_on_all_underlight(sn3218: &Sn3218a, r: u8, g: u8, b: u8) {
        sn3218.enable();

        let mut data = vec![0; 18];

        for i in (0..data.len()).step_by(3) {
            data[i] = r;
            data[i + 1] = g;
            data[i + 2] = b;
        }

        sn3218.output(&data);
    }

    pub fn turn_one_underlight(sn3218: &Sn3218a, light: UnderLights, r: u8, g: u8, b: u8) {
        sn3218.enable();

        let mut data = vec![0; 18];

        match light {
            UnderLights::U1 => {
                data[0] = r;
                data[1] = g;
                data[2] = b;
            }
            UnderLights::U2 => {
                data[3] = r;
                data[4] = g;
                data[5] = b;
            }
            UnderLights::U3 => {
                data[6] = r;
                data[7] = g;
                data[8] = b;
            }
            UnderLights::U4 => {
                data[9] = r;
                data[10] = g;
                data[11] = b;
            }
            UnderLights::U5 => {
                data[12] = r;
                data[13] = g;
                data[14] = b;
            }
            UnderLights::U6 => {
                data[15] = r;
                data[16] = g;
                data[17] = b;
            }
        }
        sn3218.output(&data);
    }
}

pub struct Sn3218a {
    i2c: I2c,
    gamma_table: Vec<Vec<i32>>,
}

impl Sn3218a {
    pub fn init() -> Self {
        let mut gamma_table = Vec::new();

        for i in 0..256 {
            let exponent = (i - 1) as f64 / 255.0;
            let pow_result = f64::powf(255.0, exponent);
            gamma_table.push(pow_result as i32);
        }

        let mut channel_gamma_table = Vec::new();
        for _ in 0..18 {
            channel_gamma_table.push(gamma_table.clone());
        }

        let mut i2c = i2c::I2c::new().expect("failed to get i2c");
        i2c.set_slave_address(I2C_ADDR as u16)
            .expect("failed to set i2c slave address");

        Self {
            i2c,
            gamma_table: channel_gamma_table,
        }
    }

    pub fn enable(&self) {
        self.i2c.block_write(CMD_ENABLE_OUTPUT, &[0x1]).unwrap();
    }

    pub fn disable(&self) {
        self.i2c.block_write(CMD_ENABLE_OUTPUT, &[0x0]).unwrap();
    }

    pub fn disable_led(&self, mask: u32) {
        let data = [
            (mask ^ 0x3f) as u8,
            ((mask >> 6) ^ 0x3f) as u8,
            ((mask >> 12) ^ 0x3f) as u8,
        ];
        self.i2c.block_write(CMD_ENABLE_LED, &data).unwrap();
        self.i2c.block_write(CMD_UPDATE, &[0xff]).unwrap();
    }

    pub fn enable_some_led(&self, mask: u32) {
        let data = [
            (mask & 0x3f) as u8,
            ((mask >> 6) & 0x3f) as u8,
            ((mask >> 12) & 0x3f) as u8,
        ];
        self.i2c.block_write(CMD_ENABLE_LED, &data).unwrap();
        self.i2c.block_write(CMD_UPDATE, &[0xff]).unwrap();
    }

    pub fn enable_all_led(&self) {
        let data = [0x3f, 0x3f, 0x3f];

        self.i2c.block_write(CMD_ENABLE_LED, &data).unwrap();
        self.i2c.block_write(CMD_UPDATE, &[0xff]).unwrap();
    }

    pub fn reset(&mut self) {
        self.i2c.block_write(CMD_RESET, &[0xff]).unwrap();
    }

    pub fn update(&mut self) {
        self.i2c.block_write(CMD_RESET, &[0xff]).unwrap();
    }

    pub fn output(&self, data: &[u8]) {
        let result: Vec<u8> = (0..18)
            .map(|i| self.gamma_table[i][data[i] as usize] as u8)
            .collect();

        self.i2c
            .block_write(CMD_SET_PWM, &result)
            .expect("Error writing to i2c");
        self.i2c
            .block_write(CMD_UPDATE, &[0xFF])
            .expect("Error writing to i2c");
    }
}
