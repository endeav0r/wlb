-- Load a library into memory. If the library is already loaded, this will just
-- return a module handle to the library
function LoadLibraryA(library)
    return winapi.kernel32.LoadLibraryA(
        winapi.types.pointer(winapi.types.cstring(library))
    )
end

-- Pop the message box
function MessageBoxA(caption, message)
    local MB_OK = 0

    return winapi.user32.MessageBoxA(
        winapi.types.u64(0),
        winapi.types.pointer(winapi.types.cstring(message)),
        winapi.types.pointer(winapi.types.cstring(caption)),
        winapi.types.u32(MB_OK)
    )
end

print(string.format("user32 0x%x", LoadLibraryA("user32.dll")))
MessageBoxA("Test", "Testing things")