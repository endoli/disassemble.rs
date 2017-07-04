// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use parity_wasm::elements::Opcode;
use super::address::Address;
use super::instruction::Instruction;

#[allow(missing_docs)]
#[derive(Debug)]
pub struct WasmInstruction {
    idx: u64,
    op: Opcode,
}

impl WasmInstruction {
    #[allow(missing_docs)]
    pub fn new(idx: u64, op: Opcode) -> Self {
        WasmInstruction { idx, op }
    }
}

impl Instruction for WasmInstruction {
    fn address(&self) -> Address {
        Address::new(self.idx)
    }

    fn comment(&self) -> Option<String> {
        None
    }

    fn cycle_count(&self) -> Option<u32> {
        None
    }

    fn is_call(&self) -> bool {
        match self.op {
            Opcode::Call(..) |
            Opcode::CallIndirect(..) => true,
            _ => false,
        }
    }

    fn is_local_conditional_jump(&self) -> bool {
        match self.op {
            Opcode::If(..) |
            Opcode::BrIf(..) |
            Opcode::BrTable(..) => true,
            _ => false,
        }
    }

    fn is_local_jump(&self) -> bool {
        match self.op {
            Opcode::Br(..) => true,
            _ => false,
        }
    }

    fn is_return(&self) -> bool {
        match self.op {
            Opcode::Return => true,
            _ => false,
        }
    }

    fn target_address(&self) -> Option<Address> {
        match self.op {
            Opcode::Call(a) => Some(Address::new(a as u64)),
            _ => None,
        }
    }
}
