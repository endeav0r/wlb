use std::fmt;
use std::sync::Arc;

#[derive(Debug)]
pub enum Error {
    BufTooLarge(usize),
    Custom(String),
    GetLastError(u32),
    Io(std::io::Error),
    NulError(String),
    OverlappingFields,
    Poison(String),
    InsufficientAccess,
    Rlua(rlua::Error),
    StructAsValue,
    StructDuplicateName(String),
    StructFieldNotFound,
    StructSetInvalidType,
}

impl Error {
    pub fn into_rlua_external(self) -> rlua::Error {
        rlua::Error::ExternalError(Arc::new(self))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BufTooLarge(len) => write!(f, "Buf too large ({} requested)", len),
            Error::Custom(s) => write!(f, "{}", s),
            Error::GetLastError(e) => write!(f, "GetLastError: 0x{:08x}", e),
            Error::Io(e) => write!(f, "Io: {}", e),
            Error::NulError(s) => write!(f, "NulError: {}", s),
            Error::OverlappingFields => write!(f, "Field overlaps another field"),
            Error::Poison(s) => write!(f, "Poison error: {}", s),
            Error::InsufficientAccess => write!(f, "Insufficient access to perform command"),
            Error::Rlua(e) => write!(f, "Rlua: {}", e),
            Error::StructAsValue => write!(f, "Tried to get struct as value"),
            Error::StructDuplicateName(s) => write!(f, "Struct already has field with name {}", s),
            Error::StructFieldNotFound => write!(f, "Struct field not found"),
            Error::StructSetInvalidType => {
                write!(f, "That struct field cannot be set to that type")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(s: &str) -> Error {
        Error::Custom(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error::Custom(s)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}

impl Into<rlua::Error> for Error {
    fn into(self) -> rlua::Error {
        rlua::Error::ExternalError(Arc::new(self))
    }
}

impl From<rlua::Error> for Error {
    fn from(e: rlua::Error) -> Error {
        Error::Rlua(e)
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(e: std::ffi::NulError) -> Error {
        Error::NulError(e.to_string())
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(e: std::sync::PoisonError<T>) -> Error {
        Error::Poison(e.to_string())
    }
}
