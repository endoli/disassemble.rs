// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

/// The location of something in an address space.
///
/// This is used to provide a location of a [`Function`], [`Instruction`],
/// or other item.
///
/// The meaning of this address is flexibly interpreted by the rest of this
/// library as the meaning depends upon the application embedding and using
/// this library. It may be an actual machine address or it might be something
/// as simple as the offset of an instruction into an array.
///
/// XXX: Should this have any indication for what type of address it is?
///      An address might be an address within a file, a resolved address
///      after being loaded, etc.
///
/// XXX: Should this include any information about the address space
///      that it is from?
///
/// ## Formatting
///
/// `Address` implements the `fmt::Binary`, `fmt::Octal`, `fmt::LowerHex`
/// and `fmt::UpperHex` traits from `std::fmt`. This makes it integrate
/// readily with Rust's standard I/O facilities:
///
/// ```
/// # use disassemble::Address;
/// let a = Address::new(0x6502);
/// // Print with 0x in hex.
/// assert_eq!("0x6502", format!("{:#x}", a));
/// // Print with 0x, zero padded, 10 characters wide, in hex.
/// assert_eq!("0x00006502", format!("{:#010x}", a));
/// ```
///
/// [`Function`]: struct.Function.html
/// [`Instruction`]: trait.Instruction.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Address {
    address: u64,
}

impl Address {
    /// Construct an `Address`.
    pub fn new(address: u64) -> Self {
        Address { address: address }
    }
}

impl fmt::Binary for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.address.fmt(f)
    }
}

impl fmt::Octal for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.address.fmt(f)
    }
}

impl fmt::LowerHex for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.address.fmt(f)
    }
}

impl fmt::UpperHex for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.address.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::Address;

    #[test]
    fn address_comparison() {
        let addr1 = Address::new(5);
        let addr2 = Address::new(3);
        let addr3 = Address::new(5);
        assert!(addr2 < addr1);
        assert_eq!(addr1, addr3);
    }

    #[test]
    fn binary_fmt() {
        let a = Address::new(4);
        assert_eq!("100", format!("{:b}", a));
    }

    #[test]
    fn octal_fmt() {
        let a = Address::new(10);
        assert_eq!("12", format!("{:o}", a));
    }

    #[test]
    fn lower_hex_fmt() {
        let a = Address::new(0xc1);
        assert_eq!("0xc1", format!("{:#x}", a));
        assert_eq!("  0xc1", format!("{:#6x}", a));
        assert_eq!("    c1", format!("{:6x}", a));
        assert_eq!("0000c1", format!("{:06x}", a));
    }

    #[test]
    fn upper_hex_fmt() {
        let a = Address::new(0xc1);
        assert_eq!("0xC1", format!("{:#X}", a));
        assert_eq!("  0xC1", format!("{:#6X}", a));
        assert_eq!("    C1", format!("{:6X}", a));
        assert_eq!("0000C1", format!("{:06X}", a));
    }
}
