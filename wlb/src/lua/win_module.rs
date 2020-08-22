use super::WinFunction;
use crate::{win, Error};

#[derive(Clone, Debug)]
pub struct WinModule {
    module: win::Module,
}

impl WinModule {
    pub fn new(module: win::Module) -> WinModule {
        WinModule { module }
    }

    pub fn module(&self) -> win::Module {
        self.module
    }

    pub fn get_function<S: AsRef<str>>(&self, function_name: S) -> Result<WinFunction, Error> {
        Ok(WinFunction::new(
            self.module()
                .GetProcAddress(function_name.as_ref())?
                .ok_or_else(|| format!("Unable to find function: {}", function_name.as_ref()))?,
        ))
    }
}

impl rlua::UserData for WinModule {
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(
            rlua::MetaMethod::Index,
            |_, win_module, function_name: String| Ok(win_module.get_function(function_name).ok()),
        );

        methods.add_method("f", |_, win_module, function_name: String| {
            win_module
                .get_function(function_name)
                .map_err(|e| e.into_rlua_external())
        });

        methods.add_method("GetModuleBaseName", |_, win_module, ()| {
            win_module
                .module()
                .GetModuleBaseNameA()
                .map_err(|e| e.into_rlua_external())
        });
    }
}
