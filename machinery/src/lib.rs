mod generated;
pub mod tm;
pub mod tracing;

use std::{
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};

use machinery_sys::foundation::tm_api_registry_api;
use tm::foundation::ApiRegistryApi;

pub type PluginInstance<P> = AtomicPtr<P>;

#[macro_export]
macro_rules! plugin {
    ($ty:ident) => {
        static INSTANCE: machinery::PluginInstance<$ty> =
            machinery::PluginInstance::new(std::ptr::null_mut());

        #[no_mangle]
        pub unsafe extern "C" fn tm_load_plugin(
            registry: *const machinery_sys::foundation::tm_api_registry_api,
            load: bool,
        ) {
            machinery::load_plugin::<$ty>(&INSTANCE, registry, load);
        }

        impl $ty {
            unsafe fn as_ptr() -> *const $ty {
                INSTANCE.load(std::sync::atomic::Ordering::SeqCst)
            }
        }
    };
}

pub fn load_plugin<P: Plugin>(
    instance: &PluginInstance<P>,
    registry: *const tm_api_registry_api,
    load: bool,
) {
    if load {
        let registry = ApiRegistryApi(registry);

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
    fn load(registry: ApiRegistryApi) -> Self;
}

pub trait Api {
    const NAME: *const i8;

    unsafe fn from_raw(raw: *const std::ffi::c_void) -> Self;
}
