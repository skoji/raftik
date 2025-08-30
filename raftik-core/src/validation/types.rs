use crate::ast::types::Limits;

pub fn validate_limits(limits: &Limits, maximum: u32) -> bool {
    match limits.max {
        Some(max) => limits.min <= max && max <= maximum,
        None => limits.min <= maximum,
    }
}
