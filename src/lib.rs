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
//!   help of [`petgraph`].
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
mod cfg;
mod function;
mod instruction;
mod symbol;

pub use self::address::Address;
pub use self::basicblock::{BasicBlock, BasicBlockEdge, EdgeType};
pub use self::cfg::CFG;
pub use self::function::Function;
pub use self::instruction::Instruction;
pub use self::symbol::Symbol;

#[cfg(test)]
mod tests {
    //! Test Instructions
    //!
    //! This provides some test implementations of `Instruction` for
    //! use in testing out this crate without having to hook up to
    //! a real disassembler.

    use address::Address;
    use instruction::Instruction;

    /// A named register location.
    #[derive(Clone,Copy,Debug)]
    pub struct Register {
        name: i32,
    }

    impl Register {
        pub fn new(name: i32) -> Self {
            Register { name: name }
        }
    }

    /// Opcodes that we'll use as instructions.
    /// The first address value in each is their
    /// address in 'memory'.
    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum Opcode {
        /// Add 2 register values and place the result in a third.
        Add(Address, Register, Register, Register),
        /// Multiply 2 register values and place the result in a third.
        Mul(Address, Register, Register, Register),
        /// Jump to address if the register value is true.
        CJmp(Address, Register, Address),
        /// Jump to `address`.
        Jmp(Address, Address),
        /// Call the subroutine at `address`.
        Call(Address, Address),
        /// Return from this function.
        Ret(Address),
    }

    impl Instruction for Opcode {
        fn address(&self) -> Address {
            match *self {
                Opcode::Add(addr, _, _, _) => addr,
                Opcode::Mul(addr, _, _, _) => addr,
                Opcode::CJmp(addr, _, _) => addr,
                Opcode::Jmp(addr, _) => addr,
                Opcode::Call(addr, _) => addr,
                Opcode::Ret(addr) => addr,
            }
        }

        fn is_call(&self) -> bool {
            match *self {
                Opcode::Call(_, _) => true,
                _ => false,
            }
        }

        fn is_local_jump(&self) -> bool {
            match *self {
                Opcode::CJmp(_, _, _) => true,
                Opcode::Jmp(_, _) => true,
                _ => false,
            }
        }

        fn is_return(&self) -> bool {
            match *self {
                Opcode::Ret(_) => true,
                _ => false,
            }
        }
    }
}
