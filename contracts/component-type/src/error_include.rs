#[repr(i8)]
#[cfg_attr(test, allow(dead_code))]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    // Add customized errors here...
    InvalidArgs,
    ComponentDefinitionNotFound,
}
