//! Demonstrate the use of a blocking `Delay` using the SYST (sysclock) timer.

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::pac;

mod acquisition;
mod data_transfer;

// fn init(dp: &mut Peripherals) {
//     data_transfer::init(dp);
//     acquisition::init(dp);
// }

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().expect("Peripherals unavailable");
    // init(&mut dp);

    data_transfer::init(dp.GPIOA, dp.RCC, dp.USART2);

    loop {}
}
