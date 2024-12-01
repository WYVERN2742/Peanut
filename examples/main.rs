// Don't include standard rust library, since it's incompatible with embedded systems
#![no_std]

// We use our own entrypoint marked with #[riscv_rt::entry], not rust's default.
#![no_main]

// import Hardware Abstraction Layer for bl602
use bl602_hal as hal;
use peanut::drivers::PCF8574;

use core::fmt::Write;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

use hal::{
	clock::{Strict, SysclkFreq, UART_PLL_FREQ},
	pac,
	prelude::*,
	serial::*,
};

use crate::drivers::MCP4728;

// We want our embedded system to halt indefinitely when we panic.
// We discard the import since its functionality is a side-effect
use panic_halt as _;

// define a baudrate for serial communication
const BAUD_RATE:u32 = 1000000;

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

	// Configure our UART to our baudrate, and use the pins we configured above
	let mut serial = Serial::new(dp.UART0, Config::default().baudrate(BAUD_RATE.Bd()), ((pin16, mux0), (pin7, mux7)), clocks);

	// Pins for LED for Pinecone dev board
	let mut blue_led = parts.pin11.into_pull_down_output();
	let mut green_led = parts.pin14.into_pull_down_output();
	let mut red_led = parts.pin17.into_pull_down_output();

	// Create a delay function using the riscv chip cycle counter
	let mut delay = bl602_hal::delay::McycleDelay::new(clocks.sysclk().0);

	let mut i2c = hal::i2c::I2c::new(
		dp.I2C,
		(scl, sda),
		100_000u32.Hz(),
		clocks,
	);

	let mut pcf = PCF8574::new();

	loop {
	}
}
