// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use parity_wasm::elements::Opcode;
use super::address::Address;
use super::instruction::Instruction;

/// A representation of a WebAssembly instruction.
#[derive(Debug)]
pub struct WasmInstruction {
    idx: u64,
    op: Opcode,
}

impl WasmInstruction {
    /// Create a `WasmInstruction` from an `Opcode`.
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

    fn mnemonic(&self) -> &str {
        match self.op {
            Opcode::Unreachable => "unreachable",
            Opcode::Nop => "nop",
            Opcode::Block(..) => "block",
            Opcode::Loop(..) => "loop",
            Opcode::If(..) => "if",
            Opcode::Else => "else",
            Opcode::End => "end",
            Opcode::Br(..) => "br",
            Opcode::BrIf(..) => "br_if",
            Opcode::BrTable(..) => "br_table",
            Opcode::Return => "return",
            Opcode::Call(..) => "call",
            Opcode::CallIndirect(..) => "call_indirect",
            Opcode::Drop => "drop",
            Opcode::Select => "select",
            Opcode::GetLocal(..) => "get_local",
            Opcode::SetLocal(..) => "set_local",
            Opcode::TeeLocal(..) => "tee_local",
            Opcode::GetGlobal(..) => "get_global",
            Opcode::SetGlobal(..) => "set_global",
            Opcode::I32Load(..) => "i32.load",
            Opcode::I64Load(..) => "i64.load",
            Opcode::F32Load(..) => "f32.load",
            Opcode::F64Load(..) => "f64.load",
            Opcode::I32Load8S(..) => "i32.load8_s",
            Opcode::I32Load8U(..) => "i32.load8_u",
            Opcode::I32Load16S(..) => "i32.load16_s",
            Opcode::I32Load16U(..) => "i32.load16_u",
            Opcode::I64Load8S(..) => "i64.load8_s",
            Opcode::I64Load8U(..) => "i64.load8_u",
            Opcode::I64Load16S(..) => "i64.load16_s",
            Opcode::I64Load16U(..) => "i64.load16_u",
            Opcode::I64Load32S(..) => "i64.load32_s",
            Opcode::I64Load32U(..) => "i64.load32_u",
            Opcode::I32Store(..) => "i32.store",
            Opcode::I64Store(..) => "i64.store",
            Opcode::F32Store(..) => "f32.store",
            Opcode::F64Store(..) => "f64.store",
            Opcode::I32Store8(..) => "i32.store8",
            Opcode::I32Store16(..) => "i32.store16",
            Opcode::I64Store8(..) => "i64.store8",
            Opcode::I64Store16(..) => "i64.store16",
            Opcode::I64Store32(..) => "i64.store32",
            Opcode::CurrentMemory(..) => "current_memory",
            Opcode::GrowMemory(..) => "grow_memory",
            Opcode::I32Const(..) => "i32.const",
            Opcode::I64Const(..) => "i64.const",
            Opcode::F32Const(..) => "f32.const",
            Opcode::F64Const(..) => "f64.const",
            Opcode::I32Eq => "i32.eq",
            Opcode::I32Eqz => "i32.eqz",
            Opcode::I32Ne => "i32.ne",
            Opcode::I32LtS => "i32.lt_s",
            Opcode::I32LtU => "i32.lt_u",
            Opcode::I32GtS => "i32.gt_s",
            Opcode::I32GtU => "i32.gt_u",
            Opcode::I32LeS => "i32.le_s",
            Opcode::I32LeU => "i32.le_u",
            Opcode::I32GeS => "i32.ge_s",
            Opcode::I32GeU => "i32.ge_u",
            Opcode::I64Eq => "i64.eq",
            Opcode::I64Eqz => "i64.eqz",
            Opcode::I64Ne => "i64.ne",
            Opcode::I64LtS => "i64.lt_s",
            Opcode::I64LtU => "i64.lt_u",
            Opcode::I64GtS => "i64.gt_s",
            Opcode::I64GtU => "i64.gt_u",
            Opcode::I64LeS => "i64.le_s",
            Opcode::I64LeU => "i64.le_u",
            Opcode::I64GeS => "i64.ge_s",
            Opcode::I64GeU => "i64.ge_u",
            Opcode::F32Eq => "f32.eq",
            Opcode::F32Ne => "f32.ne",
            Opcode::F32Lt => "f32.lt",
            Opcode::F32Gt => "f32.gt",
            Opcode::F32Le => "f32.le",
            Opcode::F32Ge => "f32.ge",
            Opcode::F64Eq => "f64.eq",
            Opcode::F64Ne => "f64.ne",
            Opcode::F64Lt => "f64.lt",
            Opcode::F64Gt => "f64.gt",
            Opcode::F64Le => "f64.le",
            Opcode::F64Ge => "f64.ge",
            Opcode::I32Clz => "i32.clz",
            Opcode::I32Ctz => "i32.ctz",
            Opcode::I32Popcnt => "i32.popcnt",
            Opcode::I32Add => "i32.add",
            Opcode::I32Sub => "i32.sub",
            Opcode::I32Mul => "i32.mul",
            Opcode::I32DivS => "i32.div_s",
            Opcode::I32DivU => "i32.div_u",
            Opcode::I32RemS => "i32.rem_s",
            Opcode::I32RemU => "i32.rem_u",
            Opcode::I32And => "i32.and",
            Opcode::I32Or => "i32.or",
            Opcode::I32Xor => "i32.xor",
            Opcode::I32Shl => "i32.shl",
            Opcode::I32ShrS => "i32.shr_s",
            Opcode::I32ShrU => "i32.shr_u",
            Opcode::I32Rotl => "i32.rotl",
            Opcode::I32Rotr => "i32.rotr",
            Opcode::I64Clz => "i64.clz",
            Opcode::I64Ctz => "i64.ctz",
            Opcode::I64Popcnt => "i64.popcnt",
            Opcode::I64Add => "i64.add",
            Opcode::I64Sub => "i64.sub",
            Opcode::I64Mul => "i64.mul",
            Opcode::I64DivS => "i64.div_s",
            Opcode::I64DivU => "i64.div_u",
            Opcode::I64RemS => "i64.rem_s",
            Opcode::I64RemU => "i64.rem_u",
            Opcode::I64And => "i64.and",
            Opcode::I64Or => "i64.or",
            Opcode::I64Xor => "i64.xor",
            Opcode::I64Shl => "i64.shl",
            Opcode::I64ShrS => "i64.shr_s",
            Opcode::I64ShrU => "i64.shr_u",
            Opcode::I64Rotl => "i64.rotl",
            Opcode::I64Rotr => "i64.rotr",
            Opcode::F32Abs => "f32.abs",
            Opcode::F32Neg => "f32.neg",
            Opcode::F32Ceil => "f32.ceil",
            Opcode::F32Floor => "f32.floor",
            Opcode::F32Trunc => "f32.trunc",
            Opcode::F32Nearest => "f32.nearest",
            Opcode::F32Sqrt => "f32.sqrt",
            Opcode::F32Add => "f32.add",
            Opcode::F32Sub => "f32.sub",
            Opcode::F32Mul => "f32.mul",
            Opcode::F32Div => "f32.div",
            Opcode::F32Min => "f32.min",
            Opcode::F32Max => "f32.max",
            Opcode::F32Copysign => "f32.copysign",
            Opcode::F64Abs => "f64.abs",
            Opcode::F64Neg => "f64.neg",
            Opcode::F64Ceil => "f64.ceil",
            Opcode::F64Floor => "f64.floor",
            Opcode::F64Trunc => "f64.trunc",
            Opcode::F64Nearest => "f64.nearest",
            Opcode::F64Sqrt => "f64.sqrt",
            Opcode::F64Add => "f64.add",
            Opcode::F64Sub => "f64.sub",
            Opcode::F64Mul => "f64.mul",
            Opcode::F64Div => "f64.div",
            Opcode::F64Min => "f64.min",
            Opcode::F64Max => "f64.max",
            Opcode::F64Copysign => "f64.copysign",
            Opcode::I32WarpI64 => "i32.wrap/i64",
            Opcode::I32TruncSF32 => "i32.trunc_s/f32",
            Opcode::I32TruncUF32 => "i32.trunc_u/f32",
            Opcode::I32TruncSF64 => "i32.trunc_s/f64",
            Opcode::I32TruncUF64 => "i32.trunc_u/f64",
            Opcode::I64ExtendSI32 => "i64.extend_s/i32",
            Opcode::I64ExtendUI32 => "i64.extend_u/i32",
            Opcode::I64TruncSF32 => "i64.trunc_s/f32",
            Opcode::I64TruncUF32 => "i64.trunc_u/f32",
            Opcode::I64TruncSF64 => "i64.trunc_s/f64",
            Opcode::I64TruncUF64 => "i64.trunc_u/f64",
            Opcode::F32ConvertSI32 => "f32.convert_s/i32",
            Opcode::F32ConvertUI32 => "f32.convert_u/i32",
            Opcode::F32ConvertSI64 => "f32.convert_s/i64",
            Opcode::F32ConvertUI64 => "f32.convert_u/i64",
            Opcode::F32DemoteF64 => "f32.demote/f64",
            Opcode::F64ConvertSI32 => "f64.convert_s/i32",
            Opcode::F64ConvertUI32 => "f64.convert_u/i32",
            Opcode::F64ConvertSI64 => "f64.convert_s/i64",
            Opcode::F64ConvertUI64 => "f64.convert_u/i64",
            Opcode::F64PromoteF32 => "f64.promote/f32",
            Opcode::I32ReinterpretF32 => "i32.reinterpret/f32",
            Opcode::I64ReinterpretF64 => "i64.reinterpret/f64",
            Opcode::F32ReinterpretI32 => "f32.reinterpret/i32",
            Opcode::F64ReinterpretI64 => "f64.reinterpret/i64",
        }
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
            Opcode::Call(a) => Some(Address::new(u64::from(a))),
            _ => None,
        }
    }
}
