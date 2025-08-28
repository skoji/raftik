mod stacks;
use nom::combinator::iterator;

use super::{Context, error::ValidationError};
use crate::{
    ast::{
        instructions::{Opcode, RawExpression},
        types::{FunctionType, NumberType, ValueType},
    },
    binary::parser::instructions::parse_instruction,
};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum StackValue {
    Unknown,
    Value(ValueType),
}

impl StackValue {
    fn i32() -> Self {
        NumberType::I32.into()
    }
}

impl From<NumberType> for StackValue {
    fn from(n: NumberType) -> Self {
        StackValue::Value(ValueType::Number(n))
    }
}

#[derive(Debug, Default)]
struct ControlFrame {
    #[allow(dead_code)]
    pub start_types: Vec<ValueType>,
    pub end_types: Vec<ValueType>,
    pub height_of_value_stack: usize,
    pub unreachable: bool,
}

trait ValueStack {
    fn push_val(&mut self, value: StackValue);
    fn pop_val(&mut self) -> Result<StackValue, ValidationError>;
    fn pop_expect_val(&mut self, expected: StackValue) -> Result<StackValue, ValidationError>;
    #[allow(dead_code)]
    fn push_vals(&mut self, values: &[StackValue]);
    #[allow(dead_code)]
    fn pop_vals(
        &mut self,
        expected_values: &[StackValue],
    ) -> Result<Vec<StackValue>, ValidationError>;
}

trait ControlStack {
    fn push_ctrl(&mut self, frame: ControlFrame);
    fn pop_ctrl(&mut self) -> Result<ControlFrame, ValidationError>;
    #[allow(dead_code)]
    fn unreachable(&mut self);
}

fn validate_opcode(
    opcode: &Opcode,
    stack: &mut (impl ValueStack + ControlStack),
    ctx: &mut Context,
) -> Result<(), ValidationError> {
    match opcode {
        Opcode::LocalGet(index) => {
            let t = ctx
                .locals
                .get(*index as usize)
                .ok_or(ValidationError::NoLocalAtIndex(*index))?;
            stack.push_val(StackValue::Value(*t));
        }
        Opcode::I32Add => {
            stack.pop_expect_val(StackValue::i32())?;
            stack.pop_expect_val(StackValue::i32())?;
            stack.push_val(StackValue::i32());
        }
    }
    Ok(())
}

pub fn validate_raw_expression(
    #[allow(unused_mut)] mut ctx: &mut Context,
    t: &FunctionType,
    expr: &RawExpression,
    _position_string_on_error: String,
) -> Result<(), ValidationError> {
    let mut stack = stacks::generate_stack();

    // push outermost control frame (regarding as a block)
    stack.push_ctrl(ControlFrame {
        start_types: vec![],
        end_types: t.results.to_vec(),
        ..Default::default()
    });

    let mut it = iterator(expr.instructions, parse_instruction);
    for opcode in &mut it {
        validate_opcode(&opcode, &mut stack, ctx)?;
    }

    it.finish()
        .map_err(|e| ValidationError::OpcodeParseFailed(e.to_string()))?;
    stack.pop_ctrl()?;

    Ok(())
}
