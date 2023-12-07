use super::*;
use ckb_testtool::{
    ckb_types::{bytes::Bytes, core::TransactionBuilder, packed, prelude::*},
    context::Context,
};

include!("../../contracts/component-definition-type/src/error_include.rs");

pub struct Setup {
    pub context: Context,

    pub component_definition_type_out_point: packed::OutPoint,
    pub always_success_out_point: packed::OutPoint,

    pub always_success_script: packed::Script,
}

fn data() -> Bytes {
    use ckb_ecs_schemas::*;
    use molecule::prelude::*;

    let script = ScriptBuilder::default()
        .code_hash([1u8; 32].into())
        .hash_type(1.into())
        .args((&[] as &[u8]).into())
        .build();
    let definition_v1 = ComponentDefinitionV1Builder::default()
        .component_name("test".into())
        .info_hash([42u8; 32].into())
        .delegate(script)
        .build();
    ComponentDefinitionBuilder::default()
        .set(definition_v1)
        .build()
        .as_bytes()
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
        }
    }

    fn c(&mut self) -> &mut Context {
        return &mut self.context;
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

    let dummy_id = Bytes::from(vec![1u8; 32]);

    let tx = TransactionBuilder::default()
        .input(env.input(Some(dummy_id.clone())))
        .output(env.output(Some(dummy_id.clone())))
        .output_data(data().pack())
        .build();
    assert_tx_ok(env.c(), tx, "update cell with type_id");
}

#[test]
fn test_delete_type_id() {
    let mut env = Setup::new();

    let dummy_id = Bytes::from(vec![1u8; 32]);

    // delete
    let tx = TransactionBuilder::default()
        .input(env.input(Some(dummy_id.clone())))
        .output(env.output(None))
        .output_data(data().pack())
        .build();
    assert_tx_ok(env.c(), tx, "delete cell with type_id");
}

#[test]
fn test_create_type_id() {
    let mut env = Setup::new();

    // create
    let input = env.input(None);
    let new_id = new_type_id(&input, 0);
    let tx = TransactionBuilder::default()
        .input(input)
        .output(env.output(Some(new_id)))
        .output_data(data().pack())
        .build();
    assert_tx_ok(env.c(), tx, "create cell with type_id");
}

#[test]
fn test_invalid_data() {
    let mut env = Setup::new();

    // create
    let input = env.input(None);
    let new_id = new_type_id(&input, 0);
    let mut data = data().to_vec();
    data[2] = !data[2];
    let tx = TransactionBuilder::default()
        .input(input)
        .output(env.output(Some(new_id)))
        .output_data(data.pack())
        .build();
    assert_tx_err(
        env.c(),
        tx,
        "create cell with invalid data",
        Error::InvalidData as i8,
    );
}

fn new_type_id(input: &packed::CellInput, output_index: u64) -> Bytes {
    let mut blake2b = new_blake2b();

    blake2b.update(input.as_slice());
    blake2b.update(&output_index.to_le_bytes());
    let mut ret = vec![0; 32];
    blake2b.finalize(&mut ret);
    Bytes::from(ret)
}
