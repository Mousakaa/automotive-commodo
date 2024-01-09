use crate::hal::{
    pac::{Peripherals, GPIOA, RCC, USART2},
    prelude::*,
    rcc::Clocks,
    serial::{self, Config, Tx},
};
use core::{cell::RefCell, fmt::Write};
use cortex_m::interrupt::{free, Mutex};

static UART_TX: Mutex<RefCell<Option<Tx<USART2, u8>>>> = Mutex::new(RefCell::new(None));

pub fn init(gpioa: GPIOA, rcc: RCC, usart2: USART2) {
    let gpioa = gpioa.split();

    let tx_pin = gpioa.pa2.into_alternate();

    let rcc = rcc.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    let tx: serial::Tx<USART2, u8> = usart2
        .tx(
            tx_pin,
            Config::default()
                .baudrate(115200.bps())
                .wordlength_8()
                .parity_none(),
            &clocks,
        )
        .unwrap();

    free(|cs| *UART_TX.borrow(cs).borrow_mut() = Some(tx));
}

pub fn transfer_data(data: u8) {
    free(|cs| {
        if let Some(tx) = UART_TX.borrow(cs).borrow_mut().as_mut() {
            writeln!(tx, "{}", data).expect("UART write failed");
        }
    });
}
