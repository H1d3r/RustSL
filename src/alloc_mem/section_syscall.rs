use dinvoke_rs::dinvoke;
use core::ffi::c_void;
use core::ptr;

#[allow(non_snake_case)]
type NtCreateSection = unsafe extern "system" fn(
    SectionHandle: *mut *mut c_void,
    DesiredAccess: u32,
    ObjectAttributes: *mut c_void,
    MaximumSize: *mut i64,
    SectionPageProtection: u32,
    AllocationAttributes: u32,
    FileHandle: *mut c_void,
) -> i32;

#[allow(non_snake_case)]
type NtMapViewOfSection = unsafe extern "system" fn(
    SectionHandle: *mut c_void,
    ProcessHandle: *mut c_void,
    BaseAddress: *mut *mut c_void,
    ZeroBits: usize,
    CommitSize: usize,
    SectionOffset: *mut i64,
    ViewSize: *mut usize,
    InheritDisposition: u32,
    AllocationType: u32,
    Win32Protect: u32,
) -> i32;

#[allow(dead_code)]
pub unsafe fn alloc_mem(size: usize) -> Result<*mut u8, String> {
    // Constants
    const SECTION_ALL_ACCESS: u32 = 0xF001F;
    const PAGE_EXECUTE_READWRITE: u32 = 0x40;
    const SEC_COMMIT: u32 = 0x0800_0000;
    // ViewUnmap = 2
    const VIEW_UNMAP: u32 = 2;

    // NtCreateSection via indirect syscall
    let create_type: NtCreateSection;
    #[allow(unused_assignments)]
    let mut create_status: Option<i32> = None;
    let mut section_handle: *mut c_void = ptr::null_mut();
    let mut max_size: i64 = size as i64;

    dinvoke::execute_syscall!(
        "NtCreateSection",
        create_type,
        create_status,
        &mut section_handle as *mut *mut c_void,
        SECTION_ALL_ACCESS,
        ptr::null_mut(),
        &mut max_size as *mut i64,
        PAGE_EXECUTE_READWRITE,
        SEC_COMMIT,
        ptr::null_mut(),
    );

    if create_status != Some(0) || section_handle.is_null() {
        return Err(match create_status {
            Some(code) => format!("NtCreateSection failed: 0x{:x}", code),
            None => "NtCreateSection syscall execution failed".to_string(),
        });
    }

    // NtMapViewOfSection via indirect syscall
    let map_type: NtMapViewOfSection;
    #[allow(unused_assignments)]
    let mut map_status: Option<i32> = None;
    let mut base_addr: *mut c_void = ptr::null_mut();
    let mut view_size: usize = size;

    dinvoke::execute_syscall!(
        "NtMapViewOfSection",
        map_type,
        map_status,
        section_handle,
        (-1isize) as *mut c_void, // Current process pseudo-handle
        &mut base_addr as *mut *mut c_void,
        0,
        size,
        ptr::null_mut(),
        &mut view_size as *mut usize,
        VIEW_UNMAP,
        0,
        PAGE_EXECUTE_READWRITE,
    );

    if map_status == Some(0) && !base_addr.is_null() {
        Ok(base_addr as *mut u8)
    } else {
        Err(match map_status {
            Some(code) => format!("NtMapViewOfSection failed: 0x{:x}", code),
            None => "NtMapViewOfSection syscall execution failed".to_string(),
        })
    }
}
