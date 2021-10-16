#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::hal::Adc;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    let delay = arduino_hal::Delay::new();

    let mut clock = Adc::new(peripherals.ADC, Default::default());
    let input = pin.into_analog_input(&mut clock);

    loop {
        arduino_hal::delay_ms(1000);
    }
}
