use machinery_sys::foundation::{
    tm_api_registry_api, tm_api_registry_listener_i, tm_strhash_t, TM_API_REGISTRY_API_NAME,
};
use unsafe_unwrap::UnsafeUnwrap;

use crate::Api;

pub use crate::generated::foundation::*;

pub struct ApiRegistryApi(pub *const tm_api_registry_api);

impl ApiRegistryApi {
    pub unsafe fn set(&self, name: &str, api: *const ::std::os::raw::c_void, bytes: u32) {
        let name = std::ffi::CString::new(name).unwrap();
        ((*self.0).set).unsafe_unwrap()(name.as_ptr(), api, bytes)
    }

    pub unsafe fn remove(&self, api: *const ::std::os::raw::c_void) {
        ((*self.0).remove).unsafe_unwrap()(api)
    }

    pub fn get<T: Api>(&self) -> T {
        unsafe {
            let raw = ((*self.0).get).unsafe_unwrap()(T::NAME);
            T::from_raw(raw)
        }
    }

    pub fn get_optional<T: Api>(&self) -> Option<T> {
        unsafe {
            let raw = ((*self.0).get_optional).unsafe_unwrap()(T::NAME);
            if raw.is_null() {
                None
            } else {
                Some(T::from_raw(raw))
            }
        }
    }

    pub unsafe fn add_implementation(
        &self,
        name: &str,
        implementation: *const ::std::os::raw::c_void,
    ) {
        let name = std::ffi::CString::new(name).unwrap();
        ((*self.0).add_implementation).unsafe_unwrap()(name.as_ptr(), implementation)
    }

    pub unsafe fn remove_implementation(
        &self,
        name: &str,
        implementation: *const ::std::os::raw::c_void,
    ) {
        let name = std::ffi::CString::new(name).unwrap();
        ((*self.0).remove_implementation).unsafe_unwrap()(name.as_ptr(), implementation)
    }

    pub unsafe fn implementations(
        &self,
        name: &str,
        count: *mut u32,
    ) -> *mut *mut ::std::os::raw::c_void {
        let name = std::ffi::CString::new(name).unwrap();
        ((*self.0).implementations).unsafe_unwrap()(name.as_ptr(), count)
    }

    pub unsafe fn add_listener(&self, listener: *const tm_api_registry_listener_i) {
        ((*self.0).add_listener).unsafe_unwrap()(listener)
    }

    pub unsafe fn static_variable(
        &self,
        id: tm_strhash_t,
        size: u32,
        file: &str,
        line: u32,
    ) -> *mut ::std::os::raw::c_void {
        let file = std::ffi::CString::new(file).unwrap();
        ((*self.0).static_variable).unsafe_unwrap()(id, size, file.as_ptr(), line)
    }

    pub unsafe fn log_missing_apis(&self) {
        ((*self.0).log_missing_apis).unsafe_unwrap()()
    }
}

impl crate::Api for ApiRegistryApi {
    const NAME: *const i8 = TM_API_REGISTRY_API_NAME as *const _ as *const i8;

    unsafe fn from_raw(raw: *const std::ffi::c_void) -> Self {
        Self(raw as *const tm_api_registry_api)
    }
}

unsafe impl Send for ApiRegistryApi {}
unsafe impl Sync for ApiRegistryApi {}
