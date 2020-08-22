use super::{gle, Module};
use crate::Error;
use winapi::shared::minwindef::HMODULE;
use winapi::um::winnt::{HANDLE, PROCESS_ALL_ACCESS, PROCESS_QUERY_LIMITED_INFORMATION};

#[derive(Clone, Copy, Debug)]
pub struct Process {
    handle: HANDLE,
    access_rights: u32,
}

impl Process {
    pub fn handle(&self) -> HANDLE {
        self.handle
    }

    #[allow(non_snake_case)]
    pub fn GetCurrentProcess() -> Process {
        Process {
            handle: unsafe { winapi::um::processthreadsapi::GetCurrentProcess() },
            access_rights: PROCESS_ALL_ACCESS,
        }
    }

    pub fn has_access_rights(&self, access_rights: u32) -> Result<(), Error> {
        if self.access_rights & access_rights != access_rights {
            Err(Error::InsufficientAccess)
        } else {
            Ok(())
        }
    }

    #[allow(non_snake_case)]
    pub fn EnumProcessModules(&self) -> Result<Vec<Module>, Error> {
        let mut hmodules: [HMODULE; 4096] = [std::ptr::null_mut(); 4096];
        let mut lpcbNeeded: u32 = 0;

        if unsafe {
            winapi::um::psapi::EnumProcessModules(
                self.handle(),
                hmodules.as_mut_ptr(),
                std::mem::size_of::<[HMODULE; 4096]>() as u32,
                &mut lpcbNeeded,
            )
        } == 0
        {
            Err(gle())
        } else {
            let mut hmodules = hmodules.to_vec();
            hmodules.truncate(lpcbNeeded as usize / std::mem::size_of::<HMODULE>());
            Ok(hmodules
                .into_iter()
                .map(|hmodule| Module::new(*self, hmodule))
                .collect())
        }
    }

    #[allow(non_snake_case)]
    pub fn GetProcessId(&self) -> Result<u32, Error> {
        self.has_access_rights(PROCESS_QUERY_LIMITED_INFORMATION)?;
        Ok(unsafe { winapi::um::processthreadsapi::GetProcessId(self.handle()) })
    }

    pub fn get_module<S: AsRef<str>>(&self, module_name: S) -> Result<Option<Module>, Error> {
        for module in self.EnumProcessModules()? {
            if module
                .GetModuleBaseNameA()?
                .to_lowercase()
                .as_str()
                .trim_end_matches(".dll")
                == module_name.as_ref()
            {
                return Ok(Some(module));
            }
        }
        Ok(None)
    }

    pub fn find_function_module<S: AsRef<str>>(
        &self,
        function_name: S,
    ) -> Result<Vec<String>, Error> {
        let mut modules = Vec::new();

        for module in self.EnumProcessModules()? {
            if module.GetProcAddress(function_name.as_ref()).is_ok() {
                modules.push(module.GetModuleBaseNameA()?);
            }
        }

        Ok(modules)
    }
}
