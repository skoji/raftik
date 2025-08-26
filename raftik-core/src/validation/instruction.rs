use super::Context;
use super::error::ValidationError;
use crate::ast::instructions::RawExpression;
use crate::ast::types::FunctionType;

pub fn validate_raw_expression(
    mut _ctx: &mut Context,
    _t: &FunctionType,
    _expr: &RawExpression,
    _position_string_on_error: String,
) -> Result<(), ValidationError> {
    Ok(())
}
