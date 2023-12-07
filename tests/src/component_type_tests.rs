use super::*;
use ckb_ecs_schemas::ComponentDefinition;
use ckb_testtool::{
    ckb_types::{bytes::Bytes, core::TransactionBuilder, packed, prelude::*},
    context::Context,
};

include!("../../contracts/component-type/src/error_include.rs");

pub struct Setup {
    pub context: Context,

    pub always_success_out_point: packed::OutPoint,
    pub always_success_script: packed::Script,

    pub component_type_out_point: packed::OutPoint,
}

impl Setup {
    fn new() -> Self {
        let mut context = Context::default();
        let always_success_out_point =
            context.deploy_cell(Loader::default().load_binary("always-success"));
        let always_success_script = context
            .build_script(&always_success_out_point, Bytes::new())
            .expect("script");

        let component_type_out_point =
            context.deploy_cell(Loader::default().load_binary("component-type"));

        Self {
            context,
            always_success_out_point,
            always_success_script,
            component_type_out_point,
        }
    }

    fn c(&mut self) -> &mut Context {
        return &mut self.context;
    }

    fn input(&mut self) -> packed::CellInput {
        let out_point = self.context.create_cell(
            packed::CellOutput::new_builder()
                .capacity(2000u64.pack())
                .lock(self.always_success_script.clone())
                .build(),
            Bytes::new(),
        );
        packed::CellInput::new_builder()
            .previous_output(out_point)
            .build()
    }

    fn output(&mut self, args: Bytes) -> packed::CellOutput {
        let script = self
            .context
            .build_script(&self.component_type_out_point, args)
            .expect("script");
        packed::CellOutput::new_builder()
            .capacity(2000u64.pack())
            .lock(self.always_success_script.clone())
            .type_(Some(script).pack())
            .build()
    }

    fn definition_cell(
        &mut self,
        definition: &ComponentDefinition,
        type_opt: Option<packed::Script>,
    ) -> packed::OutPoint {
        self.context.create_cell(
            packed::CellOutput::new_builder()
                .capacity(2000u64.pack())
                .lock(self.always_success_script.clone())
                .type_(type_opt.pack())
                .build(),
            definition.as_bytes(),
        )
    }
}

fn create_definition(delegate: packed::Script) -> ComponentDefinition {
    use ckb_ecs_schemas::{ComponentDefinitionBuilder, ComponentDefinitionV1Builder};

    let definition_v1 = ComponentDefinitionV1Builder::default()
        .component_name("test".into())
        .info_hash([0u8; 32].into())
        .delegate(ckb_ecs_schemas::Script::from_slice(delegate.as_slice()).expect("compatible"))
        .build();
    ComponentDefinitionBuilder::default()
        .set(definition_v1)
        .build()
}

#[test]
fn test_invalid_args_len() {
    let mut env = Setup::new();

    let tx = TransactionBuilder::default()
        .input(env.input())
        .output(env.output(Bytes::from(vec![0u8; 32])))
        .output_data(Bytes::new().pack())
        .build();

    assert_tx_err_code(env.c(), tx, "invalid args", Error::InvalidArgs as i8);
}

#[test]
fn test_component_definition_not_found() {
    let mut env = Setup::new();

    let tx = TransactionBuilder::default()
        .input(env.input())
        .output(env.output(Bytes::from(vec![0u8; 33])))
        .output_data(Bytes::new().pack())
        .build();

    assert_tx_err_code(
        env.c(),
        tx,
        "invalid args",
        Error::ComponentDefinitionNotFound as i8,
    );
}

#[test]
fn test_component_definition_found_by_data_hash() {
    let mut env = Setup::new();

    let definition = create_definition(env.always_success_script.clone());
    let definition_cell = env.definition_cell(&definition, None);

    let mut args = ckb_hash(definition.as_slice());
    args.push(0);

    let tx = TransactionBuilder::default()
        .input(env.input())
        .output(env.output(Bytes::from(args)))
        .output_data(Bytes::new().pack())
        .cell_dep(
            packed::CellDepBuilder::default()
                .out_point(definition_cell)
                .dep_type(0u8.into())
                .build(),
        )
        .build();

    assert_tx_ok(env.c(), tx, "found by data hash");
}

#[test]
fn test_component_definition_found_by_type_hash() {
    let mut env = Setup::new();

    let definition = create_definition(env.always_success_script.clone());
    let definition_type = env.always_success_script.clone();
    let definition_type_hash = ckb_hash(definition_type.as_slice());
    let definition_cell = env.definition_cell(&definition, Some(definition_type));

    let mut args = definition_type_hash;
    args.push(1);

    let tx = TransactionBuilder::default()
        .input(env.input())
        .output(env.output(Bytes::from(args)))
        .output_data(Bytes::new().pack())
        .cell_dep(
            packed::CellDepBuilder::default()
                .out_point(definition_cell)
                .dep_type(0u8.into())
                .build(),
        )
        .build();

    assert_tx_ok(env.c(), tx, "found by type hash");
}

#[test]
fn test_component_definition_delegate_err() {
    let mut env = Setup::new();

    // Create an invalid script cell
    let delegate_script_cell_out_point = env.context.deploy_cell(Bytes::from(vec![0u8]));

    // Use component type it self as delegate, it will fail because of invalid args len
    let delegate = env
        .context
        .build_script(&delegate_script_cell_out_point, Bytes::new())
        .expect("script");
    let definition = create_definition(delegate);
    let definition_cell = env.definition_cell(&definition, None);

    let mut args = ckb_hash(definition.as_slice());
    args.push(0);

    let tx = TransactionBuilder::default()
        .input(env.input())
        .output(env.output(Bytes::from(args)))
        .output_data(Bytes::new().pack())
        .cell_dep(
            packed::CellDepBuilder::default()
                .out_point(definition_cell)
                .dep_type(0u8.into())
                .build(),
        )
        .cell_dep(
            packed::CellDepBuilder::default()
                .out_point(delegate_script_cell_out_point)
                .dep_type(0u8.into())
                .build(),
        )
        .build();

    assert_tx_err_message(env.c(), tx, "delegate err", "VM Internal Error");
}
