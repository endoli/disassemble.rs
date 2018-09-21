// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate parity_wasm;

use self::parity_wasm::elements::{
    deserialize_file, External, Instruction, Instructions, Internal,
};
use super::address::Address;
use super::function::Function;
use super::instruction;
use super::module::Module;
use super::symbol::Symbol;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;

/// A representation of a WebAssembly instruction.
#[derive(Debug)]
pub struct WasmInstruction {
    idx: u64,
    insn: Instruction,
}

impl WasmInstruction {
    /// Create a `WasmInstruction` from an opcode.
    pub fn new(idx: u64, insn: Instruction) -> Self {
        WasmInstruction { idx, insn }
    }
}

impl instruction::Instruction for WasmInstruction {
    fn address(&self) -> Address {
        Address::new(self.idx)
    }

    fn comment(&self) -> Option<String> {
        None
    }

    fn mnemonic(&self) -> &str {
        match self.insn {
            Instruction::Unreachable => "unreachable",
            Instruction::Nop => "nop",
            Instruction::Block(..) => "block",
            Instruction::Loop(..) => "loop",
            Instruction::If(..) => "if",
            Instruction::Else => "else",
            Instruction::End => "end",
            Instruction::Br(..) => "br",
            Instruction::BrIf(..) => "br_if",
            Instruction::BrTable(..) => "br_table",
            Instruction::Return => "return",
            Instruction::Call(..) => "call",
            Instruction::CallIndirect(..) => "call_indirect",
            Instruction::Drop => "drop",
            Instruction::Select => "select",
            Instruction::GetLocal(..) => "get_local",
            Instruction::SetLocal(..) => "set_local",
            Instruction::TeeLocal(..) => "tee_local",
            Instruction::GetGlobal(..) => "get_global",
            Instruction::SetGlobal(..) => "set_global",
            Instruction::I32Load(..) => "i32.load",
            Instruction::I64Load(..) => "i64.load",
            Instruction::F32Load(..) => "f32.load",
            Instruction::F64Load(..) => "f64.load",
            Instruction::I32Load8S(..) => "i32.load8_s",
            Instruction::I32Load8U(..) => "i32.load8_u",
            Instruction::I32Load16S(..) => "i32.load16_s",
            Instruction::I32Load16U(..) => "i32.load16_u",
            Instruction::I64Load8S(..) => "i64.load8_s",
            Instruction::I64Load8U(..) => "i64.load8_u",
            Instruction::I64Load16S(..) => "i64.load16_s",
            Instruction::I64Load16U(..) => "i64.load16_u",
            Instruction::I64Load32S(..) => "i64.load32_s",
            Instruction::I64Load32U(..) => "i64.load32_u",
            Instruction::I32Store(..) => "i32.store",
            Instruction::I64Store(..) => "i64.store",
            Instruction::F32Store(..) => "f32.store",
            Instruction::F64Store(..) => "f64.store",
            Instruction::I32Store8(..) => "i32.store8",
            Instruction::I32Store16(..) => "i32.store16",
            Instruction::I64Store8(..) => "i64.store8",
            Instruction::I64Store16(..) => "i64.store16",
            Instruction::I64Store32(..) => "i64.store32",
            Instruction::CurrentMemory(..) => "current_memory",
            Instruction::GrowMemory(..) => "grow_memory",
            Instruction::I32Const(..) => "i32.const",
            Instruction::I64Const(..) => "i64.const",
            Instruction::F32Const(..) => "f32.const",
            Instruction::F64Const(..) => "f64.const",
            Instruction::I32Eq => "i32.eq",
            Instruction::I32Eqz => "i32.eqz",
            Instruction::I32Ne => "i32.ne",
            Instruction::I32LtS => "i32.lt_s",
            Instruction::I32LtU => "i32.lt_u",
            Instruction::I32GtS => "i32.gt_s",
            Instruction::I32GtU => "i32.gt_u",
            Instruction::I32LeS => "i32.le_s",
            Instruction::I32LeU => "i32.le_u",
            Instruction::I32GeS => "i32.ge_s",
            Instruction::I32GeU => "i32.ge_u",
            Instruction::I64Eq => "i64.eq",
            Instruction::I64Eqz => "i64.eqz",
            Instruction::I64Ne => "i64.ne",
            Instruction::I64LtS => "i64.lt_s",
            Instruction::I64LtU => "i64.lt_u",
            Instruction::I64GtS => "i64.gt_s",
            Instruction::I64GtU => "i64.gt_u",
            Instruction::I64LeS => "i64.le_s",
            Instruction::I64LeU => "i64.le_u",
            Instruction::I64GeS => "i64.ge_s",
            Instruction::I64GeU => "i64.ge_u",
            Instruction::F32Eq => "f32.eq",
            Instruction::F32Ne => "f32.ne",
            Instruction::F32Lt => "f32.lt",
            Instruction::F32Gt => "f32.gt",
            Instruction::F32Le => "f32.le",
            Instruction::F32Ge => "f32.ge",
            Instruction::F64Eq => "f64.eq",
            Instruction::F64Ne => "f64.ne",
            Instruction::F64Lt => "f64.lt",
            Instruction::F64Gt => "f64.gt",
            Instruction::F64Le => "f64.le",
            Instruction::F64Ge => "f64.ge",
            Instruction::I32Clz => "i32.clz",
            Instruction::I32Ctz => "i32.ctz",
            Instruction::I32Popcnt => "i32.popcnt",
            Instruction::I32Add => "i32.add",
            Instruction::I32Sub => "i32.sub",
            Instruction::I32Mul => "i32.mul",
            Instruction::I32DivS => "i32.div_s",
            Instruction::I32DivU => "i32.div_u",
            Instruction::I32RemS => "i32.rem_s",
            Instruction::I32RemU => "i32.rem_u",
            Instruction::I32And => "i32.and",
            Instruction::I32Or => "i32.or",
            Instruction::I32Xor => "i32.xor",
            Instruction::I32Shl => "i32.shl",
            Instruction::I32ShrS => "i32.shr_s",
            Instruction::I32ShrU => "i32.shr_u",
            Instruction::I32Rotl => "i32.rotl",
            Instruction::I32Rotr => "i32.rotr",
            Instruction::I64Clz => "i64.clz",
            Instruction::I64Ctz => "i64.ctz",
            Instruction::I64Popcnt => "i64.popcnt",
            Instruction::I64Add => "i64.add",
            Instruction::I64Sub => "i64.sub",
            Instruction::I64Mul => "i64.mul",
            Instruction::I64DivS => "i64.div_s",
            Instruction::I64DivU => "i64.div_u",
            Instruction::I64RemS => "i64.rem_s",
            Instruction::I64RemU => "i64.rem_u",
            Instruction::I64And => "i64.and",
            Instruction::I64Or => "i64.or",
            Instruction::I64Xor => "i64.xor",
            Instruction::I64Shl => "i64.shl",
            Instruction::I64ShrS => "i64.shr_s",
            Instruction::I64ShrU => "i64.shr_u",
            Instruction::I64Rotl => "i64.rotl",
            Instruction::I64Rotr => "i64.rotr",
            Instruction::F32Abs => "f32.abs",
            Instruction::F32Neg => "f32.neg",
            Instruction::F32Ceil => "f32.ceil",
            Instruction::F32Floor => "f32.floor",
            Instruction::F32Trunc => "f32.trunc",
            Instruction::F32Nearest => "f32.nearest",
            Instruction::F32Sqrt => "f32.sqrt",
            Instruction::F32Add => "f32.add",
            Instruction::F32Sub => "f32.sub",
            Instruction::F32Mul => "f32.mul",
            Instruction::F32Div => "f32.div",
            Instruction::F32Min => "f32.min",
            Instruction::F32Max => "f32.max",
            Instruction::F32Copysign => "f32.copysign",
            Instruction::F64Abs => "f64.abs",
            Instruction::F64Neg => "f64.neg",
            Instruction::F64Ceil => "f64.ceil",
            Instruction::F64Floor => "f64.floor",
            Instruction::F64Trunc => "f64.trunc",
            Instruction::F64Nearest => "f64.nearest",
            Instruction::F64Sqrt => "f64.sqrt",
            Instruction::F64Add => "f64.add",
            Instruction::F64Sub => "f64.sub",
            Instruction::F64Mul => "f64.mul",
            Instruction::F64Div => "f64.div",
            Instruction::F64Min => "f64.min",
            Instruction::F64Max => "f64.max",
            Instruction::F64Copysign => "f64.copysign",
            Instruction::I32WrapI64 => "i32.wrap/i64",
            Instruction::I32TruncSF32 => "i32.trunc_s/f32",
            Instruction::I32TruncUF32 => "i32.trunc_u/f32",
            Instruction::I32TruncSF64 => "i32.trunc_s/f64",
            Instruction::I32TruncUF64 => "i32.trunc_u/f64",
            Instruction::I64ExtendSI32 => "i64.extend_s/i32",
            Instruction::I64ExtendUI32 => "i64.extend_u/i32",
            Instruction::I64TruncSF32 => "i64.trunc_s/f32",
            Instruction::I64TruncUF32 => "i64.trunc_u/f32",
            Instruction::I64TruncSF64 => "i64.trunc_s/f64",
            Instruction::I64TruncUF64 => "i64.trunc_u/f64",
            Instruction::F32ConvertSI32 => "f32.convert_s/i32",
            Instruction::F32ConvertUI32 => "f32.convert_u/i32",
            Instruction::F32ConvertSI64 => "f32.convert_s/i64",
            Instruction::F32ConvertUI64 => "f32.convert_u/i64",
            Instruction::F32DemoteF64 => "f32.demote/f64",
            Instruction::F64ConvertSI32 => "f64.convert_s/i32",
            Instruction::F64ConvertUI32 => "f64.convert_u/i32",
            Instruction::F64ConvertSI64 => "f64.convert_s/i64",
            Instruction::F64ConvertUI64 => "f64.convert_u/i64",
            Instruction::F64PromoteF32 => "f64.promote/f32",
            Instruction::I32ReinterpretF32 => "i32.reinterpret/f32",
            Instruction::I64ReinterpretF64 => "i64.reinterpret/f64",
            Instruction::F32ReinterpretI32 => "f32.reinterpret/i32",
            Instruction::F64ReinterpretI64 => "f64.reinterpret/i64",
            Instruction::AtomicWake(_) => "atomic.wake",
            Instruction::I32AtomicWait(_) => "i32.atomic.wait",
            Instruction::I64AtomicWait(_) => "i64.atomic.wait",
            Instruction::I32AtomicLoad(_) => "i32.atomic.load",
            Instruction::I64AtomicLoad(_) => "i64.atomic.load",
            Instruction::I32AtomicLoad8u(_) => "i32.atomic.load8_u",
            Instruction::I32AtomicLoad16u(_) => "i32.atomic.load16_u",
            Instruction::I64AtomicLoad8u(_) => "i64.atomic.load8_u",
            Instruction::I64AtomicLoad16u(_) => "i64.atomic.load16_u",
            Instruction::I64AtomicLoad32u(_) => "i64.atomic.load32_u",
            Instruction::I32AtomicStore(_) => "i32.atomic.store",
            Instruction::I64AtomicStore(_) => "i64.atomic.store",
            Instruction::I32AtomicStore8u(_) => "i32.atomic.store8_u",
            Instruction::I32AtomicStore16u(_) => "i32.atomic.store16_u",
            Instruction::I64AtomicStore8u(_) => "i64.atomic.store8_u",
            Instruction::I64AtomicStore16u(_) => "i64.atomic.store16_u",
            Instruction::I64AtomicStore32u(_) => "i64.atomic.store32_u",
            Instruction::I32AtomicRmwAdd(_) => "i32.atomic.rmw.add",
            Instruction::I64AtomicRmwAdd(_) => "i64.atomic.rmw.add",
            Instruction::I32AtomicRmwAdd8u(_) => "i32.atomic.rmw8_u.add",
            Instruction::I32AtomicRmwAdd16u(_) => "i32.atomic.rmw16_u.add",
            Instruction::I64AtomicRmwAdd8u(_) => "i64.atomic.rmw8_u.add",
            Instruction::I64AtomicRmwAdd16u(_) => "i64.atomic.rmw16_u.add",
            Instruction::I64AtomicRmwAdd32u(_) => "i64.atomic.rmw32_u.add",
            Instruction::I32AtomicRmwSub(_) => "i32.atomic.rmw.sub",
            Instruction::I64AtomicRmwSub(_) => "i64.atomic.rmw.sub",
            Instruction::I32AtomicRmwSub8u(_) => "i32.atomic.rmw8_u.sub",
            Instruction::I32AtomicRmwSub16u(_) => "i32.atomic.rmw16_u.sub",
            Instruction::I64AtomicRmwSub8u(_) => "i64.atomic.rmw8_u.sub",
            Instruction::I64AtomicRmwSub16u(_) => "i64.atomic.rmw16_u.sub",
            Instruction::I64AtomicRmwSub32u(_) => "i64.atomic.rmw32_u.sub",
            Instruction::I32AtomicRmwAnd(_) => "i32.atomic.rmw.and",
            Instruction::I64AtomicRmwAnd(_) => "i64.atomic.rmw.and",
            Instruction::I32AtomicRmwAnd8u(_) => "i32.atomic.rmw8_u.and",
            Instruction::I32AtomicRmwAnd16u(_) => "i32.atomic.rmw16_u.and",
            Instruction::I64AtomicRmwAnd8u(_) => "i64.atomic.rmw8_u.and",
            Instruction::I64AtomicRmwAnd16u(_) => "i64.atomic.rmw16_u.and",
            Instruction::I64AtomicRmwAnd32u(_) => "i64.atomic.rmw32_u.and",
            Instruction::I32AtomicRmwOr(_) => "i32.atomic.rmw.or",
            Instruction::I64AtomicRmwOr(_) => "i64.atomic.rmw.or",
            Instruction::I32AtomicRmwOr8u(_) => "i32.atomic.rmw8_u.or",
            Instruction::I32AtomicRmwOr16u(_) => "i32.atomic.rmw16_u.or",
            Instruction::I64AtomicRmwOr8u(_) => "i64.atomic.rmw8_u.or",
            Instruction::I64AtomicRmwOr16u(_) => "i64.atomic.rmw16_u.or",
            Instruction::I64AtomicRmwOr32u(_) => "i64.atomic.rmw32_u.or",
            Instruction::I32AtomicRmwXor(_) => "i32.atomic.rmw.xor",
            Instruction::I64AtomicRmwXor(_) => "i64.atomic.rmw.xor",
            Instruction::I32AtomicRmwXor8u(_) => "i32.atomic.rmw8_u.xor",
            Instruction::I32AtomicRmwXor16u(_) => "i32.atomic.rmw16_u.xor",
            Instruction::I64AtomicRmwXor8u(_) => "i64.atomic.rmw8_u.xor",
            Instruction::I64AtomicRmwXor16u(_) => "i64.atomic.rmw16_u.xor",
            Instruction::I64AtomicRmwXor32u(_) => "i64.atomic.rmw32_u.xor",
            Instruction::I32AtomicRmwXchg(_) => "i32.atomic.rmw.xchg",
            Instruction::I64AtomicRmwXchg(_) => "i64.atomic.rmw.xchg",
            Instruction::I32AtomicRmwXchg8u(_) => "i32.atomic.rmw8_u.xchg",
            Instruction::I32AtomicRmwXchg16u(_) => "i32.atomic.rmw16_u.xchg",
            Instruction::I64AtomicRmwXchg8u(_) => "i64.atomic.rmw8_u.xchg",
            Instruction::I64AtomicRmwXchg16u(_) => "i64.atomic.rmw16_u.xchg",
            Instruction::I64AtomicRmwXchg32u(_) => "i64.atomic.rmw32_u.xchg",
            Instruction::I32AtomicRmwCmpxchg(_) => "i32.atomic.rmw.cmpxchg",
            Instruction::I64AtomicRmwCmpxchg(_) => "i64.atomic.rmw.cmpxchg",
            Instruction::I32AtomicRmwCmpxchg8u(_) => "i32.atomic.rmw8_u.cmpxchg",
            Instruction::I32AtomicRmwCmpxchg16u(_) => "i32.atomic.rmw16_u.cmpxchg",
            Instruction::I64AtomicRmwCmpxchg8u(_) => "i64.atomic.rmw8_u.cmpxchg",
            Instruction::I64AtomicRmwCmpxchg16u(_) => "i64.atomic.rmw16_u.cmpxchg",
            Instruction::I64AtomicRmwCmpxchg32u(_) => "i64.atomic.rmw32_u.cmpxchg",
        }
    }

    fn is_call(&self) -> bool {
        match self.insn {
            Instruction::Call(..) | Instruction::CallIndirect(..) => true,
            _ => false,
        }
    }

    fn is_local_conditional_jump(&self) -> bool {
        match self.insn {
            Instruction::If(..) | Instruction::BrIf(..) | Instruction::BrTable(..) => true,
            _ => false,
        }
    }

    fn is_local_jump(&self) -> bool {
        self.is_local_conditional_jump() || match self.insn {
            Instruction::Br(..) => true,
            _ => false,
        }
    }

    fn is_return(&self) -> bool {
        match self.insn {
            Instruction::Return => true,
            _ => false,
        }
    }

    fn target_address(&self) -> Option<Address> {
        match self.insn {
            Instruction::Call(a) => Some(Address::new(u64::from(a))),
            _ => None,
        }
    }
}

impl Function<WasmInstruction> {
    /// Create a function from WebAssembly bytecode.
    pub fn from_wasm(symbol: Symbol, instructions: &Instructions) -> Function<WasmInstruction> {
        let is = instructions
            .elements()
            .into_iter()
            .enumerate()
            .map(|(idx, insn)| WasmInstruction::new(idx as u64, insn.clone()))
            .collect::<Vec<_>>();
        Function::new(symbol, is)
    }
}

impl Module<WasmInstruction> {
    /// Load a module from a binary WebAssembly file.
    pub fn from_wasm_file<P: AsRef<Path>>(path: P) -> Option<Self> {
        if let Ok(m) = deserialize_file(path) {
            let mut symbol_table = HashMap::<Address, &str>::new();
            if let Some(exports) = m.export_section() {
                for export in exports.entries() {
                    if let Internal::Function(index) = *export.internal() {
                        symbol_table.insert(Address::new(u64::from(index)), export.field());
                    }
                }
            }
            if let Some(imports) = m.import_section() {
                for import in imports.entries() {
                    if let External::Function(index) = *import.external() {
                        symbol_table.insert(Address::new(u64::from(index)), import.field());
                    }
                }
            }
            if let Some(code) = m.code_section() {
                let functions = code
                    .bodies()
                    .iter()
                    .enumerate()
                    .map(|(idx, body)| {
                        let addr = Address::new(idx as u64);
                        let name = symbol_table.get(&addr).map(|n| *n);
                        Function::from_wasm(Symbol::new(addr, name), body.code())
                    }).collect();
                Some(Module { functions })
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl fmt::Display for WasmInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.insn.fmt(f)
    }
}
