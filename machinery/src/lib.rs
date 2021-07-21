use std::sync::Mutex;

use foundation::ApiRegistryApi;
use machinery_sys::foundation::tm_api_registry_api;
use once_cell::sync::OnceCell;

pub mod foundation;
mod generated;

pub type PluginInstance<T> = OnceCell<Mutex<Option<T>>>;

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
    };
}

pub fn load_plugin<T: Plugin>(
    instance: &PluginInstance<T>,
    registry: *const tm_api_registry_api,
    load: bool,
) {
    let registry = ApiRegistryApi(registry);

    if load {
        // Load the plugin and store it
        let plugin = T::load(&registry);
        if let Err(_) = instance.set(std::sync::Mutex::new(Some(plugin))) {
            panic!("Instance was double initialized");
        }
    } else {
        // Unload the plugin (drop will unload)
        let instance = instance.get().unwrap();
        instance.lock().unwrap().take();
    }
}

pub trait Plugin {
    fn load(registry: &ApiRegistryApi) -> Self;
}

pub trait Api {
    const NAME: *const i8;

    unsafe fn from_raw(raw: *const std::ffi::c_void) -> Self;
}
