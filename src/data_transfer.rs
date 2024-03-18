use crate::hal::{
    prelude::*,
    rcc,
    serial::{self, Tx},
    stm32::{GPIOA, RCC, USART2},
};
use core::{cell::RefCell, fmt::Write};
use cortex_m::interrupt::{free, Mutex};
use stm32l1xx_hal::serial::SerialExt;

static UART_TX: Mutex<RefCell<Option<Tx<USART2>>>> = Mutex::new(RefCell::new(None));

pub fn init(gpioa: GPIOA, rcc: RCC, usart2: USART2) {
    let gpioa = gpioa.split();

    let tx_pin = gpioa.pa2;
    let rx_pin = gpioa.pa3;

    let mut rcc = rcc.freeze(rcc::Config::hsi());

    let serial = usart2
        .usart(
            (tx_pin, rx_pin),
            serial::Config::default()
                .baudrate(115200.bps())
                .wordlength_8()
                .parity_none(),
            &mut rcc,
        )
        .expect("Couldnt initialize USART2");

    let (tx, _) = serial.split();

    free(|cs| UART_TX.borrow(cs).replace(Some(tx)));
}

pub fn transfer_data(data: u8) {
    free(|cs| {
        if let Some(tx) = UART_TX.borrow(cs).borrow_mut().as_mut() {
            writeln!(tx, "{}", data).expect("UART write failed");
        }
    });
}
