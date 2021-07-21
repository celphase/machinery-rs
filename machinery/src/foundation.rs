pub use crate::generated::foundation::*;

impl ApiRegistryApi {
    pub fn get_typed<T: RegistryApi>(&self) -> T {
        unsafe { T::from_raw(self.get(T::NAME)) }
    }
}

pub trait RegistryApi {
    const NAME: &'static str;

    unsafe fn from_raw(raw: *const std::ffi::c_void) -> Self;
}
