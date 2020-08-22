use super::{gle, Process};
use crate::Error;
use winapi::shared::minwindef::HMODULE;

#[derive(Clone, Copy, Debug)]
pub struct Module {
    process: Process,
    handle: HMODULE,
}

impl Module {
    pub fn new(process: Process, handle: HMODULE) -> Module {
        Module { process, handle }
    }

    pub fn process(&self) -> &Process {
        &self.process
    }
    pub fn handle(&self) -> HMODULE {
        self.handle
    }

    #[allow(non_snake_case)]
    pub fn GetModuleBaseNameA(&self) -> Result<String, Error> {
        let mut lpBaseName: [i8; 1024] = [0; 1024];

        let string_length = unsafe {
            winapi::um::psapi::GetModuleBaseNameA(
                self.process().handle(),
                self.handle(),
                lpBaseName.as_mut_ptr(),
                1024,
            )
        };

        if string_length == 0 {
            Err(gle())
        } else {
            Ok(unsafe { std::ffi::CStr::from_ptr(lpBaseName.as_ptr()) }
                .to_string_lossy()
                .to_string())
        }
    }

    #[allow(non_snake_case)]
    pub fn GetProcAddress<S: AsRef<str>>(&self, symbol: S) -> Result<Option<u64>, Error> {
        use std::ffi::CString;
        let symbol = CString::new(symbol.as_ref())?;

        let address = unsafe {
            winapi::um::libloaderapi::GetProcAddress(self.handle(), symbol.as_ptr()) as u64
        };

        if address == 0 {
            Ok(None)
        } else {
            Ok(Some(address))
        }
    }
}

unsafe impl Send for Module {}
