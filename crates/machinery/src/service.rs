use std::any::Any;

/// Service utility for exposing callbacks to The Machinery registry.
///
/// This trait can be derived to be implemented, and is used in combination with procedural macros
/// like `tm_service_export` to create implementations of interfaces and APIs.
/// When registered into a `Plugin` instance, the service will become available for callbacks.
pub trait Service: Any + Send + Sync + 'static {
    fn set_ptr(value: *const Self);

    fn unset_ptr();

    /// # Safety
    /// The data behind the pointer is not checked automatically.
    unsafe fn ptr() -> *const Self;
}
