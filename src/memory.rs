// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use address::Address;
use std::fmt::Debug;
use std::io;
use std::ops::Range;

#[allow(missing_docs)]
pub enum Error {
    NoSegment,
    IOError(io::Error),
}

#[allow(missing_docs)]
pub trait Segment: Debug {
    fn range(&self) -> Range<Address>;

    fn contains(&self, address: &Address, length: u64) -> bool;

    fn read_bytes(&self, address: &Address, length: u64) -> Result<&[u8], Error>;
}

#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct Memory<'m> {
    segments: Vec<&'m Segment>,
}

impl<'m> Memory<'m> {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Default::default()
    }

    #[allow(missing_docs)]
    pub fn read_bytes(&self, address: &Address, length: u64) -> Result<&[u8], Error> {
        self.segments
            .iter()
            .find(|&s| s.contains(address, length))
            .map_or(Err(Error::NoSegment), |s| s.read_bytes(address, length))
    }
}
