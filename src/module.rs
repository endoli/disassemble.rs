// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{Function, Instruction};

/// A shared library or other component of a target.
pub struct Module<'f, I: 'f + Instruction> {
    /// Functions defined within this module
    pub functions: Vec<Function<'f, I>>,
}
