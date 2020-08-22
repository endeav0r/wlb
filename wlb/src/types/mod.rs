mod buf;
mod primitive;
mod struct_;

use crate::Error;
pub use buf::Buf;
pub use primitive::Primitive;
use serde::{Deserialize, Serialize};
use std::ffi::{c_void, CStr, CString};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
pub use struct_::{Struct, StructField};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Type {
    Primitive(Primitive),
    Struct(Struct),
}

impl Type {
    pub fn size(&self) -> usize {
        match self {
            Type::Primitive(primitive) => primitive.size(),
            Type::Struct(s) => s.size(),
        }
    }

    pub fn fits_within(&self, other: &Type) -> bool {
        match self {
            Type::Primitive(this) => match other {
                Type::Primitive(other) => this.fits_within(other),
                _ => false,
            },
            Type::Struct(this) => match other {
                Type::Struct(other) => this.fits_within(other),
                _ => false,
            },
        }
    }
}

/// VMVal, short for, "Value Memory Value," for historical reasons
///
/// Provides a pinned memory location for values. This is what we pass to the
/// lua interpreter.
#[derive(Clone, Debug)]
pub enum VMVal {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    CString(CString),
    Pointer(*mut c_void),
}

unsafe impl Send for VMVal {}
unsafe impl Sync for VMVal {}

impl VMVal {
    pub fn pointer_to(&self) -> *mut c_void {
        unsafe {
            match self {
                VMVal::U8(ref u) => std::mem::transmute(u),
                VMVal::U16(ref u) => std::mem::transmute(u),
                VMVal::U32(ref u) => std::mem::transmute(u),
                VMVal::U64(ref u) => std::mem::transmute(u),
                VMVal::CString(c_string) => c_string.as_ptr() as *mut c_void,
                VMVal::Pointer(ref p) => std::mem::transmute(p),
            }
        }
    }

    pub fn value_u64(&self) -> Option<u64> {
        Some(match self {
            VMVal::U8(u) => *u as u64,
            VMVal::U16(u) => *u as u64,
            VMVal::U32(u) => *u as u64,
            VMVal::U64(u) => *u as u64,
            VMVal::Pointer(p) => *p as u64,
            VMVal::CString(_) => return None,
        })
    }

    pub fn cstr(&self) -> Option<&CStr> {
        match self {
            VMVal::CString(cstr) => Some(cstr),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Value {
    value: Arc<Pin<Box<VMVal>>>,
    type_: Type,
    child: Option<Box<Value>>,
}

impl Value {
    pub fn new_u8(u: u8) -> Value {
        Value {
            value: Arc::new(Box::pin(VMVal::U8(u))),
            type_: Primitive::U8.into(),
            child: None,
        }
    }

    pub fn new_u16(u: u16) -> Value {
        Value {
            value: Arc::new(Box::pin(VMVal::U16(u))),
            type_: Primitive::U16.into(),
            child: None,
        }
    }

    pub fn new_u32(u: u32) -> Value {
        Value {
            value: Arc::new(Box::pin(VMVal::U32(u))),
            type_: Primitive::U32.into(),
            child: None,
        }
    }

    pub fn new_u64(u: u64) -> Value {
        Value {
            value: Arc::new(Box::pin(VMVal::U64(u))),
            type_: Primitive::U64.into(),
            child: None,
        }
    }

    pub fn new_cstring(s: String) -> Result<Value, Error> {
        let size = s.len() + 1;
        Ok(Value {
            value: Arc::new(Box::pin(VMVal::CString(CString::new(s)?))),
            type_: Primitive::CString(size).into(),
            child: None,
        })
    }

    pub fn new_pointer(value: Value) -> Value {
        let child = Box::new(value);
        Value {
            value: Arc::new(Box::pin(VMVal::Pointer(child.value.pointer_to()))),
            type_: Primitive::Pointer(child.type_().clone().into()).into(),
            child: Some(child),
        }
    }

    pub fn new_raw_pointer(p: *mut c_void) -> Value {
        Value {
            value: Arc::new(Box::pin(VMVal::Pointer(p))),
            type_: Primitive::RawPointer.into(),
            child: None,
        }
    }

    pub fn value(&self) -> &Arc<Pin<Box<VMVal>>> {
        &self.value
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }
}

impl rlua::UserData for Value {
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(rlua::MetaMethod::ToString, |_, value, ()| {
            Ok(match Pin::into_inner(value.value().as_ref().as_ref()) {
                VMVal::U8(u) => format!("0x{:02x}", u),
                VMVal::U16(u) => format!("0x{:04x}", u),
                VMVal::U32(u) => format!("0x{:08x}", u),
                VMVal::U64(u) => format!("0x{:016x}", u),
                VMVal::CString(s) => s.to_string_lossy().to_string(),
                VMVal::Pointer(p) => format!("*0x{:x}", *p as u64),
            })
        });

        methods.add_method("int", |_, value, ()| {
            Ok(Some(
                match Pin::into_inner(value.value().as_ref().as_ref()) {
                    VMVal::U8(u) => *u as i64,
                    VMVal::U16(u) => *u as i64,
                    VMVal::U32(u) => *u as i64,
                    VMVal::U64(u) => *u as i64,
                    VMVal::Pointer(p) => *p as i64,
                    VMVal::CString(_) => return Ok(None),
                },
            ))
        });

        methods.add_method("ptr", |_, value, ()| Ok(Value::new_pointer(value.clone())))
    }
}

pub struct StructBuf {
    struct_: Struct,
    buf: Arc<Mutex<Buf>>,
}

impl StructBuf {
    pub fn new(struct_: Struct) -> Result<StructBuf, Error> {
        let buf = Buf::new(struct_.size())?;
        Ok(StructBuf {
            struct_,
            buf: Arc::new(Mutex::new(buf)),
        })
    }

    pub fn struct_(&self) -> &Struct {
        &self.struct_
    }

    pub fn pointer_to(&self) -> Result<*mut c_void, Error> {
        Ok(self.buf.lock()?.data().as_ptr() as *mut c_void)
    }

    pub fn get_field<S: AsRef<str>>(&self, name: S) -> Result<Option<Value>, Error> {
        use byteorder::{LittleEndian, ReadBytesExt};
        use std::io::Cursor;

        let field = match self.struct_().get_field(name) {
            Some(field) => field,
            None => return Err(Error::StructFieldNotFound),
        };

        let buf_data = self.buf.lock()?;
        let buf_data = buf_data.data();
        let mut cursor = Cursor::new(
            buf_data
                .get(field.offset()..(field.offset() + field.type_().size()))
                .ok_or_else(|| "Internal error getting struct buf data")?,
        );

        Ok(Some(match field.type_() {
            Type::Primitive(Primitive::Empty) => return Ok(None),
            Type::Primitive(Primitive::U8) => Value::new_u8(cursor.read_u8()?),
            Type::Primitive(Primitive::U16) => Value::new_u16(cursor.read_u16::<LittleEndian>()?),
            Type::Primitive(Primitive::U32) => Value::new_u32(cursor.read_u32::<LittleEndian>()?),
            Type::Primitive(Primitive::U64) => Value::new_u64(cursor.read_u64::<LittleEndian>()?),
            Type::Primitive(Primitive::CString(size)) => {
                let mut bytes = Vec::new();
                for _ in 0..*size {
                    let byte = cursor.read_u8()?;
                    if byte == 0 {
                        break;
                    }
                    bytes.push(byte);
                }
                Value::new_cstring(String::from_utf8_lossy(&bytes).to_string())?
            }
            Type::Primitive(Primitive::Pointer(_)) | Type::Primitive(Primitive::RawPointer) => {
                match field.type_().size() {
                    32 => Value::new_raw_pointer(cursor.read_u32::<LittleEndian>()? as *mut c_void),
                    64 => Value::new_raw_pointer(cursor.read_u64::<LittleEndian>()? as *mut c_void),
                    _ => return Err("Internal error with struct buf pointer field".into()),
                }
            }
            Type::Struct(_) => return Err(Error::StructAsValue),
        }))
    }

    pub fn set_field<S: AsRef<str>>(&mut self, name: S, value: &Value) -> Result<(), Error> {
        use byteorder::{LittleEndian, WriteBytesExt};

        let field = match self.struct_().get_field(name) {
            Some(field) => field,
            None => return Err(Error::StructFieldNotFound),
        };

        if value.type_().fits_within(field.type_()) {
            return Err(Error::StructSetInvalidType);
        }

        let field_offset = field.offset();
        let field_size = field.type_().size();

        let mut buf_data = self.buf.lock()?;
        let mut buf_data = buf_data
            .data_mut()
            .get_mut()
            .get_mut(field_offset..(field_offset + field_size))
            .ok_or_else(|| "Internal error getting struct buf data")?;

        match value.type_() {
            Type::Primitive(primitive) => match primitive {
                Primitive::Empty => {}
                Primitive::U8 => buf_data.write_u8(value.value().value_u64().unwrap() as u8)?,
                Primitive::U16 => {
                    buf_data.write_u16::<LittleEndian>(value.value().value_u64().unwrap() as u16)?
                }
                Primitive::U32 => {
                    buf_data.write_u32::<LittleEndian>(value.value().value_u64().unwrap() as u32)?
                }
                Primitive::U64 => {
                    buf_data.write_u64::<LittleEndian>(value.value().value_u64().unwrap())?
                }
                Primitive::CString(_) => {
                    for (i, c) in value
                        .value()
                        .cstr()
                        .unwrap()
                        .to_bytes_with_nul()
                        .into_iter()
                        .enumerate()
                    {
                        buf_data[i] = *c;
                    }
                }
                Primitive::RawPointer | Primitive::Pointer(_) => match value.type_().size() {
                    32 => buf_data
                        .write_u32::<LittleEndian>(value.value().value_u64().unwrap() as u32)?,
                    64 => buf_data.write_u64::<LittleEndian>(value.value().value_u64().unwrap())?,
                    _ => {
                        return Err(Error::Custom(
                            "Internal error writing pointer size to struct buf".into(),
                        ))
                    }
                },
            },
            Type::Struct(_) => return Err(Error::StructAsValue),
        }

        Ok(())
    }
}

impl rlua::UserData for StructBuf {
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_field", |_, struct_buf, name: String| {
            struct_buf
                .get_field(name)
                .map_err(|e| e.into_rlua_external())
        });

        methods.add_method_mut(
            "set_field",
            |_, struct_buf, (name, value): (String, Value)| {
                struct_buf
                    .set_field(name, &value)
                    .map_err(|e| e.into_rlua_external())
            },
        );

        methods.add_method_mut("pointer_to", |_, struct_buf, ()| {
            Ok(Value::new_raw_pointer(
                struct_buf
                    .pointer_to()
                    .map_err(|e| e.into_rlua_external())?,
            ))
        });
    }
}
