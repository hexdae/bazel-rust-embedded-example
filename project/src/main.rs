#![no_main]
#![no_std]

use nrf52840_hal as hal;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;
use hal::gpio::Level;

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let peripherals = hal::pac::Peripherals::take().unwrap();
    let core = hal::pac::CorePeripherals::take().unwrap();

    let mut timer = hal::delay::Delay::new(core.SYST);
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);

    let mut led = port0.p0_06.into_push_pull_output(Level::Low);

    loop {
        led.set_low().ok();
        timer.delay_ms(500u32);
        led.set_high().ok();
        timer.delay_ms(500u32);
    }
}
