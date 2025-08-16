extern crate libc;
extern crate sgx_types;

use libc::syscall;
use sgx_types::{c_int, c_long, c_void};


#[no_mangle]
pub extern "C" fn ocall_aio_io_setup_syscall(
    syscall_code: c_long,
    maxevents: c_long,
    ctxp: *mut *mut c_void,
) -> c_int {
    let res = unsafe { syscall(syscall_code, maxevents, ctxp as c_long) };
    if res == -1 {
        let errno = unsafe { *libc::__errno_location() };
        -errno
    } else {
        res as c_int
    }
}

#[no_mangle]
pub extern "C" fn ocall_aio_io_destroy_syscall(
    syscall_code: c_long,
    ctxp: *const c_void,
) -> c_int {
    let res = unsafe { syscall(syscall_code, ctxp as c_long) };
    if res == -1 {
        let errno = unsafe { *libc::__errno_location() };
        -errno
    } else {
        res as c_int
    }
}

#[no_mangle]
pub extern "C" fn ocall_aio_io_submit_syscall(
    syscall_code: c_long,
    ctxp: *const c_void,
    nr: c_long,
    iocb: *mut *mut c_void,
) -> c_int {
    let res = unsafe { syscall(syscall_code, ctxp as c_long, nr, iocb as c_long) };
    if res == -1 {
        let errno = unsafe { *libc::__errno_location() };
        -errno
    } else {
        res as c_int
    }
}

#[no_mangle]
pub extern "C" fn ocall_aio_io_cancel_syscall(
    syscall_code: c_long,
    ctxp: *const c_void,
    iocb: *const c_void,
    _iocb_size: c_long,
    evt: *const c_void,
    _evt_size: c_long,
) -> c_int {
    let res = unsafe { syscall(syscall_code, ctxp as c_long, iocb as c_long, evt as c_long) };
    if res == -1 {
        let errno = unsafe { *libc::__errno_location() };
        -errno
    } else {
        res as c_int
    }
}

#[no_mangle]
pub extern "C" fn ocall_aio_io_getevents_syscall(
    syscall_code: c_long,
    ctxp: *const c_void,
    min_nr: c_long,
    nr: c_long,
    events: *const c_void,
    _event_size: c_long, // only useful to EDL
    timeout: *const c_void,
    _time_size: c_long,
) -> c_int {
    let res = unsafe { syscall(syscall_code, ctxp as c_long, min_nr, nr, events as c_long, timeout as c_long) };
    if res == -1 {
        let errno = unsafe { *libc::__errno_location() };
        -errno
    } else {
        res as c_int
    }
}
