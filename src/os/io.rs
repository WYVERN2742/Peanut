use bl602_hal::i2c::Error as I2cError;
use embedded_hal::digital::OutputPin;

use crate::Peanut;

// internal:
// pin17 - LED red
// pin14 - LED green
// pin11 - LED blue

// pcf i2c multiplexer:
// p7 - LED yellow
// p6 - LED red
// p5 - Switch toggle
// p4 - Switch button 1
// p3 - Switch button 2
// p2 - Switch button 4
// p1 - Switch button 3
// p0 - battery monitor transistor

pub trait InternalLED {
	fn led_rbg_red_on<'a>(&'a mut self) -> Result<(), core::convert::Infallible>;
	fn led_rbg_red_off<'a>(&'a mut self) -> Result<(), core::convert::Infallible>;
	fn led_rbg_blue_on<'a>(&'a mut self) -> Result<(), core::convert::Infallible>;
	fn led_rbg_blue_off<'a>(&'a mut self) -> Result<(), core::convert::Infallible>;
	fn led_rbg_green_on<'a>(&'a mut self) -> Result<(), core::convert::Infallible>;
	fn led_rbg_green_off<'a>(&'a mut self) -> Result<(), core::convert::Infallible>;
}

impl<'a> InternalLED for Peanut<'a> {
	fn led_rbg_red_on<'b>(&'b mut self) -> Result<(), core::convert::Infallible> {
		return self.onboard_red_led.set_low();
	}

	fn led_rbg_red_off<'b>(&'b mut self) -> Result<(), core::convert::Infallible> {
		return self.onboard_red_led.set_high();
	}

	fn led_rbg_blue_on<'b>(&'b mut self) -> Result<(), core::convert::Infallible> {
		return self.onboard_blue_led.set_low();
	}

	fn led_rbg_blue_off<'b>(&'b mut self) -> Result<(), core::convert::Infallible> {
		return self.onboard_blue_led.set_high();
	}

	fn led_rbg_green_on<'b>(&'b mut self) -> Result<(), core::convert::Infallible> {
		return self.onboard_green_led.set_low();
	}

	fn led_rbg_green_off<'b>(&'b mut self) -> Result<(), core::convert::Infallible> {
		return self.onboard_green_led.set_high();
	}
}

pub trait ExternalLED {
	fn led_red_on<'b>(&'b mut self) -> Result<(), I2cError>;
	fn led_red_off<'b>(&'b mut self) -> Result<(), I2cError>;

	fn led_yellow_on<'b>(&'b mut self) -> Result<(), I2cError>;
	fn led_yellow_off<'b>(&'b mut self) -> Result<(), I2cError>;
}

impl<'a> ExternalLED for Peanut<'a> {
	fn led_red_off<'b>(&'b mut self) -> Result<(), I2cError> {
		return self.pcf.set_pin(&mut self.i2c, 6, false);
	}

	fn led_red_on<'b>(&'b mut self) -> Result<(), I2cError> {
		return self.pcf.set_pin(&mut self.i2c, 6, true);
	}

	fn led_yellow_off<'b>(&'b mut self) -> Result<(), I2cError> {
		return self.pcf.set_pin(&mut self.i2c, 7, false);
	}

	fn led_yellow_on<'b>(&'b mut self) -> Result<(), I2cError> {
		return self.pcf.set_pin(&mut self.i2c, 7, true);
	}
}
