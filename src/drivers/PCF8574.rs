use bl602_hal::i2c::Error;


struct PCF8574 {
	address: SevenBitAddress,
	name: str,
}

impl PCF8574 {
	pub fn new(address: SevenBitAddress, name:&str) -> PCF8574 {

	}
	pub fn set_all_pins(&self) -> Result<(), Error> {}
	pub fn set_pin(&self, pin:u8) -> Result<(), Error> {}
	pub fn read_all_pins(&self) -> Result<(u8), Error> {}
	pub fn read_pin(&self, pin:u8) -> Result<(bool), Error> {}
}
