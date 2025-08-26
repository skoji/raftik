use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error(
        "index {referred_index} out of bounds in {referred} section (referenced from {referring} section at index {referring_index})"
    )]
    IndexOutOfBoundsIn {
        referring: &'static str,
        referring_index: usize,
        referred: &'static str,
        referred_index: u32,
    },
}
