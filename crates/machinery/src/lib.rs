mod registry_storage;
pub mod singleton;
pub mod tracing;

pub use self::{registry_storage::RegistryStorage, singleton::Singleton};

use const_cstr::ConstCStr;
use machinery_api::{
    foundation::{ApiRegistryApi, StrhashT, TtIdT, TtTypeT, TtUndoScopeT},
    Api,
};

// Re-export macros for convenience
pub use machinery_macros::*;

#[macro_export]
macro_rules! plugin {
    ($ty:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn tm_load_plugin(
            registry: *const machinery_api::foundation::ApiRegistryApi,
            load: bool,
        ) {
            machinery::load_plugin::<$ty>(registry, load);
        }
    };
}

/// # Safety
/// This should only be called once for load and once for unload.
pub unsafe fn load_plugin<P: Plugin>(registry: *const ApiRegistryApi, load: bool) {
    if load {
        // Load the plugin
        let plugin = P::load(registry);
        P::create(plugin);
    } else {
        // Unload the plugin
        P::destroy();
    }
}

pub trait Plugin: Singleton {
    fn load(registry: *const ApiRegistryApi) -> Self;
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
    unsafe { registry.get(T::NAME.as_ptr()) as *const T }
}

/// Convenience utility for getting an API type-safe from the registry, or returning None if it does
/// not exist.
pub fn get_api_optional<T: Api>(registry: &ApiRegistryApi) -> Option<*const T> {
    unsafe {
        let raw = registry.get_optional(T::NAME.as_ptr());
        if raw.is_null() {
            None
        } else {
            Some(raw as *const T)
        }
    }
}

/// Compares two Truth IDs for equality.
pub fn tt_id_eq(left: TtIdT, right: TtIdT) -> bool {
    unsafe { left.__bindgen_anon_1.u64_ == right.__bindgen_anon_1.u64_ }
}

/// Get the Truth type for an ID.
pub fn tt_id_type(id: TtIdT) -> TtTypeT {
    unsafe {
        TtTypeT {
            u64_: id.__bindgen_anon_1.__bindgen_anon_1.type_(),
        }
    }
}

/// Used as `undo_scope` for operations that shouldn't be undoable.
pub const TM_TT_NO_UNDO_SCOPE: TtUndoScopeT = TtUndoScopeT { u64_: 0 };

/// Couldn't be generated as carray.inl includes a standard library header.
#[repr(C)]
pub struct CArrayHeaderT {
    pub capacity: u64,
    pub size: u64,
}
