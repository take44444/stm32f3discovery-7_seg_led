#![no_main]
#![no_std]

extern crate panic_semihosting; // panic handler
extern crate cortex_m;
extern crate f3;
extern crate embedded_hal as hal;
extern crate cast;

mod timer;
mod shift;

use cortex_m_rt::entry;
use f3::hal::prelude::*;

#[entry]
fn main() -> ! {
    let (mut timer, mut shift_reg): (timer::Timer, shift::ShiftReg) = shift::init();
    loop {
        for i in 10..200 {
            timer.change_period(i.hz());
            
            shift_reg.select_digit(3);
            shift_reg.display_num(1);
            wait!(timer.sr_uif_is_set());
    
            shift_reg.select_digit(2);
            shift_reg.display_num(2);
            wait!(timer.sr_uif_is_set());
    
            shift_reg.select_digit(1);
            shift_reg.display_num(3);
            wait!(timer.sr_uif_is_set());
    
            shift_reg.select_digit(0);
            shift_reg.display_num(4);
            wait!(timer.sr_uif_is_set());
        }
    }
}
