// Don't include standard rust library, since it's incompatible with embedded systems
#![no_std]
// We use our own entrypoint marked with #[riscv_rt::entry], not rust's default.
#![no_main]

use embedded_hal::delay::DelayNs;
// We want our embedded system to halt indefinitely when we panic.
// We discard the import since its functionality is a side-effect
use panic_halt as _;

use peanut::{
	os::io::{ExternalLED, InternalLED},
	Peanut,
};

// RiscV entrypoint. The ! notation indicates we have no panic handler
#[riscv_rt::entry]
fn main() -> ! {
	let mut peanut = Peanut::init();

	peanut.led_rbg_red_off();
	peanut.led_rbg_green_off();
	peanut.led_rbg_blue_off();

	loop {
		// blue
		peanut.delay.delay_ms(200);
		peanut.led_rbg_blue_on();
		peanut.delay.delay_ms(200);
		peanut.led_rbg_blue_off();

		// green
		peanut.delay.delay_ms(200);
		peanut.led_rbg_green_on();
		peanut.delay.delay_ms(200);
		peanut.led_rbg_green_off();

		// red
		peanut.delay.delay_ms(200);
		peanut.led_rbg_red_on();
		peanut.delay.delay_ms(200);
		peanut.led_rbg_red_off();

		// yellow
		peanut.delay.delay_ms(200);
		peanut.led_yellow_on();
		peanut.delay.delay_ms(200);
		peanut.led_yellow_off();

		// red
		peanut.delay.delay_ms(200);
		peanut.led_red_on();
		peanut.delay.delay_ms(200);
		peanut.led_red_off();
	}
}
