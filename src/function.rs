// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::callgraphanalysis::{CallGraphAnalysis, CallSite};
use crate::cfg::ControlFlowGraph;
use crate::instruction::Instruction;
use crate::symbol::Symbol;

/// A function within a program.
pub struct Function<I: Instruction> {
    /// The [symbol] for this function. This provides the name and [`Address`].
    ///
    /// [`Address`]: crate::Address
    /// [symbol]: Symbol
    pub symbol: Symbol,
    /// The [instructions] that comprise this function.
    ///
    /// [instructions]: Instruction
    pub instructions: Vec<I>,
    /// The [control flow graph] for this function. This is built from the
    /// `instructions`. It is made up of [basic blocks].
    ///
    /// [basic blocks]: crate::BasicBlock
    /// [control flow graph]: ControlFlowGraph
    pub control_flow_graph: ControlFlowGraph,
}

impl<I: Instruction> Function<I> {
    /// Construct a new function
    pub fn new(symbol: Symbol, instructions: Vec<I>) -> Self {
        let control_flow_graph = ControlFlowGraph::new(&instructions);
        Function {
            symbol,
            instructions,
            control_flow_graph,
        }
    }
}

impl<I: Instruction> CallGraphAnalysis<I> for Function<I> {
    fn identify_call_sites(&self) -> Vec<CallSite> {
        self.identify_call_sites_in_instructions(&self.instructions)
    }
}
