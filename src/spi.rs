use core::ptr;
use cast::u32;
use stm32f30x::{RCC, SPI1};

use super::hertz::*;

const HSI: u32 = 8_000_000;

pub fn spi1<T>(freq: T)
where
    T: Into<Hertz>,
{
    let rcc = unsafe { &*RCC::ptr() }; // 1073876992

    // enable or reset SPI1
    rcc.apb2enr.modify(|_, w| w.spi1en().enabled());
    rcc.apb2rstr.modify(|_, w| w.spi1rst().set_bit());
    rcc.apb2rstr.modify(|_, w| w.spi1rst().clear_bit());

    let spi = unsafe { &*SPI1::ptr() };

    // DS: 8-bit data size
    // SSOE: Slave Select output disabled
    spi.cr2.write(|w| unsafe {
        w.ds().bits(0b111).ssoe().clear_bit()
    });

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

    let br = match pclk1 / freq.into().0 {
        0 => unreachable!(),
        1..=2 => 0b000,
        3..=5 => 0b001,
        6..=11 => 0b010,
        12..=23 => 0b011,
        24..=39 => 0b100,
        40..=95 => 0b101,
        96..=191 => 0b110,
        _ => 0b111,
    };

    // CPHA: 0
    // CPOL: 0
    // MSTR: master mode
    // BR: 1 MHz
    // SPE: SPI enabled
    // LSBFIRST: MSB first
    // SSM: enable software slave management (NSS pin free for other uses)
    // SSI: set nss high = master mode
    // CRCEN: hardware CRC calculation disabled
    // BIDIMODE: 2 line unidirectional (full duplex)
    spi.cr1.write(|w| unsafe {
        w.cpha()
            .bit(false)
            .cpol()
            .bit(false)
            .mstr()
            .set_bit()
            .br()
            .bits(br)
            .spe()
            .set_bit()
            .lsbfirst()
            .clear_bit()
            .ssi()
            .set_bit()
            .ssm()
            .set_bit()
            .crcen()
            .clear_bit()
            .bidimode()
            .clear_bit()
    });
}

pub fn spi1_send(data: u8) {
    unsafe { ptr::write_volatile(&(*SPI1::ptr()).dr as *const _ as *mut u8, data) };
}