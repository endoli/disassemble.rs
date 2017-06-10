// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use address::Address;
use cfg::ControlFlowGraph;
use instruction::Instruction;
use symbol::Symbol;

/// Information about the target of a `CallSite`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CallSiteTarget {
    /// The call site directly invokes the function at the `Address`.
    Direct(Address),
    /// The call site is indirect, and we haven't yet done further
    /// analysis.
    Indirect,
}

/// Information about a call site.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallSite {
    /// The address of the call site.
    pub call_site_address: Address,
    /// Information about the target of the call site.
    pub target: CallSiteTarget,
}

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

    /// Get information about the function calls made by this function.
    pub fn identify_call_sites(&self) -> Vec<CallSite> {
        self.instructions
            .iter()
            .filter(|i| i.is_call())
            .map(|i| {
                CallSite {
                    call_site_address: i.address(),
                    target: match i.target_address() {
                        Some(a) => CallSiteTarget::Direct(a),
                        None => CallSiteTarget::Indirect,
                    },
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use address::Address;
    use super::*;
    use symbol::Symbol;
    use tests::*;

    #[test]
    fn calls_none() {
        let insts = [TestInstruction::new(0, Opcode::Add),
                     TestInstruction::new(1, Opcode::Add),
                     TestInstruction::new(2, Opcode::Ret)];
        let f = Function::new(Symbol::new(Address::new(100), None), &insts);
        let calls = f.identify_call_sites();
        assert!(calls.is_empty());
    }

    #[test]
    fn calls_some() {
        let insts = [TestInstruction::new(0, Opcode::Add),
                     TestInstruction::new(1, Opcode::Call(Address::new(500))),
                     TestInstruction::new(2, Opcode::Add),
                     TestInstruction::new(3, Opcode::Call(Address::new(400))),
                     TestInstruction::new(4, Opcode::Call(Address::new(500))),
                     TestInstruction::new(5, Opcode::Ret)];
        let f = Function::new(Symbol::new(Address::new(100), None), &insts);
        let calls = f.identify_call_sites();
        assert_eq!(calls,
                   vec![CallSite {
                            call_site_address: Address::new(1),
                            target: CallSiteTarget::Direct(Address::new(500)),
                        },
                        CallSite {
                            call_site_address: Address::new(3),
                            target: CallSiteTarget::Direct(Address::new(400)),
                        },
                        CallSite {
                            call_site_address: Address::new(4),
                            target: CallSiteTarget::Direct(Address::new(500)),
                        }]);
    }
}
