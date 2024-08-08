use core::result;

use ckb_linked_list_tool::error::Error as ToolError;
use ckb_std::error::SysError;

pub type Result<T> = result::Result<T, Error>;

#[repr(i8)]
pub enum InternalError {
    // 0x01 ~ 0x0f: Errors from SDK, or other system errors.
    IndexOutOfBound = 0x01,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    Unknown,

    // 0x10 ~ 0x1f: Errors before doing operations.
    UnknownEnvironment = 0x10,
    UnknownOperation,

    // 0x20 ~ 0x2f: Errors when create.
    CreateInvalidArgsLength = 0x20,
    CreateIncorrectUniqueId,
    CreateInvalidOutputData,
    CreateIncompleteList,

    // 0x30 ~ 0x3f: Errors when destory.
    DestroyInvalidInputData = 0x30,
    DestroyIncompleteList,

    // 0x40 ~ 0x4f: Errors when update.
    UpdateMismatchRange = 0x40,

    // This is not an error, just make sure the error code is less than 0x60.
    Unreachable = 0x60,
}

pub enum Error {
    // 0x01 ~ 0x5f: Errors that not from external crates.
    Internal(InternalError),
    // 0x60 ~ 0x7f: Errors from demo lib.
    Tool(ToolError),
}

impl From<SysError> for InternalError {
    fn from(err: SysError) -> Self {
        match err {
            SysError::IndexOutOfBound => Self::IndexOutOfBound,
            SysError::ItemMissing => Self::ItemMissing,
            SysError::LengthNotEnough(_) => Self::LengthNotEnough,
            SysError::Encoding => Self::Encoding,
            SysError::Unknown(_) => Self::Unknown,
        }
    }
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        Into::<InternalError>::into(err).into()
    }
}

impl From<InternalError> for Error {
    fn from(err: InternalError) -> Self {
        Self::Internal(err)
    }
}

impl From<ToolError> for Error {
    fn from(err: ToolError) -> Self {
        Self::Tool(err)
    }
}

impl From<Error> for i8 {
    fn from(err: Error) -> Self {
        match err {
            Error::Internal(e) => e as i8,
            Error::Tool(e) => 0x60 + e as i8,
        }
    }
}
