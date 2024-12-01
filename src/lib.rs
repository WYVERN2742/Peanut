//! The Peanut is a motherboard for the PINE64 Pinenut Model-01S BL602 Wifi/BLE5
//! Module. It aims to offer a very large suite of sensors, including Motion,
//! Humidity, Temperature, Ambient Light, and Pressure. While also providing
//! some LEDs and buttons for interaction, with audio output and buzzers for the
//! classic electronic bleeps. It's entirely powered by a single LiFePO4
//! battery, allowing for easy charging and a compact size.

#![no_std]

pub mod drivers;
pub mod os;
