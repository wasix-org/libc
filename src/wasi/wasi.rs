pub type sigset_t = c_uchar;

pub static CLOCK_PROCESS_CPUTIME_ID: clockid_t =
    unsafe { clockid_t(ptr_addr_of!(_CLOCK_PROCESS_CPUTIME_ID)) };
pub static CLOCK_THREAD_CPUTIME_ID: clockid_t =
    unsafe { clockid_t(ptr_addr_of!(_CLOCK_THREAD_CPUTIME_ID)) };
