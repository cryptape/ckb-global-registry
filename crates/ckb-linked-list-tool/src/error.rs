//! Errors.

#[repr(i8)]
pub enum Error {
    EmptyList = 0x00,
    Discontinuous,
    ReachLastTwice,
    NextIsSelfItem,
    // This is not an error, just make sure the error code is less than 32.
    Unreachable = 0x20,
}
