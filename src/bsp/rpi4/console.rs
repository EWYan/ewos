// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2022 Andre Richter <andre.o.richter@gmail.com>

//! BSP console facilities.

// use crate::console;
use core::fmt;
use super::uart;

pub struct UARTOutput;

impl fmt::Write for UARTOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            uart::uart_write_byte(c as u8);
        }

        Ok(())
    }
}

