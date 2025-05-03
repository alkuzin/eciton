// Eciton - experimental exokernel.
// Copyright (C) 2025 Alexander (@alkuzin).
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Main kernel tests module. Responsible for initializing test framework.

#[allow(unused_imports)]
pub use eciton_sdk::{
    exotest_register_handlers, exotest_test_cases, exotest_ignore,
    exotest_run, exotest, exotest::{Test, TestSuite},
};

exotest! {
    use core::panic::PanicInfo;
    use crate::printk;

    exotest_register_handlers!(
        // Function to log before all tests are running.
        |name: &str| {
            printk!("[TEST ] Running tests for module: <{}>", name);
        },

        // Function to log when a test is running.
        |name: &str| {
            printk!("[TEST ] Running test <{}>", name);
        },

        // Function to log when a test has passed.
        |name: &str| {
            printk!("[TEST ] Test <{name}> passed [OK]");
        },

        // Panic handler for when a test fails.
        |info: &PanicInfo| -> ! {
            let location = info.location().unwrap();

            // TODO: add pr_test!.
            printk!("[TEST ] Test failed in file: '{}' on line: {} at column: {}",
                location.file(), location.line(), location.column()
            );

            loop {}
        }
    );
}
