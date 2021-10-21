mod plugin;
mod service;
pub mod tracing;

use std::sync::atomic::{AtomicPtr, Ordering};

pub use self::{
    plugin::Plugin,
    service::{Service, ServiceAssociated, ServiceRegistry, ServiceInit},
};

use const_cstr::ConstCStr;
use machinery_api::foundation::{ApiRegistryApi, StrhashT, TtIdT, TtTypeT, TtUndoScopeT};

// Re-export macros for convenience
pub use machinery_macros::*;

#[doc(hidden)]
pub unsafe fn load_plugin<F: Fn(&mut Plugin)>(
    loader: F,
    api_registry: *const ApiRegistryApi,
    load: bool,
) {
    if load {
        let mut plugin = Plugin::new(api_registry);
        loader(&mut plugin);

        // Store the plugin instance
        let boxed = Box::new(plugin);
        let result = INSTANCE.swap(Box::into_raw(boxed), Ordering::SeqCst);

        // If the plugin was already loaded, the engine did something very wrong, or the function
        // was incorrectly called by the user
        if !result.is_null() {
            panic!("Plugin was already loaded on load!");
        }
    } else {
        // Take out the plugin instance
        let plugin = INSTANCE.swap(std::ptr::null_mut(), Ordering::SeqCst);

        // Sanity doublecheck
        if plugin.is_null() {
            panic!("Plugin wasn't loaded on unload!");
        }

        let _ = Box::from_raw(plugin);
    }
}

static INSTANCE: AtomicPtr<Plugin> = AtomicPtr::new(std::ptr::null_mut());

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
