use ckb_testtool::{
    builtin::ALWAYS_SUCCESS,
    ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*},
    context::Context,
};

use super::FullListCase;
use crate::{prelude::*, utilities, Loader};

#[test]
fn success_shortest() {
    let case = FullListCase {
        demo_data: &[
            (b"1-st", &[1, 1, 1], &[2, 2, 2]),
            (b"2-nd", &[2, 2, 2], &[1, 1, 1]),
        ],
        should_passed: true,
    };
    run_test(case);
}

#[test]
fn success_case_1() {
    let case = FullListCase {
        demo_data: &[
            (b"1-st", &[1, 1, 1], &[1, 1, 2]),
            (b"2-nd", &[1, 1, 2], &[1, 2, 2]),
            (b"3-rd", &[1, 2, 2], &[3, 2, 2]),
            (b"4-th", &[3, 2, 2], &[1, 1, 1]),
        ],
        should_passed: true,
    };
    run_test(case);
}

#[test]
fn single_item() {
    let case = FullListCase {
        demo_data: &[(b"1-st", &[1, 1, 1], &[1, 1, 1])],
        should_passed: false,
    };
    run_test(case);
}

#[test]
fn unordered() {
    let case = FullListCase {
        demo_data: &[
            (b"1-st", &[1, 1, 1], &[1, 1, 2]),
            (b"3-rd", &[1, 2, 2], &[3, 2, 2]),
            (b"2-nd", &[1, 1, 2], &[1, 2, 2]),
            (b"4-th", &[3, 2, 2], &[1, 1, 1]),
        ],
        should_passed: true,
    };
    run_test(case);
}

#[test]
fn incomplete() {
    let case = FullListCase {
        demo_data: &[
            (b"1-st", &[1, 1, 1], &[1, 1, 2]),
            (b"3-rd", &[1, 2, 2], &[3, 2, 2]),
            (b"2-nd", &[1, 1, 2], &[1, 2, 2]),
            (b"4-th", &[3, 2, 2], &[4, 4, 4]),
        ],
        should_passed: false,
    };
    run_test(case);
}

#[test]
fn corrupted() {
    let case = FullListCase {
        demo_data: &[
            (b"1-st", &[1, 1, 1], &[1, 1, 2]),
            (b"3-rd", &[1, 2, 2], &[3, 2, 2]),
            (b"4-th", &[3, 2, 2], &[1, 1, 1]),
        ],
        should_passed: false,
    };
    run_test(case);
}

fn run_test(case: FullListCase) {
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
            .type_(type_script_opt)
            .build();
        case.demo_data()
            .into_iter()
            .map(|bytes| {
                let out_point = context.create_cell(output.clone(), bytes);
                CellInput::new_builder().previous_output(out_point).build()
            })
            .collect::<Vec<_>>()
    };

    // prepare outputs
    let outputs = vec![CellOutput::new_builder()
        .capacity(500u64.pack())
        .lock(lock_script.clone())
        .build()];
    let outputs_data = vec![Bytes::new(); outputs.len()];

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
