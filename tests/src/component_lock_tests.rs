use super::*;
use ckb_testtool::{
    ckb_error::Error as CKBError,
    ckb_jsonrpc_types,
    ckb_types::{
        bytes::Bytes,
        core::{TransactionBuilder, TransactionView},
        packed,
        prelude::*,
    },
    context::Context,
};

const MAX_CYCLES: u64 = 10_000_000;

include!("../../contracts/component-lock/src/error_include.rs");

pub struct Setup {
    pub context: Context,

    pub dump_tx: bool,

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

            dump_tx: false,
        }
    }

    fn build_tx(
        &mut self,
        inputs: Vec<(packed::Script, u64)>,
        outputs: Vec<(packed::Script, u64)>,
    ) -> TransactionView {
        let mut builder = TransactionBuilder::default();
        for (lock, capacity) in inputs {
            // prepare cells
            let out_point = self.context.create_cell(
                packed::CellOutput::new_builder()
                    .capacity(capacity.pack())
                    .lock(lock)
                    .build(),
                Bytes::new(),
            );
            let input = packed::CellInput::new_builder()
                .previous_output(out_point)
                .build();
            builder = builder.input(input);
        }
        let outputs_data = vec![Bytes::new(); outputs.len()];
        for (lock, capacity) in outputs {
            let output = packed::CellOutput::new_builder()
                .capacity(capacity.pack())
                .lock(lock)
                .build();
            builder = builder.output(output);
        }
        let tx = builder.outputs_data(outputs_data.pack()).build();
        if self.dump_tx {
            let json: ckb_jsonrpc_types::TransactionView = tx.clone().into();
            println!("{}", serde_json::to_string(&json).unwrap());
        }
        self.context.complete_tx(tx)
    }

    fn verify_tx(
        &mut self,
        inputs: Vec<(packed::Script, u64)>,
        outputs: Vec<(packed::Script, u64)>,
    ) -> Result<u64, CKBError> {
        let tx = self.build_tx(inputs, outputs);
        self.context.verify_tx(&tx, MAX_CYCLES)
    }

    fn assert_tx_ok(
        &mut self,
        inputs: Vec<(packed::Script, u64)>,
        outputs: Vec<(packed::Script, u64)>,
        msg: &str,
    ) {
        if let Err(err) = self.verify_tx(inputs, outputs) {
            panic!("expect {} ok but got err: {}", msg, err);
        }
    }

    fn assert_tx_err(
        &mut self,
        inputs: Vec<(packed::Script, u64)>,
        outputs: Vec<(packed::Script, u64)>,
        msg: &str,
        err_code: Error,
    ) {
        let err_code = err_code as i8;
        match self.verify_tx(inputs, outputs) {
            Ok(_) => panic!("expect {} with err code {} but got ok", msg, err_code),
            Err(err) => {
                assert!(
                    err.to_string()
                        .contains(format!("error code {} ", err_code).as_str()),
                    "expect {} with err code {} but got: {}",
                    msg,
                    err_code,
                    err
                )
            }
        }
    }
}

#[test]
fn test_only_component_lock() {
    let mut env = Setup::new();
    env.dump_tx = true;

    env.assert_tx_ok(
        vec![(env.alice_component_lock_script.clone(), 200u64)],
        vec![(env.alice_component_lock_script.clone(), 200u64)],
        "same",
    );
    env.assert_tx_err(
        vec![(env.alice_component_lock_script.clone(), 201u64)],
        vec![(env.alice_component_lock_script.clone(), 200u64)],
        "insufficient balance",
        Error::BalanceError,
    );
}

#[test]
fn test_owner_unlocking() {
    let mut env = Setup::new();
    env.dump_tx = true;

    env.assert_tx_ok(
        vec![
            (env.alice_owner_lock_script.clone(), 200u64),
            (env.alice_component_lock_script.clone(), 300u64),
        ],
        vec![(env.bob_owner_lock_script.clone(), 100u64)],
        "transfer some to bob and burn the rest",
    );
}

#[test]
fn test_transfer_to_owner_lock_without_unlocking() {
    let mut env = Setup::new();
    env.dump_tx = true;

    env.assert_tx_ok(
        vec![(env.alice_component_lock_script.clone(), 300u64)],
        vec![(env.alice_owner_lock_script.clone(), 300u64)],
        "transfer to owner",
    );
    env.assert_tx_err(
        vec![(env.alice_component_lock_script.clone(), 301u64)],
        vec![(env.alice_owner_lock_script.clone(), 300u64)],
        "insufficent balance",
        Error::BalanceError,
    );

    // split cells
    env.assert_tx_ok(
        vec![
            (env.alice_component_lock_script.clone(), 100u64),
            (env.alice_component_lock_script.clone(), 200u64),
        ],
        vec![
            (env.alice_owner_lock_script.clone(), 150u64),
            (env.alice_owner_lock_script.clone(), 150u64),
        ],
        "transfer to owner",
    );
    env.assert_tx_err(
        vec![
            (env.alice_component_lock_script.clone(), 101u64),
            (env.alice_component_lock_script.clone(), 200u64),
        ],
        vec![
            (env.alice_owner_lock_script.clone(), 150u64),
            (env.alice_owner_lock_script.clone(), 150u64),
        ],
        "insufficent balance",
        Error::BalanceError,
    );
}
