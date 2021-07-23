pub mod tm;
pub mod tracing;

use std::{
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};

use const_cstr::ConstCStr;
use tm::foundation::ApiRegistryApi;

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

// TODO: Safer registry wrapper with utilities

impl ApiRegistryApi {
    pub fn ext_get<T: Api>(&self) -> *const T {
        unsafe { self.get(T::NAME.as_cstr()) as *const T }
    }

    pub fn ext_get_optional<T: Api>(&self) -> Option<*const T> {
        unsafe {
            let raw = self.get_optional(T::NAME.as_cstr());
            if raw.is_null() {
                None
            } else {
                Some(raw as *const T)
            }
        }
    }
}
