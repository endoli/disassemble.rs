// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use capstone_rust::capstone;
use super::address::Address;
use super::instruction::Instruction;
use super::disassembler::{Architecture, Disassembler};

impl Instruction for capstone::Instr {
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

impl From<capstone::cs_arch> for Architecture {
    fn from(arch: capstone::cs_arch) -> Self {
        match arch {
            capstone::cs_arch::CS_ARCH_X86 => Architecture::X86,
            _ => panic!("Unexpected arch: {:?}", arch),
        }
    }
}

impl Disassembler for capstone::Capstone {
    fn architecture(&self) -> Architecture {
        self.architecture().into()
    }
}
