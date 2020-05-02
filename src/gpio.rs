use stm32f30x::RCC;

pub fn activate_gpioa() {
    let rcc = unsafe { &*RCC::ptr() }; // 1073876992

    rcc.ahbenr.modify(|_, w| w.iopaen().enabled());
    rcc.ahbrstr.modify(|_, w| w.ioparst().set_bit());
    rcc.ahbrstr.modify(|_, w| w.ioparst().clear_bit());
}

pub fn activate_gpiod() {
    let rcc = unsafe { &*RCC::ptr() }; // 1073876992

    rcc.ahbenr.modify(|_, w| w.iopden().enabled());
    rcc.ahbrstr.modify(|_, w| w.iopdrst().set_bit());
    rcc.ahbrstr.modify(|_, w| w.iopdrst().clear_bit());
}