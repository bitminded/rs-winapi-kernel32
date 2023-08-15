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

/// Retrieves a module handle for the specified module. The module must have been
/// loaded by the calling process.
/// 
/// # Parameters
/// ## name
/// The name of the loaded module (either a .dll or .exe). If the file name
/// extension is omitted, the default library extension .dll is appended. The file
/// name string can include a trailing point character (.) to indicate that the
/// module name has no extension. The string does not have to specify a path.
/// When specifying a path, be sure to use backslashes (\), not forward slashes (/).
/// The name is compared (case independently) to the names of modules currently
/// mapped into the address space of the calling process.
///
/// If this parameter is None, a handle to the file used to create the calling
/// process (.exe) is returned.
///
/// Does not retrieve handles for modules that were loaded using the LOAD_LIBRARY_AS_DATAFILE
/// flag.
///
/// # Return value
/// If the function succeeds, a handle to the specified module is returned.
///
/// If the function fails, the return value is None. To get extended error
/// information, call get_last_error.
///
/// # Remarks
/// The returned handle is not global or inheritable. It cannot be duplicated or
/// used by another process.
/// 
/// If _name_ does not include a path and there is more than one loaded module
/// with the same base name and extension, you cannot predict which module handle
/// will be returned. To work around this problem, you could specify a path, use
/// side-by-side assemblies, or use get_module_handle_ex_w to specify a memory
/// location rather than a DLL name.
///
/// The get_module_function_w function returns a handle to a mapped module without
/// incrementing its reference count. However, if this handle is passed to the
/// free_library function, the reference count of the mapped module will be
/// decremented. Therefore, do not pass a handle returned by get_module_function_w
/// to the free_library function. Doing so can cause a DLL module to be unmapped
/// prematurely.
///
/// This function must be used carefully in a multithreaded application. There is
/// no guarantee that the module handle remains valid between the time this function
/// returns the handle and the time it is used. For example, suppose that a thread
/// retrieves a module handle, but before it uses the handle, a second thread frees
/// the module. If the system loads another module, it could reuse the module handle
/// that was recently freed. Therefore, the first thread would have a handle to a
/// different module than the one intended.
pub fn get_module_handle_w(name: Option<&str>) -> Option<HMODULE>
{
    unsafe
    {
        let name = match name
        {
            None =>
            {
                let name: Vec<u16> = vec![0];
            },
            Some(name) =>
            {
                let mut name: Vec<u16> = name.encode_utf16().collect();
                name.push(0);
            }
        };
        let name = name.as_ptr();
        let handle = GetModuleHandleW(name);
        if handle.is_null()
        {
            return None;
        }
        else
        {
            return Some(handle);
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
