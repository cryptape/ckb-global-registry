#![no_std]

use core::cmp::{Ord, Ordering};

use molecule::{bytes::Bytes, prelude::*, Number, NUMBER_SIZE};

#[allow(warnings)]
#[allow(clippy::all)]
pub mod types;

impl Ord for types::BytesReader<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.raw_data().cmp(other.raw_data())
    }
}

impl Ord for types::Bytes {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_reader().cmp(&other.as_reader())
    }
}

impl PartialOrd for types::BytesReader<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for types::Bytes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for types::BytesReader<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.raw_data() == other.raw_data()
    }
}

impl PartialEq for types::Bytes {
    fn eq(&self, other: &Self) -> bool {
        self.as_reader() == other.as_reader()
    }
}

impl Eq for types::BytesReader<'_> {}

impl Eq for types::Bytes {}

impl types::Bytes {
    pub fn new_from_raw_slice(slice: &[u8]) -> Self {
        let len = slice.len();
        let mut vec: Vec<u8> = Vec::with_capacity(NUMBER_SIZE + len);
        vec.extend_from_slice(&(len as Number).to_le_bytes()[..]);
        vec.extend_from_slice(slice);
        Self::new_unchecked(Bytes::from(vec))
    }
}

impl types::DemoData {
    pub fn new_from_raw_slices(demo: &[u8], curr: &[u8], next: &[u8]) -> Self {
        Self::new_builder()
            .demo(types::Bytes::new_from_raw_slice(demo))
            .curr(types::Bytes::new_from_raw_slice(curr))
            .next(types::Bytes::new_from_raw_slice(next))
            .build()
    }
}
