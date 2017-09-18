// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! disassemble-bpf
//!
//! Utility for disassembling and inspecting eBPF bytecode.

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

extern crate disassemble;

use disassemble::{Address, Function, Symbol};

fn main() {
    let prog = &[
            0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r0, 0
            0x79, 0x12, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, // load ptr from r1[0x40] to r2
            0x07, 0x02, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, // add r2, 5
            0x79, 0x11, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, // load ptr from r1[0x50] to r1
            0x2d, 0x12, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, // if r2 > r1 skip 3 instructions
            0x71, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // load r2 (= *(mem + 5)) into r0
            0x67, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, // r0 >>= 56
            0xc7, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, // r0 <<= 56 (arsh)
            0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
        ];

    let f = Function::from_bpf(Symbol::new(Address::new(100000), Some("test")), prog);
    for i in f.instructions {
        println!("{:?}", i);
    }
}
