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

fn blink<T: OutputPin>(timer: &mut hal::delay::Delay, led: &mut T) {
    const BLINK_DELAY: u16 = 100;
    led.set_low().unwrap_or_else(|_x| panic!());
    timer.delay_ms(BLINK_DELAY);
    led.set_high().unwrap_or_else(|_x| panic!());
    timer.delay_ms(BLINK_DELAY);
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let peripherals = hal::pac::Peripherals::take().unwrap();
    let core = hal::pac::CorePeripherals::take().unwrap();

    let mut timer = hal::delay::Delay::new(core.SYST);
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);

    let mut led_blue = port0.p0_06.into_push_pull_output(Level::Low);
    let mut led_red = port0.p0_26.into_push_pull_output(Level::Low);
    let mut led_green = port0.p0_30.into_push_pull_output(Level::Low);

    loop {
        blink(&mut timer, &mut led_red);
        blink(&mut timer, &mut led_green);
        blink(&mut timer, &mut led_blue);
    }
}
