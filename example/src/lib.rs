use std::{
    ffi::{c_void, CStr},
    mem::size_of,
    os::raw::c_char,
    sync::Mutex,
};

use const_cstr::{const_cstr, ConstCStr};
use machinery::{
    plugin,
    tm::{
        foundation::{
            ApiRegistryApi, TheTruthApi, TheTruthCommonTypesApi, TheTruthO,
            TheTruthPropertyDefinitionT, TtIdT, TM_THE_TRUTH_CREATE_TYPES_INTERFACE_NAME,
            TM_THE_TRUTH_PROPERTY_TYPE_SUBOBJECT,
        },
        plugins::{
            entity::{
                ComponentI, ComponentManagerO, EntityApi, EntityContextO, EntityT,
                TM_ENTITY_CREATE_COMPONENT_INTERFACE_NAME,
            },
            the_machinery_shared::CiEditorUiI,
        },
    },
    Plugin, TM_CI_EDITOR_UI, TT_TYPE_HASH__POSITION,
};
use machinery_macro::export_plugin_fn;
use tracing::{event, Level};
use ultraviolet::Vec3;

plugin!(ExamplePlugin);

struct ExamplePlugin {
    registry: *const ApiRegistryApi,
    tt_api: *const TheTruthApi,
    tt_common_types: *const TheTruthCommonTypesApi,
    entity_api: *const EntityApi,

    // Stored memory for the lifetime of the plugin
    editor_aspects: Mutex<Vec<Box<CiEditorUiI>>>,
    components: Mutex<Vec<Box<ComponentI>>>,
}

unsafe impl Send for ExamplePlugin {}
unsafe impl Sync for ExamplePlugin {}

impl Plugin for ExamplePlugin {
    fn load(registry: *const ApiRegistryApi) -> Self {
        unsafe {
            // Integration with the tracing crate
            machinery::tracing::initialize(&*registry);
            event!(Level::INFO, foo = 42, "Example logging with data.");

            let plugin = ExamplePlugin {
                registry,
                tt_api: (*registry).ext_get(),
                tt_common_types: (*registry).ext_get(),
                entity_api: (*registry).ext_get(),

                editor_aspects: Mutex::new(Vec::new()),
                components: Mutex::new(Vec::new()),
            };

            (*plugin.registry).add_implementation(
                &CStr::from_ptr(TM_THE_TRUTH_CREATE_TYPES_INTERFACE_NAME.as_ptr() as *const i8),
                Self::truth_create_types as *const c_void,
            );

            (*plugin.registry).add_implementation(
                &CStr::from_ptr(TM_ENTITY_CREATE_COMPONENT_INTERFACE_NAME.as_ptr() as *const i8),
                Self::component_create as *const c_void,
            );

            event!(Level::INFO, "Example rust plugin loaded.");

            plugin
        }
    }
}

#[export_plugin_fn]
impl ExamplePlugin {
    fn truth_create_types(&self, tt: *mut TheTruthO) {
        // The Machinery stores component data in "entity assets", which are then constructed into
        // real components at runtime.

        event!(Level::INFO, "Registering truth types");

        unsafe {
            (*self.tt_common_types).create_common_types(tt);

            // Create a the example component truth data
            let properties = TheTruthPropertyDefinitionT {
                name: const_cstr!("angular_velocity").as_ptr(),
                type_: TM_THE_TRUTH_PROPERTY_TYPE_SUBOBJECT as u32,
                type_hash: TT_TYPE_HASH__POSITION,
                ..Default::default()
            };

            let spin_type = (*self.tt_api).create_object_type(
                tt,
                EXAMPLE_COMPONENT_NAME.as_cstr(),
                &properties,
                1,
            );
            (*self.tt_api).set_default_object_to_create_subobjects(tt, spin_type);

            // Register the component with the editor
            let editor_aspect = Box::new(CiEditorUiI {
                category: Some(component_category),
                ..Default::default()
            });
            (*self.tt_api).set_aspect(
                tt,
                spin_type,
                TM_CI_EDITOR_UI,
                &*editor_aspect as *const _ as *const _,
            );
            self.editor_aspects.lock().unwrap().push(editor_aspect);
        }
    }

    fn component_create(&self, ctx: *mut EntityContextO) {
        event!(Level::INFO, "Registering components");

        unsafe {
            // Register the component for entities
            let component = Box::new(ComponentI {
                name: EXAMPLE_COMPONENT_NAME.as_ptr(),
                bytes: size_of::<Vec3>() as u32,
                load_asset: Some(Self::component_load_asset),
                ..Default::default()
            });

            (*self.entity_api).register_component(ctx, &*component);
            self.components.lock().unwrap().push(component);
        }
    }

    fn component_load_asset(
        &self,
        _man: *mut ComponentManagerO,
        _e: EntityT,
        data: *mut c_void,
        tt: *const TheTruthO,
        asset: TtIdT,
    ) -> bool {
        let component = data as *mut Vec3;

        unsafe {
            let object = (*self.tt_api).read(tt as *mut TheTruthO, std::mem::transmute(asset));
            let data = (*self.tt_common_types).get_position(tt as *mut TheTruthO, object, 0);

            *component = Vec3::new(data.x, data.y, data.z);
        }

        true
    }
}

impl Drop for ExamplePlugin {
    fn drop(&mut self) {
        unsafe {
            (*self.registry).remove_implementation(
                &CStr::from_ptr(TM_THE_TRUTH_CREATE_TYPES_INTERFACE_NAME.as_ptr() as *const i8),
                Self::truth_create_types as *const c_void,
            );

            (*self.registry).remove_implementation(
                &CStr::from_ptr(TM_ENTITY_CREATE_COMPONENT_INTERFACE_NAME.as_ptr() as *const i8),
                Self::component_create as *const c_void,
            );
        }

        event!(Level::INFO, "Example rust plugin unloaded.");
    }
}

extern "C" fn component_category() -> *const c_char {
    return const_cstr!("Samples").as_ptr();
}

const EXAMPLE_COMPONENT_NAME: ConstCStr = const_cstr!("tm_rust_example_component");
