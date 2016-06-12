// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Disassemble
//!
//! This crate provides basic functionality for working with
//! disassembled code.
//!
//! The actual disassembly with the implementation of [`Instruction`]
//! and other elements of the system will be provided by other
//! crates that integrate with other systems, such as the [Capstone
//! Engine].
//!
//! It is possible (likely?) that some functionality from within
//! this crate may move in the future to a separate crate for broader
//! re-use. This might impact [`Symbol`] among other things.
//!
//! ## Installation
//!
//! This crate works with Cargo and is on
//! [crates.io](https://crates.io/crates/disassemble).
//! Add it to your `Cargo.toml` like so:
//!
//! ```toml
//! [dependencies]
//! disassemble = "0.0.1"
//! ```
//!
//! Then, let `rustc` know that you're going to use this crate at the
//! top of your own crate:
//!
//! ```
//! extern crate disassemble;
//! # fn main() {}
//! ```
//!
//! ## Future Directions
//!
//! In the future, we want to extend this library to support a number of
//! additional features:
//!
//! * Actually implement building the CFG from the instructions.
//! * Implement DOT output for the CFG. This can probably be done with the
//!   help of `petgraph`.
//! * HTML output modes?
//! * Implement loop finding. (Havlak)
//! * Implement the [Capstone Engine] backend as a separate crate.
//! * Make [`Instruction`] aware of operands, registers
//! * Data flow support. Memory SSA?
//! * Should we deal with mangled symbols at this level?
//! * So much more!
//!
//! ## Contributions
//!
//! Contributions are welcome.
//!
//! [Capstone Engine]: http://www.capstone-engine.org/
//! [`Function`]: struct.Function.html
//! [`Instruction`]: trait.Instruction.html
//! [petgraph]: https://crates.io/crates/petgraph
//! [`Symbol`]: struct.Symbol.html

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

extern crate petgraph;

mod address;
mod basicblock;
mod function;
mod instruction;
mod symbol;

pub use self::address::Address;
pub use self::basicblock::{BasicBlock, BasicBlockEdge, EdgeDirection, EdgeType};
pub use self::function::Function;
pub use self::instruction::Instruction;
pub use self::symbol::Symbol;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
