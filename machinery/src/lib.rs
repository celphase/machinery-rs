pub mod tm;
pub mod tracing;

use std::{
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};

use const_cstr::ConstCStr;
use tm::foundation::{ApiRegistryApi, StrhashT};

#[macro_export]
macro_rules! plugin {
    ($ty:ident) => {
        static INSTANCE: std::sync::atomic::AtomicPtr<$ty> =
            std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

        #[no_mangle]
        pub unsafe extern "C" fn tm_load_plugin(
            registry: *const machinery::tm::foundation::ApiRegistryApi,
            load: bool,
        ) {
            machinery::load_plugin::<$ty>(&INSTANCE, registry, load);
        }

        impl $ty {
            /// # Safety
            ///
            /// The data behind this pointer will only be valid for as long as the singleton is
            /// initialized.
            unsafe fn as_ptr() -> *const $ty {
                INSTANCE.load(std::sync::atomic::Ordering::SeqCst)
            }
        }
    };
}

pub fn load_plugin<P: Plugin>(
    instance: &AtomicPtr<P>,
    registry: *const ApiRegistryApi,
    load: bool,
) {
    if load {
        // Load the plugin
        let plugin = Box::new(P::load(registry));
        let result = instance.swap(Box::into_raw(plugin), Ordering::SeqCst);

        if !result.is_null() {
            panic!("Plugin was already loaded!");
        }
    } else {
        // Unload the plugin by dropping it
        let plugin = instance.swap(null_mut(), Ordering::SeqCst);
        unsafe {
            let _ = Box::from_raw(plugin);
        }
    }
}

pub trait Plugin: Sized + Send + Sync {
    fn load(registry: *const ApiRegistryApi) -> Self;
}

pub trait Api {
    const NAME: ConstCStr;
}

/// Unique identifier, made up of a string name and a hash generated from that name.
///
/// Generated using the `tm_ident!` macro:
/// ```no_run
/// const FOO: Identifier = tm_ident!("tm_foo");
/// ```
pub struct Identifier {
    pub name: ConstCStr,
    pub hash: StrhashT,
}

/// Convenience utility for getting an API type-safe from the registry.
pub fn get_api<T: Api>(registry: &ApiRegistryApi) -> *const T {
    unsafe { registry.get(T::NAME.as_cstr()) as *const T }
}

/// Convenience utility for getting an API type-safe from the registry, or returning None if it does
/// not exist.
pub fn get_api_optional<T: Api>(registry: &ApiRegistryApi) -> Option<*const T> {
    unsafe {
        let raw = registry.get_optional(T::NAME.as_cstr());
        if raw.is_null() {
            None
        } else {
            Some(raw as *const T)
        }
    }
}
