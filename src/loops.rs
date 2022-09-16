// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]

use crate::cfg::ControlFlowGraph;
use petgraph::graph::NodeIndex;

#[derive(Debug, Default)]
pub struct SimpleLoop<'lsg> {
    pub basic_blocks: Vec<NodeIndex>,
    pub children: Vec<&'lsg SimpleLoop<'lsg>>,
    pub counter: usize,

    // pub parent: ...
    pub header: Option<NodeIndex>,

    pub is_root: bool,
    pub is_reducible: bool,
    pub nesting_level: usize,
    pub depth_level: usize,
}

impl<'lsg> SimpleLoop<'lsg> {
    pub fn new(counter: usize) -> Self {
        SimpleLoop {
            counter,
            ..Default::default()
        }
    }
}

#[allow(dead_code)]
enum BasicBlockType {
    /// Uninitialized basic block
    Top,
    /// Regular basic block
    NonHeader,
    /// Reducible loop type
    Reducible,
    /// Single basic block loop
    Single,
    /// Irreducible loop
    Irreducible,
    /// Dead basic block
    Dead,
}

/// Maintain loop structure for a given [`ControlFlowGraph`]
///
///
/// Two values are maintained for this loop graph: depth and nesting level.
/// For example:
///
/// ```text
///    loop        nesting level    depth
///   ----------------------------------------
///    loop-0      2                0
///      loop-1    1                1
///      loop-3    1                1
///        loop-2  0                2
/// ```
pub struct LoopStructureGraph<'lsg> {
    pub loop_counter: usize,
    pub loops: Vec<SimpleLoop<'lsg>>,
    pub root: SimpleLoop<'lsg>,
}

impl<'lsg> Default for LoopStructureGraph<'lsg> {
    fn default() -> Self {
        LoopStructureGraph {
            loop_counter: 1,
            loops: vec![],
            root: SimpleLoop::new(0),
        }
    }
}

/// Find loops and build loop forest using Havlak's algorithm, which
/// is derived from Tarjan.
///
/// Variable names and step numbering has been chosen to be identical
/// to the nomenclature in Havlak's paper (which, in turn, is similar
/// to the one used by Tarjan).
pub fn find_loops<'havlak>(
    cfg: &'havlak ControlFlowGraph,
    _lsg: &'havlak mut LoopStructureGraph<'havlak>,
) -> usize {
    if cfg.entry_block.is_none() {
        return 0;
    }
    unimplemented!();
}
