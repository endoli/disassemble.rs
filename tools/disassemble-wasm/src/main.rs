// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! disassemble-wasm
//!
//! Utility for disassembling and inspecting eBPF bytecode.

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

extern crate clap;
extern crate disassemble;

use disassemble::Module;

fn main() {
    let matches = clap::App::new("disassemble-wasm")
        .version("0.0.1")
        .author("Bruce Mitchener, Jr.")
        .about("Disassemble binary WebAssembly files")
        .arg(
            clap::Arg::with_name("FILE")
                .help("The binary WebAssembly file to use.")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = matches.value_of("FILE").unwrap();

    if let Some(m) = Module::from_wasm_file(file) {
        for f in m.functions {
            for i in f.instructions {
                println!("{}", i);
            }
        }
    }
}
