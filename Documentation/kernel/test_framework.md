# Kernel test framework

<!--toc:start-->
- [Kernel test framework](#kernel-test-framework)
  - [Description](#description)
  - [Purpose](#purpose)
  - [How it works](#how-it-works)
  - [How to write test?](#how-to-write-test)
  - [Test behavior](#test-behavior)
  - [Invariants](#invariants)
<!--toc:end-->

## Description

Testing a kernel is very different than testing a userland application. Especially on specific target when you don't have access to cargo test. So not being able to use cargo test, I decided to write my own test framework for the kernel.
The framework is trivial but sufficient.
So the kernel has 2 running modes:

- Release runtime: just run the kernel like a kernel.
- Test runtime: run the kernel in test mode, run all test suites, don't do anything else than just run all test suites.

## Purpose

The purpose of this framework is to make the kernel testable in isolation and to detect regressions early during development.

## How it works

To be able to run different test suites, with different test inside, and the test can have different behavior, I needed to create like three entities:

- TestManager: contains all test suite and the number of test suites to run.
- TestSuite: a test suite, like its name, contains all tests and the number of tests inside.
- TestCase: just a test, define the test's name and a function pointer to the test itself.

This hierarchy enforces a strict ownership model and avoids dynamic discovery or allocation during test execution.

## How to write test?

You can't directly just write a test and expect to run it just by itself, a test must be in a test suite.
Here's an example of how to write a test suite:

```rust
// Create a test file in the tests directory.
// The test directory works as a mirror of the src dir. 
// So if you want to write a test for the driver `src/drivers/serials/foo.rs`, you will create a test file `src/tests/drivers/serials/foo.rs`
// I find it easier to find test for a specific file like that.

// src/tests/drivers/serials/foo.rs

/// All test function must start by test_ (just a convention, it's easier to know that's a test function, anyway)
pub fn test_foo_impl() -> u8 {
    // Write your test...
}

/// Function to create the test suite, and register it to the TestManager
pub fn foo_test_suite() {
    const FOO_TEST_SUITE: TestSuite = TestSuite {
        // Array of all tests inside the suite
        tests: &[TestCase::init(
            "Foo driver basic implementation",
            test_foo_impl,
            TestBehavior::Default,
        )],
        // Name of the test suite
        name: "Foo",
        // Number of tests inside the suite
        tests_nb: 1,
    };
    #[allow(static_mut_refs)]
    // Register the suite inside the TestManager
    // This operation is unsafe because TEST_MANAGER is a global static.
    // It is safe in this context because test suite registration happens during single-threaded early boot.
    unsafe {
        TEST_MANAGER.add_suite(&FOO_TEST_SUITE)
    };
}

// Then, call the foo_test_suite function in the test_suites function

// src/tests/mod.rs

/// Function calling all test suites function 
fn test_suites() {
   // All suites... 

   // Make sure that this function is correctly import in the module. And if the kernel compile, the suite would be register in the TestManager.
   foo_test_suite();
}
```

That's it, not so hard to write a correct test suite.

## Test behavior

A test can have 3 different conclusion:

- The test pass, everything is ok.
- The test fail, test failures are reported, stop the execution of current test suite, but do not stop execution of other test suites.
- Kernel integrity failure, if the test encounter a critical failure, like a device not correctly initialized. This failure indicate that the state after the fail could lead to an unstable kernel. That shouldn't happened, so panic directly.

## Invariants
- All test suites must be registered before test execution starts.
- Test suites are static and must remain valid for the entire test runtime.
- Test code must not rely on subsystems that are not initialized in test mode.
