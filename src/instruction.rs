// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use address::Address;

#[allow(missing_docs)]
pub trait Instruction: fmt::Debug {
    fn address(&self) -> Address;

    fn is_local_jump(&self) -> bool;

    fn is_call(&self) -> bool;

    fn is_block_terminator(&self) -> bool {
        self.is_local_jump() || self.is_call()
    }
}
