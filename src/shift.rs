use super::timer::Timer;

use core::ptr;
use stm32f30x_hal::{
    gpio::{
        gpioa::PAx,
        gpiod::PDx,
        Output,
        PushPull,
    },
    prelude::*,
    spi::Spi,
    stm32f30x::{self, SPI1},
};
use hal::spi::{Mode, Phase, Polarity};

static NUMBERS: [u8; 11] = [
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
    pub digits: [PDx<Output<PushPull>>; 4],
    pub rck: PAx<Output<PushPull>>,
}

pub fn init() -> (Timer, ShiftReg) {
    let dp = stm32f30x::Peripherals::take().unwrap();
    
    let mut rcc = dp.RCC.constrain();

    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpiod = dp.GPIOD.split(&mut rcc.ahb);

    let timer = Timer::tim2(100.hz(), clocks);

    let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
 
    Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        Mode {
            phase: Phase::CaptureOnFirstTransition,
            polarity: Polarity::IdleLow,
        },
        1.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut rck = gpioa
        .pa4
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let mut dig1 = gpiod
        .pd1
        .into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let mut dig2 = gpiod
        .pd2
        .into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let mut dig3 = gpiod
        .pd3
        .into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let mut dig4 = gpiod
        .pd4
        .into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);

    #[allow(deprecated)]
    {
        rck.set_high();
        dig1.set_low();
        dig2.set_low();
        dig3.set_low();
        dig4.set_low();
    }
    
    (
        timer, 
        ShiftReg {
            digits: [
                dig1.downgrade(),
                dig2.downgrade(),
                dig3.downgrade(),
                dig4.downgrade()
            ],
            rck: rck.downgrade(),
        }
    )
}

impl ShiftReg {
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