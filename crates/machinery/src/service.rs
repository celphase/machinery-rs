use std::{any::Any, marker::PhantomData};

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

/// Guard type for creating constants associated with a service, used for verifying C exported
/// functions are for the expected service.
pub struct ServiceAssociated<S, T> {
    pub value: T,
    _s: PhantomData<S>,
}

impl<S, T> ServiceAssociated<S, T> {
    /// # Safety
    /// Safe API and implementation registration relies on the associaton to be correct.
    pub const unsafe fn new(value: T) -> Self {
        Self {
            value,
            _s: PhantomData,
        }
    }
}
