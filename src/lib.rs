extern crate winapi_typedefs;

use std::os::raw::c_void;
use winapi_typedefs::intsafe::*;
use winapi_typedefs::winnt::*;
use winapi_typedefs::windef::*;
use std::ffi::CString;

// TODO: Create real definitions for va_list
pub type va_list = c_void;

// Import declarations
#[allow(non_snake_case)]
#[link(name="kernel32")]
extern "system"
{
    fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;
    fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
    fn GetLastError() -> DWORD;
    fn FormatMessageA(dwFlags: DWORD, lpSource: LPCVOID, dwMessageId: DWORD, dwLanguageId: DWORD, lpBuffer: LPSTR, nSize: DWORD, arguments: *mut va_list) -> DWORD;
    fn FormatMessageW(dwFlags: DWORD, lpSource: LPCVOID, dwMessageId: DWORD, dwLanguageId: DWORD, lpBuffer: LPWSTR, nSize: DWORD, arguments: *mut va_list) -> DWORD;
    fn LoadLibraryA(lpFileName: LPCSTR) -> HMODULE;
    fn FreeLibrary(hModule: HMODULE) -> BOOL;
    fn GetProcAddress(hModule: HMODULE, lpProcName: LPCSTR) -> FARPROC;
}

// Safe interfaces
/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms683199(v=vs.85).aspx
pub fn get_module_handle_a(lp_module_name: LPCSTR) -> Option<HMODULE>
{
    unsafe
    {
        let hmodule = GetModuleHandleA(lp_module_name);
        if hmodule.is_null()
        {
            return None;
        }
        else
        {
            return Some(hmodule);
        }
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms683199(v=vs.85).aspx
pub fn get_module_handle_w(lp_module_name: LPCWSTR) -> Option<HMODULE>
{
    unsafe
    {
        let hmodule = GetModuleHandleW(lp_module_name);
        if hmodule.is_null()
        {
            return None;
        }
        else
        {
            return Some(hmodule);
        }
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/ms679360(v=vs.85).aspx
pub fn get_last_error() -> DWORD
{
    unsafe
    {
        GetLastError()
    }
}

pub fn format_message_a(dw_flags: DWORD, lp_source: LPCVOID, dw_message_id: DWORD, dw_language_id: DWORD, lp_buffer: LPSTR, n_size: DWORD, arguments: *mut va_list) -> DWORD
{
    unsafe
    {
        FormatMessageA(dw_flags, lp_source, dw_message_id, dw_language_id, lp_buffer, n_size, arguments)
    }
}

pub fn format_message_w(dw_flags: DWORD, lp_source: LPCVOID, dw_message_id: DWORD, dw_language_id: DWORD, lp_buffer: LPWSTR, n_size: DWORD, arguments: *mut va_list) -> DWORD
{
    unsafe
    {
        FormatMessageW(dw_flags, lp_source, dw_message_id, dw_language_id, lp_buffer, n_size, arguments)
    }
}

pub fn load_library_a(file_name: &str) -> Option<HMODULE>
{
    unsafe
    {
        //println!("{}", file_name);
        let cstr;
        match CString::new(file_name)
        {
            Err(_msg) =>
            {
                return None;
            },
            Ok(value) =>
            {
                cstr = value;
            }
        }

        let module = LoadLibraryA(cstr.as_ptr());
        if module.is_null()
        {
            return None;
        }
        else
        {
            return Some(module);
        }
    }
}

pub fn free_library(h_module: HMODULE) -> bool
{
    unsafe
    {
        if FreeLibrary(h_module) == FALSE
        {
            return false;
        }
        else
        {
            return true;
        }
    }
}

pub fn get_proc_address(h_module: HMODULE, proc_name: &str) -> Option<FARPROC>
{
    unsafe
    {
        let cstr;
        match CString::new(proc_name)
        {
            Err(_msg) =>
            {
                return None;
            },
            Ok(value) =>
            {
                cstr = value;
            }
        }
        let fnptr = GetProcAddress(h_module, cstr.as_ptr());
        if (fnptr as *const ()).is_null()
        {
            return None;
        }
        else
        {
            return Some(fnptr);
        }
    }
}
