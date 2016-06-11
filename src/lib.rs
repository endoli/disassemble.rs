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
//! [Capstone Engine]: http://www.capstone-engine.org/
//! [`Instruction`]: trait.Instruction.html
//! [`Symbol`]: struct.Symbol.html

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

mod address;
mod basicblock;
mod function;
mod instruction;
mod symbol;

pub use self::address::Address;
pub use self::basicblock::BasicBlock;
pub use self::function::Function;
pub use self::instruction::Instruction;
pub use self::symbol::Symbol;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
