// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::address::Address;
use crate::basicblock::{BasicBlock, BasicBlockEdge, EdgeType};
use crate::instruction::Instruction;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::BTreeMap;

/// A [control flow graph].
///
/// [control flow graph]: https://en.wikipedia.org/wiki/Control_flow_graph
pub struct ControlFlowGraph {
    /// The [`Graph`] that stores the actual ControlFlowGraph
    pub graph: Graph<BasicBlock, BasicBlockEdge>,
    /// The [`NodeIndex`] for the entry [`BasicBlock`] for this function.
    pub entry_block: Option<NodeIndex>,
    /// Map an [address] to the corresponding [basic block].
    ///
    /// [address]: Address
    /// [basic block]: BasicBlock
    pub block_finder: BTreeMap<Address, NodeIndex>,
}

impl ControlFlowGraph {
    /// Build the `ControlFlowGraph` from the [`instructions`].
    ///
    /// This is conducted in a 2 step process:
    ///
    /// * First, each instruction is examined to identify block boundaries.
    /// * Then, we go through each instruction again, looking for the
    ///   previously identified block boundaries and build the edges.
    ///
    /// This two step process prevents us from having to construct and then
    /// subsequently split blocks as we find backward edges.
    ///
    /// [`instructions`]: Instruction
    pub fn new<I: Instruction>(instructions: &[I]) -> Self {
        let mut cfg = ControlFlowGraph {
            graph: Graph::new(),
            entry_block: None,
            block_finder: BTreeMap::new(),
        };
        if !instructions.is_empty() {
            cfg.identify_blocks(instructions);
            cfg.build_edges(instructions);
        }
        cfg
    }

    /// Identify basic blocks by their boundaries.
    ///
    /// Inspect each instruction to see if it is a 'leader' or the start of
    /// a new basic block. An instruction is a leader if:
    ///
    /// * It is the first instruction in the basic block.
    /// * It comes after a branch of any sort. We identify these as things
    ///   for which `Instruction::is_block_terminator` returns `true`.
    /// * It is the target of a jump (conditional or unconditional) within
    ///   the function.
    fn identify_blocks<I: Instruction>(&mut self, instructions: &[I]) {
        let start_addr = instructions[0].address();
        let end_addr = instructions.last().map(|i| i.address()).unwrap();
        let mut next_is_leader: bool = true;
        for inst in instructions {
            if next_is_leader {
                self.add_node_to_graph(inst.address());
                next_is_leader = false;
            }
            if inst.is_block_terminator() {
                if let Some(target_addr) = inst.target_address() {
                    if target_addr >= start_addr && target_addr <= end_addr {
                        self.add_node_to_graph(target_addr);
                    }
                }
                // The next instruction, if any, will be the start of a new block.
                next_is_leader = true;
            }
        }
        self.entry_block = Some(self.block_finder[&instructions[0].address()]);
    }

    /// Adds a new node to the graph only if the address does not exist
    fn add_node_to_graph(&mut self, address: Address) {
        if self.block_finder.contains_key(&address) {
            return;
        }

        let idx = self.graph.add_node(BasicBlock::new(address));
        self.block_finder.insert(address, idx);
    }

    /// Build an edge between 2 basic blocks.
    fn build_edge<I: Instruction>(
        &mut self,
        current_block_idx: NodeIndex,
        next_block_idx: Option<NodeIndex>,
        current_inst: &I,
    ) {
        if current_inst.is_local_conditional_jump() {
            // We have one edge for the jump target and one for the fallthrough.
            // We jump through some hoops here to keep the borrow checker happy.
            if let Some(target_addr) = current_inst.target_address() {
                if let Some(target_block_idx) = self.block_finder.get(&target_addr) {
                    let edge = BasicBlockEdge {
                        edge_type: EdgeType::ConditionalTaken,
                    };
                    self.graph
                        .add_edge(current_block_idx, *target_block_idx, edge);
                }

                if let Some(index) = next_block_idx {
                    let edge = BasicBlockEdge {
                        edge_type: EdgeType::ConditionalFallthrough,
                    };
                    self.graph.add_edge(current_block_idx, index, edge);
                }
            }
        } else if current_inst.is_call() {
            // We are calling a function, which will jump to target address and will return
            // to the instruction just after the current one.
            if let Some(index) = next_block_idx {
                let edge = BasicBlockEdge {
                    edge_type: EdgeType::Unconditional,
                };
                self.graph.add_edge(current_block_idx, index, edge);
            }
        } else if current_inst.is_local_jump() {
            // We are on an unconditional jump and we need to add an edge to the target
            // block (if it exists).
            if let Some(target_addr) = current_inst.target_address() {
                if let Some(target_block_idx) = self.block_finder.get(&target_addr) {
                    let edge = BasicBlockEdge {
                        edge_type: EdgeType::Unconditional,
                    };
                    self.graph
                        .add_edge(current_block_idx, *target_block_idx, edge);
                }
            }
        } else if current_inst.is_return() {
            // Do we want to record this exit anywhere?
        } else if let Some(index) = next_block_idx {
            // We are here because someone has a reference to the current instruction, but
            // it is non branching instruction, so we have to add an edge to the
            // following instruction.
            let edge = BasicBlockEdge {
                edge_type: EdgeType::Unconditional,
            };
            self.graph.add_edge(current_block_idx, index, edge);
        }
    }

    /// Build edges after having identified the basic blocks.
    ///
    /// After the basic blocks have been identified, we can go back and
    /// connect the basic blocks with their appropriate edges.
    ///
    /// We do this by iterating through the instructions looking for
    /// boundaries between the basic blocks and then setting up the
    /// new edges.
    fn build_edges<I: Instruction>(&mut self, instructions: &[I]) {
        // Here, we're going to walk through the instructions again,
        // looking at the current instruction, while also maintaining
        // a separate iterator giving us the next instruction (if there
        // is one.
        let current_inst_iter = instructions.iter();
        let mut next_inst_iter = current_inst_iter.clone();
        // Skip the first one so that we're actually working with the next one.
        next_inst_iter.next();
        let mut current_block_idx = self.entry_block.unwrap();
        for (current_inst_idx, current_inst) in current_inst_iter.enumerate() {
            // Add this instruction to the current block
            if let Some(current_block) = self.graph.node_weight_mut(current_block_idx) {
                current_block.instruction_indices.push(current_inst_idx);
            }
            if let Some(next_inst) = next_inst_iter.next() {
                // Does the next instruction begin a basic block?
                let next_block_idx = *self
                    .block_finder
                    .get(&next_inst.address())
                    .unwrap_or(&current_block_idx);
                // If we're at a block boundary, create an edge between the
                // current and next blocks. The type of the edge is determined
                // by looking at the current instruction.
                if next_block_idx != current_block_idx {
                    self.build_edge(current_block_idx, Some(next_block_idx), current_inst);
                    current_block_idx = next_block_idx;
                }
            } else {
                self.build_edge(current_block_idx, None, current_inst);
                // No next instruction, so we're at the end.
                // Do we want to record this exit anywhere?
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ControlFlowGraph;
    use crate::address::Address;
    use crate::tests::*;
    use petgraph::graph::NodeIndex;
    use petgraph::EdgeDirection;
    use std::collections::HashSet;

    #[test]
    fn construct() {
        let insts: Vec<TestInstruction> = vec![];
        let cfg = ControlFlowGraph::new(&insts);
        assert!(cfg.entry_block.is_none());
        assert_eq!(cfg.graph.node_count(), 0);
    }

    #[test]
    fn build_one_basic_block() {
        let insts = [
            TestInstruction::new(0, Opcode::Add),
            TestInstruction::new(1, Opcode::Add),
            TestInstruction::new(2, Opcode::Ret),
        ];
        let cfg = ControlFlowGraph::new(&insts);
        assert!(cfg.entry_block.is_some());
        assert_eq!(cfg.graph.node_count(), 1);

        let inbound = cfg.graph.externals(EdgeDirection::Incoming);
        assert_eq!(inbound.count(), 1);

        let outbound = cfg.graph.externals(EdgeDirection::Outgoing);
        assert_eq!(outbound.count(), 1);
    }

    #[test]
    fn build_cfg_containing_local_unconditional_jump() {
        let insts = [
            TestInstruction::new(0, Opcode::Add),
            TestInstruction::new(1, Opcode::CJmp(Address::new(4))),
            TestInstruction::new(2, Opcode::Add),
            TestInstruction::new(3, Opcode::Jmp(Address::new(5))),
            TestInstruction::new(4, Opcode::Add),
            TestInstruction::new(5, Opcode::Ret),
        ];

        let cfg = ControlFlowGraph::new(&insts);

        assert_eq!(4, cfg.graph.edge_count());
        assert_eq!(4, cfg.graph.node_count());

        let root_idx = *cfg.block_finder.get(&Address::new(0)).unwrap();
        let cond_taken_idx = *cfg.block_finder.get(&Address::new(4)).unwrap();
        let falltrough_idx = *cfg.block_finder.get(&Address::new(2)).unwrap();
        let ret_idx = *cfg.block_finder.get(&Address::new(5)).unwrap();

        let neighbours = cfg.graph.neighbors(root_idx).into_iter().collect();
        assert_neighbours(neighbours, vec![cond_taken_idx, falltrough_idx]);

        let neighbours = cfg.graph.neighbors(cond_taken_idx).into_iter().collect();
        assert_neighbours(neighbours, vec![ret_idx]);

        let neighbours = cfg.graph.neighbors(falltrough_idx).into_iter().collect();
        assert_neighbours(neighbours, vec![ret_idx]);

        let neighbours = cfg.graph.neighbors(ret_idx).into_iter().collect();
        assert_neighbours(neighbours, vec![]);
    }

    #[test]
    fn build_cfg_with_goto_ending() {
        let insts = [
            TestInstruction::new(0, Opcode::Add),
            TestInstruction::new(1, Opcode::CJmp(Address::new(3))),
            TestInstruction::new(2, Opcode::Ret),
            TestInstruction::new(3, Opcode::Jmp(Address::new(2))),
        ];

        let cfg = ControlFlowGraph::new(&insts);

        assert_eq!(3, cfg.graph.edge_count());
        assert_eq!(3, cfg.graph.node_count());

        let root_idx = *cfg.block_finder.get(&Address::new(0)).unwrap();
        let ret_idx = *cfg.block_finder.get(&Address::new(2)).unwrap();
        let goto_idx = *cfg.block_finder.get(&Address::new(3)).unwrap();

        let neighbours = cfg.graph.neighbors(root_idx).into_iter().collect();
        assert_neighbours(neighbours, vec![ret_idx, goto_idx]);

        let neighbours = cfg.graph.neighbors(ret_idx).into_iter().collect();
        assert_neighbours(neighbours, vec![]);

        let neighbours = cfg.graph.neighbors(goto_idx).into_iter().collect();
        assert_neighbours(neighbours, vec![ret_idx]);
    }

    #[test]
    fn build_cfg_with_branch_ending() {
        let insts = [
            TestInstruction::new(0, Opcode::Add),
            TestInstruction::new(1, Opcode::CJmp(Address::new(0))),
        ];

        let cfg = ControlFlowGraph::new(&insts);

        assert_eq!(1, cfg.graph.edge_count());
        assert_eq!(1, cfg.graph.node_count());

        let root_idx = *cfg.block_finder.get(&Address::new(0)).unwrap();

        let neighbours = cfg.graph.neighbors(root_idx).into_iter().collect();
        assert_neighbours(neighbours, vec![root_idx]);
    }

    #[test]
    fn build_cfg_with_call_ending() {
        let insts = [
            TestInstruction::new(0, Opcode::Add),
            TestInstruction::new(1, Opcode::Call(Address::new(0))),
        ];

        let cfg = ControlFlowGraph::new(&insts);

        assert_eq!(0, cfg.graph.edge_count());
        assert_eq!(1, cfg.graph.node_count());
    }

    #[test]
    fn build_cfg_with_call_before_return() {
        let insts = [
            TestInstruction::new(0, Opcode::Add),
            TestInstruction::new(1, Opcode::Call(Address::new(0))),
            TestInstruction::new(2, Opcode::Ret),
        ];

        let cfg = ControlFlowGraph::new(&insts);

        assert_eq!(1, cfg.graph.edge_count());
        assert_eq!(2, cfg.graph.node_count());

        let root_idx = *cfg.block_finder.get(&Address::new(0)).unwrap();
        let ret_idx = *cfg.block_finder.get(&Address::new(2)).unwrap();

        let neighbours = cfg.graph.neighbors(root_idx).into_iter().collect();
        assert_neighbours(neighbours, vec![ret_idx]);
    }

    #[test]
    fn build_cfg_with_regular_instruction_ending() {
        let insts = [
            TestInstruction::new(0, Opcode::Add),
            TestInstruction::new(1, Opcode::Jmp(Address::new(2))),
            TestInstruction::new(2, Opcode::Add),
        ];

        let cfg = ControlFlowGraph::new(&insts);

        assert_eq!(1, cfg.graph.edge_count());
        assert_eq!(2, cfg.graph.node_count());

        let root_idx = *cfg.block_finder.get(&Address::new(0)).unwrap();
        let add_idx = *cfg.block_finder.get(&Address::new(2)).unwrap();

        let neighbours = cfg.graph.neighbors(root_idx).into_iter().collect();
        assert_neighbours(neighbours, vec![add_idx]);
    }

    fn assert_neighbours(actual: Vec<NodeIndex>, expected: Vec<NodeIndex>) {
        let actual_set: HashSet<NodeIndex> = actual.into_iter().collect();
        let expected_set: HashSet<NodeIndex> = expected.into_iter().collect();

        let diff = actual_set.difference(&expected_set);

        if diff.count() != 0 {
            assert_eq!(actual_set, expected_set);
        }
    }
}
