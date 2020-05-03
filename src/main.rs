#![no_main]
#![no_std]

extern crate cast;
extern crate panic_semihosting; // panic handler
extern crate stm32f30x;

use cortex_m_rt::entry;

mod gpio;
mod hertz;
mod pin;
mod shift;
mod spi;
mod timer;

use crate::hertz::U32Ext;

#[entry]
fn main() -> ! {
    let mut shift_reg = shift::ShiftReg::new();
    loop {
        for i in 10..200 {
            timer::tim2_change_period(i.hz());

            shift_reg.select_digit(3);
            shift_reg.display_num(1);
            wait!(timer::tim2_updated());
    
            shift_reg.select_digit(2);
            shift_reg.display_num(2);
            wait!(timer::tim2_updated());
    
            shift_reg.select_digit(1);
            shift_reg.display_num(3);
            wait!(timer::tim2_updated());
    
            shift_reg.select_digit(0);
            shift_reg.display_num(4);
            wait!(timer::tim2_updated());
        }
    }
}
