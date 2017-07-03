// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use capstone_rust::capstone;
use super::address::Address;
use super::instruction::Instruction;

impl Instruction for capstone::Instr {
    fn address(&self) -> Address {
        Address::new(self.address)
    }

    fn comment(&self) -> Option<String> {
        None
    }

    fn cycle_count(&self) -> Option<u32> {
        None
    }

    fn is_call(&self) -> bool {
        false
    }

    fn is_local_conditional_jump(&self) -> bool {
        false
    }

    fn is_local_jump(&self) -> bool {
        false
    }

    fn is_return(&self) -> bool {
        false
    }

    fn target_address(&self) -> Option<Address> {
        None
    }
}
