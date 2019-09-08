mod properties;
mod protocol;
mod reason_code;

#[derive(Debug, Clone, PartialEq)]
pub struct VariableHeader
{
    data : u8
}