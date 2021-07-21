pub mod foundation;
mod generated;
pub mod plugins;
mod tracing_sub;

use foundation::ApiRegistryApi;
use machinery_sys::foundation::tm_api_registry_api;

use crate::tracing_sub::MachinerySubscriber;

#[macro_export]
macro_rules! plugin {
    ($ty:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn tm_load_plugin(
            registry: *const machinery_sys::foundation::tm_api_registry_api,
            load: bool,
        ) {
            machinery::load_plugin::<$ty>(registry, load);
        }
    };
}

pub fn load_plugin<P: Plugin>(registry: *const tm_api_registry_api, load: bool) {
    let registry = ApiRegistryApi(registry);

    if load {
        // Initialize logging
        let subscriber = MachinerySubscriber::new(&registry);
        tracing::subscriber::set_global_default(subscriber).unwrap();

        // Load the plugin
        P::load(&registry);
    } else {
        // Unload the plugin
        P::unload(&registry);
    }
}

pub trait Plugin {
    fn load(registry: &ApiRegistryApi);
    fn unload(registry: &ApiRegistryApi);
}

pub trait Api {
    const NAME: *const i8;

    unsafe fn from_raw(raw: *const std::ffi::c_void) -> Self;
}
