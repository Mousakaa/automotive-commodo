//! Manages the imput part of the software

#![allow(clippy::empty_loop, unsafe_code)]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m::interrupt::{free, Mutex};
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

use core::cell::RefCell;

#[derive(PartialEq, Eq)]
enum LightsState {
    Off,
    Low,
    High
}

pub struct ControlData {
    left_blinker: bool,
    right_blinker: bool,
    lights: LightsState,
    auto: bool
}

impl ControlData {
    pub fn new(auto_enabled: bool) -> Self {
        Self {
            left_blinker: false,
            right_blinker: false,
            lights: LightsState::Off,
            auto: auto_enabled
        }
    }

    pub fn set_left_blinker(&mut self, value: bool) {
        self.left_blinker = value;
    }

    pub fn set_right_blinker(&mut self, value: bool) {
        self.right_blinker = value;
    }

    pub fn set_lights(&mut self, value: LightsState) {
        self.lights = value;
    }

    pub fn set_auto(&mut self, value: bool) {
        self.auto = value;
    }

    pub fn to_byte(&self) -> u8 {
        let mut byte = 0u8
            | self.left_blinker as u8
            | ((self.right_blinker as u8) << 1);

        if self.lights == LightsState::Off {
            byte &= !(1<<3);
        }
        else {
            byte |= 1<<3;
        }

        match self.lights {
            LightsState::Low => byte &= !(1<<2),
            LightsState::High => byte |= 1<<2,
            _ => ()
        }

        return byte;
    }
}

pub static CONTROLS: Mutex<RefCell<Option<ControlData>>> = Mutex::new(
    RefCell::new(None)
);

pub fn init(dp_gpioc: pac::GPIOC, dp_syscfg: pac::SYSCFG, exti: &mut pac::EXTI) {
    let gpioc = dp_gpioc.split();
    let mut syscfg = dp_syscfg.constrain();

    let mut left_blink = gpioc.pc0.into_pull_up_input();
    left_blink.make_interrupt_source(&mut syscfg);
    left_blink.enable_interrupt(exti);
    left_blink.trigger_on_edge(exti, hal::gpio::Edge::RisingFalling);

    let mut right_blink = gpioc.pc1.into_pull_up_input();
    right_blink.make_interrupt_source(&mut syscfg);
    right_blink.enable_interrupt(exti);
    right_blink.trigger_on_edge(exti, hal::gpio::Edge::RisingFalling);

    let mut lights_on = gpioc.pc2.into_pull_up_input();
    lights_on.make_interrupt_source(&mut syscfg);
    lights_on.enable_interrupt(exti);
    lights_on.trigger_on_edge(exti, hal::gpio::Edge::RisingFalling);

    let mut lights_high = gpioc.pc3.into_pull_up_input();
    lights_high.make_interrupt_source(&mut syscfg);
    lights_high.enable_interrupt(exti);
    lights_high.trigger_on_edge(exti, hal::gpio::Edge::RisingFalling);

    let mut lights_auto = gpioc.pc4.into_pull_up_input();
    lights_auto.make_interrupt_source(&mut syscfg);
    lights_auto.enable_interrupt(exti);
    lights_auto.trigger_on_edge(exti, hal::gpio::Edge::RisingFalling);

    // ENable interrupt vectors
    pac::NVIC::unpend(left_blink.interrupt());
    pac::NVIC::unpend(right_blink.interrupt());
    pac::NVIC::unpend(lights_on.interrupt());
    pac::NVIC::unpend(lights_high.interrupt());
    pac::NVIC::unpend(lights_auto.interrupt());
    unsafe {
        pac::NVIC::unmask(left_blink.interrupt());
        pac::NVIC::unmask(right_blink.interrupt());
        pac::NVIC::unmask(lights_on.interrupt());
        pac::NVIC::unmask(lights_high.interrupt());
        pac::NVIC::unmask(lights_auto.interrupt());
    }

    free(|cs| {
        CONTROLS.borrow(cs).replace(Some(
            ControlData::new(lights_auto.is_high())
        ));
    });
}
