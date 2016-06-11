// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// The location of something in an address space.
///
/// This is used to provide a location of a [`Function`], [`Instruction`],
/// or other item.
///
/// XXX: Should this have any indication for what type of address it is?
///      An address might be an address within a file, a resolved address
///      after being loaded, etc.
///
/// XXX: Should this include any information about the address space
///      that it is from?
///
/// [`Function`]: struct.Function.html
/// [`Instruction`]: trait.Instruction.html
#[derive(Clone,Copy,Debug)]
pub struct Address {
    address: usize,
}

impl Address {
    /// Construct an `Address`.
    pub fn new(address: usize) -> Self {
        Address { address: address }
    }
}
