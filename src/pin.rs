use core::marker::PhantomData;
use stm32f30x::{GPIOA, GPIOD};

pub struct AF5;
pub struct Floating;
pub struct OutputPushPull;

pub struct PAxL<MODE> {
    x: u8,
    _mode: PhantomData<MODE>,
}

impl<MODE> PAxL<MODE> {
    pub fn new(x: u8) -> PAxL<MODE> {
        PAxL {
            x: x,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as an push pull output pin
    pub fn mode_push_pull_output(&self) -> PAxL<OutputPushPull> {
        let gpioa = unsafe { &*GPIOA::ptr() };

        let offset = 2 * self.x;

        // general purpose output mode
        let mode = 0b01;
        gpioa.moder.modify(|r, w| unsafe {
            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
        });

        // push pull output
        gpioa.otyper
            .modify(|r, w| unsafe { w.bits(r.bits() & !(0b1 << self.x)) });

        PAxL {
            x: self.x,
            _mode: PhantomData,
        }
    }

    /// Configures the pin to serve as alternate function 5 (AF5)
    pub fn mode_af5(&self) -> PAxL<AF5> {
        let gpioa = unsafe { &*GPIOA::ptr() };

        let offset = 2 * self.x;

        // alternate function mode
        let mode = 0b10;
        gpioa.moder.modify(|r, w| unsafe {
            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
        });

        let af = 5;
        let offset = 4 * (self.x % 8);
        gpioa.afrl.modify(|r, w| unsafe {
            w.bits((r.bits() & !(0b1111 << offset)) | (af << offset))
        });

        PAxL {
            x: self.x,
            _mode: PhantomData,
        }
    }

    pub fn set_high(&self) {
        unsafe { (*GPIOA::ptr()).bsrr.write(|w| w.bits(1 << self.x)) }
    }

    pub fn set_low(&self) {
        unsafe { (*GPIOA::ptr()).bsrr.write(|w| w.bits(1 << (16 + self.x))) }
    }
}

pub struct PDxL<MODE> {
    x: u8,
    _mode: PhantomData<MODE>,
}

impl<MODE> PDxL<MODE> {
    pub fn new(x: u8) -> PDxL<MODE> {
        PDxL {
            x: x,
            _mode: PhantomData,
        }
    }

    /// Configures the pin to operate as an push pull output pin
    pub fn mode_push_pull_output(&self) -> PDxL<OutputPushPull> {
        let gpiod = unsafe { &*GPIOD::ptr() };

        let offset = 2 * self.x;

        // general purpose output mode
        let mode = 0b01;
        gpiod.moder.modify(|r, w| unsafe {
            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
        });

        // push pull output
        gpiod.otyper
            .modify(|r, w| unsafe { w.bits(r.bits() & !(0b1 << self.x)) });

        PDxL {
            x: self.x,
            _mode: PhantomData,
        }
    }

    pub fn set_high(&self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*GPIOD::ptr()).bsrr.write(|w| w.bits(1 << self.x)) }
    }

    pub fn set_low(&self) {
        unsafe { (*GPIOD::ptr()).bsrr.write(|w| w.bits(1 << (16 + self.x))) }
    }
}