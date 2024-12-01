use embedded_hal::i2c::{I2c, SevenBitAddress};
use bl602_hal::i2c::Error;

/// 
pub struct PCF8574<'a> {
	address: SevenBitAddress,
	name: &'a str,
	i2c: &'a dyn I2c<Error = Error>,
}

impl<'a> PCF8574<'a> {
	pub fn new(i2c:&dyn I2c<Error = Error>, address: SevenBitAddress, name:&str) -> PCF8574<'a> {
		return Self {
			i2c: i2c,
			address: address,
			name: name,
		};
	}

	pub fn set_all_pins(&self, value:u8) -> Result<(), Error> {
		return self.i2c.write(self.address, &[value])
	}

	pub fn set_pin(&self, pin:u8) -> Result<(), Error> {
		let value:u8 = 1;
		return self.i2c.write(self.address, &[value << pin])
	}

	pub fn read_all_pins(&self) -> Result<u8, Error> {
		let mut value:[u8; 1] = [0];
		let result = self.i2c.read(self.address, &mut value);
		return Ok(value[0])
	}

	pub fn read_pin(&self, pin:u8) -> Result<bool, Error> {
		let mut value:[u8; 1] = [0];
		let mask:u8 = 1 << pin;
		let result = self.i2c.read(self.address, &mut value);
		return Ok((value[0] & mask) == mask)
	}
}
