# Intro to Rust

These are the code examples I used in the March 9 talk at OSC.  They are intended to show how the Rust compiler rejects memory-related bugs that are typically uncaught in other languages.  Here is a breakdown of the examples:

* `lifetime.cpp` demonstrates a perfectly valid C++ program that uses an invalid pointer.  `lifetime.rs` is the equivalent Rust code that is rejected by the compiler.
* `ownership.cpp` demonstrates a perfectly valid C++ program that makes use of deallocated memory.  `ownership.rs` is the equivalent Rust code that is rejected by the compiler.
* `concurrency.cpp` demonstrates a perfectly valid C++ program that contains a data race.  `concurrency.rs` is the equivalent Rust code that is rejected by the compiler.  A valid version of the Rust code is commented out in `concurrency.rs`

Note that the examples are necessarily contrived.
