use crate::tools;
use std::ffi::CStr;
use std::os::raw::c_int;
use anyhow::{Result, Context};

#[link(name = "System")]
extern "C" {
    fn sysctl(mib: *const c_int, miblen: u32, oldp: *mut std::os::raw::c_void, oldlenp: *mut usize, newp: *const std::os::raw::c_void, newlen: usize) -> c_int;
    fn getpid() -> c_int;
}

const CTL_KERN: c_int = 1;
const KERN_PROC: c_int = 14;
const KERN_PROC_PID: c_int = 1;
const P_TRACED: i32 = 0x00000004;

#[repr(C)]
struct kinfo_proc {
    kp_proc: proc,
}

#[repr(C)]
struct proc {
    p_flag: i32,
}

pub fn is_debugger_attached() -> bool {
    let mut mib: [c_int; 4] = [CTL_KERN, KERN_PROC, KERN_PROC_PID, unsafe { getpid() }];
    let mut info: kinfo_proc = kinfo_proc { kp_proc: proc { p_flag: 0 } };
    let mut size = std::mem::size_of::<kinfo_proc>();

    let result = unsafe {
        sysctl(
            mib.as_mut_ptr(),
            4,
            &mut info as *mut _ as *mut std::os::raw::c_void,
            &mut size,
            std::ptr::null(),
            0,
        )
    };

    if result != 0 {
        return false;
    }

    (info.kp_proc.p_flag & P_TRACED) != 0
}

pub fn check_vm() -> Result<bool> {
    let result = tools::exec("system_profiler SPHardwareDataType | grep 'Model Identifier'")?;
    Ok(result.contains("Virtual"))
}

