//! Manages the imput part of the software

#![allow(clippy::empty_loop, unsafe_code)]

use embedded_hal::digital::v2::InputPin;
// Halt on panic
use panic_halt as _; // panic handler

use cortex_m::interrupt::{free, CriticalSection, Mutex};

use crate::hal::{
    gpio::{gpioc, Floating, GpioExt, Input, PullUp},
    stm32::GPIOC,
};

use core::cell::RefCell;

static LB_SWITCH: Mutex<RefCell<Option<gpioc::PC10<Input<PullUp>>>>> =
    Mutex::new(RefCell::new(None));
static RB_SWITCH: Mutex<RefCell<Option<gpioc::PC11<Input<PullUp>>>>> =
    Mutex::new(RefCell::new(None));
static ON_SWITCH: Mutex<RefCell<Option<gpioc::PC12<Input<PullUp>>>>> =
    Mutex::new(RefCell::new(None));
static HIGH_SWITCH: Mutex<RefCell<Option<gpioc::PC13<Input<PullUp>>>>> =
    Mutex::new(RefCell::new(None));
static AUTO_SWITCH: Mutex<RefCell<Option<gpioc::PC14<Input<PullUp>>>>> =
    Mutex::new(RefCell::new(None));
static LIGHT_SENSOR: Mutex<RefCell<Option<gpioc::PC15<Input<Floating>>>>> =
    Mutex::new(RefCell::new(None));

pub fn init(dp_gpioc: GPIOC) {
    let gpioc = dp_gpioc.split();

    let left_blink = gpioc.pc10.into_pull_up_input();
    let right_blink = gpioc.pc11.into_pull_up_input();
    let lights_on = gpioc.pc12.into_pull_up_input();
    let lights_high = gpioc.pc13.into_pull_up_input();
    let lights_auto = gpioc.pc14.into_pull_up_input();
    let light_sensor = gpioc.pc15.into_floating_input();

    free(|cs| {
        LB_SWITCH.borrow(cs).replace(Some(left_blink));
        RB_SWITCH.borrow(cs).replace(Some(right_blink));
        ON_SWITCH.borrow(cs).replace(Some(lights_on));
        HIGH_SWITCH.borrow(cs).replace(Some(lights_high));
        AUTO_SWITCH.borrow(cs).replace(Some(lights_auto));
        LIGHT_SENSOR.borrow(cs).replace(Some(light_sensor));
    });
}

pub fn serialize(cs: &CriticalSection) -> u8 {
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

    left_blinker.is_low().unwrap() as u8                                                  // Left blinker value
        | ((right_blinker.is_low().unwrap() as u8) << 1)                                  // Right blinker value
        | ((on_switch.is_low().unwrap()                                                   // Switch lights on or off
            || (auto_switch.is_low().unwrap() && light_sensor.is_high().unwrap())) as u8) << 2     // Even in auto mode
        | ((high_switch.is_low().unwrap() as u8) << 3) // High or low beam lights
}
