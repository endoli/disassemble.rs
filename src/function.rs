// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use cfg::CFG;
use instruction::Instruction;
use symbol::Symbol;

/// A function within a program.
pub struct Function<'f> {
    /// The [symbol] for this function. This provides the name and [`Address`].
    ///
    /// [`Address`]: struct.Symbol.html
    /// [symbol]: struct.Symbol.html
    pub symbol: Symbol,
    /// The [instructions] that comprise this function.
    ///
    /// [instructions]: trait.Instruction.html
    pub instructions: Vec<Box<Instruction>>,
    /// The [control flow graph] for this function. This is build from the
    /// `instructions`. It is made up of [basic blocks].
    ///
    /// [basic blocks]: struct.BasicBlock.html
    /// [control flow graph]: https://en.wikipedia.org/wiki/Control_flow_graph
    pub cfg: CFG<'f>,
}

impl<'f> Function<'f> {
    /// Construct a new function
    pub fn new(symbol: Symbol, instructions: Vec<Box<Instruction>>) -> Self {
        let mut f = Function {
            symbol: symbol,
            instructions: instructions,
            cfg: CFG::new(),
        };
        f.cfg.build(&f.instructions);
        f
    }
}
