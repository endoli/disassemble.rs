// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Disassemble
//!
//! This crate provides basic functionality for working with
//! disassembled code. It provides (or will provide) functionality
//! for performing:
//!
//! * Working with code that has already been compiled to machine
//!   code for any CPU, bytecode, compiler IR or JIT compiler
//!   output like that of V8 or the JVM. You provide an adapter
//!   to teach [`Instruction`] how to report the correct information
//!   for your generated code.
//! * Reconstructing the [control flow graph] from a body of compiled
//!   code.
//! * **(Future)** Reconstructing loops and higher level control
//!   flow constructs.
//! * **(Future)** Performing [data flow analysis].
//! * **(Future)** Generating HTML and other rich output formats
//!   to assist in visualizing structure and higher level presentations
//!   of the data derived from the generated code.
//! * **(Future)** Writing decompilers that generate C or
//!   other languages from lower level generated code.
//! * **(Future)** Writing tools for reverse engineering. Many tools
//!   for reverse engineering require extracting and inferring higher
//!   level structure from the low level generated code. This crate
//!   provides those capabilities for re-use by anyone.
//! * **(Future)** Writing views that display disassembled code for
//!   debuggers, profilers and other systems level tools.
//!
//! The actual disassembly with the implementation of [`Instruction`]
//! and other elements of the system will be provided by other
//! crates that integrate with other systems, such as the [Capstone
//! Engine]. This crate is written such that it should work with
//! nearly any machine code, VM bytecode or JIT compiler output,
//! given an appropriate implementation of [`Instruction`].
//!
//! It is possible (likely?) that some functionality from within
//! this crate may move in the future to a separate crate for broader
//! re-use. This might impact [`Symbol`] among other things.
//!
//! ## Installation
//!
//! This crate works with Cargo and is on
//! [crates.io](https://crates.io/crates/disassemble).
//! Add it to your `Cargo.toml` like so:
//!
//! ```toml
//! [dependencies]
//! disassemble = "0.0.1"
//! ```
//!
//! Then, let `rustc` know that you're going to use this crate at the
//! top of your own crate:
//!
//! ```
//! extern crate disassemble;
//! # fn main() {}
//! ```
//!
//! ## Future Directions
//!
//! In the future, we want to extend this library to support a number of
//! additional features:
//!
//! * Implement DOT output for the [`ControlFlowGraph`]. This can probably be done with the
//!   help of [`petgraph`].
//! * HTML output modes?
//! * Implement loop finding. (Havlak)
//! * Implement the [Capstone Engine] backend as a separate crate.
//! * Make [`Instruction`] aware of operands, registers
//! * Data flow support. Memory SSA?
//! * Should we deal with mangled symbols at this level?
//! * So much more!
//!
//! ## Contributions
//!
//! Contributions are welcome.
//!
//! [Capstone Engine]: http://www.capstone-engine.org/
//! [`ControlFlowGraph`]: struct.ControlFlowGraph.html
//! [control flow graph]: https://en.wikipedia.org/wiki/Control_flow_graph
//! [data flow analysis]: https://en.wikipedia.org/wiki/Data-flow_analysis
//! [`Function`]: struct.Function.html
//! [`Instruction`]: trait.Instruction.html
//! [`petgraph`]: https://crates.io/crates/petgraph
//! [`Symbol`]: struct.Symbol.html

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

extern crate petgraph;

mod address;
mod basicblock;
mod callgraphanalysis;
mod cfg;
mod function;
mod instruction;
mod loops;
mod memory;
mod module;
mod symbol;
mod target;
#[cfg(feature = "webassembly")]
mod webassembly;

pub use self::address::Address;
pub use self::basicblock::{BasicBlock, BasicBlockEdge, EdgeType};
pub use self::callgraphanalysis::{CallGraphAnalysis, CallSite, CallSiteTarget};
pub use self::cfg::ControlFlowGraph;
pub use self::function::Function;
pub use self::instruction::Instruction;
pub use self::loops::{find_loops, LoopStructureGraph, SimpleLoop};
pub use self::memory::{Error, Memory, Segment};
pub use self::module::Module;
pub use self::symbol::Symbol;
pub use self::target::Target;
#[cfg(feature = "webassembly")]
pub use self::webassembly::WasmInstruction;

#[cfg(feature = "capstone")]
mod capstone;
#[cfg(feature = "capstone")]
pub use self::capstone::CapstoneInstruction;

#[cfg(feature = "bpf")]
mod bpf;
#[cfg(feature = "bpf")]
pub use self::bpf::BpfInstruction;

#[cfg(feature = "burst")]
mod burst;
#[cfg(feature = "burst")]
pub use self::burst::BurstInstruction;

#[cfg(test)]
mod tests {
    //! Test Instructions
    //!
    //! This provides some test implementations of `Instruction` for
    //! use in testing out this crate without having to hook up to
    //! a real disassembler.

    use address::Address;
    use instruction::Instruction;

    /// Opcodes that we'll use as instructions.
    #[derive(Debug)]
    pub enum Opcode {
        Add,
        CJmp(Address),
        Jmp(Address),
        Call(Address),
        Ret,
    }

    #[derive(Debug)]
    pub struct TestInstruction {
        address: Address,
        opcode: Opcode,
    }

    impl ::std::fmt::Display for TestInstruction {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "{:?}", self.opcode)
        }
    }

    impl TestInstruction {
        /// Construct a `TestInstruction`.
        pub fn new(address: u64, opcode: Opcode) -> Self {
            TestInstruction {
                address: Address::new(address),
                opcode: opcode,
            }
        }
    }

    impl Instruction for TestInstruction {
        fn address(&self) -> Address {
            self.address
        }

        fn comment(&self) -> Option<String> {
            None
        }

        fn mnemonic(&self) -> &str {
            match self.opcode {
                Opcode::Add => "add",
                Opcode::CJmp(..) => "conditional-jump",
                Opcode::Jmp(..) => "jump",
                Opcode::Call(..) => "call",
                Opcode::Ret => "return",
            }
        }

        fn cycle_count(&self) -> Option<u32> {
            None
        }

        fn is_call(&self) -> bool {
            match self.opcode {
                Opcode::Call(..) => true,
                _ => false,
            }
        }

        fn is_local_conditional_jump(&self) -> bool {
            match self.opcode {
                Opcode::CJmp(..) => true,
                _ => false,
            }
        }

        fn is_local_jump(&self) -> bool {
            match self.opcode {
                Opcode::CJmp(..) => true,
                Opcode::Jmp(..) => true,
                _ => false,
            }
        }

        fn is_return(&self) -> bool {
            match self.opcode {
                Opcode::Ret => true,
                _ => false,
            }
        }

        fn target_address(&self) -> Option<Address> {
            match self.opcode {
                Opcode::CJmp(addr) => Some(addr),
                Opcode::Jmp(addr) => Some(addr),
                Opcode::Call(addr) => Some(addr),
                _ => None,
            }
        }
    }
}
