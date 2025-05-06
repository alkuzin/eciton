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

//! Main libOS tests module. Responsible for initializing test framework.

#[allow(unused_imports)]
pub use eciton_sdk::{
    exotest_register_handlers, exotest_test_cases, exotest_ignore,
    exotest_run, exotest_custom_run, exotest_run_modules, exotest,
    exotest::{Test, TestSuite},
};

#[cfg(feature = "exotest")]
pub use tests::*;

exotest! {
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
        }
    );
}
