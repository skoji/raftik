use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("{referring} index {referring_index} out of bounds at {referred_index} for {referred}")]
    IndexOutOfBoundsIn {
        referring: &'static str,
        referring_index: usize,
        referred: &'static str,
        referred_index: u32,
    },
}
