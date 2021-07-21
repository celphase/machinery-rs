use std::{
    ffi::c_void,
    mem::ManuallyDrop,
    os::raw::c_char,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use const_cstr::const_cstr;
use machinery::{
    foundation::{ApiRegistryApi, TheTruthApi, TheTruthCommonTypesApi, TT_TYPE_HASH__POSITION},
    plugin,
    plugins::the_machinery_shared::TM_CI_EDITOR_UI,
    Plugin,
};
use machinery_sys::{
    foundation::{
        tm_the_truth_o, tm_the_truth_property_definition_t,
        TM_THE_TRUTH_CREATE_TYPES_INTERFACE_NAME, TM_THE_TRUTH_PROPERTY_TYPE_SUBOBJECT,
    },
    plugins::the_machinery_shared::tm_ci_editor_ui_i,
};
use once_cell::sync::OnceCell;
use tracing::{event, Level};

plugin!(ExamplePlugin);

enum ExamplePlugin {}

impl Plugin for ExamplePlugin {
    fn load(registry: &ApiRegistryApi) {
        let state = PluginState {
            truth_api: registry.get(),
            truth_common_types: registry.get(),
            editor_aspects: Vec::new(),
        };
        STATE
            .set(RwLock::new(ManuallyDrop::new(state)))
            .unwrap_or_else(|_| panic!("STATE was already set"));

        unsafe {
            registry.add_implementation(
                TM_THE_TRUTH_CREATE_TYPES_INTERFACE_NAME,
                truth_create_types as *const c_void,
            );
        }

        event!(Level::INFO, "Example rust plugin loaded.");
    }

    fn unload(registry: &ApiRegistryApi) {
        unsafe {
            registry.remove_implementation(
                TM_THE_TRUTH_CREATE_TYPES_INTERFACE_NAME,
                truth_create_types as *const c_void,
            );
        }

        unsafe {
            let mut state = STATE.get().unwrap().write().unwrap();
            ManuallyDrop::drop(&mut state);
        }

        event!(Level::INFO, "Example rust plugin unloaded.");
    }
}

static STATE: OnceCell<RwLock<ManuallyDrop<PluginState>>> = OnceCell::new();

#[allow(dead_code)]
fn read_state() -> RwLockReadGuard<'static, ManuallyDrop<PluginState>> {
    STATE.get().unwrap().read().unwrap()
}

fn write_state() -> RwLockWriteGuard<'static, ManuallyDrop<PluginState>> {
    STATE.get().unwrap().write().unwrap()
}

struct PluginState {
    truth_api: TheTruthApi,
    truth_common_types: TheTruthCommonTypesApi,
    editor_aspects: Vec<Box<tm_ci_editor_ui_i>>,
}

extern "C" fn truth_create_types(tt: *mut tm_the_truth_o) {
    event!(Level::INFO, "Registering truth types");

    let mut state = write_state();

    unsafe {
        state.truth_common_types.create_common_types(tt);

        let properties = tm_the_truth_property_definition_t {
            name: const_cstr!("angular_velocity").as_ptr(),
            type_: TM_THE_TRUTH_PROPERTY_TYPE_SUBOBJECT as u32,
            type_hash: TT_TYPE_HASH__POSITION,
            ..Default::default()
        };

        let spin_type = (*state.truth_api.0).create_object_type.unwrap()(
            tt,
            const_cstr!("tm_rust_example_component").as_ptr(),
            &properties,
            1,
        );
        state
            .truth_api
            .set_default_object_to_create_subobjects(tt, spin_type);

        let editor_aspect = Box::new(tm_ci_editor_ui_i {
            category: Some(component_category),
            ..Default::default()
        });
        state.truth_api.set_aspect(
            tt,
            spin_type,
            TM_CI_EDITOR_UI,
            &*editor_aspect as *const _ as *const _,
        );
        state.editor_aspects.push(editor_aspect);
    }
}

extern "C" fn component_category() -> *const c_char {
    return const_cstr!("Samples").as_ptr();
}
