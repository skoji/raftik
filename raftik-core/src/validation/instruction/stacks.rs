use super::{ControlFrame, ControlStack, StackValue, ValueStack};
use crate::validation::error::VInstError;

macro_rules! controls_last {
    ($controls: expr) => {{ $controls.last().ok_or(VInstError::ControlStackUnderflow) }};
}

#[allow(dead_code)]
#[derive(Debug)]
struct TheStack {
    values: Vec<StackValue>,
    controls: Vec<ControlFrame>,
}

impl TheStack {
    fn new() -> TheStack {
        let values = Vec::new();
        let controls = Vec::new();
        TheStack { values, controls }
    }
}

impl ValueStack for TheStack {
    fn push_val(&mut self, value: StackValue) {
        self.values.push(value);
    }

    fn pop_val(&mut self) -> Result<StackValue, VInstError> {
        let controls_top = controls_last!(self.controls)?;
        if self.values.len() == controls_top.height_of_value_stack && controls_top.unreachable {
            return Ok(StackValue::Unknown);
        }
        if self.values.len() == controls_top.height_of_value_stack {
            return Err(VInstError::ValueStackUnderflow);
        }
        self.values.pop().ok_or(VInstError::ValueStackUnderflow)
    }

    fn pop_expect_val(&mut self, expected: StackValue) -> Result<StackValue, VInstError> {
        let actual = self.pop_val()?;
        match (&actual, &expected) {
            (StackValue::Unknown, _) | (_, StackValue::Unknown) => Ok(actual),
            (StackValue::Value(a), StackValue::Value(b)) if a == b => Ok(actual),
            (StackValue::Value(a), StackValue::Value(b)) => Err(VInstError::PopValueTypeMismatch {
                expected: *b,
                actual: *a,
            }),
        }
    }

    fn push_vals(&mut self, values: &[StackValue]) {
        for v in values {
            self.push_val(*v);
        }
    }

    fn pop_vals(&mut self, expected_values: &[StackValue]) -> Result<Vec<StackValue>, VInstError> {
        expected_values
            .iter()
            .rev()
            .map(|v| self.pop_expect_val(*v))
            .collect()
    }

    fn get_clone_of_value_stack(&self) -> Vec<StackValue> {
        self.values.clone()
    }
}

impl ControlStack for TheStack {
    fn push_ctrl(&mut self, frame: ControlFrame) {
        self.controls.push(frame);
    }

    fn pop_ctrl(&mut self) -> Result<ControlFrame, VInstError> {
        let frame = controls_last!(self.controls)?;
        self.pop_vals(
            frame
                .end_types
                .iter()
                .map(|v| StackValue::Value(*v))
                .collect::<Vec<_>>()
                .as_ref(),
        )?;
        let frame = self
            .controls
            .pop()
            .expect("control does exist here, for controls_last! has checked the existence");
        if self.values.len() != frame.height_of_value_stack {
            Err(VInstError::ValueStackUnderflow)
        } else {
            Ok(frame)
        }
    }

    fn unreachable(&mut self) {
        if let Some(frame) = self.controls.last_mut() {
            frame.unreachable = true;
        }
    }

    fn get_clone_of_control_stack(&self) -> Vec<ControlFrame> {
        self.controls.clone()
    }
}

pub fn generate_stack() -> impl ValueStack + ControlStack {
    TheStack::new()
}
