use crate::I2cBus;
use bl602_hal::i2c::Error;
use embedded_hal::i2c::{I2c as embedded_i2c, SevenBitAddress};

/// PCF8574A - Remote 8-Bit I2C IO Expander
///
/// This 8-bit IO expander features an 8-bit quasi-bidirectional latched
/// outputs. Each port can be used as an input or output without the use of a
/// data-direction signal.
pub struct PCF8574<'a> {
	address: SevenBitAddress,
	name: &'a str,
	last_set: u8,
}

impl<'a> PCF8574<'a> {
	pub fn new(address: SevenBitAddress, name: &'a str) -> PCF8574<'a> {
		return Self {
			address: address,
			name: name,
			last_set: 0,
		};
	}

	/// Initialize the PCF8574 by setting all pins to LOW
	pub fn init(&mut self, i2c: &mut I2cBus) -> Result<(), Error> {
		return self.set_all_pins(i2c, 0b0000_0000);
	}

	/// Set all IO pins to the corresponding output value interpreted as each bit per pin:
	/// e.g 0b0000_1010 sets pins P4 and P2 to HIGH, and all other pins to LOW.
	pub fn set_all_pins(&mut self, i2c: &mut I2cBus, value: u8) -> Result<(), Error> {
		self.last_set = value;
		return i2c.write(self.address, &[value]);
	}

	/// Set a pin to HIGH or LOW
	pub fn set_pin(&mut self, i2c: &mut I2cBus, pin: u8, high: bool) -> Result<(), Error> {
		if high {
			self.last_set = self.last_set | (1 << pin)
		} else {
			self.last_set = self.last_set & !(1 << pin)
		}
		return i2c.write(self.address, &[self.last_set]);
	}

	/// Read all pins as LOW or HIGH.
	///
	/// A pin may be pulled down to LOW or up to HIGH by external sources.
	///
	/// Result is a 8-bit value, e.g 0b0010_0010 with P2 and P6 pulled HIGH
	pub fn read_all_pins(&mut self, i2c: &mut I2cBus) -> Result<u8, Error> {
		let mut value: [u8; 1] = [0];
		let result = i2c.read(self.address, &mut value);
		if result.is_err() {
			return Err(result.unwrap_err());
		} else {
			return Ok(value[0]);
		}
	}

	/// Read a specific pin as LOW (false) or HIGH (true).
	///
	/// A pin may be pulled down to LOW or up to HIGH by external sources.
	pub fn read_pin(&mut self, i2c: &mut I2cBus, pin: u8) -> Result<bool, Error> {
		let mut value: [u8; 1] = [0];
		let mask: u8 = 1 << pin;
		let result = i2c.read(self.address, &mut value);
		if result.is_err() {
			return Err(result.unwrap_err());
		} else {
			return Ok((value[0] & mask) == mask);
		}
	}
}
