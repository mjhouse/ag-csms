#![no_std]

use core::convert::Infallible;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::digital::v2::InputPin;

const POLL_DELAY: u16 = 1;

pub struct MoistureSensor<T, D>
where
    T: InputPin<Error = Infallible> + Sized,
    D: DelayUs<u16> + Sized,
{
    input: T,
    delay: D,
    count: u16,
    min: u16,
    max: u16,
}