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

//! Custom exokernel test framework module.

/// Test case struct.
pub struct Test {
    /// Test name (same as test function name).
    pub name: &'static str,
    /// Test function.
    pub test_fn: fn(),
}

/// Tests handling struct.
pub struct TestSuite {
    /// Array of test cases.
    pub tests: &'static [Test],
    /// Passed test handler function pointer.
    pub output_result: fn(&str),
}

impl TestSuite {
    /// Run all tests.
    pub fn run(&self) {
        for test in self.tests {
            // Test failed only when panic occurred.
            (test.test_fn)();
            (self.output_result)(test.name);
        }
    }
}

/// Macro for running all tests.
#[macro_export]
macro_rules! exotest_run {
    () => {
        #[cfg(feature = "exotest")]
        run_tests();
    };
}

/// Macro for custom test runner implementation.
///
/// # Parameters
/// - `block` - given custom test runner contents.
///
/// # Usage
/// ```
///  exotest_custom_run! {
///     // Run tests for other modules.
///     module1::run_tests();
///     module2::run_tests();
///
///     print_some_debug_info(...);
///
///     // Run tests for current module.
///     run_tests();
///  }
/// ```
#[macro_export]
macro_rules! exotest_custom_run {
    ($($block:tt)*) => {
        #[cfg(feature = "exotest")]
        {
            $($block)*
        }
    };
}

/// Macro for compiling its items only during testing.
///
/// # Parameters
/// - `item` - given language item.
#[macro_export]
macro_rules! exotest {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "exotest")]
            $item
        )*
    };
}

/// Macro for not compiling its items during testing.
///
/// # Parameters
/// - `item` - given language item.
#[macro_export]
macro_rules! exotest_ignore {
    ($($item:item)*) => {
        $(
            #[cfg(not(feature = "exotest"))]
            $item
        )*
    };
}

/// Macro to define test cases.
///
/// # Parameters
/// - `test_name` - given name of the test.
/// - `test_body` - given test contents.
///
/// # Usage
/// ```
/// exotest! {
///     fn add(a: i32, b: i32) -> i32 {
///         a + b
///     }
///
///     exotest_test_cases!(
///         test_addition, {
///             assert_eq!(add(1336, 1), 1337);
///         },
///
///         test_addition2, {
///             assert_eq!(add(40, 2), 42);
///         },
///     );
/// }
/// ```
#[macro_export]
macro_rules! exotest_test_cases {
    ($($test_name:ident, $test_body:block),*) => {
        // Generate test functions.
        $(
            fn $test_name() {
                test_running(stringify!($test_name));
                $test_body
            }
        )*

        // Create entry point for running tests.
        pub fn run_tests() {
            // Create the test suite.
            let test_suite = TestSuite {
                output_result: test_passed,
                tests: &[
                    $(Test {
                        name: stringify!($test_name),
                        test_fn: $test_name,
                    },)*
                ],
            };

            test_running_global(module_path!());

            // Run the test suite.
            test_suite.run();
        }
    };
}

/// Macro to register test handlers.
///
/// # Warning
/// - This macro should be called `once` during exokernel or libOS initialization.
///
/// # Parameters
/// - `running_global_body` - given function contents for log before all tests
/// are running.
/// - `running_body` - given function contents for log when a test is running.
/// - `passed_body`  - given function contents for log when a test has passed.
/// - `failed_body`  - given panic handler contents when a test has failed.
///
/// # Usage
/// ```
/// exotest! {
///     exotest_register_handlers!(
///         |name: &str| { ... },
///         |name: &str| { ... },
///         |name: &str| { ... },
///         |info: &PanicInfo| -> ! { ... }
///     );
/// }
/// ```
#[macro_export]
macro_rules! exotest_register_handlers {
    ($running_global_body:expr, $running_body:expr, $passed_body:expr, $failed_body:expr) => {
        // Function to log before all tests are running.
        pub fn test_running_global(name: &str) {
            ($running_global_body)(name);
        }

        // Function to log when a test is running.
        pub fn test_running(name: &str) {
            ($running_body)(name);
        }

        // Function to log when a test has passed.
        pub fn test_passed(name: &str) {
            ($passed_body)(name);
        }

        // Panic handler for when a test fails.
        #[panic_handler]
        pub fn test_failed(info: &PanicInfo) -> ! {
            ($failed_body)(info);
        }
    };
}