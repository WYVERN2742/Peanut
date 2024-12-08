//! The Peanut is a motherboard for the PINE64 Pinenut Model-01S BL602 Wifi/BLE5
//! Module. It aims to offer a very large suite of sensors, including Motion,
//! Humidity, Temperature, Ambient Light, and Pressure. While also providing
//! some LEDs and buttons for interaction, with audio output and buzzers for the
//! classic electronic bleeps. It's entirely powered by a single LiFePO4
//! battery, allowing for easy charging and a compact size.

#![no_std]

use bl602_hal::gpio::I2c as gpio_i2c;
use bl602_hal::gpio::{Pin3, Pin4};
use bl602_hal::i2c::I2c as hal_i2c;
use bl602_pac::I2C as pac_i2c;

pub type I2cBus = hal_i2c<pac_i2c, (Pin4<gpio_i2c>, Pin3<gpio_i2c>)>;

pub mod drivers;
pub mod os;

pub mod peanut;
pub use peanut::Peanut;