use ckb_testtool::ckb_types::{bytes::Bytes, prelude::*};
use demo_linked_list_lib::types;

mod create;
mod destroy;
mod update;

pub(crate) struct FullListCase<'a, 'b> {
    demo_data: &'a [(&'b [u8], &'b [u8], &'b [u8])],
    should_passed: bool,
}

impl FullListCase<'_, '_> {
    pub(crate) fn demo_data(&self) -> Vec<Bytes> {
        self.demo_data
            .iter()
            .map(|(x, y, z)| {
                let demo_data = types::DemoData::new_from_raw_slices(x, y, z);
                Bytes::copy_from_slice(demo_data.as_slice())
            })
            .collect()
    }
}
