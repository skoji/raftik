mod stacks;
use nom::combinator::iterator;

use super::{
    Context,
    error::{VInstError, ValidationError},
};
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
    fn pop_val(&mut self) -> Result<StackValue, VInstError>;
    fn pop_expect_val(&mut self, expected: StackValue) -> Result<StackValue, VInstError>;
    #[allow(dead_code)]
    fn push_vals(&mut self, values: &[StackValue]);
    #[allow(dead_code)]
    fn pop_vals(&mut self, expected_values: &[StackValue]) -> Result<Vec<StackValue>, VInstError>;
}

trait ControlStack {
    fn push_ctrl(&mut self, frame: ControlFrame);
    fn pop_ctrl(&mut self) -> Result<ControlFrame, VInstError>;
    #[allow(dead_code)]
    fn unreachable(&mut self);
}

fn validate_opcode(
    opcode: &Opcode,
    stack: &mut (impl ValueStack + ControlStack),
    ctx: &mut Context,
) -> Result<(), VInstError> {
    match opcode {
        Opcode::LocalGet(index) => {
            let t = ctx
                .locals
                .get(*index as usize)
                .ok_or(VInstError::NoLocalAtIndex(*index))?;
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

fn validate_instructions(
    instructions: &[u8],
    stack: &mut (impl ValueStack + ControlStack),
    ctx: &mut Context,
) -> Result<(), VInstError> {
    let mut it = iterator(instructions, parse_instruction);
    for opcode in &mut it {
        validate_opcode(&opcode, stack, ctx)?;
    }

    it.finish()
        .map_err(|e| VInstError::OpcodeParseFailed(e.to_string()))?;
    stack.pop_ctrl()?;
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

    // TODO; add parsing context to instruction validation error;
    validate_instructions(expr.instructions, &mut stack, ctx)
        .map_err(ValidationError::InstructionValidationError)
}
