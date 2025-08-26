use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("{refering} index {refering_index} out of bounds at {refered_index} for {refered}")]
    IndexOutOfBoundsIn {
        refering: &'static str,
        refering_index: usize,
        refered: &'static str,
        refered_index: u32,
    },

    // should never happen
    #[error("{0} Section: No {1} Section found")]
    ReferingSectionNotFound(&'static str, &'static str),
    #[error("{0} Section: {1} index {2} out of bounds at {3} for {1} Section")]
    IndexOutOfBoundsInSection(&'static str, &'static str, u32, usize),
}
