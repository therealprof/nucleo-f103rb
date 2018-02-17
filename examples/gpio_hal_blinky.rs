#![feature(used)]
#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate nb;
extern crate stm32f103xx_hal as hal;

use hal::prelude::*;
use hal::stm32f103xx;
use hal::timer::Timer;

fn main() {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    let mut timer = Timer::syst(cp.SYST, 1.hz(), clocks);

    loop {
        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
    }
}
