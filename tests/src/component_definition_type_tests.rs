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

include!("../../contracts/component-definition-type/src/error_include.rs");

pub struct Setup {
    pub context: Context,

    pub dump_tx: bool,

    pub component_definition_type_out_point: packed::OutPoint,
    pub always_success_out_point: packed::OutPoint,

    pub always_success_script: packed::Script,
}

impl Setup {
    fn new() -> Self {
        let mut context = Context::default();

        let always_success_out_point =
            context.deploy_cell(Loader::default().load_binary("always-success"));
        let component_definition_type_out_point =
            context.deploy_cell(Loader::default().load_binary("component-definition-type"));

        let always_success_script = context
            .build_script(&always_success_out_point, Bytes::new())
            .expect("script");

        Self {
            context,
            component_definition_type_out_point,
            always_success_out_point,
            always_success_script,

            dump_tx: false,
        }
    }

    fn verify_tx(&mut self, tx: &TransactionView) -> Result<u64, CKBError> {
        if self.dump_tx {
            let json: ckb_jsonrpc_types::TransactionView = tx.clone().into();
            println!("{}", serde_json::to_string(&json).unwrap());
        }
        self.context.verify_tx(tx, MAX_CYCLES)
    }

    fn assert_tx_ok(&mut self, tx: &TransactionView, msg: &str) {
        if let Err(err) = self.verify_tx(tx) {
            panic!("expect {} ok but got err: {}", msg, err);
        }
    }

    fn assert_tx_err(&mut self, tx: &TransactionView, msg: &str, err_code: Error) {
        let err_code = err_code as i8;
        match self.verify_tx(tx) {
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

    fn input(&mut self, id_opt: Option<Bytes>) -> packed::CellInput {
        let type_opt = id_opt.map(|id| {
            self.context
                .build_script(&self.component_definition_type_out_point, id)
                .expect("script")
        });
        // prepare cells
        let out_point = self.context.create_cell(
            packed::CellOutput::new_builder()
                .capacity(2000.pack())
                .lock(self.always_success_script.clone())
                .type_(type_opt.pack())
                .build(),
            Bytes::new(),
        );
        packed::CellInput::new_builder()
            .previous_output(out_point)
            .build()
    }

    fn output(&mut self, id_opt: Option<Bytes>) -> packed::CellOutput {
        packed::CellOutput::new_builder()
            .capacity(200.pack())
            .lock(self.always_success_script.clone())
            .type_(
                id_opt
                    .map(|id| {
                        self.context
                            .build_script(&self.component_definition_type_out_point, id)
                            .expect("script")
                    })
                    .pack(),
            )
            .build()
    }
}

#[test]
fn test_update_type_id() {
    let mut env = Setup::new();
    env.dump_tx = true;

    let dummy_id = Bytes::from(vec![1u8; 32]);

    let tx_skeleton = TransactionBuilder::default()
        .input(env.input(Some(dummy_id.clone())))
        .output(env.output(Some(dummy_id.clone())))
        .output_data(Bytes::new().pack())
        .build();
    let tx = env.context.complete_tx(tx_skeleton);
    env.assert_tx_ok(&tx, "update cell with type_id");
}

#[test]
fn test_delete_type_id() {
    let mut env = Setup::new();
    env.dump_tx = true;

    let dummy_id = Bytes::from(vec![1u8; 32]);

    // delete
    let tx_skeleton = TransactionBuilder::default()
        .input(env.input(Some(dummy_id.clone())))
        .output(env.output(None))
        .output_data(Bytes::new().pack())
        .build();
    let tx = env.context.complete_tx(tx_skeleton);
    env.assert_tx_ok(&tx, "delete cell with type_id");
}

#[test]
fn test_create_type_id() {
    let mut env = Setup::new();
    env.dump_tx = true;

    // create
    let input = env.input(None);
    let new_id = new_type_id(&input, 0);
    let tx_skeleton = TransactionBuilder::default()
        .input(input)
        .output(env.output(Some(new_id)))
        .output_data(Bytes::new().pack())
        .build();
    let tx = env.context.complete_tx(tx_skeleton);
    env.assert_tx_ok(&tx, "create cell with type_id");
}

fn new_type_id(input: &packed::CellInput, output_index: u64) -> Bytes {
    let mut blake2b = new_blake2b();

    blake2b.update(input.as_slice());
    blake2b.update(&output_index.to_le_bytes());
    let mut ret = vec![0; 32];
    blake2b.finalize(&mut ret);
    Bytes::from(ret)
}
