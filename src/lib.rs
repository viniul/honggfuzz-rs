//! ## About Honggfuzz
//! Honggfuzz is a security oriented fuzzer with powerful analysis options. Supports evolutionary, feedback-driven fuzzing based on code coverage (software- and hardware-based)
//!   * project homepage [honggfuzz.com](http://honggfuzz.com/)
//!   * project repository [github.com/google/honggfuzz](https://github.com/google/honggfuzz)
//!   * this upstream project is maintained by Google, but ...
//!   * this is NOT an official Google product
//!  
//! ### Description (from upstream project)
//!   * It's __multi-process__ and __multi-threaded__: no need to run multiple copies of your fuzzer, as honggfuzz can unlock potential of all your available CPU cores with one process. The file corpus is automatically shared and improved between the fuzzing threads.
//!   * It's blazingly fast when in the [persistent fuzzing mode](https://github.com/google/honggfuzz/blob/master/docs/PersistentFuzzing.md)). A simple/empty _LLVMFuzzerTestOneInput_ function can be tested with __up to 1mo iterations per second__ on a relatively modern CPU (e.g. i7-6700K)
//!   * Has a [solid track record](#trophies) of uncovered security bugs: the __only__ (to the date) __vulnerability in OpenSSL with the [critical](https://www.openssl.org/news/secadv/20160926.txt) score mark__ was discovered by honggfuzz. See the [Trophies](#trophies) paragraph for the summary of findings to the date
//!   * Uses low-level interfaces to monitor processes (e.g. _ptrace_ under Linux). As opposed to other fuzzers, it __will discover and report hijacked/ignored signals__ (intercepted and potentially hidden by signal handlers)
//!   * Easy-to-use, feed it a simple corpus directory (can even be empty) and it will work its way up expanding it utilizing feedback-based coverage metrics
//!   * Supports several (more than any other coverage-based feedback-driven fuzzer) hardware-based (CPU: branch/instruction counting, __Intel BTS__, __Intel PT__) and software-based [feedback-driven fuzzing](https://github.com/google/honggfuzz/blob/master/docs/FeedbackDrivenFuzzing.md) methods known from other fuzzers (libfuzzer, afl)
//!   * Works (at least) under GNU/Linux, FreeBSD, Mac OS X, Windows/CygWin and [Android](https://github.com/google/honggfuzz/blob/master/docs/Android.md)
//!   * Supports the __persistent fuzzing mode__ (long-lived process calling a fuzzed API repeatedly) with libhfuzz/libhfuzz.a. More on that can be found [here](https://github.com/google/honggfuzz/blob/master/docs/PersistentFuzzing.md)
//!   * [Can fuzz remote/standalone long-lasting processes](https://github.com/google/honggfuzz/blob/master/docs/AttachingToPid.md) (e.g. network servers like __Apache's httpd__ and __ISC's bind__), though the [persistent fuzzing mode](https://github.com/google/honggfuzz/blob/master/docs/PersistentFuzzing.md) is suggested instead: as it's faster and multiple instances of a service can be fuzzed with this
//!   * It comes with the __[examples](https://github.com/google/honggfuzz/tree/master/examples) directory__, consisting of real world fuzz setups for widely-used software (e.g. Apache and OpenSSL)
//! 
//! ## How to use this crate
//! Install honggfuzz commands to build with instrumentation and fuzz
//! ```sh
//! cargo install honggfuzz # installs hfuzz-build, hfuzz-clean and honggfuzz subcommands in cargo
//! ```
//! Add to your dependencies
//! ```toml
//! [dependencies]
//! honggfuzz = "0.3"
//! ```
//! Create a target to fuzz
//! ```rust
//! #[macro_use] extern crate honggfuzz;
//! 
//! fn main() {
//!     // Here you can parse `std::env::args and 
//!     // setup / initialize your project
//! 
//!     // You have full control over the loop but
//!     // you're supposed to call `fuzz` ad vitam aeternam
//!     loop {
//!         // The fuzz macro gives an arbitrary object (see `arbitrary crate`)
//!         // to a closure-like block of code.
//!         // For performance, it is recommended that you use the native type
//!         // `&[u8]` when possible.
//!         // Here, this slice will contain a "random" quantity of "random" data.
//!         fuzz!(|data: &[u8]| {
//!             if data.len() != 10 {return}
//!             if data[0] != 'q' as u8 {return}
//!             if data[1] != 'w' as u8 {return}
//!             if data[2] != 'e' as u8 {return}
//!             if data[3] != 'r' as u8 {return}
//!             if data[4] != 't' as u8 {return}
//!             if data[5] != 'y' as u8 {return}
//!             if data[6] != 'u' as u8 {return}
//!             if data[7] != 'i' as u8 {return}
//!             if data[8] != 'o' as u8 {return}
//!             if data[9] != 'p' as u8 {return}
//!             panic!("BOOM")
//!         });
//!     }
//! }
//! 
//! ```
//! Build with instrumentation
//! ```sh
//! # a wrapper on "cargo build" with fuzzing instrumentation enabled.
//! # produces binaries in "fuzzing_target" directory
//! cargo hfuzz-build
//! ```
//! 
//! Fuzz
//! ```sh
//! mkdir -p workspace/input
//! # a wrapper on honggfuzz executable with settings adapted to work with Rust code
//! cargo honggfuzz -W workspace -f workspace/input -P -- fuzzing_target/x86_64-unknown-linux-gnu/debug/example
//! ```
//! 
//! Clean
//! ```sh
//! # a wrapper on "cargo clean" which cleans the fuzzing_target directory
//! cargo hfuzz-clean 
//! ```
//! 
//! ## Relevant documentation about honggfuzz usage
//!   * [USAGE](https://github.com/google/honggfuzz/blob/master/docs/USAGE.md)
//!   * [FeedbackDrivenFuzzing](https://github.com/google/honggfuzz/blob/master/docs/FeedbackDrivenFuzzing.md)
//!   * [PersistentFuzzing](https://github.com/google/honggfuzz/blob/master/docs/PersistentFuzzing.md)
//! 
//! ## About Rust fuzzing
//! 
//! There is other projects providing Rust fuzzing support at [github.com/rust-fuzz](https://github.com/rust-fuzz). 
//! 
//! You'll find support for [AFL](https://github.com/rust-fuzz/afl.rs) and LLVM's [LibFuzzer](https://github.com/rust-fuzz/cargo-fuzz) and there is also a [trophy case](https://github.com/rust-fuzz/trophy-case) ;-) .
//! 
//! This crate was inspired by those projects!

extern "C" {
    fn HF_ITER(buf_ptr: *mut *const u8, len_ptr: *mut usize );
}

pub fn fuzz<F>(closure: F) where F: Fn(&[u8]) {
    let buf;
    unsafe {
        let mut buf_ptr: *const u8 = std::mem::uninitialized();
        let mut len_ptr: usize = std::mem::uninitialized();
        HF_ITER(&mut buf_ptr, &mut len_ptr);
        buf = ::std::slice::from_raw_parts(buf_ptr, len_ptr);
    }
    closure(buf);
}

#[macro_export]
macro_rules! fuzz {
    (|$buf:ident| $body:block) => {
        honggfuzz::fuzz(|$buf| $body);
    };
    (|$buf:ident: &[u8]| $body:block) => {
        honggfuzz::fuzz(|$buf| $body);
    };
    (|$buf:ident: $dty: ty| $body:block) => {
        honggfuzz::fuzz(|$buf| {
            let $buf: $dty = {
                use arbitrary::{Arbitrary, RingBuffer};
                if let Ok(d) = RingBuffer::new($buf, $buf.len()).and_then(|mut b|{
                        Arbitrary::arbitrary(&mut b).map_err(|_| "")
                    }) {
                    d
                } else {
                    return
                }
            };

            $body
        });
    };
}