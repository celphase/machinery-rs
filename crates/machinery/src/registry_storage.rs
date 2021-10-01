use std::{any::Any, ffi::c_void};

use machinery_api::foundation::{ApiRegistryApi, VersionT};

/// Utility type for storing APIs and implementations registered with the API registry.
#[derive(Default)]
pub struct RegistryStorage {
    implementations: Vec<(*const i8, VersionT, *const c_void)>,
    boxes: Vec<Box<dyn Any>>,
}

unsafe impl Send for RegistryStorage {}
unsafe impl Sync for RegistryStorage {}

impl RegistryStorage {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an implementation of an interface to the registry.
    ///
    /// # Safety
    /// - `name` has to have a static lifetime.
    /// - `T` has to be the right interface type.
    /// - Safety rules for [RegistryStorage::add] also apply
    pub unsafe fn add_implementation<T: Any>(
        &mut self,
        registry: &ApiRegistryApi,
        name: *const i8,
        version: VersionT,
        implementation: T,
    ) -> *const T {
        let ptr = self.add(implementation);
        self.add_raw_implementation(registry, name, version, ptr as *const _ as *const c_void);
        ptr
    }

    /// Adds a raw implementation pointer of an interface to the registry.
    ///
    /// This implementation will be removed automatically on `clear`.
    ///
    /// # Safety
    /// - `name` has to have a static lifetime.
    /// - `implementation` has to contain a valid implementation.
    pub unsafe fn add_raw_implementation(
        &mut self,
        registry: &ApiRegistryApi,
        name: *const i8,
        version: VersionT,
        implementation: *const c_void,
    ) {
        registry.add_implementation(name, version, implementation);
        self.implementations.push((name, version, implementation));
    }

    /// Adds a value to the internal storage and returns a pointer.
    ///
    /// # Safety
    /// - Send is not enforced by this call automatically, as most The Machinery types do not have
    ///   Send implemented despite being thread safe.
    pub unsafe fn add<T: Any>(&mut self, value: T) -> *const T {
        let value = Box::new(value);
        let ptr: *const T = value.as_ref();
        self.boxes.push(value);
        ptr
    }

    /// Clean up all stored registered types.
    pub fn clear(&mut self, registry: &ApiRegistryApi) {
        for (name, version, implementation) in self.implementations.drain(..) {
            unsafe {
                registry.remove_implementation(name, version, implementation);
            }
        }

        // Free implementation memory
        self.boxes.clear();
    }
}
