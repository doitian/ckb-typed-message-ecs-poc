use super::*;
use ckb_testtool::{
    ckb_types::{bytes::Bytes, core::TransactionBuilder, packed, prelude::*},
    context::Context,
};

include!("../../contracts/component-lock/src/error_include.rs");

pub struct Setup {
    pub context: Context,

    pub owner_lock_out_point: packed::OutPoint,
    pub component_lock_out_point: packed::OutPoint,

    pub alice_owner_lock_script: packed::Script,
    pub alice_component_lock_script: packed::Script,
    pub bob_owner_lock_script: packed::Script,
    pub bob_component_lock_script: packed::Script,
}

impl Setup {
    fn new() -> Self {
        let mut context = Context::default();

        let owner_lock_out_point =
            context.deploy_cell(Loader::default().load_binary("always-success"));
        let component_lock_out_point =
            context.deploy_cell(Loader::default().load_binary("component-lock"));

        let alice_owner_lock_script = context
            .build_script(&owner_lock_out_point, Bytes::from(vec![42]))
            .expect("script");
        let alice_component_lock_script = context
            .build_script(
                &component_lock_out_point,
                alice_owner_lock_script.as_bytes(),
            )
            .expect("script");

        let bob_owner_lock_script = context
            .build_script(&owner_lock_out_point, Bytes::from(vec![37]))
            .expect("script");
        let bob_component_lock_script = context
            .build_script(&component_lock_out_point, bob_owner_lock_script.as_bytes())
            .expect("script");

        Self {
            context,

            owner_lock_out_point,
            component_lock_out_point,
            alice_owner_lock_script,
            alice_component_lock_script,
            bob_owner_lock_script,
            bob_component_lock_script,
        }
    }

    fn c(&mut self) -> &mut Context {
        return &mut self.context;
    }

    fn input(&mut self, lock: packed::Script, capacity: u64) -> packed::CellInput {
        let out_point = self.context.create_cell(
            packed::CellOutput::new_builder()
                .capacity(capacity.pack())
                .lock(lock)
                .build(),
            Bytes::new(),
        );
        packed::CellInput::new_builder()
            .previous_output(out_point)
            .build()
    }

    fn output(&self, lock: packed::Script, capacity: u64) -> packed::CellOutput {
        packed::CellOutput::new_builder()
            .capacity(capacity.pack())
            .lock(lock)
            .build()
    }
}

#[test]
fn test_component_lock_same_balance() {
    let mut env = Setup::new();

    let tx = TransactionBuilder::default()
        .input(env.input(env.alice_component_lock_script.clone(), 200u64))
        .output(env.output(env.alice_component_lock_script.clone(), 200u64))
        .output_data(Bytes::new().pack())
        .build();
    assert_tx_ok(env.c(), tx, "same balance");
}

#[test]
fn test_component_lock_insufficient_balance() {
    let mut env = Setup::new();

    let tx = TransactionBuilder::default()
        .input(env.input(env.alice_component_lock_script.clone(), 201u64))
        .output(env.output(env.alice_component_lock_script.clone(), 200u64))
        .output_data(Bytes::new().pack())
        .build();

    assert_tx_err(
        env.c(),
        tx,
        "insufficient balance",
        Error::BalanceError as i8,
    );
}

#[test]
fn test_owner_unlocking() {
    let mut env = Setup::new();

    let tx = TransactionBuilder::default()
        .input(env.input(env.alice_owner_lock_script.clone(), 200u64))
        .input(env.input(env.alice_component_lock_script.clone(), 300u64))
        .output(env.output(env.bob_owner_lock_script.clone(), 100u64))
        .output_data(Bytes::new().pack())
        .build();
    assert_tx_ok(env.c(), tx, "transfer some to bob and burn the rest");
}

#[test]
fn test_transfer_to_owner_with_same_balance() {
    let mut env = Setup::new();

    let tx = TransactionBuilder::default()
        .input(env.input(env.alice_component_lock_script.clone(), 300u64))
        .output(env.output(env.alice_owner_lock_script.clone(), 300u64))
        .output_data(Bytes::new().pack())
        .build();
    assert_tx_ok(env.c(), tx, "transfer to owner");
}

#[test]
fn test_transfer_to_owner_with_insufficient_balance() {
    let mut env = Setup::new();

    let tx = TransactionBuilder::default()
        .input(env.input(env.alice_component_lock_script.clone(), 301u64))
        .output(env.output(env.alice_owner_lock_script.clone(), 300u64))
        .output_data(Bytes::new().pack())
        .build();
    assert_tx_err(
        env.c(),
        tx,
        "insufficent balance",
        Error::BalanceError as i8,
    );
}

#[test]
fn test_transfer_to_owner_with_same_balance_in_multiple_cells() {
    let mut env = Setup::new();

    let tx = TransactionBuilder::default()
        .input(env.input(env.alice_component_lock_script.clone(), 100u64))
        .input(env.input(env.alice_component_lock_script.clone(), 200u64))
        .output(env.output(env.alice_owner_lock_script.clone(), 150u64))
        .output(env.output(env.alice_owner_lock_script.clone(), 150u64))
        .outputs_data(vec![Bytes::new(); 2].pack())
        .build();
    assert_tx_ok(env.c(), tx, "transfer to owner");
}

#[test]
fn test_transfer_to_owner_with_insufficient_balance_in_multiple_cells() {
    let mut env = Setup::new();

    let tx = TransactionBuilder::default()
        .input(env.input(env.alice_component_lock_script.clone(), 101u64))
        .input(env.input(env.alice_component_lock_script.clone(), 200u64))
        .output(env.output(env.alice_owner_lock_script.clone(), 150u64))
        .output(env.output(env.alice_owner_lock_script.clone(), 150u64))
        .outputs_data(vec![Bytes::new(); 2].pack())
        .build();

    assert_tx_err(
        env.c(),
        tx,
        "insufficent balance",
        Error::BalanceError as i8,
    );
}
