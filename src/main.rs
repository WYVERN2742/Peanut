// Don't include standard rust library, since it's incompatible with embedded systems
#![no_std]

// We use our own entrypoint marked with #[riscv_rt::entry], not rust's default.
#![no_main]

use core::borrow::Borrow;
use core::fmt::{Debug, Write};

// import Hardware Abstraction Layer for bl602
use bl602_hal::{self as hal, clock};

use embedded_hal::{delay::DelayNs, i2c::I2c};
use embedded_hal::digital::OutputPin;

use hal::{
	clock::{Strict, SysclkFreq, UART_PLL_FREQ},
	pac,
	prelude::*,
	serial::*,
};

// We want our embedded system to halt indefinitely when we panic.
// We discard the import since its functionality is a side-effect
use panic_halt as _;

// define a baudrate for serial communication
const BAUD_RATE:u32 = 115_200;

// RiscV entrypoint. The ! notation indicates we have no panic handler
#[riscv_rt::entry]
fn main() -> ! {

	// Load all peripherals
	let dp = pac::Peripherals::take().unwrap();
	// split the register block into individual pins and modules
	let mut parts = dp.GLB.split();

	// Set up all the clocks we need
	let clocks = Strict::new()
		.use_pll(40_000_000u32.Hz())
		.sys_clk(SysclkFreq::Pll160Mhz)
		.uart_clk(UART_PLL_FREQ.Hz())
		.freeze(&mut parts.clk_cfg);

	// Set up uart output. Bl602 uses a pin matrix so we set muxes too.
	let pin16 = parts.pin16.into_uart_sig0();
	let pin7 = parts.pin7.into_uart_sig7();
	let mux0 = parts.uart_mux0.into_uart0_tx();
	let mux7 = parts.uart_mux7.into_uart0_rx();
	let scl = parts.pin4.into_i2c_scl();
	let sda = parts.pin3.into_i2c_sda();

	// Configure our UART to our baudrate, and use the pins we configured above
	let mut serial = Serial::new(dp.UART0, Config::default().baudrate(BAUD_RATE.Bd()), ((pin16, mux0), (pin7, mux7)), clocks);
	serial.write_str("---------------------\n").unwrap();
	serial.write_str("Setup Passed!\n").unwrap();

	// Pins for LED for Pinecone dev board
	let mut green_led = parts.pin14.into_pull_down_output();
	// let mut red_led = parts.pin14.into_pull_down_output();
	let mut red_led = parts.pin17.into_pull_down_output();
 
	// // Create a delay function using the riscv chip cycle counter
	let mut delay = bl602_hal::delay::McycleDelay::new(clocks.sysclk().0);

	serial.write_str("i2c init\n").unwrap();
	// Toggle the LED on and off once a sec	ond. Report LED status over UART
	let mut i2c = hal::i2c::I2c::new(
		dp.I2C,
		(scl, sda),
		100_000u32.Hz(),
		clocks,
	);

	serial.write_str("loop start\n").unwrap();
	loop {
		green_led.set_low().unwrap();
		red_led.set_high().unwrap();
		delay.delay_ms(1000);
		// serial.write_str("## - Loop\n").unwrap();
		// serial.write_str("Writing\n").unwrap();
		
		// let result = i2c.write(0b0111_0000, &[0b1111_1111]);
		serial.write_str("i2c send").unwrap();
		let result = i2c.write(0b1100_0000, &[0b01011_000, 0b1_00_1111, 0b1111_1111]);
		if result.is_err() {
			serial.write_str("i2c err").unwrap();
			red_led.set_low().unwrap();
			match result.unwrap_err() {
				hal::i2c::Error::RxOverflow => {serial.write_str("RxOverflow\n").unwrap();},
				hal::i2c::Error::TxOverflow => {serial.write_str("TxOverflow\n").unwrap();},
				hal::i2c::Error::RxUnderflow => {serial.write_str("RxUnderflow\n").unwrap();},
				hal::i2c::Error::TxUnderflow => {serial.write_str("TxUnderflow\n").unwrap();}
				hal::i2c::Error::Timeout => {serial.write_str("Timeout\n").unwrap();}
			}
		}
		serial.write_str("##\n").unwrap();
		green_led.set_high().unwrap();
		delay.delay_ms(2000);
	}
}
