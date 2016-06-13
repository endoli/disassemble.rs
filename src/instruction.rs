// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use address::Address;

/// An assembly instruction, bytecode operation, VM operation, etc.
///
/// This trait will be implemented for a variety of backends and
/// provides the general means by which the rest of the code in this
/// library can be re-used.
///
/// This is intended to be fairly generic and is how other parts
/// of this library query information that is specific to a given
/// platform and body of generated code.
pub trait Instruction: fmt::Debug {
    /// The [`address`] of this `Instruction`.
    ///
    /// The [`address`] of an instruction must be unique within a
    /// [`function`].
    ///
    /// [`address`]: struct.Address.html
    /// [`function`]: struct.Function.html
    fn address(&self) -> Address;

    /// Does this instruction terminate a `BasicBlock`?
    ///
    /// This is used when constructing a [control flow graph]
    /// to help break a sequence of instructions into basic
    /// blocks.
    ///
    /// [`BasicBlock`]: struct.BasicBlock.html
    fn is_block_terminator(&self) -> bool {
        self.is_call() || self.is_local_jump() || self.is_return()
    }

    /// Does this instruction represent a call?
    fn is_call(&self) -> bool;

    /// Does this instruction represent a local jump?
    fn is_local_jump(&self) -> bool;

    /// Does this instruction represent a function return?
    fn is_return(&self) -> bool;
}
