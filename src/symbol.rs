// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use address::Address;

/// A symbol within an executable or library. This is a named address.
#[derive(Debug)]
pub struct Symbol {
    /// The [address] of this symbol.
    ///
    /// [address]: struct.Address.html
    pub address: Address,
    /// The name of this symbol.
    pub name: String,
}

impl Symbol {
    /// Construct a `Symbol`.
    pub fn new(address: Address, name: &str) -> Self {
        Symbol {
            address: address,
            name: name.to_owned(),
        }
    }
}
