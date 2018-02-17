#![feature(used)]
#![feature(const_fn)]
#![no_std]

#[macro_use(block)]
extern crate nb;

extern crate cortex_m;
extern crate stm32f103xx_hal as hal;

use hal::prelude::*;
use hal::serial::*;
use hal::stm32f103xx;

fn main() {
    if let Some(p) = stm32f103xx::Peripherals::take() {
        let mut flash = p.FLASH.constrain();
        let mut rcc = p.RCC.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);
        let mut afio = p.AFIO.constrain(&mut rcc.apb2);

        /* Split GPIO pins */
        let mut gpioa = p.GPIOA.split(&mut rcc.apb2);

        /* Prepare pins USART2 is connected to */
        let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
        let rx = gpioa.pa3;

        /* Setup USART2 which is connected to the on board ST-Link for output */
        let serial = Serial::usart2(
            p.USART2,
            (tx, rx),
            &mut afio.mapr,
            115_200.bps(),
            clocks,
            &mut rcc.apb1,
        );

        let (mut tx, mut rx) = serial.split();

        /* Print a nice hello message */
        let s = b"\r\nPlease type characters to echo:\r\n";

        let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();

        /* Endless loop */
        loop {
            /* Read and echo back */
            if let Ok(c) = block!(rx.read()) {
                let _ = block!(tx.write(c));
            }
        }
    }
}
