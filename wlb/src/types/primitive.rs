use super::Type;
use serde::{Deserialize, Serialize};
use std::ffi::c_void;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Primitive {
    Empty,
    U8,
    U16,
    U32,
    U64,
    CString(usize),
    Pointer(Box<Type>),
    RawPointer,
}

impl Primitive {
    pub fn size(&self) -> usize {
        match self {
            Primitive::Empty => 0,
            Primitive::U8 => 1,
            Primitive::U16 => 2,
            Primitive::U32 => 4,
            Primitive::U64 => 8,
            Primitive::CString(size) => *size,
            Primitive::Pointer(_) => std::mem::size_of::<*const c_void>(),
            Primitive::RawPointer => std::mem::size_of::<*const c_void>(),
        }
    }

    /// True if this value fits within the other value.
    /// Examples are u8 fits within u16. u16 fits within u16.=
    /// A CString of length 12 fits within a CString of length 20.
    pub fn fits_within(&self, other: &Primitive) -> bool {
        match self {
            Primitive::Empty => match other {
                Primitive::Empty => true,
                _ => false,
            },
            Primitive::U8 => match other {
                Primitive::U8 | Primitive::U16 | Primitive::U32 | Primitive::U64 => true,
                _ => false,
            },
            Primitive::U16 => match other {
                Primitive::U16 | Primitive::U32 | Primitive::U64 => true,
                _ => false,
            },
            Primitive::U32 => match other {
                Primitive::U32 | Primitive::U64 => true,
                _ => false,
            },
            Primitive::U64 => match other {
                Primitive::U64 => true,
                _ => false,
            },
            Primitive::CString(len) => match other {
                Primitive::CString(other_len) => len <= other_len,
                _ => false,
            },
            Primitive::Pointer(_) => match other {
                Primitive::Pointer(_) | Primitive::U64 => true,
                Primitive::U32 => self.size() == 32,
                _ => false,
            },
            Primitive::RawPointer => match other {
                Primitive::RawPointer => true,
                _ => false,
            },
        }
    }
}

impl Into<Type> for Primitive {
    fn into(self) -> Type {
        Type::Primitive(self)
    }
}

impl Into<Box<Type>> for Primitive {
    fn into(self) -> Box<Type> {
        Box::new(Type::Primitive(self))
    }
}

impl rlua::UserData for Primitive {}
