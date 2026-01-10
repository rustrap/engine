use std::ffi::c_void;

unsafe extern "C" {
    pub fn runtilappkit_init(ud: *const c_void, callback: unsafe extern "C" fn(*const c_void));
    pub fn runtilappkit_schedule();
    pub fn runtilappkit_run();
    pub fn runtilappkit_destroy();
}
