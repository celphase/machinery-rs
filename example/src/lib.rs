use std::{ffi::c_void, os::raw::c_char};

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
use tracing::{event, Level};

plugin!(ExamplePlugin);

struct ExamplePlugin {
    registry: ApiRegistryApi,
    truth_api: TheTruthApi,
    truth_common_types: TheTruthCommonTypesApi,

    // Stored memory that we need to keep alive for the duration of the plugin
    editor_aspects: Vec<Box<tm_ci_editor_ui_i>>,
}

impl Plugin for ExamplePlugin {
    fn load(registry: ApiRegistryApi) -> Self {
        let plugin = ExamplePlugin {
            truth_api: registry.get(),
            truth_common_types: registry.get(),
            editor_aspects: Vec::new(),
            registry,
        };

        unsafe {
            plugin.registry.add_implementation(
                TM_THE_TRUTH_CREATE_TYPES_INTERFACE_NAME,
                truth_create_types as *const c_void,
            );
        }

        event!(Level::INFO, "Example rust plugin loaded.");

        plugin
    }
}

impl Drop for ExamplePlugin {
    fn drop(&mut self) {
        unsafe {
            self.registry.remove_implementation(
                TM_THE_TRUTH_CREATE_TYPES_INTERFACE_NAME,
                truth_create_types as *const c_void,
            );
        }

        event!(Level::INFO, "Example rust plugin unloaded.");
    }
}

extern "C" fn truth_create_types(tt: *mut tm_the_truth_o) {
    event!(Level::INFO, "Registering truth types");

    let mut guard = ExamplePlugin::write();
    let plugin = guard.as_mut().unwrap();

    unsafe {
        plugin.truth_common_types.create_common_types(tt);

        // Create a the example component
        let properties = tm_the_truth_property_definition_t {
            name: const_cstr!("angular_velocity").as_ptr(),
            type_: TM_THE_TRUTH_PROPERTY_TYPE_SUBOBJECT as u32,
            type_hash: TT_TYPE_HASH__POSITION,
            ..Default::default()
        };

        let spin_type =
            plugin
                .truth_api
                .create_object_type(tt, "tm_rust_example_component", &properties, 1);
        plugin
            .truth_api
            .set_default_object_to_create_subobjects(tt, spin_type);

        // Register the component with the editor
        let editor_aspect = Box::new(tm_ci_editor_ui_i {
            category: Some(component_category),
            ..Default::default()
        });
        plugin.truth_api.set_aspect(
            tt,
            spin_type,
            TM_CI_EDITOR_UI,
            &*editor_aspect as *const _ as *const _,
        );
        plugin.editor_aspects.push(editor_aspect);
    }
}

extern "C" fn component_category() -> *const c_char {
    return const_cstr!("Samples").as_ptr();
}
