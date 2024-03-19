//! Demonstrate the use of a blocking `Delay` using the SYST (sysclock) timer.

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m::interrupt::free;
use cortex_m_rt::entry;
use stm32l1xx_hal as hal;

use hal::stm32::Peripherals;

use rtt_target::{rprintln, rtt_init_print};

mod acquisition;
mod data_transfer;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = Peripherals::take().expect("Peripherals unavailable");

    acquisition::init(dp.GPIOC);
    data_transfer::init(dp.GPIOA, dp.RCC, dp.USART2);

    let mut last_acquisition_data = 0;

    loop {
        data_transfer::transfer_data(0b01010101);
        rprintln!("test");
        free(|cs| {
            let data = acquisition::serialize(cs);

            if data != last_acquisition_data {
                data_transfer::transfer_data(data);
            }

            last_acquisition_data = data;
        });
    }
}
