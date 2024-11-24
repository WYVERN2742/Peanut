use bl602_hal::i2c::Error;

type u12 = u8;

struct DACChannel {
	v_ref: VoltageReference,
	gain: Gain,
	power: PowerState,
}

// Each channel has two modes of operation:
// a. Normal mode, where each analog voltage is available
// b. Power-down mode, turns off most of the internal circuits for power savings.
// In power-down mode, the pins will be connected to ground with a known internal resistance.
enum PowerState {
	Normal = 0,
	Ground1k = 1,
	Ground100k = 2,
	Ground500k = 3,
}

// Each channel has a programmable gain block. The rail-to-rail output has an
// amplifier with a configurable gain of x1 or x2.
// A gain of x2 is not applicable when vDD is selected as the voltage reference.
enum Gain {
	x1 = 0,
	x2 = 1
}

enum VoltageReference {
	vdd = 0,
	internal = 1
}

struct MCP4728 {
	address: SevenBitAddress,
	name: str,
	
	// Channels
	a: Channel,
	b: Channel,
	c: Channel,
	d: Channel,
}

struct ChannelValue {
	channel: Channel,
	value: u12,
}

impl MCP4728 {


	pub fn new(address: SevenBitAddress, name:&str) -> PCF8574 {}

	// Fast Write writes to the DAC input registers sequentially
	// with limited configuration bits. The data is sent sequentially from
	// channels A to D. The input register is written at the acknowledge clock
	// pulse of the channel's last input byte. EEPROM is not affected.
	pub fn write_fast(&self) -> Result<(), Error> {}

	// Sequentially write to DAC from channel A to D with minimal configuration bits.
	// EEPROM is not affected.
	// pub fn set_all_channels(&self, power:PowerState) -> Result<(), Error> {}

	// This command writes to multiple DAC input registers, one DAC input
	// register at a time. The writing channel register is defined by the DAC
	// selection bits (DAC1, DAC0). EEPROM is not affected.
	pub fn write_multi(&self) -> Result<(), Error> {}

	// Write to multiple DAC channels with full configuration bits.
	// EEPROM is not affected.
	// pub fn set_multiple(&self, values: [&ChannelValue]) -> Result<(), Error> {}

	// This command writes to both the DAC input registers and EEPROM
	// sequentially. The sequential writing is carried out from a starting
	// channel to channel D. The starting channel is defined by the DAC
	// selection bits (DAC1, DAC0). The input register is written at the
	// acknowledge clock pulse of the last input data byte of each register.
	// However, the EEPROM data is written altogether at the same time
	// sequentially at the end of the last byte.
	pub fn write_sequential(&self) -> Result<(), Error> {}

	// Theis command writes to a single selected DAC input register and its
	// EEPROM. Both the input register and EEPROM are written at the acknowledge
	// clock pulse of the last input data byte. The writing channel is defined
	// by the DAC selection bits (DAC1, DAC0).
	pub fn write_single(&self) -> Result<(), Error> {}

	// v_ref, Gain and Power-Down

	// This command writes Reference (V_ref) selction bits of each channel.
	pub fn set_ref(&self) -> Result<(), Error> {}
	// This command writes Gain selection bits of each channel.
	pub fn set_gain(&self) -> Result<(), Error> {}
	// This command writes Power-Down bits of each channel.
	pub fn set_off(&self) -> Result<(), Error> {}
}