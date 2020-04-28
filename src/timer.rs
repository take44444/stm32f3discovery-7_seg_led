use cast::{u16, u32};
use f3::hal::{
    rcc::Clocks,
    stm32f30x::{RCC, TIM2},
    time::*,
};

pub struct Timer {
    clocks: Clocks,
}

impl Timer {
    pub fn tim2<T>(timeout: T, clocks: Clocks) -> Self
    where
        T: Into<Hertz>,
    {
        // let dp = stm32f30x::Peripherals::take().unwrap();
        // let mut rcc = dp.RCC.constrain();
        let rcc = unsafe { &*RCC::ptr() }; // 1073876992
        // enable and reset peripheral to a clean slate state
        rcc.apb1enr.modify(|_, w| w.tim2en().set_bit());
        rcc.apb1rstr.modify(|_, w| w.tim2rst().set_bit());
        rcc.apb1rstr.modify(|_, w| w.tim2rst().clear_bit());

        let timer = Timer {
            clocks,
        };
        timer.start(timeout.into());

        timer
    }

    /// Releases the TIM peripheral
    fn free(&self) {
        let tim = unsafe { &*TIM2::ptr() };
        // pause counter
        tim.cr1.modify(|_, w| w.cen().clear_bit());
    }

    pub fn change_period<T>(&mut self, timeout: T)
    where
        T: Into<Hertz>,
    {
        self.free();
        let rcc = unsafe { &*RCC::ptr() }; // 1073876992
        // enable and reset peripheral to a clean slate state
        rcc.apb1enr.modify(|_, w| w.tim2en().set_bit());
        rcc.apb1rstr.modify(|_, w| w.tim2rst().set_bit());
        rcc.apb1rstr.modify(|_, w| w.tim2rst().clear_bit());

        self.start(timeout.into());
    }

    // NOTE(allow) `w.psc().bits()` is not safe for TIM2 due to some SVD omission
    #[allow(unused_unsafe)]
    pub fn start(&self, timeout: Hertz) {
        let rcc = unsafe { &*RCC::ptr() };
        let ppre1_bits = (rcc.cfgr.read().bits() << 21) >> 29;
        let ppre1 = 1 << (ppre1_bits - 0b011);

        let tim = unsafe { &*TIM2::ptr() };
        // pause
        tim.cr1.modify(|_, w| w.cen().clear_bit());
        // restart counter
        tim.cnt.reset();

        let frequency = timeout.0;
        let ticks = self.clocks.pclk1().0 * if ppre1 == 1 { 1 } else { 2 }
            / frequency;

        let psc = u16((ticks - 1) / (1 << 16)).unwrap();
        tim.psc.write(|w| unsafe { w.psc().bits(psc) });

        let arr = u16(ticks / u32(psc + 1)).unwrap();
        tim.arr.write(|w| unsafe { w.bits(u32(arr)) });

        // start counter
        tim.cr1.modify(|_, w| w.cen().set_bit());
    }

    pub fn sr_uif_is_set(&self) -> Result<(), ()> {
        let tim = unsafe { &*TIM2::ptr() };
        if tim.sr.read().uif().bit_is_clear() {
            Err(())
        } else {
            tim.sr.modify(|_, w| w.uif().clear_bit());
            Ok(())
        }
    }
}

#[macro_export]
macro_rules! wait {
    ($e:expr) => {
        loop {
            match $e {
                Err(_) => {},
                Ok(_) => break,
            }
        }
    }
}