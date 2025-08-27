mod stacks;
use super::Context;
use super::error::ValidationError;
use crate::ast::instructions::RawExpression;
use crate::ast::types::{FunctionType, ValueType};

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum StackValue {
    Unknown,
    Value(ValueType),
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct ControlFrame {
    pub start_types: Vec<ValueType>,
    pub end_types: Vec<ValueType>,
    pub height_of_value_stack: usize,
    pub unreachable: bool,
}

#[allow(dead_code)]
trait ValueStack {
    fn push_val(&mut self, value: StackValue);
    fn pop_val(&mut self) -> Result<StackValue, ValidationError>;
    fn pop_expect_val(&mut self, expected: StackValue) -> Result<StackValue, ValidationError>;
    fn push_vals(&mut self, values: &[StackValue]);
    fn pop_vals(
        &mut self,
        expected_values: &[StackValue],
    ) -> Result<Vec<StackValue>, ValidationError>;
}

#[allow(dead_code)]
trait ControlStack {
    fn push_ctrl(&mut self, frame: ControlFrame);
    fn pop_ctrl(&mut self) -> Result<ControlFrame, ValidationError>;
    fn unreachable(&mut self);
}

pub fn validate_raw_expression(
    mut _ctx: &mut Context,
    t: &FunctionType,
    _expr: &RawExpression,
    _position_string_on_error: String,
) -> Result<(), ValidationError> {
    let mut stack = stacks::generate_stack();

    // push outermost control frame (regarding as a block)
    stack.push_ctrl(ControlFrame {
        start_types: vec![],
        end_types: t.results.to_vec(),
        ..Default::default()
    });

    // TODO; validate expression
    // TODO; pop and check control frame

    Ok(())
}
