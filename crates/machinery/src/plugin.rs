use std::any::Any;

use crate::Service;

/// Plugin management helper.
///
/// Stores plugin lifetime data and simplifies providing APIs.
pub struct Plugin {
    services: Vec<(BoxedService, fn())>,
}

impl Plugin {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    /// Register a service to be kept alive for the duration of the plugin's lifetime.
    pub fn service<S: Service>(&mut self, service: S) {
        let service = Box::new(service);
        S::set_ptr(service.as_ref());
        self.services.push((service, S::unset_ptr));
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
