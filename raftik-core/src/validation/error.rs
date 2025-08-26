use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("{0} Section: No {1} Section found")]
    ReferingSectionNotFound(&'static str, &'static str),
    #[error("{0} Section: {1} index {2} out of bounds at {3} for {1} Section")]
    IndexOutOfBoundsInSection(&'static str, &'static str, u32, usize),
}
