use crate::hal::{
    gpio::{gpioa::PA5, Output, PushPull},
    prelude::*,
    rcc,
    serial::{self, SerialExt, Tx},
    stm32::{GPIOA, RCC, USART1},
};
use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use embedded_hal::digital::v2::OutputPin;

static UART_TX: Mutex<RefCell<Option<Tx<USART1>>>> = Mutex::new(RefCell::new(None));
static LED: Mutex<RefCell<Option<PA5<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

pub fn init(gpioa: GPIOA, rcc: RCC, usart1: USART1) {
    let gpioa = gpioa.split();

    let tx_pin = gpioa.pa9;
    let rx_pin = gpioa.pa10;

    let mut rcc = rcc.freeze(rcc::Config::default());

    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_high().unwrap();

    free(|cs| LED.borrow(cs).replace(Some(led)));

    let serial = usart1
        .usart(
            (tx_pin, rx_pin),
            serial::Config::default()
                .baudrate(115200.bps())
                .wordlength_8()
                .parity_none(),
            &mut rcc,
        )
        .expect("Couldnt initialize USART2");

    let (mut tx, _) = serial.split();
    tx.write(0b01010101).expect("ckc");

    free(|cs| UART_TX.borrow(cs).replace(Some(tx)));
}

pub fn transfer_data(data: u8) {
    free(|cs| {
        if let Some(tx) = UART_TX.borrow(cs).borrow_mut().as_mut() {
            tx.write(data).expect("UART write failed");
        }
    });
}
