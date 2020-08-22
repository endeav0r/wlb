# Windows Lua Bridge

Rust is awesome. Windows is more awesome than it's given credit for. If only there was a way to programmatically explore the inconsistent windows API with the convenience of scripting, and the awesomeness of Rust. Well... now there is!

The Windows-Lua Bridge bridges the Windows API with lua, and makes it available in a package available via Rust. Influenced by what I have read of meterpreter's railgun (though I have never used railgun), wlb attempts to make it possible, and easy, to call into the C-level windows API from lua, all via code controlled through Rust.

Consider it a tool for, "Windows API exploration for people who don't Windows, but like Rust, and like Lua, and want to learn to Windows good too." Who knows, maybe you'll find other useful applications for it as well.

## Examples

### MessageBoxA

In our first example, we'll try to pop a Message box using the Windows API. For Windows noobs like me, let's take a look at the [MessageBoxA documentation provided by Microsoft](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxa).

Here are the takeaways from the documentation:
* Four arguments to this function:
  1. HWND, which can be null, so we'll just pass a 0.
  2. Pointer to an ASCII string (the `A` in `MessageBoxA` is for ASCII) for our message.
  3. Pointer to an ASCII string for our message box title.
  4. A DWORD (uint32) to describe the type of message box.
* If you scroll to the bottom, you'll see this library is found in user32.dll. We need to make sure that library exists in the current process.

We have two primary problems to solve:

* How do we actually locate the `MessageBoxA` function in memory so we can call it?
* How do we make sure our arguments are interpreted correctly, especially things like pointers to strings?

#### Finding MessageBoxA

wlb will locate the target function by calling [EnumProcessModules](https://docs.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodules) with the file handle returned by [GetCurrentProcess](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess). Those modules are walked to find one that matches indexed name into `winapi`. Once that module is located, the api calls [GetProcAddress](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress).

#### Passing arguments

TBD