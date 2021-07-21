pub mod foundation;
mod generated;

pub trait Api {
    const NAME: *const i8;

    unsafe fn from_raw(raw: *const std::ffi::c_void) -> Self;
}
