use std::any::Any;

use machinery_api::{foundation::ApiRegistryApi, Api};

use crate::{Service, ServiceRegistry};

/// Plugin management helper.
///
/// Stores plugin lifetime data and simplifies providing APIs.
pub struct Plugin {
    api_registry: *const ApiRegistryApi,
    services: Vec<(BoxedService, fn())>,
}

unsafe impl Send for Plugin {}
unsafe impl Sync for Plugin {}

impl Plugin {
    #[allow(clippy::new_without_default)]
    pub fn new(api_registry: *const ApiRegistryApi) -> Self {
        Self {
            api_registry,
            services: Vec::new(),
        }
    }

    pub fn api_registry(&self) -> *const ApiRegistryApi {
        self.api_registry
    }

    /// Register a service to be kept alive for the duration of the plugin's lifetime.
    pub fn service<F: FnOnce(&mut Plugin, ServiceRegistry<S>) -> S, S: Service>(
        &mut self,
        service_factory: F,
    ) {
        let registry = unsafe { ServiceRegistry::new(self.api_registry) };

        let service = Box::new(service_factory(self, registry));

        // ServiceRegistry safety relies on this being done right after.
        S::set_ptr(&*service);

        self.services.push((service, S::unset_ptr));
    }

    /// Gets an API from the API registry.
    pub fn get_api<T: Api>(&self) -> *const T {
        unsafe { (*self.api_registry).get(T::NAME.as_ptr(), T::VERSION) as *const T }
    }
}

impl Drop for Plugin {
    fn drop(&mut self) {
        for (_service, unset) in self.services.drain(..) {
            unset();
        }
    }
}

type BoxedService = Box<dyn Any + Send + Sync>;
