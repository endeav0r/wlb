function GetLastError()
    return winapi.kernel32.GetLastError()
end

function p(value) return winapi.types.pointer(value) end
function cs(s) return winapi.types.cstring(s) end
function u32(u) return winapi.types.u32(u) end
function u64(u) return winapi.types.u64(u) end

Module = {}

function Module:new(process_handle, module_handle)
    o = {
        process_handle = process_handle,
        module_handle = module_handle
    }
    setmetatable(o, self)
    self.__index = self
    return o
end

function Module:GetModuleBaseNameA()
    if self.module_base_name_a ~= nil then
        return self.module_base_name_a
    end

    local address = GlobalAlloc(1024)
    local f = winapi.kernel32.GetModuleBaseNameA
    if f == nil then f = winapi.psapi.GetModuleBaseNameA end
    local string_length = f(
        u64(self.process_handle),
        u64(self.module_handle),
        u64(address),
        u32(1024)
    )

    local base_name = ""
    for i = 1,string_length do
        local byte = winapi.peek8(address + i - 1)
        base_name = base_name .. string.char(byte)
    end

    GlobalFree(1024)

    self.module_base_name_a = base_name

    return base_name
end

function LoadLibraryA(library)
    return winapi.kernel32.LoadLibraryA(p(cs(library)))
end

function print_last_error()
    print(string.format("last error: 0x%08x", GetLastError()))
end

function GetCurrentProcess()
    return winapi.kernel32.GetCurrentProcess()
end

function GlobalAlloc(size)
    GMEM_FIXED = 0
    return winapi.kernel32.GlobalAlloc(
        winapi.types.u32(GMEM_FIXED),
        winapi.types.u32(size)
    )
end

function GlobalFree(ptr)
    return winapi.kernel32.GlobalFree(winapi.types.u64(ptr))
end

function EnumProcessModules()
    process_handle = GetCurrentProcess()
    local hProcess = u64(process_handle)
    local handles = GlobalAlloc(8 * 1024)
    local handles_ptr = winapi.types.u64(handles)
    local cb = winapi.types.u32(8 * 1024)
    local lpcbNeeded = winapi.types.u32(0xdeadbeef)

    local f = winapi.kernel32.EnumProcessModules
    if f == nil then
        f = winapi.psapi.EnumProcessModules
    end

    local result = f(hProcess, handles_ptr, cb, lpcbNeeded:ptr() )

    print(string.format("Found %u modules", lpcbNeeded:int() / 8))

    local h = {}
    for i = 1,(lpcbNeeded:int() / 8) do
        h[i] = Module:new(process_handle, winapi.peek64(handles + ((i - 1) * 8)))
        print(i, string.format("0x%016x", h[i].module_handle), h[i]:GetModuleBaseNameA())
    end

    GlobalFree(handles)
    return h
end

EnumProcessModules()