#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use nucleo_f103rb as board;

use board::hal::{pac, prelude::*, timer::Timer};
use cortex_m_rt::entry;

use nb::block;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    let mut timer = Timer::tim2(dp.TIM2, &clocks, &mut rcc.apb1).start_count_down(2.hz());

    loop {
        led.toggle().ok();
        block!(timer.wait()).unwrap();
    }
}
