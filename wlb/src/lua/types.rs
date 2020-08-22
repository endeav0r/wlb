use crate::{types, Error};
use rlua::prelude::*;

#[derive(Clone)]
pub struct Type {
    type_: types::Type,
}

impl Type {
    pub fn new(type_: types::Type) -> Type {
        Type { type_ }
    }

    pub fn type_(&self) -> &types::Type {
        &self.type_
    }
}

impl From<types::Type> for Type {
    fn from(t: types::Type) -> Type {
        Type::new(t)
    }
}

impl rlua::UserData for Type {
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(
            rlua::MetaMethod::Call,
            |lua_ctx, type_, value: rlua::Value| {
                Ok(Some(match type_.type_ {
                    types::Type::Primitive(types::Primitive::Empty) => {
                        return Ok(None);
                    }
                    types::Type::Primitive(types::Primitive::U8) => {
                        types::Value::new_u8(u8::from_lua(value, lua_ctx)?)
                    }
                    types::Type::Primitive(types::Primitive::U16) => {
                        types::Value::new_u16(u16::from_lua(value, lua_ctx)?)
                    }
                    types::Type::Primitive(types::Primitive::U32) => {
                        types::Value::new_u32(u32::from_lua(value, lua_ctx)?)
                    }
                    types::Type::Primitive(types::Primitive::U64) => {
                        types::Value::new_u64(i64::from_lua(value, lua_ctx)? as u64)
                    }
                    types::Type::Primitive(types::Primitive::CString(_)) => {
                        types::Value::new_cstring(String::from_lua(value, lua_ctx)?)
                            .map_err(|e| e.into_rlua_external())?
                    }
                    types::Type::Primitive(types::Primitive::Pointer(_)) => {
                        types::Value::new_pointer(types::Value::from_lua(value, lua_ctx)?)
                    }
                    types::Type::Primitive(types::Primitive::RawPointer) => return Ok(None),
                    types::Type::Struct(_) => {
                        return Err(
                            Error::Custom("Struct not implemented".into()).into_rlua_external()
                        )
                    }
                }))
            },
        );
    }
}

pub struct Types {}

impl rlua::UserData for Types {
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(rlua::MetaMethod::Index, |lua_ctx, _, name: String| {
            use rlua::prelude::*;
            Ok(Some(match name.as_str() {
                "u8" => Type::new(types::Primitive::U8.into()).to_lua(lua_ctx)?,
                "u16" => Type::new(types::Primitive::U16.into()).to_lua(lua_ctx)?,
                "u32" => Type::new(types::Primitive::U32.into()).to_lua(lua_ctx)?,
                "u64" => Type::new(types::Primitive::U64.into()).to_lua(lua_ctx)?,
                "cstring" => Type::new(types::Primitive::CString(0).into()).to_lua(lua_ctx)?,
                "pointer" => Type::new(
                    types::Primitive::Pointer(Box::new(types::Primitive::Empty.into())).into(),
                )
                .to_lua(lua_ctx)?,
                "struct" => lua_ctx
                    .create_function(|_, ()| Ok(types::Struct::new()))?
                    .to_lua(lua_ctx)?,
                "struct_field" => lua_ctx
                    .create_function(|_, (name, offset, type_): (String, usize, Type)| {
                        Ok(types::StructField::new(name, offset, type_.type_().clone()))
                    })?
                    .to_lua(lua_ctx)?,
                _ => return Ok(None),
            }))
        })
    }
}
