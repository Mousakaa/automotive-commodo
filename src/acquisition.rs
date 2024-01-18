//! Manages the imput part of the software

#![allow(clippy::empty_loop, unsafe_code)]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m::interrupt::{free, CriticalSection, Mutex};
use stm32l1xx_hal as hal;

use crate::hal::{gpio, pac, prelude::*};

use core::cell::RefCell;

static LB_SWITCH: Mutex<RefCell<Option<gpio::PC10<gpio::Input>>>> = Mutex::new(RefCell::new(None));
static RB_SWITCH: Mutex<RefCell<Option<gpio::PC11<gpio::Input>>>> = Mutex::new(RefCell::new(None));
static ON_SWITCH: Mutex<RefCell<Option<gpio::PC12<gpio::Input>>>> = Mutex::new(RefCell::new(None));
static HIGH_SWITCH: Mutex<RefCell<Option<gpio::PC13<gpio::Input>>>> = Mutex::new(RefCell::new(None));
static AUTO_SWITCH: Mutex<RefCell<Option<gpio::PC14<gpio::Input>>>> = Mutex::new(RefCell::new(None));
static LIGHT_SENSOR: Mutex<RefCell<Option<gpio::PC15<gpio::Input>>>> = Mutex::new(RefCell::new(None));

pub fn init(dp_gpioc: pac::GPIOC, dp_syscfg: pac::SYSCFG, exti: &mut pac::EXTI) {
    let gpioc = dp_gpioc.split();
    let mut syscfg = dp_syscfg.constrain();

    let mut left_blink = gpioc.pc10.into_pull_up_input();
    left_blink.make_interrupt_source(&mut syscfg);
    left_blink.enable_interrupt(exti);
    left_blink.trigger_on_edge(exti, hal::gpio::Edge::RisingFalling);

    let mut right_blink = gpioc.pc11.into_pull_up_input();
    right_blink.make_interrupt_source(&mut syscfg);
    right_blink.enable_interrupt(exti);
    right_blink.trigger_on_edge(exti, hal::gpio::Edge::RisingFalling);

    let mut lights_on = gpioc.pc12.into_pull_up_input();
    lights_on.make_interrupt_source(&mut syscfg);
    lights_on.enable_interrupt(exti);
    lights_on.trigger_on_edge(exti, hal::gpio::Edge::RisingFalling);

    let mut lights_high = gpioc.pc13.into_pull_up_input();
    lights_high.make_interrupt_source(&mut syscfg);
    lights_high.enable_interrupt(exti);
    lights_high.trigger_on_edge(exti, hal::gpio::Edge::RisingFalling);

    let mut lights_auto = gpioc.pc14.into_pull_up_input();
    lights_auto.make_interrupt_source(&mut syscfg);
    lights_auto.enable_interrupt(exti);
    lights_auto.trigger_on_edge(exti, hal::gpio::Edge::RisingFalling);

    let mut light_sensor = gpioc.pc15.into_floating_input();
    light_sensor.make_interrupt_source(&mut syscfg);
    light_sensor.enable_interrupt(exti);
    light_sensor.trigger_on_edge(exti, hal::gpio::Edge::RisingFalling);

    // ENable interrupt vectors
    pac::NVIC::unpend(left_blink.interrupt());
    pac::NVIC::unpend(right_blink.interrupt());
    pac::NVIC::unpend(lights_on.interrupt());
    pac::NVIC::unpend(lights_high.interrupt());
    pac::NVIC::unpend(lights_auto.interrupt());
    pac::NVIC::unpend(light_sensor.interrupt());
    unsafe {
        pac::NVIC::unmask(left_blink.interrupt());
        pac::NVIC::unmask(right_blink.interrupt());
        pac::NVIC::unmask(lights_on.interrupt());
        pac::NVIC::unmask(lights_high.interrupt());
        pac::NVIC::unmask(lights_auto.interrupt());
        pac::NVIC::unmask(light_sensor.interrupt());
    }

    free(|cs| {
        LB_SWITCH.borrow(cs).replace(Some(left_blink));
        RB_SWITCH.borrow(cs).replace(Some(right_blink));
        ON_SWITCH.borrow(cs).replace(Some(lights_on));
        HIGH_SWITCH.borrow(cs).replace(Some(lights_high));
        AUTO_SWITCH.borrow(cs).replace(Some(lights_auto));
        LIGHT_SENSOR.borrow(cs).replace(Some(light_sensor));
    });
}

pub fn send_byte(cs: &CriticalSection) -> u8 {
    let left_blinker_mut = LB_SWITCH.borrow(cs).borrow();
    let left_blinker = left_blinker_mut
        .as_ref()
        .expect("Left blinker uninitialized");

    let right_blinker_mut = RB_SWITCH.borrow(cs).borrow();
    let right_blinker = right_blinker_mut
        .as_ref()
        .expect("Right blinker uninitialized");

    let on_switch_mut = ON_SWITCH.borrow(cs).borrow();
    let on_switch = on_switch_mut
        .as_ref()
        .expect("Lights ON switch uninitialized");

    let high_switch_mut = HIGH_SWITCH.borrow(cs).borrow();
    let high_switch = high_switch_mut
        .as_ref()
        .expect("High beam switch uninitialized");

    let auto_switch_mut = AUTO_SWITCH.borrow(cs).borrow();
    let auto_switch = auto_switch_mut
        .as_ref()
        .expect("Auto mode switch uninitialized");

    let light_sensor_mut = LIGHT_SENSOR.borrow(cs).borrow();
    let light_sensor = light_sensor_mut
        .as_ref()
        .expect("Light sensor uninitialized");

    left_blinker.is_low() as u8                                                  // Left blinker value
        | ((right_blinker.is_low() as u8) << 1)                                  // Right blinker value
        | ((on_switch.is_low()                                                   // Switch lights on or off
            || (auto_switch.is_low() && light_sensor.is_low())) as u8)  << 2     // Even in auto mode
        | ((high_switch.is_low() as u8) << 3) // High or low beam lights
}
