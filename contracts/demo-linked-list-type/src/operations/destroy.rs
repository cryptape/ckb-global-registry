use alloc::vec::Vec;
use core::result::Result as CoreResult;

use ckb_linked_list_tool::{check_linked_list_with_unordered_items, types::ListItem};
use ckb_std::{ckb_constants::Source, ckb_types::prelude::*, error::SysError, high_level as hl};
use demo_linked_list_lib::types;

use crate::error::{InternalError, Result};

// The checks, which are checked in the create operation, are ignored.
pub(crate) fn destroy(indexes: Vec<usize>) -> Result<()> {
    debug!("execute destroy operation: {indexes:?}");

    // An example:
    // - To parse data during the check.
    // - The items could be unordered, in case users have their own data which
    //   required to be ordered.
    {
        let inputs_data_iter = indexes.into_iter().map(|index| {
            debug!("load the data from inputs[{index}]");
            hl::load_cell_data(index, Source::Input)
        });

        let summary =
            check_linked_list_with_unordered_items(inputs_data_iter, parse_linked_list_items)?;

        if !summary.is_complete() {
            return Err(InternalError::DestroyIncompleteList.into());
        }
    }

    Ok(())
}

pub(crate) fn parse_linked_list_items(
    data_res: CoreResult<Vec<u8>, SysError>,
) -> Result<ListItem<types::Bytes>> {
    let data = data_res?;
    let reader = types::DemoDataReader::from_slice(&data)
        .map_err(|_| InternalError::DestroyInvalidInputData)?;
    Ok(ListItem::new(
        reader.curr().to_entity(),
        reader.next().to_entity(),
    ))
}
