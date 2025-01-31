// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>

//! A panic handler that infinitely waits.

use crate::arch::cpu_core;
use crate::log::console;
use core::{fmt, panic::PanicInfo};

fn _panic_print(args: fmt::Arguments) {
    use fmt::Write;

    unsafe { console::panic_console_out().write_fmt(args).unwrap() };
}

/// Prints with a newline - only use from the panic handler.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! panic_println {
    ($($arg:tt)*) => ({
        _panic_print(format_args_nl!($($arg)*));
    })
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let (Some(args), Some(location)) = (
        info.message(),
        info.location(),
    ) {
        panic_println!(
            "\nKernel panic: occurred in file '{}' at line {}, msg: {}",
            location.file(),
            location.line(),
            args,
        );
    } else {
        panic_println!("\nKernel panic!");
    }

    cpu_core::wait_forever()
}
