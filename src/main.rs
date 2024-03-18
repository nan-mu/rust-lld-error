#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, IO};
use max7219::MAX7219;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let (sck, cs, data) = (
        io.pins.gpio0.into_push_pull_output(),
        io.pins.gpio2.into_push_pull_output(),
        io.pins.gpio4.into_push_pull_output(),
    );

    let mut max7219 = MAX7219::from_pins(1, data, cs, sck).unwrap();
    let mut delay = esp_hal::delay::Delay::new(&clocks);
    max7219.power_on().unwrap();

    loop {
        max7219.write_str(0, b"       1", 0b00010000).unwrap();
        delay.delay_ms(1000u32);
        max7219.write_str(0, b"       -", 0b00010000).unwrap();
        delay.delay_ms(1000u32);
    }
}
