use crate::Error;

mod module;
mod process;

pub use module::Module;
pub use process::Process;

fn gle() -> Error {
    Error::GetLastError(unsafe { winapi::um::errhandlingapi::GetLastError() })
}

#[allow(non_snake_case)]
pub fn GetComputerNameA() -> Result<String, Error> {
    // Max is supposedly 15
    let mut lpBuffer: [i8; 1024] = [0; 1024];
    let mut nSize: u32 = 0;

    let result =
        unsafe { winapi::um::winbase::GetComputerNameA(&mut lpBuffer[0] as *mut i8, &mut nSize) };

    if result == 0 {
        Err(gle())
    } else {
        Ok(unsafe { std::ffi::CStr::from_ptr(lpBuffer.as_ptr()) }
            .to_string_lossy()
            .to_string())
    }
}

#[allow(non_snake_case)]
pub fn GetUserNameA() -> Result<String, Error> {
    // Max is supposedly 256
    let mut lpBuffer: [i8; 1024] = [0; 1024];
    let mut nSize: u32 = 0;

    let result =
        unsafe { winapi::um::winbase::GetUserNameA(&mut lpBuffer[0] as *mut i8, &mut nSize) };

    if result == 0 {
        Err(gle())
    } else {
        Ok(unsafe { std::ffi::CStr::from_ptr(lpBuffer.as_ptr()) }
            .to_string_lossy()
            .to_string())
    }
}
