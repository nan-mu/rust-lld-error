#![no_std]
#![no_main]

extern crate alloc;
use core::mem::MaybeUninit;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio,
    peripherals::Peripherals,
    prelude::*,
    spi::{master::Spi, SpiMode},
    IO,
};
use log::info;
use max7219::MAX7219;
#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

#[entry]
fn main() -> ! {
    init_heap();
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let (sclk, miso, mosi) = (io.pins.gpio0, io.pins.gpio2, io.pins.gpio4);

    let spi_bus = Spi::new(peripherals.SPI2, 1000u32.kHz(), SpiMode::Mode0, &clocks).with_pins(
        Some(sclk),
        Some(miso),
        Some(mosi),
        gpio::NO_PIN,
    );
    let mut max7219 = MAX7219::from_spi(1, spi_bus).unwrap();
    max7219.power_on().unwrap();
    max7219.write_str(0, b"pls help", 0b00100000).unwrap();
    max7219.set_intensity(0, 0x1).unwrap();

    esp_println::logger::init_logger_from_env();
    info!("Logger is setup");

    loop {}
}
