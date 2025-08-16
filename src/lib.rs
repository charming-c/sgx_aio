#![cfg_attr(feature = "sgx", no_std)]

#[cfg(feature = "sgx")]
#[macro_use]
extern crate sgx_tstd;
#[cfg(feature = "sgx")]
extern crate sgx_libc;
#[cfg(feature = "sgx")]
extern crate sgx_tstd as std;

use libc::*;
use sgx_types::sgx_status_t;

mod sys;
use sys::*;

pub unsafe fn io_setup(max_events: c_int, ctxp: *mut *mut c_void) -> c_int {
    let mut ret: c_int = 0;
    ocall_aio_io_setup_syscall(
        &mut ret,
        __NR_io_setup as c_long,
        max_events as c_long,
        ctxp,
    );
    ret
}

pub unsafe fn io_destroy(ctx: *const c_void) -> c_int {
    let mut ret: c_int = 0;
    ocall_aio_io_destroy_syscall(&mut ret, __NR_io_destroy as c_long, ctx as *const c_void);
    ret
}

pub unsafe fn io_submit(ctx: *const c_void, nr: c_long, iocb: *mut *mut c_void) -> c_int {
    let mut ret: c_int = 0;
    let iocb_size = core::mem::size_of::<sys::iocb>();
    ocall_aio_io_submit_syscall(
        &mut ret,
        __NR_io_submit as c_long,
        ctx as *const c_void,
        nr as c_long,
        iocb as *mut *mut c_void,
        // iocb_size as c_long,
    );
    ret
}

pub unsafe fn io_cancel(ctx: *const c_void, iocb: *const c_void, evt: *const c_void) -> c_int {
    let mut ret: c_int = 0;
    let iocb_size = core::mem::size_of::<sys::iocb>();
    let evt_size = core::mem::size_of::<sys::io_event>();
    ocall_aio_io_cancel_syscall(
        &mut ret,
        __NR_io_cancel as c_long,
        ctx as *const c_void,
        iocb as *const c_void,
        iocb_size as c_long,
        evt as *const c_void,
        evt_size as c_long,
    );
    ret
}

pub unsafe fn io_getevents(
    ctx: *const c_void,
    min_nr: c_long,
    nr: c_long,
    io_event: *const c_void,
    timeout: *const c_void,
) -> c_int {
    let mut ret: c_int = 0;
    let event_size = core::mem::size_of::<io_event>();
    let time_size = core::mem::size_of::<libc::timespec>();
    ocall_aio_io_getevents_syscall(
        &mut ret,
        __NR_io_getevents as c_long,
        ctx as *const c_void,
        min_nr as c_long,
        nr as c_long,
        io_event as *const c_void,
        event_size as c_long,
        timeout as *const c_void,
        time_size as c_long,
    );
    ret
}

extern "C" {
    fn ocall_aio_io_setup_syscall(
        ret: *mut c_int,
        syscall_code: c_long,
        maxevents: c_long,
        ctxp: *mut *mut c_void,
    ) -> sgx_status_t;

    fn ocall_aio_io_destroy_syscall(
        ret: *mut c_int,
        syscall_code: c_long,
        ctx: *const c_void,
    ) -> sgx_status_t;

    fn ocall_aio_io_submit_syscall(
        ret: *mut c_int,
        syscall_code: c_long,
        ctx: *const c_void,
        nr: c_long,
        iocb: *mut *mut c_void,
        // iocb_size: c_long,
    ) -> sgx_status_t;

    fn ocall_aio_io_cancel_syscall(
        ret: *mut c_int,
        syscall_code: c_long,
        ctx: *const c_void,
        iocb: *const c_void,
        iocb_size: c_long,
        evt: *const c_void,
        evt_size: c_long,
    ) -> sgx_status_t;

    fn ocall_aio_io_getevents_syscall(
        ret: *mut c_int,
        syscall_code: c_long,
        ctx: *const c_void,
        min_nr: c_long,
        nr: c_long,
        events: *const c_void,
        event_size: c_long, // only useful to EDL
        timeout: *const c_void,
        time_size: c_long,
    ) -> sgx_status_t;
}
