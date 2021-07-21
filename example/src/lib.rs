use std::{ffi::c_void, mem::size_of, os::raw::c_char, sync::Mutex};

use const_cstr::{const_cstr, ConstCStr};
use machinery::{
    plugin,
    tm::{
        foundation::{ApiRegistryApi, TheTruthApi, TheTruthCommonTypesApi, TT_TYPE_HASH__POSITION},
        plugins::{entity::EntityApi, the_machinery_shared::TM_CI_EDITOR_UI},
    },
    Plugin,
};
use machinery_macro::export_plugin_fn;
use machinery_sys::{
    foundation::{
        tm_the_truth_o, tm_the_truth_property_definition_t,
        TM_THE_TRUTH_CREATE_TYPES_INTERFACE_NAME, TM_THE_TRUTH_PROPERTY_TYPE_SUBOBJECT,
    },
    plugins::{
        entity::{
            tm_component_i, tm_component_manager_o, tm_entity_context_o, tm_entity_t,
            TM_ENTITY_CREATE_COMPONENT_INTERFACE_NAME,
        },
        the_machinery_shared::tm_ci_editor_ui_i,
    },
};
use tracing::{event, Level};
use ultraviolet::Vec3;

plugin!(ExamplePlugin);

struct ExamplePlugin {
    registry: ApiRegistryApi,
    tt_api: TheTruthApi,
    tt_common_types: TheTruthCommonTypesApi,
    entity_api: EntityApi,

    // Stored memory for the lifetime of the plugin
    editor_aspects: Mutex<Vec<Box<tm_ci_editor_ui_i>>>,
    components: Mutex<Vec<Box<tm_component_i>>>,
}

unsafe impl Send for ExamplePlugin {}
unsafe impl Sync for ExamplePlugin {}

impl Plugin for ExamplePlugin {
    fn load(registry: ApiRegistryApi) -> Self {
        // Integration with the tracing crate
        machinery::tracing::initialize(&registry);
        event!(Level::INFO, foo = 42, "Example logging with data.");

        let plugin = ExamplePlugin {
            tt_api: registry.get(),
            tt_common_types: registry.get(),
            entity_api: registry.get(),

            editor_aspects: Mutex::new(Vec::new()),
            components: Mutex::new(Vec::new()),

            // Stored last because we use it previously
            registry,
        };

        unsafe {
            plugin.registry.add_implementation(
                TM_THE_TRUTH_CREATE_TYPES_INTERFACE_NAME,
                Self::truth_create_types as *const c_void,
            );

            plugin.registry.add_implementation(
                TM_ENTITY_CREATE_COMPONENT_INTERFACE_NAME,
                Self::component_create as *const c_void,
            );
        }

        event!(Level::INFO, "Example rust plugin loaded.");

        plugin
    }
}

#[export_plugin_fn]
impl ExamplePlugin {
    fn truth_create_types(&self, tt: *mut tm_the_truth_o) {
        // The Machinery stores component data in "entity assets", which are then constructed into
        // real components at runtime.

        event!(Level::INFO, "Registering truth types");

        unsafe {
            self.tt_common_types.create_common_types(tt);

            // Create a the example component truth data
            let properties = tm_the_truth_property_definition_t {
                name: const_cstr!("angular_velocity").as_ptr(),
                type_: TM_THE_TRUTH_PROPERTY_TYPE_SUBOBJECT as u32,
                type_hash: TT_TYPE_HASH__POSITION,
                ..Default::default()
            };

            let spin_type = self.tt_api.create_object_type(
                tt,
                EXAMPLE_COMPONENT_NAME.as_cstr().to_str().unwrap(),
                &properties,
                1,
            );
            self.tt_api
                .set_default_object_to_create_subobjects(tt, spin_type);

            // Register the component with the editor
            let editor_aspect = Box::new(tm_ci_editor_ui_i {
                category: Some(component_category),
                ..Default::default()
            });
            self.tt_api.set_aspect(
                tt,
                spin_type,
                TM_CI_EDITOR_UI,
                &*editor_aspect as *const _ as *const _,
            );
            self.editor_aspects.lock().unwrap().push(editor_aspect);
        }
    }

    fn component_create(&self, ctx: *mut tm_entity_context_o) {
        event!(Level::INFO, "Registering components");

        unsafe {
            // Register the component for entities
            let component = Box::new(tm_component_i {
                name: EXAMPLE_COMPONENT_NAME.as_ptr(),
                bytes: size_of::<Vec3>() as u32,
                load_asset: Some(Self::component_load_asset),
                ..Default::default()
            });

            self.entity_api.register_component(ctx, &*component);
            self.components.lock().unwrap().push(component);
        }
    }

    fn component_load_asset(
        &self,
        _man: *mut tm_component_manager_o,
        _e: tm_entity_t,
        data: *mut c_void,
        // TODO: This type actually are part of foundation, map them correctly
        tt: *const machinery_sys::plugins::entity::tm_the_truth_o,
        asset: machinery_sys::plugins::entity::tm_tt_id_t,
    ) -> bool {
        let component = data as *mut Vec3;

        unsafe {
            let object = self
                .tt_api
                .read(tt as *mut tm_the_truth_o, std::mem::transmute(asset));
            let data = self
                .tt_common_types
                .get_position(tt as *mut tm_the_truth_o, object, 0);

            *component = Vec3::new(data.x, data.y, data.z);
        }

        true
    }
}

impl Drop for ExamplePlugin {
    fn drop(&mut self) {
        unsafe {
            self.registry.remove_implementation(
                TM_THE_TRUTH_CREATE_TYPES_INTERFACE_NAME,
                Self::truth_create_types as *const c_void,
            );
        }

        event!(Level::INFO, "Example rust plugin unloaded.");
    }
}

extern "C" fn component_category() -> *const c_char {
    return const_cstr!("Samples").as_ptr();
}

const EXAMPLE_COMPONENT_NAME: ConstCStr = const_cstr!("tm_rust_example_component");
