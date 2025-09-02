mod stacks;
use nom::combinator::iterator;

use super::{
    Context,
    error::{VInstError, ValidationError},
};
use crate::{
    ast::{
        instructions::{Opcode, RawExpression},
        types::{FunctionType, NumberType, ReferenceType, ValueType},
    },
    binary::parser::instructions::parse_instruction,
};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum StackValue {
    Unknown,
    Value(ValueType),
}

impl StackValue {
    fn i32() -> Self {
        NumberType::I32.into()
    }
    fn i64() -> Self {
        NumberType::I64.into()
    }
    fn f32() -> Self {
        NumberType::F32.into()
    }
    fn f64() -> Self {
        NumberType::F64.into()
    }
}

impl From<NumberType> for StackValue {
    fn from(n: NumberType) -> Self {
        StackValue::Value(ValueType::Number(n))
    }
}

impl From<ValueType> for StackValue {
    fn from(t: ValueType) -> Self {
        StackValue::Value(t)
    }
}

#[derive(Debug, Default, Clone)]
pub struct ControlFrame {
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
    fn pop_vals(&mut self, expected_values: &[StackValue]) -> Result<Vec<StackValue>, VInstError>;

    fn get_clone_of_value_stack(&self) -> Vec<StackValue>;
}

trait ControlStack {
    fn push_ctrl(&mut self, frame: ControlFrame);
    fn pop_ctrl(&mut self) -> Result<ControlFrame, VInstError>;
    #[allow(dead_code)]
    fn unreachable(&mut self);
    fn get_clone_of_control_stack(&self) -> Vec<ControlFrame>;
}

fn get_local(i: u32, ctx: &Context) -> Result<ValueType, VInstError> {
    ctx.locals
        .get(i as usize)
        .ok_or(VInstError::NoLocalAtIndex(i))
        .cloned()
}

fn get_global(i: u32, ctx: &Context) -> Result<ValueType, VInstError> {
    let g = ctx
        .globals
        .get(i as usize)
        .ok_or(VInstError::NoGlobalAtIndex(i))?;
    Ok(g.t().val_type)
}

fn get_func<'a>(i: u32, ctx: &'a Context) -> Result<&'a FunctionType, VInstError> {
    let f = ctx
        .functions
        .get(i as usize)
        .ok_or(VInstError::NoFunctionAtIndex(i))?;
    ctx.types
        .get(*f.t() as usize)
        .cloned()
        .ok_or(VInstError::NoFunctionAtIndex(i))
}

fn validate_opcode_variable(
    opcode: &Opcode,
    stack: &mut (impl ValueStack + ControlStack),
    ctx: &mut Context,
) -> Result<(), VInstError> {
    match opcode {
        Opcode::LocalGet(i) => {
            stack.push_val(StackValue::Value(get_local(*i, ctx)?));
        }
        Opcode::LocalSet(i) => {
            stack.pop_expect_val(StackValue::Value(get_local(*i, ctx)?))?;
        }
        Opcode::LocalTee(i) => {
            let t = get_local(*i, ctx)?;
            stack.pop_expect_val(StackValue::Value(t))?;
            stack.push_val(StackValue::Value(t));
        }
        Opcode::GlobalGet(i) => {
            stack.push_val(StackValue::Value(get_global(*i, ctx)?));
        }
        Opcode::GlobalSet(i) => {
            stack.pop_expect_val(StackValue::Value(get_global(*i, ctx)?))?;
        }
        _ => unreachable!("opcode in variable caregoty not processed {:?}", opcode),
    }
    Ok(())
}

fn validate_opcode_numeric(
    opcode: &Opcode,
    stack: &mut (impl ValueStack + ControlStack),
    _ctx: &mut Context,
) -> Result<(), VInstError> {
    match opcode {
        Opcode::I32Add => {
            stack.pop_expect_val(StackValue::i32())?;
            stack.pop_expect_val(StackValue::i32())?;
            stack.push_val(StackValue::i32());
        }
        _ => unreachable!("opcode in numeric category not processed {:?}", opcode),
    }
    Ok(())
}

fn validate_opcode_numeric_const(
    opcode: &Opcode,
    stack: &mut (impl ValueStack + ControlStack),
    _ctx: &Context,
) -> Result<(), VInstError> {
    match opcode {
        Opcode::I32Const(_) => stack.push_val(StackValue::i32()),
        Opcode::I64Const(_) => stack.push_val(StackValue::i64()),
        Opcode::F32Const(_) => stack.push_val(StackValue::f32()),
        Opcode::F64Const(_) => stack.push_val(StackValue::f64()),
        _ => unreachable!(
            "opcode in numeric const category not processed {:?}",
            opcode
        ),
    }
    Ok(())
}

fn validate_opcode_reference(
    opcode: &Opcode,
    stack: &mut (impl ValueStack + ControlStack),
    ctx: &Context,
) -> Result<(), VInstError> {
    match opcode {
        Opcode::RefNull(t) => stack.push_val(ValueType::Reference(*t).into()),
        Opcode::RefIsNull => {
            let v = stack.pop_val()?;
            let StackValue::Value(ValueType::Reference(_)) = v else {
                return Err(VInstError::StackValueShouldBeRefType(v));
            };
            stack.push_val(StackValue::i32());
        }
        Opcode::RefFunc(i) => {
            get_func(*i, ctx)?;
            // TODO; should check i is included in refs
            stack.push_val(ValueType::Reference(ReferenceType::FuncRef).into());
        }
        _ => unreachable!("opcode in reference category not processed {:?}", opcode),
    }
    Ok(())
}

fn validate_opcode(
    opcode: &Opcode,
    stack: &mut (impl ValueStack + ControlStack),
    ctx: &mut Context,
) -> Result<(), VInstError> {
    match opcode.category() {
        crate::ast::instructions::OpcodeCategory::Variable => {
            validate_opcode_variable(opcode, stack, ctx)?
        }
        crate::ast::instructions::OpcodeCategory::Reference => {
            validate_opcode_reference(opcode, stack, ctx)?
        }
        crate::ast::instructions::OpcodeCategory::NumericConst => {
            validate_opcode_numeric_const(opcode, stack, ctx)?
        }
        crate::ast::instructions::OpcodeCategory::Numeric => {
            validate_opcode_numeric(opcode, stack, ctx)?
        }
    }

    if ctx.instructions_should_be_constant {
        if !opcode.is_constant() {
            return Err(VInstError::OpcodeShouldBeConstant(*opcode));
        }
        if let Opcode::GlobalGet(i) = opcode {
            let g = *ctx.globals[*i as usize].t();
            if !matches!(g.mutability, crate::ast::types::Mutability::Const) {
                return Err(VInstError::GlobalGetShouldBeConstant(*i));
            }
        }
    }

    Ok(())
}

fn validate_instructions(
    instructions: &[u8],
    stack: &mut (impl ValueStack + ControlStack),
    ctx: &mut Context,
    progress: &mut Vec<Opcode>,
) -> Result<(), VInstError> {
    let mut it = iterator(instructions, parse_instruction);
    for opcode in &mut it {
        progress.push(opcode);
        validate_opcode(&opcode, stack, ctx)?;
    }
    it.finish()
        .map_err(|e| VInstError::OpcodeParseFailed(e.to_string()))?;
    stack.pop_ctrl()?;
    Ok(())
}

pub fn validate_raw_expression(
    ctx: &mut Context,
    t: &FunctionType,
    expr: &RawExpression,
    desc_on_error: String,
) -> Result<(), ValidationError> {
    let mut stack = stacks::generate_stack();
    let mut progress = Vec::new();

    // push outermost control frame (regarding as a block)
    stack.push_ctrl(ControlFrame {
        start_types: vec![],
        end_types: t.results.to_vec(),
        ..Default::default()
    });

    validate_instructions(expr.instructions, &mut stack, ctx, &mut progress).map_err(|e| {
        ValidationError::InstructionValidationError {
            desc: desc_on_error,
            error: e,
            progress,
            value_stack: stack.get_clone_of_value_stack(),
            control_stack: stack.get_clone_of_control_stack(),
        }
    })
}
