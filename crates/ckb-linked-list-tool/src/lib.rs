//! A tool to create a linked list between CKB cells, so that to help users to
//! build a global registry based on the linked list.

#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::cmp::{Ord, Ordering};

pub mod error;
pub mod types;

use crate::{
    error::Error,
    types::{DataParseFunc, ListItem, ListItemsSummary},
};

/// Checks a part of linked list with ordered items.
///
/// There are 2 arguments:
///
/// - `inputs`: a collection of data, that each data contains an item of the
///    linked list.
///
///    It's a type which implements `IntoIterator`, to avoid many raw data are
///    existed in a same time so that out of memory.
///
///    Note: the items in `inputs` should be ordered.
///
/// - `parse_func`: a function to parse the fields of an itme on a linked list
///   the raw data.
///
///   It requires a raw data as input, and returns a tuple with 2 items: the
///   first item in the tuple is the current data in the linked list, and
///   another item is the next data in the linked list.
pub fn check_linked_list_with_ordered_items<DataLoader, Data, Field, E>(
    inputs: DataLoader,
    parse_func: DataParseFunc<Data, ListItem<Field>, E>,
) -> Result<ListItemsSummary<Field>, E>
where
    DataLoader: IntoIterator<Item = Data>,
    Field: Ord,
    E: From<Error>,
{
    let mut inputs_iter = inputs.into_iter();
    let mut reach_last = false;
    let (start, mut next) = {
        let data = if let Some(data) = inputs_iter.next() {
            data
        } else {
            return Err(Error::EmptyList.into());
        };
        let item = parse_func(data)?;
        match item.curr.cmp(&item.next) {
            Ordering::Less => {}
            Ordering::Equal => {
                return Err(Error::NextIsSelfItem.into());
            }
            Ordering::Greater => {
                reach_last = true;
            }
        }
        (item.curr, item.next)
    };
    for data in inputs_iter {
        let item = parse_func(data)?;
        if next != item.curr {
            return Err(Error::Discontinuous.into());
        }
        match item.curr.cmp(&item.next) {
            Ordering::Less => {}
            Ordering::Equal => {
                return Err(Error::NextIsSelfItem.into());
            }
            Ordering::Greater => {
                if reach_last {
                    return Err(Error::ReachLastTwice.into());
                }
                reach_last = true;
            }
        }
        next = item.next;
    }
    let state = ListItemsSummary::new(start, next);
    Ok(state)
}

/// Checks a part of linked list with unordered items.
///
/// This method has the same arguments as [`check_linked_list_with_ordered_items`].
pub fn check_linked_list_with_unordered_items<DataLoader, Data, Field, E>(
    inputs: DataLoader,
    parse_func: DataParseFunc<Data, ListItem<Field>, E>,
) -> Result<ListItemsSummary<Field>, E>
where
    DataLoader: IntoIterator<Item = Data>,
    Field: Ord,
    E: From<Error>,
{
    let (has_last, mut items_iter) = {
        let mut items = inputs
            .into_iter()
            .map(parse_func)
            .collect::<Result<Vec<_>, _>>()?;
        if items.is_empty() {
            return Err(Error::EmptyList.into());
        }
        items.sort_unstable_by(|a, b| a.curr.partial_cmp(&b.curr).unwrap());
        let has_last = items[0].curr == items[items.len() - 1].next;
        (has_last, items.into_iter())
    };
    let mut reach_last = false;
    let mut end_opt = None;
    let (mut start, mut next) = {
        let item = items_iter.next().expect("checked");
        match item.curr.cmp(&item.next) {
            Ordering::Less => {}
            Ordering::Equal => {
                return Err(Error::NextIsSelfItem.into());
            }
            Ordering::Greater => {
                reach_last = true;
            }
        }
        (item.curr, item.next)
    };
    for item in items_iter {
        match item.curr.cmp(&item.next) {
            Ordering::Less => {}
            Ordering::Equal => {
                return Err(Error::NextIsSelfItem.into());
            }
            Ordering::Greater => {
                if reach_last {
                    return Err(Error::ReachLastTwice.into());
                }
                reach_last = true;
            }
        }
        if next != item.curr {
            if end_opt.is_some() || !has_last {
                return Err(Error::Discontinuous.into());
            }
            start = item.curr;
            end_opt = Some(next);
        }
        next = item.next;
    }
    let state = ListItemsSummary::new(start, end_opt.unwrap_or(next));
    Ok(state)
}
