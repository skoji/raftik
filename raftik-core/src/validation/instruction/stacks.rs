use super::{ControlFrame, ControlStack, StackValue, ValueStack};
use crate::validation::error::ValidationError;

macro_rules! controls_last {
    ($controls: expr) => {{
        $controls
            .last()
            .ok_or(ValidationError::ControlStackUnderflow)
    }};
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

    fn pop_val(&mut self) -> Result<StackValue, ValidationError> {
        let controls_top = controls_last!(self.controls)?;
        if self.values.len() == controls_top.height_of_value_stack && controls_top.unreachable {
            return Ok(StackValue::Unknown);
        }
        if self.values.len() == controls_top.height_of_value_stack {
            return Err(ValidationError::ValueStackUnderflow);
        }
        self.values
            .pop()
            .ok_or(ValidationError::ValueStackUnderflow)
    }

    fn pop_expect_val(&mut self, expected: StackValue) -> Result<StackValue, ValidationError> {
        let actual = self.pop_val()?;
        match (&actual, &expected) {
            (StackValue::Unknown, _) | (_, StackValue::Unknown) => Ok(actual),
            (StackValue::Value(a), StackValue::Value(b)) if a == b => Ok(actual),
            (StackValue::Value(a), StackValue::Value(b)) => {
                Err(ValidationError::PopValueTypeMismatch {
                    expected: *b,
                    actual: *a,
                })
            }
        }
    }

    fn push_vals(&mut self, values: &[StackValue]) {
        for v in values {
            self.push_val(*v);
        }
    }

    fn pop_vals(
        &mut self,
        expected_values: &[StackValue],
    ) -> Result<Vec<StackValue>, ValidationError> {
        expected_values
            .iter()
            .rev()
            .map(|v| self.pop_expect_val(*v))
            .collect()
    }
}

impl ControlStack for TheStack {
    fn push_ctrl(&mut self, frame: ControlFrame) {
        self.controls.push(frame);
    }

    fn pop_ctrl(&mut self) -> Result<ControlFrame, ValidationError> {
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
            Err(ValidationError::ValueStackUnderflow)
        } else {
            Ok(frame)
        }
    }

    fn unreachable(&mut self) {
        if let Some(frame) = self.controls.last_mut() {
            frame.unreachable = true;
        }
    }
}

pub fn generate_stack() -> impl ValueStack + ControlStack {
    TheStack::new()
}
