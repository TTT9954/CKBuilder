
use ckb_testtool::{
    context::Context,
    ckb_types::{
        bytes::Bytes,
        core::{TransactionBuilder, TransactionView},
    	packed::*,
        prelude::*,
    },
};
use std::path::PathBuf;

const MAX_CYCLES: u64 = 10_000_000;

fn build_test_transaction(args: &[u8]) -> (Context, TransactionView) {
    let mut context = Context::default();

    // Load the compiled RISC-V contract.
    let contract_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("contract/target/riscv64imac-unknown-none-elf/release/ckb-hello-script");

    let contract_bin: Bytes =
        std::fs::read(contract_path).expect("contract binary not found").into();

    // Deploy the contract into the mock context.
    let out_point = context.deploy_cell(contract_bin);

    // Create a lock Script with the requested arguments.
    let lock_script = context
        .build_script(&out_point, Bytes::from(args.to_vec()))
        .expect("failed to build script");

    // Create an input cell.
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64)
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );

    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    // Create an output cell.
    let output = CellOutput::new_builder()
        .capacity(1000u64)
        .lock(lock_script)
        .build();

    // Build and complete the transaction.
    let tx = TransactionBuilder::default()
        .input(input)
        .output(output)
        .outputs_data(vec![Bytes::new()].pack())
        .build();

    let tx = context.complete_tx(tx);

    (context, tx)
}

#[test]
fn test_args_hello() {
    let (context, tx) = build_test_transaction(b"hello");

    // Export the transaction for ckb-debugger.
    let dump = context.dump_tx(&tx).expect("failed to dump transaction");

    std::fs::write(
        "hello-tx.json",
        serde_json::to_string_pretty(&dump).unwrap(),
    )
    .expect("failed to write transaction file");

    let result = context.verify_tx(&tx, MAX_CYCLES);

    assert!(result.is_ok(), "Expected hello arguments to pass");

    println!("HELLO TEST PASSED");
    println!("Transaction saved to hello-tx.json");
}

#[test]
fn test_args_wrong() {
    let (context, tx) = build_test_transaction(b"wrong");

    let result = context.verify_tx(&tx, MAX_CYCLES);

    assert!(result.is_err(), "Expected wrong arguments to fail");

    println!("WRONG TEST PASSED: invalid arguments were rejected");
}
