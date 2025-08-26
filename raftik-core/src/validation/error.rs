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

    // should never happen
    #[error("{0} Section: No {1} Section found")]
    ReferingSectionNotFound(&'static str, &'static str),
    #[error("{0} Section: {1} index {2} out of bounds at {3} for {1} Section")]
    IndexOutOfBoundsInSection(&'static str, &'static str, u32, usize),
}
