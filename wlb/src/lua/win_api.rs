use super::WinModule;
use crate::{win, Error};

pub struct WinApi {}

impl WinApi {
    pub fn new() -> WinApi {
        WinApi {}
    }

    pub fn get_module<S: AsRef<str>>(module_name: S) -> Result<WinModule, Error> {
        Ok(WinModule::new(
            win::Process::GetCurrentProcess()
                .get_module(module_name.as_ref())?
                .ok_or_else(|| format!("Unable to find module {}", module_name.as_ref()))?,
        ))
    }
}

impl rlua::UserData for WinApi {
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("m", |_, module_name: String| {
            WinApi::get_module(module_name).map_err(|e| e.into_rlua_external())
        });

        methods.add_function("find_function_modules", |_, function_name: String| {
            win::Process::GetCurrentProcess()
                .find_function_module(function_name)
                .map_err(|e| e.into_rlua_external())
        });

        methods.add_function("peek8", |_, address: u64| {
            Ok(unsafe { *(address as *mut u8) })
        });

        methods.add_function("peek16", |_, address: u64| {
            Ok(unsafe { *(address as *mut u16) })
        });

        methods.add_function("peek32", |_, address: u64| {
            Ok(unsafe { *(address as *mut u32) })
        });

        methods.add_function("peek64", |_, address: u64| {
            Ok(unsafe { *(address as *mut u64) })
        });

        methods.add_function("poke8", |_, (address, value): (u64, u8)| {
            unsafe {
                *(address as *mut u8) = value;
            };
            Ok(())
        });

        methods.add_function("poke16", |_, (address, value): (u64, u16)| {
            unsafe {
                *(address as *mut u16) = value;
            };
            Ok(())
        });

        methods.add_function("poke32", |_, (address, value): (u64, u32)| {
            unsafe {
                *(address as *mut u32) = value;
            };
            Ok(())
        });

        methods.add_function("poke64", |_, (address, value): (u64, u64)| {
            unsafe {
                *(address as *mut u64) = value;
            };
            Ok(())
        });

        methods.add_meta_method(
            rlua::MetaMethod::Index,
            |lua_ctx, _, module_name: String| {
                use rlua::ToLua;
                if module_name == "types" {
                    Ok(super::Types {}.to_lua(lua_ctx))
                } else {
                    WinApi::get_module(module_name)
                        .map_err(|e| e.into_rlua_external())
                        .map(|module| module.to_lua(lua_ctx))
                }
            },
        );
    }
}
