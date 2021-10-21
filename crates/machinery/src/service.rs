use std::{any::Any, marker::PhantomData};

use machinery_api::{foundation::ApiRegistryApi, Interface};

/// Utility for exporting callbacks for The Machinery registry.
///
/// Services are used for implementing APIs and interfaces in The Machinery plugins.
/// They are available as global singletons after being registered with the `Plugin` type.
///
/// When registered into a `Plugin` instance, the service will become available for callbacks.
pub trait Service: Any + Sized + Send + Sync + 'static {
    fn set_ptr(value: *const Self);

    fn unset_ptr();

    /// # Safety
    /// The data behind the pointer is not checked automatically.
    unsafe fn ptr() -> *const Self;
}

pub trait ServiceInit: Service {
    fn register(&self, registry: ServiceRegistry<Self>);
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

/// Utility that allows services to register their interfaces safely.
pub struct ServiceRegistry<S> {
    api_registry: *const ApiRegistryApi,
    _s: PhantomData<S>,
}

impl<S> ServiceRegistry<S> {
    /// # Safety
    /// A service registry may only exist after the service's singleton pointer has been
    /// initialized.
    pub unsafe fn new(api_registry: *const ApiRegistryApi) -> Self {
        Self {
            api_registry,
            _s: PhantomData,
        }
    }

    pub fn add_implementation<I: Interface>(
        &self,
        implementation: &'static ServiceAssociated<S, I>,
    ) {
        unsafe {
            (*self.api_registry).add_implementation(
                I::NAME.as_ptr(),
                I::VERSION,
                implementation.value.to_registry_ptr(),
            );
        }
    }
}
