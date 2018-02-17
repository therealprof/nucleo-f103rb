#![no_std]
#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(use_extern_macros))]
#![cfg_attr(feature = "rt", feature(used))]
#![feature(const_fn)]
#![allow(non_camel_case_types)]

pub extern crate stm32f103xx_hal as hal;
pub extern crate stm32f103xx;

extern crate bare_metal;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate vcell;

pub use stm32f103xx::*;
pub use stm32f103xx::interrupt::*;
pub use cortex_m_rt::*;
pub use cortex_m::*;
pub use hal::*;
