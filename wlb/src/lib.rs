//! Windows Lua Bridge. Provides the ability to wrap the Windows API with
//! Lua scripts, similar perhaps to meterpreter railgun.

pub mod error;
pub mod lua;
pub mod types;
pub mod win;

pub use error::Error;

pub struct Context {
    lua: rlua::Lua,
}

impl Context {
    pub fn new() -> Result<Context, Error> {
        let lua = rlua::Lua::new();

        lua.context::<fn(rlua::Context) -> Result<(), Error>, Result<(), Error>>(|lua_ctx| {
            lua_ctx.globals().set("winapi", lua::WinApi::new())?;

            Ok(())
        })?;

        Ok(Context { lua })
    }

    pub fn lua(&self) -> &rlua::Lua {
        &self.lua
    }

    pub fn execute<S: AsRef<str>>(&mut self, script: S) -> Result<(), Error> {
        self.lua
            .context(|lua_ctx| lua_ctx.load(script.as_ref()).exec())?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{Context, Error};

    #[test]
    fn context() -> Result<(), Error> {
        Context::new()?;
        Ok(())
    }

    #[test]
    fn winapi() -> Result<(), Error> {
        let mut context = Context::new()?;
        context.execute("print(winapi())")?;

        Ok(())
    }

    #[test]
    fn kernel32() -> Result<(), Error> {
        let mut context = Context::new()?;
        context.execute("print(winapi().m)")?;
        context.execute("print(winapi():m(\"kernel32.dll\"))")?;

        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn LoadLibraryA() -> Result<(), Error> {
        let mut context = Context::new()?;
        context.execute("print(winapi():m(\"kernel32.dll\").f)")?;
        context.execute("print(winapi():m(\"kernel32.dll\"):f(\"LoadLibraryA\"):address())")?;

        Ok(())
    }
}
