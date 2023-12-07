mod blockchain;
mod component_definition;

#[allow(clippy::all)]
pub use blockchain::*;
#[allow(clippy::all)]
pub use component_definition::*;

#[test]
fn build_component_definition() {
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
    let definition = ComponentDefinitionBuilder::default()
        .set(definition_v1)
        .build();

    let reader = ComponentDefinitionReader::from_compatible_slice(definition.as_slice())
        .ok()
        .expect("decode ComponentDefinition");
    match reader.to_enum() {
        ComponentDefinitionUnionReader::ComponentDefinitionV1(reader) => {
            assert_eq!(reader.component_name().raw_data(), "test".as_bytes());
            assert_eq!(reader.delegate().hash_type().as_slice(), &[1u8]);
        }
    }
}
