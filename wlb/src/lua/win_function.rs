use crate::types::Value;
use crate::Error;

pub struct WinFunction {
    address: u64,
}

impl WinFunction {
    pub fn new(address: u64) -> WinFunction {
        WinFunction { address }
    }

    pub fn address(&self) -> u64 {
        self.address
    }
}

impl rlua::UserData for WinFunction {
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(
            rlua::MetaMethod::Call,
            |_, win_function, args: rlua::Variadic<Value>| {
                // we need actual u64 values we can pass to the function
                let vars = args
                    .iter()
                    .try_fold(Vec::new(), |mut values, value| {
                        values.push(value.value().value_u64().ok_or_else(|| {
                            Error::Custom(format!(
                                "Tried to get value on invalid type: {:?}",
                                value
                            ))
                        })?);
                        Ok(values)
                    })
                    .map_err(|e: Error| e.into_rlua_external())?;

                let result = unsafe {
                    match vars.len() {
                        0 => {
                            // println!("calling 0x{:x} with ()", win_function.address());
                            let f: extern "system" fn() -> i64 =
                                std::mem::transmute(win_function.address());
                            let result = f();
                            println!("result = 0x{:x}", result);
                            result
                        }
                        1 => {
                            // println!(
                            //     "calling 0x{:x} with (0x{:x})",
                            //     win_function.address(),
                            //     vars[0]
                            // );
                            let f: extern "system" fn(u64) -> i64 =
                                std::mem::transmute(win_function.address());
                            f(vars[0])
                        }
                        2 => {
                            // println!(
                            //     "calling 0x{:x} with (0x{:x}, 0x{:x})",
                            //     win_function.address(),
                            //     vars[0], vars[1]
                            // );
                            let f: extern "system" fn(u64, u64) -> i64 =
                                std::mem::transmute(win_function.address());
                            f(vars[0], vars[1])
                        }
                        3 => {
                            // println!(
                            //     "calling 0x{:x} with (0x{:x}, 0x{:x}, 0x{:x})",
                            //     win_function.address(),
                            //     vars[0], vars[1], vars[2]
                            // );
                            let f: extern "system" fn(u64, u64, u64) -> i64 =
                                std::mem::transmute(win_function.address());
                            f(vars[0], vars[1], vars[2])
                        }
                        4 => {
                            // println!(
                            //     "calling 0x{:x} with (0x{:x}, 0x{:x}, 0x{:x}, 0x{:x})",
                            //     win_function.address(),
                            //     vars[0], vars[1], vars[2], vars[3]
                            // );
                            let f: extern "system" fn(u64, u64, u64, u64) -> i64 =
                                std::mem::transmute(win_function.address());
                            f(vars[0], vars[1], vars[2], vars[3])
                        }
                        5 => {
                            let f: extern "system" fn(u64, u64, u64, u64, u64) -> i64 =
                                std::mem::transmute(win_function.address());
                            f(vars[0], vars[1], vars[2], vars[3], vars[4])
                        }
                        6 => {
                            let f: extern "system" fn(u64, u64, u64, u64, u64, u64) -> i64 =
                                std::mem::transmute(win_function.address());
                            f(vars[0], vars[1], vars[2], vars[3], vars[4], vars[5])
                        }
                        7 => {
                            let f: extern "system" fn(u64, u64, u64, u64, u64, u64, u64) -> i64 =
                                std::mem::transmute(win_function.address());
                            f(
                                vars[0], vars[1], vars[2], vars[3], vars[4], vars[5], vars[6],
                            )
                        }
                        _ => {
                            return Err(Error::Custom("Too many arguments".to_string())
                                .into_rlua_external())
                        }
                    }
                };
                Ok(result)
            },
        );

        methods.add_method("address", |_, win_function, ()| Ok(win_function.address()));
    }
}
