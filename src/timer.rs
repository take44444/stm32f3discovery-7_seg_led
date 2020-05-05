use cast::{u16, u32};
use stm32f30x::{RCC, TIM2};

use super::hertz::*;

const HSI: u32 = 8_000_000;

#[allow(unused_unsafe)]
fn start<T>(timeout: T)
where
    T: Into<Hertz>,
{
    let rcc = unsafe { &*RCC::ptr() };

    let pllmul_bits = rcc.cfgr.read().pllmul().bits();
    let pllmul: u32 = u32(pllmul_bits + 2);

    // let ppre1_bits = (rcc.cfgr.read().bits() << 21) >> 29;
    let ppre1_bits = rcc.cfgr.read().ppre1().bits();
    let ppre1: u32 = if ppre1_bits & 0b100 == 0 { 1 } else { 1 << (ppre1_bits - 0b011) };
    
    let hpre_bits = rcc.cfgr.read().hpre().bits();
    let hpre: u32 = if hpre_bits & 0b1000 == 0 { 1 } else { 1 << (hpre_bits - 0b0111) };
    
    let sysclk = pllmul * HSI / 2;
    let hclk = sysclk / hpre;
    let pclk1 = hclk / ppre1;

    let tim = unsafe { &*TIM2::ptr() };
    // pause
    tim.cr1.modify(|_, w| w.cen().clear_bit());
    // restart counter
    tim.cnt.reset();

    let frequency = timeout.into().0;
    let ticks = pclk1 * if ppre1 == 1 { 1 } else { 2 }
        / frequency;

    let psc = u16((ticks - 1) >> 16).unwrap();
    tim.psc.write(|w| unsafe { w.psc().bits(psc) });

    let arr = u16(ticks / u32(psc + 1)).unwrap();
    tim.arr.write(|w| unsafe { w.bits(u32(arr)) });

    // start counter
    tim.cr1.modify(|_, w| w.cen().set_bit());
}

fn release() {
    let tim = unsafe { &*TIM2::ptr() };
    // pause counter
    tim.cr1.modify(|_, w| w.cen().clear_bit());
}

pub fn tim2<T>(timeout: T)
where
    T: Into<Hertz>,
{
    let rcc = unsafe { &*RCC::ptr() }; // 1073876992
    // enable and reset peripheral to a clean slate state
    rcc.apb1enr.modify(|_, w| w.tim2en().set_bit());
    rcc.apb1rstr.modify(|_, w| w.tim2rst().set_bit());
    rcc.apb1rstr.modify(|_, w| w.tim2rst().clear_bit());

    start(timeout.into());
}

pub fn tim2_change_period<T>(timeout: T)
where
    T: Into<Hertz>,
{
    release();
    let rcc = unsafe { &*RCC::ptr() }; // 1073876992
    // enable and reset peripheral to a clean slate state
    rcc.apb1enr.modify(|_, w| w.tim2en().set_bit());
    rcc.apb1rstr.modify(|_, w| w.tim2rst().set_bit());
    rcc.apb1rstr.modify(|_, w| w.tim2rst().clear_bit());

    start(timeout.into());
}

pub fn tim2_updated() -> Result<(), ()> {
    let tim = unsafe { &*TIM2::ptr() };
    if tim.sr.read().uif().bit_is_clear() {
        Err(())
    } else {
        tim.sr.modify(|_, w| w.uif().clear_bit());
        Ok(())
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