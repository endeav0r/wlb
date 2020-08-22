function GetLastError()
    return winapi.kernel32.GetLastError()
end

function p(value) return winapi.types.pointer(value) end
function cs(s) return winapi.types.cstring(s) end
function u32(u) return winapi.types.u32(u) end
function u64(u) return winapi.types.u64(u) end

function LoadLibraryA(library)
    return winapi.kernel32.LoadLibraryA(p(cs(library)))
end

function MessageBoxA(caption, message)
    local MB_OK = 0

    return winapi.user32.MessageBoxA(
        u32(0),
        p(cs(message)),
        p(cs(caption)),
        u32(MB_OK)
    )
end

function print_last_error()
    print(string.format("last error: 0x%08x", GetLastError()))
end

function GetCurrentProcess()
    return winapi.kernel32.GetCurrentProcess()
end

function process_mitigation_dep_policy()
    local ProcessDEPPolicy = 0
    local EnableFlag = 1
    local DisableAt1ThunkEmulationFlag = 2
    local PermanentFlag = 32

    local struct = winapi.types.struct()
    struct:push(winapi.types.struct_field("Flags", 0, winapi.types.u32))
    struct:push(winapi.types.struct_field("Permanent", 4, winapi.types.u32))
    local struct_buf = struct:buf()

    local result = winapi.kernel32.GetProcessMitigationPolicy(
        u64(GetCurrentProcess()),
        u32(ProcessDEPPolicy),
        struct_buf:pointer_to(),
        u64(8)
    )


    local flags = struct_buf:get_field("Flags")
    local permanent = struct_buf:get_field("Permanent")
    print("Flags:", flags)
    print("Permenant:", permanent)

    if flags:int() & EnableFlag then print("Dep is enabled") end
    if flags:int() & DisableAt1ThunkEmulationFlag then
        print("Disable at 1 thunk emulation is enabled")
    end
    if permanent:int() > 0 then
        print("DEP is peramenently enabled")
    end
end

print(string.format("0x%x", GetCurrentProcess()))
process_mitigation_dep_policy()