# Disassemble

[![Build Status](https://github.com/endoli/disassemble.rs/actions/workflows/rust.yml/badge.svg)](https://github.com/endoli/disassemble.rs/actions/workflows/rust.yml)
[![](http://meritbadge.herokuapp.com/disassemble)](https://crates.io/crates/disassemble)

Dual licensed under the MIT and Apache 2 licenses.

This crate provides basic functionality for working with
disassembled code. It provides (or will provide) functionality
for performing:

* Working with code that has already been compiled to machine
  code for any CPU, bytecode, compiler IR or JIT compiler
  output like that of V8 or the JVM. You provide an adapter
  to teach [`Instruction`] how to report the correct information
  for your generated code.
* Reconstructing the [control flow graph] from a body of compiled
  code.
* **(Future)** Reconstructing loops and higher level control
  flow constructs.
* **(Future)** Performing [data flow analysis].
* **(Future)** Generating HTML and other rich output formats
  to assist in visualizing structure and higher level presentations
  of the data derived from the generated code.
* **(Future)** Writing decompilers that generate C or
  other languages from lower level generated code.
* **(Future)** Writing tools for reverse engineering. Many tools
  for reverse engineering require extracting and inferring higher
  level structure from the low level generated code. This crate
  provides those capabilities for re-use by anyone.
* **(Future)** Writing views that display disassembled code for
  debuggers, profilers and other systems level tools.

The actual disassembly with the implementation of [`Instruction`]
and other elements of the system will be provided by other
crates that integrate with other systems, such as the [Capstone
Engine]. This crate is written such that it should work with
nearly any machine code, VM bytecode or JIT compiler output,
given an appropriate implementation of [`Instruction`].

## Documentation

The API is fully documented with examples:
[https://endoli.github.io/disassemble.rs/](https://endoli.github.io/disassemble.rs/)

## Installation

This crate works with Cargo and is on
[crates.io](https://crates.io/crates/disassemble).
Add it to your `Cargo.toml` like so:

```toml
[dependencies]
disassemble = "0.0.1"
```

## Status of Implementation

Things are under active development. This project is not quite
usable yet as some of the basic functionality is being written.

## Contribution

Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed
as above, without any additional terms or conditions.

[Capstone Engine]: http://www.capstone-engine.org/
[control flow graph]: https://en.wikipedia.org/wiki/Control_flow_graph
[data flow analysis]: https://en.wikipedia.org/wiki/Data-flow_analysis
[`Instruction`]: https://endoli.github.io/disassemble.rs/disassemble/trait.Instruction.html
