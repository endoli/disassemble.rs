// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use callgraphanalysis::{CallGraphAnalysis, CallSite};
use cfg::ControlFlowGraph;
use instruction::Instruction;
use symbol::Symbol;

/// A function within a program.
pub struct Function<'f, I: 'f + Instruction> {
    /// The [symbol] for this function. This provides the name and [`Address`].
    ///
    /// [`Address`]: struct.Symbol.html
    /// [symbol]: struct.Symbol.html
    pub symbol: Symbol,
    /// The [instructions] that comprise this function.
    ///
    /// [instructions]: trait.Instruction.html
    pub instructions: &'f [I],
    /// The [control flow graph] for this function. This is built from the
    /// `instructions`. It is made up of [basic blocks].
    ///
    /// [basic blocks]: struct.BasicBlock.html
    /// [control flow graph]: struct.ControlFlowGraph.html
    pub control_flow_graph: ControlFlowGraph<'f, I>,
}

impl<'f, I: Instruction> Function<'f, I> {
    /// Construct a new function
    pub fn new(symbol: Symbol, instructions: &'f [I]) -> Self {
        let cfg = ControlFlowGraph::new(instructions);
        Function {
            symbol: symbol,
            instructions: instructions,
            control_flow_graph: cfg,
        }
    }
}

impl<'f, I: Instruction> CallGraphAnalysis<I> for Function<'f, I> {
    fn identify_call_sites(&self) -> Vec<CallSite> {
        self.identify_call_sites_in_instructions(self.instructions)
    }
}
