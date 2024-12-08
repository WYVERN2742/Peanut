use bl602_hal::gpio::{I2c as gpio_i2c, Output, Pin11, Pin14, Pin17, PullUp};
use bl602_hal::gpio::{Pin3, Pin4};
use bl602_hal::i2c::I2c as hal_i2c;
use bl602_pac::I2C as pac_i2c;

use bl602_hal::{
	clock::{Clocks, Strict, SysclkFreq, UART_PLL_FREQ},
	delay::McycleDelay,
	gpio::GlbExt,
	prelude::Extensions,
	serial::{Config, Serial},
};
use bl602_pac as pac;

pub const BAUD_RATE: u32 = 115200;

use crate::drivers::pcf8574::PCF8574;

pub struct Peanut<'a> {
	// hal device access
	pub clocks: Clocks,
	pub delay: McycleDelay,

	// Communication Busses
	pub i2c: hal_i2c<pac_i2c, (Pin4<gpio_i2c>, Pin3<gpio_i2c>)>,
	pub uart: Serial<
		bl602_pac::UART0,
		(
			(
				bl602_hal::gpio::Pin16<bl602_hal::gpio::Uart>,
				bl602_hal::gpio::UartMux0<bl602_hal::gpio::Uart0Tx>,
			),
			(
				bl602_hal::gpio::Pin7<bl602_hal::gpio::Uart>,
				bl602_hal::gpio::UartMux7<bl602_hal::gpio::Uart0Rx>,
			),
		),
	>,

	// Onboard LEDs
	pub onboard_red_led: Pin17<Output<PullUp>>,
	pub onboard_green_led: Pin14<Output<PullUp>>,
	pub onboard_blue_led: Pin11<Output<PullUp>>,

	// I2C Drivers
	pub pcf: PCF8574<'a>,
}

impl<'a> Peanut<'a> {
	/// Initialize the Peanut Board.
	/// This involves setting up system clocks, UART, I2C bus, and driver initialization
	pub fn init() -> Peanut<'a> {
		// read device peripherals
		let dp = pac::Peripherals::take().unwrap();

		// split the register block into individual pins and modules
		let mut parts = dp.GLB.split();

		// Set up all required clocks
		let clocks = Strict::new()
			.use_pll(40_000_000u32.Hz())
			.sys_clk(SysclkFreq::Pll160Mhz)
			.uart_clk(UART_PLL_FREQ.Hz())
			.freeze(&mut parts.clk_cfg);

		let delay = bl602_hal::delay::McycleDelay::new(clocks.sysclk().0);

		// Set up uart output. Bl602 uses a pin matrix so we set muxes too.
		let pin16 = parts.pin16.into_uart_sig0();
		let pin7 = parts.pin7.into_uart_sig7();
		let mux0 = parts.uart_mux0.into_uart0_tx();
		let mux7 = parts.uart_mux7.into_uart0_rx();

		// Configure our UART to our baudrate, and use the pins we configured above
		let uart = Serial::new(
			dp.UART0,
			Config::default().baudrate(BAUD_RATE.Bd()),
			((pin16, mux0), (pin7, mux7)),
			clocks,
		);

		// Pins for LED for Pinecone dev board
		let onboard_red_led = parts.pin17.into_pull_up_output();
		let onboard_green_led = parts.pin14.into_pull_up_output();
		let onboard_blue_led = parts.pin11.into_pull_up_output();

		let scl = parts.pin4.into_i2c_scl();
		let sda = parts.pin3.into_i2c_sda();

		let mut i2c = bl602_hal::i2c::I2c::new(dp.I2C, (scl, sda), 100_000u32.Hz(), clocks);

		// Driver initialization
		let mut pcf = PCF8574::new(0x20, "PCF");

		// todo: Error handling of driver initialization
		pcf.init(&mut i2c).unwrap();

		return Self {
			clocks,
			delay,
			i2c,
			uart,

			onboard_red_led,
			onboard_green_led,
			onboard_blue_led,

			pcf,
		};
	}
}
