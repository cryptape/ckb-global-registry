use alloc::vec::Vec;
use core::result::Result as CoreResult;

use ckb_linked_list_tool::{check_linked_list_with_unordered_items, types::ListItem};
use ckb_std::{ckb_constants::Source, ckb_types::prelude::*, error::SysError, high_level as hl};
use demo_linked_list_lib::types;

use crate::error::{InternalError, Result};

pub(crate) fn update(inputs_indexes: Vec<usize>, outputs_indexes: Vec<usize>) -> Result<()> {
    let inputs_summary = {
        let inputs_data_iter = inputs_indexes.into_iter().map(|index| {
            debug!("load the data from inputs[{index}]");
            hl::load_cell_data(index, Source::Input)
        });
        check_linked_list_with_unordered_items(inputs_data_iter, parse_linked_list_items)
    }?;

    let outputs_summary = {
        let outputs_data_iter = outputs_indexes.into_iter().map(|index| {
            debug!("load the data from outputs[{index}]");
            hl::load_cell_data(index, Source::Output)
        });
        check_linked_list_with_unordered_items(outputs_data_iter, parse_linked_list_items)
    }?;

    if inputs_summary != outputs_summary {
        return Err(InternalError::UpdateMismatchRange.into());
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
