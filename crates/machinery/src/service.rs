use std::any::Any;

/// Utility for exporting callbacks for The Machinery registry.
///
/// Services are used for implementing APIs and interfaces in The Machinery plugins.
/// They are available as global singletons after being registered with the `Plugin` type.
///
/// When registered into a `Plugin` instance, the service will become available for callbacks.
pub trait Service: Any + Send + Sync + 'static {
    fn set_ptr(value: *const Self);

    fn unset_ptr();

    /// # Safety
    /// The data behind the pointer is not checked automatically.
    unsafe fn ptr() -> *const Self;
}
