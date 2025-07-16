#![no_std]
#![no_main]

use attiny_hal as hal;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

#[hal::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut led = pins.pb1.into_output();
    led.set_high();

    let mut delay = hal::delay::Delay::<hal::clock::MHz8>::new();

    loop {
        led.toggle();
        delay.delay_ms(1000);
    }
}
