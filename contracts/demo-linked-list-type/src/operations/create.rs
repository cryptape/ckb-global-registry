use alloc::vec::Vec;
use core::result::Result as CoreResult;

use ckb_hash::{new_blake2b, BLAKE2B_LEN};
use ckb_linked_list_tool::{check_linked_list_with_ordered_items, types::ListItem};
use ckb_std::{ckb_constants::Source, ckb_types::prelude::*, high_level as hl};
use demo_linked_list_lib::types;

use crate::error::{InternalError, Result};

pub(crate) fn create(indexes: Vec<usize>) -> Result<()> {
    debug!("execute create operation: {indexes:?}");

    // Load script args.
    let script = hl::load_script()?;
    let script_args = script.args();
    let script_args_slice = script_args.as_reader().raw_data();

    // Check the script args: args length.
    if script_args_slice.len() != 32 {
        return Err(InternalError::CreateInvalidArgsLength.into());
    }

    // Check the script args: args[0], 32 bytes, the unique ID.
    let unique_id = load_then_calculate_unique_id(indexes[0])?;
    if unique_id != &script_args_slice[..] {
        return Err(InternalError::CreateIncorrectUniqueId.into());
    }

    // An example:
    // - To pass borrowed data as inputs.
    // - The items should be ordered.
    {
        let outputs_data = indexes
            .into_iter()
            .map(|index| {
                debug!("load the data from outputs[{index}]");
                hl::load_cell_data(index, Source::Output)
            })
            .collect::<CoreResult<Vec<_>, _>>()?;

        let outputs_data_slice = outputs_data
            .iter()
            .map(|data| {
                types::DemoDataReader::from_slice(data)
                    .map_err(|_| InternalError::CreateInvalidOutputData)
            })
            .collect::<CoreResult<Vec<_>, _>>()?;

        let summary =
            check_linked_list_with_ordered_items(outputs_data_slice, parse_linked_list_items)?;

        if !summary.is_complete() {
            return Err(InternalError::CreateIncompleteList.into());
        }
    }

    Ok(())
}

pub(crate) fn parse_linked_list_items(
    reader: types::DemoDataReader<'_>,
) -> Result<ListItem<types::BytesReader<'_>>> {
    Ok(ListItem::new(reader.curr(), reader.next()))
}

// Load the first input and the index of the first output which uses current
// script, then calculate an unique ID with them.
pub(crate) fn load_then_calculate_unique_id(output_index: usize) -> Result<[u8; BLAKE2B_LEN]> {
    let input = hl::load_input(0, Source::Input)?;
    let mut blake2b = new_blake2b();
    blake2b.update(input.as_slice());
    blake2b.update(&(output_index as u64).to_le_bytes());
    let mut ret = [0; BLAKE2B_LEN];
    blake2b.finalize(&mut ret);
    Ok(ret)
}
