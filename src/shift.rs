use core::ptr;
use stm32f30x::{self, SPI1};

use super::gpio;
use super::hertz::*;
use super::pin;
use super::spi;
use super::timer;

const NUMBERS: [u8; 11] = [
    0b11101011, // 0
    0b00101000, // 1
    0b10110011, // 2
    0b10111010, // 3
    0b01111000, // 4
    0b11011010, // 5
    0b11011011, // 6
    0b11101000, // 7
    0b11111011, // 8
    0b11111010, // 9
    0b00000000  // OFF
];

pub struct ShiftReg {
    pub digits: [pin::PDxL<pin::OutputPushPull>; 4],
    pub rck: pin::PAxL<pin::OutputPushPull>,
}

impl ShiftReg {
    pub fn new() -> Self {
        gpio::activate_gpioa();
        gpio::activate_gpiod();
    
        timer::tim2(100.hz());
    
        pin::PAxL::<pin::Floating>::new(5).mode_af5();
        pin::PAxL::<pin::Floating>::new(6).mode_af5();
        pin::PAxL::<pin::Floating>::new(7).mode_af5();
     
        spi::spi1(1.mhz());
    
        let rck = pin::PAxL::<pin::Floating>::new(4).mode_push_pull_output();
    
        let dig1 = pin::PDxL::<pin::Floating>::new(1).mode_push_pull_output();
        let dig2 = pin::PDxL::<pin::Floating>::new(2).mode_push_pull_output();
        let dig3 = pin::PDxL::<pin::Floating>::new(3).mode_push_pull_output();
        let dig4 = pin::PDxL::<pin::Floating>::new(4).mode_push_pull_output();
    
        #[allow(deprecated)]
        {
            rck.set_high();
            dig1.set_low();
            dig2.set_low();
            dig3.set_low();
            dig4.set_low();
        }
        
        ShiftReg {
            digits: [
                dig1,
                dig2,
                dig3,
                dig4
            ],
            rck: rck,
        }
    }

    #[allow(deprecated)]
    pub fn display_num(&mut self, number: usize) {
        self.rck.set_low();
        unsafe { ptr::write_volatile(&(*SPI1::ptr()).dr as *const _ as *mut u8, NUMBERS[number]) }
        self.rck.set_high();
    }

    #[allow(deprecated)]
    pub fn select_digit(&mut self, number: usize) {
        for i in 0..4 {
            if i == number {
                self.digits[number].set_low();
            } else {
                self.digits[i].set_high();
            }
        }
    }
}