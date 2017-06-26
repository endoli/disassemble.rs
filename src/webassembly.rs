// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use parity_wasm::elements::Opcode;
use super::address::Address;
use super::disassembler::Disassembler;
use super::instruction::Instruction;

impl Instruction for Opcode {
    fn address(&self, _disassembler: &Disassembler) -> Address {
        Address::new(0)
    }

    fn comment(&self, _disassembler: &Disassembler) -> Option<String> {
        None
    }

    fn cycle_count(&self, _disassembler: &Disassembler) -> Option<u32> {
        None
    }

    fn is_call(&self, _disassembler: &Disassembler) -> bool {
        false
    }

    fn is_local_conditional_jump(&self, _disassembler: &Disassembler) -> bool {
        false
    }

    fn is_local_jump(&self, _disassembler: &Disassembler) -> bool {
        false
    }

    fn is_return(&self, _disassembler: &Disassembler) -> bool {
        false
    }

    fn target_address(&self, _disassembler: &Disassembler) -> Option<Address> {
        None
    }
}
