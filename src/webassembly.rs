// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use parity_wasm::elements::Opcode;
use super::address::Address;
use super::instruction::Instruction;

impl Instruction for Opcode {
    fn address(&self) -> Address {
        // The Opcode enum doesn't include this info.
        Address::new(0)
    }

    fn comment(&self) -> Option<String> {
        None
    }

    fn cycle_count(&self) -> Option<u32> {
        None
    }

    fn is_call(&self) -> bool {
        match *self {
            Opcode::Call(..) |
            Opcode::CallIndirect(..) => true,
            _ => false,
        }
    }

    fn is_local_conditional_jump(&self) -> bool {
        match *self {
            Opcode::If(..) |
            Opcode::BrIf(..) |
            Opcode::BrTable(..) => true,
            _ => false,
        }
    }

    fn is_local_jump(&self) -> bool {
        match *self {
            Opcode::Br(..) => true,
            _ => false,
        }
    }

    fn is_return(&self) -> bool {
        match *self {
            Opcode::Return => true,
            _ => false,
        }
    }

    fn target_address(&self) -> Option<Address> {
        match *self {
            Opcode::Call(a) => Some(Address::new(a as u64)),
            _ => None,
        }
    }
}
