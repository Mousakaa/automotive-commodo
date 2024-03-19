use crate::hal::{
    prelude::*,
    rcc,
    serial::{self, Tx},
    stm32::{GPIOA, RCC, USART2},
};
use core::{cell::RefCell, fmt::Write};
use cortex_m::interrupt::{free, Mutex};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use stm32l1xx_hal::{
    gpio::{gpioa::PA5, Output, PushPull},
    serial::SerialExt,
};

static UART_TX: Mutex<RefCell<Option<Tx<USART2>>>> = Mutex::new(RefCell::new(None));
static LED: Mutex<RefCell<Option<PA5<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

pub fn init(gpioa: GPIOA, rcc: RCC, usart2: USART2) {
    let gpioa = gpioa.split();

    let tx_pin = gpioa.pa2;
    let rx_pin = gpioa.pa3;

    let mut rcc = rcc.freeze(rcc::Config::hsi());

    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_high().unwrap();

    free(|cs| LED.borrow(cs).replace(Some(led)));

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

    let (mut tx, _) = serial.split();
    tx.write(0b01010101).expect("ckc");

    free(|cs| UART_TX.borrow(cs).replace(Some(tx)));
}

pub fn transfer_data(data: u8) {
    free(|cs| {
        if let Some(tx) = UART_TX.borrow(cs).borrow_mut().as_mut() {
            writeln!(tx, "{}", data).expect("UART write failed");
            tx.write(data).expect("UART wirte failed");
        }

        if let Some(led) = LED.borrow(cs).borrow_mut().as_mut() {
            if led.is_low().unwrap() {
                for _ in 0..100_000 {
                    led.set_high().unwrap();
                }
            } else {
                for _ in 0..100_000 {
                    led.set_low().unwrap();
                }
            }
        }
    });
}
