use super::{ControlFrame, ControlStack, StackValue, ValueStack};
use crate::validation::error::ValidationError;

struct TheStack<'a> {
    values: Vec<StackValue>,
    controls: Vec<ControlFrame<'a>>,
}
impl<'a> TheStack<'a> {
    fn new() -> TheStack<'a> {
        let values: Vec<StackValue> = Vec::new();
        let controls: Vec<ControlFrame<'a>> = Vec::new();
        TheStack { values, controls }
    }
}

impl<'a> ValueStack<'a> for TheStack<'a> {
    fn push_val(&mut self, value: StackValue) {
        self.values.push(value);
    }

    fn pop_val(&mut self) -> Result<StackValue, ValidationError> {
        if let Some(val) = self.values.pop() {
            return Ok(val);
        }
        !unimplemented!()
    }

    fn pop_expect_val(&mut self, expected: StackValue) -> Result<StackValue, ValidationError> {
        self.pop_val()
    }

    fn push_vals(&mut self, values: &[StackValue]) {
        !unimplemented!()
    }

    fn pop_vals(
        &mut self,
        expected_values: &[StackValue],
    ) -> Result<&'a [StackValue], ValidationError> {
        !unimplemented!()
    }
}

impl<'a> ControlStack<'a> for TheStack<'a> {
    fn push_ctrl(&mut self, frame: ControlFrame<'a>) {
        self.controls.push(frame);
    }

    fn pop_ctrl(&mut self) -> Result<ControlFrame<'a>, ValidationError> {
        if let Some(frame) = self.controls.pop() {
            return Ok(frame);
        }
        !unimplemented!()
    }

    fn unreachable(&mut self) {
        if let Some(frame) = self.controls.last_mut() {
            frame.unreachable = true;
        }
    }
}

pub fn generate_stack<'a>() -> impl ValueStack<'a> + ControlStack<'a> {
    TheStack::new()
}
