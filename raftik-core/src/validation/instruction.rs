mod stacks;
use super::Context;
use super::error::ValidationError;
use crate::ast::instructions::RawExpression;
use crate::ast::types::{FunctionType, ValueType};

#[derive(Debug)]
enum StackValue {
    Unknown,
    Value(ValueType),
}

#[derive(Debug, Default)]
struct ControlFrame<'a> {
    pub start_types: Vec<&'a ValueType>,
    pub end_types: Vec<&'a ValueType>,
    pub height_of_value_stack: usize,
    pub unreachable: bool,
}

trait ValueStack<'a> {
    fn push_val(&mut self, value: StackValue);
    fn pop_val(&mut self) -> Result<StackValue, ValidationError>;
    fn pop_expect_val(&mut self, expected: StackValue) -> Result<StackValue, ValidationError>;
    fn push_vals(&mut self, values: &[StackValue]);
    fn pop_vals(
        &mut self,
        expected_values: &[StackValue],
    ) -> Result<&'a [StackValue], ValidationError>;
}

trait ControlStack<'a> {
    fn push_ctrl(&mut self, frame: ControlFrame<'a>);
    fn pop_ctrl(&mut self) -> Result<ControlFrame<'a>, ValidationError>;
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
        end_types: t.results.iter().collect(),
        ..Default::default()
    });

    // TODO; validate expression
    // TODO; pop and check control frame

    Ok(())
}
