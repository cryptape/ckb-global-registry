use ckb_testtool::{
    builtin::ALWAYS_SUCCESS,
    ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*},
    context::Context,
};
use demo_linked_list_lib::types;

use crate::{prelude::*, utilities, Loader};

pub(crate) struct UpdateCase<'a, 'b> {
    inputs_data: &'a [(&'b [u8], &'b [u8], &'b [u8])],
    outputs_data: &'a [(&'b [u8], &'b [u8], &'b [u8])],
    should_passed: bool,
}

impl UpdateCase<'_, '_> {
    pub(crate) fn inputs_data(&self) -> Vec<Bytes> {
        self.inputs_data
            .iter()
            .map(|(x, y, z)| {
                let demo_data = types::DemoData::new_from_raw_slices(x, y, z);
                Bytes::copy_from_slice(demo_data.as_slice())
            })
            .collect()
    }

    pub(crate) fn outputs_data(&self) -> Vec<Bytes> {
        self.outputs_data
            .iter()
            .map(|(x, y, z)| {
                let demo_data = types::DemoData::new_from_raw_slices(x, y, z);
                Bytes::copy_from_slice(demo_data.as_slice())
            })
            .collect()
    }
}

#[test]
fn success_case_1() {
    let case = UpdateCase {
        inputs_data: &[(b"1-st", &[1, 1, 1], &[9, 9, 9])],
        outputs_data: &[(b"1-st", &[1, 1, 1], &[5]), (b"2-nd", &[5], &[9, 9, 9])],
        should_passed: true,
    };
    run_test(case);
}

#[test]
fn success_case_2() {
    let case = UpdateCase {
        inputs_data: &[
            (b"1-st", &[1, 1, 1], &[5, 5]),
            (b"last", &[5, 5], &[9, 9, 9]),
        ],
        outputs_data: &[
            (b"1-st", &[1, 1, 1], &[3]),
            (b"2-nd", &[3], &[5, 5]),
            (b"last", &[5, 5], &[9, 9, 9]),
        ],
        should_passed: true,
    };
    run_test(case);
}

#[test]
fn unordered() {
    let case = UpdateCase {
        inputs_data: &[
            (b"last", &[5, 5], &[9, 9, 9]),
            (b"1-st", &[1, 1, 1], &[5, 5]),
        ],
        outputs_data: &[
            (b"2-nd", &[3], &[5, 5]),
            (b"last", &[5, 5], &[9, 9, 9]),
            (b"1-st", &[1, 1, 1], &[3]),
        ],
        should_passed: true,
    };
    run_test(case);
}

#[test]
fn update_all_range() {
    let case = UpdateCase {
        inputs_data: &[
            (b"4-th", &[6], &[8]),
            (b"2-nd", &[2], &[4]),
            (b"last", &[8], &[9]),
            (b"3-rd", &[4], &[6]),
            (b"1-st", &[1], &[2]),
        ],
        outputs_data: &[
            (b"last", &[7], &[9]),
            (b"2-nd", &[3], &[5]),
            (b"3-rd", &[5], &[7]),
            (b"1-st", &[1], &[3]),
        ],
        should_passed: true,
    };
    run_test(case);
}

#[test]
fn update_last() {
    // Before:
    // | curr | 1 | 2 | 4 | 6 | 8 | 9 |
    // | next | 2 | 4 | 6 | 8 | 9 | 1 |
    // After:
    // | curr | 2 | 4 | 6 |
    // | next | 4 | 6 | 2 |
    let case = UpdateCase {
        inputs_data: &[
            (b"3-rd", &[9], &[1]),
            (b"last", &[1], &[2]),
            (b"1-st", &[6], &[8]),
            (b"2-nd", &[8], &[9]),
        ],
        outputs_data: &[(b"1-st", &[6], &[2])],
        should_passed: true,
    };
    run_test(case);
}

#[test]
fn update_last_with_incorrect_data() {
    let case = UpdateCase {
        inputs_data: &[
            (b"3-rd", &[9], &[1]),
            (b"last", &[1], &[2]),
            (b"1-st", &[6], &[8]),
            (b"2-nd", &[8], &[9]),
        ],
        outputs_data: &[(b"1-st", &[2], &[6])],
        should_passed: false,
    };
    run_test(case);
}

#[test]
fn break_list_at_last() {
    let case = UpdateCase {
        inputs_data: &[
            (b"3-rd", &[9], &[1]),
            (b"last", &[1], &[2]),
            (b"1-st", &[6], &[8]),
            (b"2-nd", &[8], &[9]),
        ],
        outputs_data: &[(b"1-st", &[6], &[5])],
        should_passed: false,
    };
    run_test(case);
}

#[test]
fn double_last() {
    let case = UpdateCase {
        inputs_data: &[
            (b"3-rd", &[9], &[1]),
            (b"last", &[1], &[2]),
            (b"1-st", &[6], &[8]),
            (b"2-nd", &[8], &[9]),
        ],
        outputs_data: &[(b"1-st", &[6], &[2]), (b"last", &[6], &[2])],
        should_passed: false,
    };
    run_test(case);
}

fn run_test(case: UpdateCase) {
    run_test_internal(&case);
    let UpdateCase {
        inputs_data,
        outputs_data,
        should_passed,
    } = case;
    let new_case = UpdateCase {
        inputs_data: outputs_data,
        outputs_data: inputs_data,
        should_passed,
    };
    run_test_internal(&new_case);
}

fn run_test_internal(case: &UpdateCase) {
    utilities::setup();

    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("demo-linked-list-type");
    let type_out_point = context.deploy_cell(contract_bin);
    let lock_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());

    // prepare lock scripts
    let lock_script = context
        .build_script(&lock_out_point, Default::default())
        .expect("lock script");
    let type_script = context
        .build_script(&type_out_point, Bytes::from([0u8; 32].to_vec()))
        .expect("type script");
    let type_script_opt = ScriptOpt::new_builder().set(Some(type_script)).build();

    // prepare inputs
    let inputs = {
        let output = CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .type_(type_script_opt.clone())
            .build();
        case.inputs_data()
            .into_iter()
            .map(|bytes| {
                let out_point = context.create_cell(output.clone(), bytes);
                CellInput::new_builder().previous_output(out_point).build()
            })
            .collect::<Vec<_>>()
    };

    // prepare outputs
    let output = CellOutput::new_builder()
        .capacity(500u64.pack())
        .lock(lock_script.clone())
        .type_(type_script_opt.clone())
        .build();
    let outputs = vec![output.clone(); case.outputs_data.len()];
    let outputs_data = case.outputs_data();

    // build transaction
    let tx = TransactionBuilder::default()
        .inputs(inputs)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .build();
    let tx = context.complete_tx(tx);

    // run
    if case.should_passed {
        let _ = context.should_be_passed_without_limit(&tx);
    } else {
        let _ = context.should_be_failed_without_limit(&tx);
    }
}
