use alloc::vec::Vec;

#[cfg(debug_assertions)]
use ckb_std::ckb_types::prelude::*;
use ckb_std::{ckb_constants::Source, high_level as hl};

use crate::{
    error::{InternalError, Result},
    operations,
};

pub fn main() -> Result<()> {
    debug!("{} Starting ...", module_path!());

    let script_hash = hl::load_script_hash()?;
    debug!("script hash = {:#x}", script_hash.pack());

    // Find all input cells which use current script.
    let indexes_of_inputs = {
        let mut indexes = Vec::new();
        for (index, type_hash_opt) in
            hl::QueryIter::new(hl::load_cell_type_hash, Source::Input).enumerate()
        {
            if let Some(type_hash) = type_hash_opt {
                debug!("{index}-th type hash of inputs: {:#x}", type_hash.pack());
                if type_hash == script_hash {
                    debug!("found cell: inputs[{index}]");
                    indexes.push(index);
                }
            }
        }
        indexes
    };

    // Find all output cells which use current script.
    let indexes_of_outputs = {
        let mut indexes = Vec::new();
        for (index, type_hash_opt) in
            hl::QueryIter::new(hl::load_cell_type_hash, Source::Output).enumerate()
        {
            if let Some(type_hash) = type_hash_opt {
                debug!("{index}-th type hash of outputs: {:#x}", type_hash.pack());
                if type_hash == script_hash {
                    debug!("found cell: outputs[{index}]");
                    indexes.push(index);
                }
            }
        }
        indexes
    };

    debug!("cells in  inputs: {indexes_of_inputs:?}");
    debug!("cells in outputs: {indexes_of_outputs:?}");

    match (indexes_of_inputs.len(), indexes_of_outputs.len()) {
        // The current data is NOT allowed to be the same as the next data.
        // So, the length of outputs should be always greater than 1.
        (0, n) if n > 1 => {
            debug!("create a new global-registry instance with {n} items");
            operations::create(indexes_of_outputs)?;
        }
        (n, 0) if n > 1 => {
            debug!("destroy the global-registry instance with {n} items");
            operations::destroy(indexes_of_inputs)?;
        }
        (m, n) if m > 0 && n > 0 => {
            debug!("update items in the global-registry: {m} -> {n}");
            operations::update(indexes_of_inputs, indexes_of_outputs)?;
        }
        (_m, _n) => {
            debug!("unknown operation: {_m} inputs and {_n} outputs");
            return Err(InternalError::UnknownOperation.into());
        }
    }

    debug!("{} DONE.", module_path!());

    Ok(())
}
