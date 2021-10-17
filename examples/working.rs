#![no_std]
#![no_main]

use panic_halt as _;

use ag_csms::{MoistureSensor};

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    let delay = arduino_hal::Delay::new();

    let mut led = pins.d13.into_output();

    let mut sensor: MoistureSensor = MoistureSensor::new(peripherals.ADC, pins.a5, delay)
        .with_context(true)
        .build();

    loop {
        arduino_hal::delay_ms(1000);
        let value = sensor.read();
        if value > 50 {
            led.set_high();
        }
        else {
            led.set_low();
        }
    }
}
