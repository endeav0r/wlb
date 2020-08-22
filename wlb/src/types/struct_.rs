use super::{StructBuf, Type};
use crate::Error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StructField {
    name: String,
    offset: usize,
    type_: Type,
}

impl StructField {
    pub fn new<S: Into<String>>(name: S, offset: usize, type_: Type) -> StructField {
        StructField {
            name: name.into(),
            offset,
            type_,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn offset(&self) -> usize {
        self.offset
    }
    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn in_field(&self, offset: usize) -> bool {
        offset >= self.offset() && offset < self.offset() + self.type_().size()
    }
    pub fn overlaps(&self, other: &StructField) -> bool {
        let other_offset = other.offset();
        for i in 0..other.type_().size() {
            if self.in_field(other_offset + i) {
                return true;
            }
        }
        false
    }
}

impl rlua::UserData for StructField {
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(
            rlua::MetaMethod::Index,
            |lua_ctx, struct_field, name: String| {
                use rlua::prelude::*;

                Ok(match name.as_str() {
                    "name" => Some(struct_field.name().to_string().to_lua(lua_ctx)?),
                    "offset" => Some(struct_field.offset().to_lua(lua_ctx)?),
                    "type" => {
                        Some(crate::lua::Type::new(struct_field.type_().clone()).to_lua(lua_ctx)?)
                    }
                    _ => None,
                })
            },
        );

        // methods.add_meta_method(
        //     rlua::MetaMethod::Call,
        //     |_, _, (name, offset, type_): (String, usize, crate::lua::Type)| {
        //         Ok(StructField::new(name, offset, type_.type_().clone()))
        //     },
        // );
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Struct {
    fields: Vec<StructField>,
}

impl Struct {
    pub fn new() -> Struct {
        Struct { fields: Vec::new() }
    }

    pub fn fields(&self) -> &[StructField] {
        &self.fields
    }

    pub fn fits_within(&self, other: &Struct) -> bool {
        self.size() <= other.size()
    }

    pub fn size(&self) -> usize {
        self.fields()
            .iter()
            .map(|field| field.type_().size() + field.offset())
            .max()
            .unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.fields().is_empty()
    }

    pub fn push(&mut self, field: StructField) -> Result<(), Error> {
        for existing_field in self.fields() {
            if existing_field.name() == field.name() {
                return Err(Error::StructDuplicateName(field.name().to_string()));
            } else if field.overlaps(existing_field) {
                return Err(Error::OverlappingFields);
            }
        }

        self.fields.push(field);
        Ok(())
    }

    pub fn get_field<S: AsRef<str>>(&self, name: S) -> Option<&StructField> {
        self.fields()
            .iter()
            .find(|field| field.name() == name.as_ref())
    }
}

impl rlua::UserData for Struct {
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("push", |_, struct_, field: StructField| {
            struct_.push(field).map_err(|e| e.into_rlua_external())
        });

        // methods.add_method_mut("push", |_, struct_, field: usize| {
        //     println!("field: {}", field);
        //     Ok(())
        // });

        // methods.add_meta_method(rlua::MetaMethod::Call, |_, _, ()| Ok(Struct::new()));

        methods.add_method("buf", |_, struct_, ()| {
            StructBuf::new(struct_.clone()).map_err(|e| e.into_rlua_external())
        });
    }
}

impl std::default::Default for Struct {
    fn default() -> Struct {
        Struct::new()
    }
}

impl Into<Type> for Struct {
    fn into(self) -> Type {
        Type::Struct(self)
    }
}
