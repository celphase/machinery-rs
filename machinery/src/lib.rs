pub mod foundation;
mod generated;
pub mod plugins;
mod tracing_sub;

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use foundation::ApiRegistryApi;
use machinery_sys::foundation::tm_api_registry_api;
use once_cell::sync::OnceCell;

use crate::tracing_sub::MachinerySubscriber;

pub type PluginInstance<P> = OnceCell<RwLock<Option<P>>>;

pub type PluginReadGuard<P> = RwLockReadGuard<'static, Option<P>>;

pub type PluginWriteGuard<P> = RwLockWriteGuard<'static, Option<P>>;

#[macro_export]
macro_rules! plugin {
    ($ty:ident) => {
        static INSTANCE: machinery::PluginInstance<$ty> = machinery::PluginInstance::new();

        #[no_mangle]
        pub unsafe extern "C" fn tm_load_plugin(
            registry: *const machinery_sys::foundation::tm_api_registry_api,
            load: bool,
        ) {
            machinery::load_plugin::<$ty>(&INSTANCE, registry, load);
        }

        impl $ty {
            fn read() -> machinery::PluginReadGuard<Self> {
                INSTANCE.get().unwrap().read().unwrap()
            }

            fn write() -> machinery::PluginWriteGuard<Self> {
                INSTANCE.get().unwrap().write().unwrap()
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

        // Initialize logging
        let subscriber = MachinerySubscriber::new(&registry);
        tracing::subscriber::set_global_default(subscriber).unwrap();

        // Load the plugin
        let plugin = P::load(registry);
        let result = instance.set(RwLock::new(Some(plugin)));

        if result.is_err() {
            panic!("Plugin was already loaded!");
        }
    } else {
        // Unload the plugin by dropping it
        instance.get().unwrap().write().unwrap().take();
    }
}

pub trait Plugin: Sized + Send + Sync {
    fn load(registry: ApiRegistryApi) -> Self;
}

pub trait Api {
    const NAME: *const i8;

    unsafe fn from_raw(raw: *const std::ffi::c_void) -> Self;
}
