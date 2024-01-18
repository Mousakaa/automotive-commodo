//! Demonstrate the use of a blocking `Delay` using the SYST (sysclock) timer.

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m::interrupt::free;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{interrupt, pac};

mod acquisition;
mod data_transfer;

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().expect("Peripherals unavailable");

    acquisition::init(dp.GPIOC, dp.SYSCFG, &mut dp.EXTI);
    data_transfer::init(dp.GPIOA, dp.RCC, dp.USART2);

    loop {}
}

#[interrupt]
fn EXTI15_10() {
    free(|cs| {
        data_transfer::transfer_data(acquisition::send_byte(cs));
    });
}
