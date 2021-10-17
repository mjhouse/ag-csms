#![no_std]

use embedded_hal::blocking::delay::DelayUs;

use arduino_hal::Delay;
use arduino_hal::hal::{
    Adc,
    pac::ADC,
    clock::{
        MHz16,
        Clock,
    },
    port::{
        Pin, PC5,
        mode::{
            Floating,
            Analog,
            Input,
        },
    },
};

const DEFAULT_LIMIT: u16 = 4095; // maximum value that this sensor can read
const DEFAULT_COUNT: u16 = 128;  // number of samples to take for a reading
const DEFAULT_PAUSE: u16 = 1000; // microseconds to wait between samples
const DEFAULT_MIN: u16 = 3505;   // the devices tested minimum value (no moisture)
const DEFAULT_MAX: u16 = 3771;   // the device tested maximum value (all moisture)

pub struct MoistureSensor<D = Delay,S = MHz16>
where
    S: Clock,
    D: DelayUs<u16> + Sized,
{
    context: Option<Context>,
    convert: Adc<S>,
    input: Pin<Analog,PC5>,
    delay: D,
    limit: u16,
    count: u16,
    pause: u16,
    min: u16,
    max: u16,
}

#[derive(Clone)]
pub struct Context {
    pub limit: u16,
    pub count: u16,
    pub pause: u16,
    pub max: u16,
    pub min: u16,
}

impl Context {
    pub fn new() -> Self {
        Self {
            limit: 0u16,
            count: 0u16,
            pause: 0u16,
            max: 0u16,
            min: 0u16,
        }
    }
}

impl<D,S> MoistureSensor<D,S> 
where
    D: DelayUs<u16> + Sized,
    S: Clock,
{
    /// create new instance of MoistureSensor
    pub fn new(adc: ADC, pin: Pin<Input<Floating>, PC5>, delay: D) -> Self {
        let mut convert = Adc::new(adc, Default::default());
        let input = pin.into_analog_input(&mut convert);
        Self {
            context: None,
            convert: convert,
            input: input,
            delay: delay,
            limit: DEFAULT_LIMIT,
            count: DEFAULT_COUNT,
            pause: DEFAULT_PAUSE,
            min: DEFAULT_MIN,
            max: DEFAULT_MAX,
        }
    }

    /// maximum raw value of hardware
    pub fn with_limit(mut self, limit: u16) -> Self {
        self.set_limit(limit);
        self
    }

    /// number of samples to take
    pub fn with_count(mut self, count: u16) -> Self {
        self.set_count(count);
        self
    }

    /// pause between each sample
    pub fn with_pause(mut self, pause: u16) -> Self {
        self.set_pause(pause);
        self
    }

    /// minimum raw value (considered '0%')
    pub fn with_min(mut self, min: u16) -> Self {
        self.set_min(min);
        self
    }

    /// maximum raw value (considered '100%')
    pub fn with_max(mut self, max: u16) -> Self {
        self.set_max(max);
        self
    }

    /// save contextual information for debugging
    pub fn with_context(mut self, context: bool) -> Self {
        self.set_context(context);
        self
    }

    /// finalize construction of sensor
    pub fn build(self) -> Self {
        // nothing to do here
        self
    }

    /// maximum raw value of hardware
    pub fn set_limit(&mut self, limit: u16) {
        self.limit = limit;
    }

    /// number of samples to take
    pub fn set_count(&mut self, count: u16) {
        self.count = count;
    }

    /// pause between each sample
    pub fn set_pause(&mut self, pause: u16) {
        self.pause = pause;
    }

    /// minimum raw value (considered '0%')
    pub fn set_min(&mut self, min: u16) {
        self.min = min;
    }

    /// maximum raw value (considered '100%')
    pub fn set_max(&mut self, max: u16) {
        self.max = max;
    }

    /// save contextual information for debugging
    pub fn set_context(&mut self, context: bool) {
        if context { 
            self.context = Some(Context::new()); 
        }
        else {
            self.context = None;
        }
    }

    /// get the context from the last reading
    pub fn data(&self) -> Option<Context> {
        self.context.clone()
    }

    /// read the percentage moisture value
    pub fn read(&mut self) -> u16 {
        let mut value = self.limit - self.poll();

        if let Some(c) = self.context.as_mut() {
            c.limit = self.limit;
            c.count = self.count;
            c.pause = self.pause;
        }

        if value < self.min { value = self.min; }
        if value > self.max { value = self.max; }
    
        let a = self.max - self.min;
        let mut b = ((value - self.min) * 100) + a/2;
        let mut c = 0;
    
        while b > a {
            c += 1;
            b -= a;
        }
        
        c
    }

    /// get a single analog sample from the sensor
    fn sample(&mut self) -> u16 {
        self.delay.delay_us(self.pause);
        self.input.analog_read(&mut self.convert)
    }

    /// get a series of samples and average them
    fn poll(&mut self) -> u16 {
        let mut value = 0u16;
        for _ in 0..self.count {
            let s = self.sample();
            if let Some(c) = self.context.as_mut() {
                if s > c.max { c.max = s; }
                if s < c.min { c.min = s; }
            }
            value += s / self.count;
        }
        value
    }

}