//! Manages the imput part of the software

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

enum BlinkerState {
    Off,
    Right,
    Left
}

enum LightsState {
    Off,
    Code,
    Full
}

pub struct ControlData {
    blinker: BlinkerState,
    lights: LightsState,
    auto: bool
}

impl ControlData {
    pub fn new(auto_enabled: bool) -> Self {
        Self {
            blinker: Off,
            lights: Off,
            auto: auto_enabled
        }
    }

    pub fn set_blinker(&mut self, value: BlinkerState) {
        self.blinker = value;
    }

    pub fn set_lights(&mut self, value: LightsState) {
        self.lights = value;
    }

    pub fn set_auto(&mut self, value: bool) {
        self.blinker = value;
    }

    pub fn to_byte(&self) -> u8 {
        let mut byte = 0u8;

        match self.blinker {
            Off => byte &= !0b11,
            Right => {
                byte &= !(1<<0);
                byte |= 1<<1;
            },
            Left => {
                byte &= !(1<<1);
                byte |= 1<<0;
            }
        }

        if self.lights == Off {
            byte &= !(1<<3);
        }
        else {
            byte |= 1<<3;
        }

        match self.lights {
            Code => byte &= !(1<<2),
            Full => byte |= 1<<2,
            _ => ()
        }

        return byte;
    }
}

pub static CONTROLS: Mutex<RefCell<ControlData>> = Mutex::new(
    RefCell::new(
        ControlData::new(false)
        )
    );
