// Code generated by machinery's build.rs

use machinery_sys::foundation::*;
use unsafe_unwrap::UnsafeUnwrap;

pub struct LoggerApi(pub *const tm_logger_api);

impl LoggerApi {
    pub unsafe fn add_logger(&self, logger: *const tm_logger_i) {
        ((*self.0).add_logger).unsafe_unwrap()(logger)
    }

    pub unsafe fn remove_logger(&self, logger: *const tm_logger_i) {
        ((*self.0).remove_logger).unsafe_unwrap()(logger)
    }

    pub unsafe fn print(&self, log_type: tm_log_type, msg: &str) {
        let msg = std::ffi::CString::new(msg).unwrap();
        ((*self.0).print).unsafe_unwrap()(log_type, msg.as_ptr())
    }

    pub unsafe fn printf(&self, log_type: tm_log_type, format: &str) -> ::std::os::raw::c_int {
        let format = std::ffi::CString::new(format).unwrap();
        ((*self.0).printf).unsafe_unwrap()(log_type, format.as_ptr())
    }
}

impl crate::Api for LoggerApi {
    const NAME: *const i8 = TM_LOGGER_API_NAME as *const _ as *const i8;
    unsafe fn from_raw(raw: *const std::ffi::c_void) -> Self {
        Self(raw as *const tm_logger_api)
    }
}
