use std::{ffi::c_void, mem::size_of, os::raw::c_char, sync::Mutex};

use const_cstr::const_cstr;
use machinery::{
    tm_identifier, tm_plugin, tm_service_export, tm_service_impl, Identifier, Plugin, Service,
};
use machinery_api::{
    foundation::{
        ApiRegistryApi, StrhashT, TheTruthApi, TheTruthCommonTypesApi, TheTruthO,
        TheTruthPropertyDefinitionT, TtIdT, Vec4T, TM_THE_TRUTH_PROPERTY_TYPE_SUBOBJECT,
        TM_TT_TYPE_HASH__POSITION,
    },
    plugins::{
        entity::{
            ComponentI, ComponentManagerO, ComponentTypeT, EngineI, EngineO, EngineSystemCommonI,
            EngineUpdateSetT, EntityApi, EntityCommandsO, EntityContextO, EntityT,
            TransformComponentT, TM_ENGINE__SCENE_TREE, TM_ENTITY_BB__DELTA_TIME,
            TM_ENTITY_CREATE_COMPONENT_I_VERSION, TM_ENTITY_REGISTER_ENGINES_SIMULATION_I_VERSION,
            TM_TT_TYPE_HASH__TRANSFORM_COMPONENT,
        },
        the_machinery_shared::{CiEditorUiI, TM_CI_EDITOR_UI},
    },
    TheTruthCreateTypesI,
};
use tracing::{event, Level};
use ultraviolet::{Rotor3, Vec3};

#[tm_plugin]
fn load(plugin: &mut Plugin) {
    // Integration with the tracing crate
    machinery::tracing::initialize(plugin);

    // Register the example service
    plugin.service(|p| ExampleService::new(p));
}

#[allow(clippy::vec_box)]
#[derive(Service)]
struct ExampleService {
    // Stored APIs
    api_registry: *const ApiRegistryApi,
    tt_api: *const TheTruthApi,
    tt_common_types: *const TheTruthCommonTypesApi,
    entity_api: *const EntityApi,

    // Stored memory for the lifetime of the service
    editor_aspects: Mutex<Vec<Box<CiEditorUiI>>>,
    components: Mutex<Vec<Box<ComponentI>>>,
    engines: Mutex<Vec<Box<EngineI>>>,
}

unsafe impl Send for ExampleService {}
unsafe impl Sync for ExampleService {}

impl ExampleService {
    fn new(plugin: &mut Plugin) -> Self {
        event!(Level::INFO, foo = 42, "Example logging with data.");

        plugin.add_implementation(&Self::THE_TRUTH_CREATE_TYPES_I);

        unsafe {
            // TODO: Convert these
            let registry = plugin.api_registry();

            (*registry).add_implementation(
                const_cstr!("tm_entity_create_component_i").as_ptr(),
                TM_ENTITY_CREATE_COMPONENT_I_VERSION,
                Self::component_create as *const c_void,
            );

            (*registry).add_implementation(
                const_cstr!("tm_entity_register_engines_simulation_i").as_ptr(),
                TM_ENTITY_REGISTER_ENGINES_SIMULATION_I_VERSION,
                Self::register_engines as *const c_void,
            );
        }

        ExampleService {
            api_registry: plugin.api_registry(),
            tt_api: plugin.get_api(),
            tt_common_types: plugin.get_api(),
            entity_api: plugin.get_api(),

            editor_aspects: Mutex::new(Vec::new()),
            components: Mutex::new(Vec::new()),
            engines: Mutex::new(Vec::new()),
        }
    }
}

impl Drop for ExampleService {
    fn drop(&mut self) {
        unsafe {
            (*self.api_registry).remove_implementation(
                const_cstr!("tm_entity_create_component_i").as_ptr(),
                TM_ENTITY_CREATE_COMPONENT_I_VERSION,
                Self::component_create as *const c_void,
            );

            (*self.api_registry).remove_implementation(
                const_cstr!("tm_entity_register_engines_simulation_i").as_ptr(),
                TM_ENTITY_REGISTER_ENGINES_SIMULATION_I_VERSION,
                Self::register_engines as *const c_void,
            );
        }
    }
}

#[tm_service_impl]
impl TheTruthCreateTypesI for ExampleService {
    unsafe fn the_truth_create_types(&self, tt: *mut TheTruthO) {
        event!(Level::INFO, "START");

        // The Machinery stores component data in "entity assets", which are then constructed into
        // real components at runtime.

        event!(Level::INFO, "Registering example truth types");

        (*self.tt_common_types).create_common_types(tt);

        // Create a the example component truth data
        let properties = TheTruthPropertyDefinitionT {
            name: const_cstr!("angular_velocity").as_ptr(),
            type_: TM_THE_TRUTH_PROPERTY_TYPE_SUBOBJECT,
            type_hash: TM_TT_TYPE_HASH__POSITION,
            ..Default::default()
        };

        let spin_type = (*self.tt_api).create_object_type(
            tt,
            RUST_EXAMPLE_COMPONENT.name.as_ptr(),
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

        event!(Level::INFO, "END");
    }
}

impl ExampleService {
    const THE_TRUTH_CREATE_TYPES_I: TheTruthCreateTypesI = Self::the_truth_create_types_wrapper;

    unsafe extern "C" fn the_truth_create_types_wrapper(tt: *mut TheTruthO) {
        event!(Level::INFO, "CALLBACK");
        ExampleService::the_truth_create_types(&*ExampleService::ptr(), tt);
    }
}

#[tm_service_export]
impl ExampleService {
    fn component_create(&self, ctx: *mut EntityContextO) {
        event!(Level::INFO, "Registering example components");

        unsafe {
            // Register the component for entities
            let component = Box::new(ComponentI {
                name: RUST_EXAMPLE_COMPONENT.name.as_ptr(),
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
        _manager: *mut ComponentManagerO,
        _commands: *mut EntityCommandsO,
        _e: EntityT,
        data: *mut ::std::os::raw::c_void,
        tt: *const TheTruthO,
        asset: TtIdT,
    ) -> bool {
        let component = data as *mut Vec3;

        unsafe {
            let object = (*self.tt_api).read(tt as *mut TheTruthO, asset);
            let data = (*self.tt_common_types).get_position(tt as *mut TheTruthO, object, 0);

            *component = Vec3::new(data.x, data.y, data.z);
        }

        true
    }

    fn register_engines(&self, ctx: *mut EntityContextO) {
        event!(Level::INFO, "Registering example engines");

        unsafe {
            // Lookup the component types we want to use in this system
            let example_component =
                (*self.entity_api).lookup_component_type(ctx, RUST_EXAMPLE_COMPONENT.hash);
            let transform_component =
                (*self.entity_api).lookup_component_type(ctx, TM_TT_TYPE_HASH__TRANSFORM_COMPONENT);

            // Register the spin engine
            let mut components = [ComponentTypeT::default(); 16];
            components[0] = example_component;
            components[1] = transform_component;

            let mut writes = [false; 16];
            writes[1] = true;

            let mut after_me = [StrhashT::default(); 16];
            after_me[0] = TM_ENGINE__SCENE_TREE;

            let spin_engine = Box::new(EngineI {
                super_: EngineSystemCommonI {
                    ui_name: const_cstr!("Rust Spin").as_ptr(),
                    hash: RUST_EXAMPLE_ENGINE.hash,
                    num_components: 2,
                    components,
                    writes,
                    after_me,
                    ..Default::default()
                },
                update: Some(Self::engine_spin_update),
                inst: ctx as *mut EngineO,
                ..Default::default()
            });
            (*self.entity_api).register_engine(ctx, spin_engine.as_ref());
            self.engines.lock().unwrap().push(spin_engine);
        }
    }

    fn engine_spin_update(
        &self,
        inst: *mut EngineO,
        data: *mut EngineUpdateSetT,
        _commands: *mut EntityCommandsO,
    ) {
        unsafe {
            let ctx = inst as *mut EntityContextO;

            let mut updated_entities = Vec::new();

            // Fetch the scene delta-time from the blackboard
            let mut delta = 1.0 / 60.0;
            let mut current = (*data).blackboard_start;
            while current != (*data).blackboard_end {
                if (*current).id.u64_ == TM_ENTITY_BB__DELTA_TIME.u64_ {
                    delta = (*current).__bindgen_anon_1.double_value as f32;
                }

                current = current.offset(1);
            }

            // Go through the components to update
            let start = (*data).arrays.as_ptr();
            for array_i in 0..(*data).num_arrays {
                let current_array = start.offset(array_i as isize);
                let examples = (*current_array).components[0] as *const Vec3;
                let transforms = (*current_array).components[1] as *mut TransformComponentT;

                for i in 0..(*current_array).n as isize {
                    let example = *examples.offset(i);
                    let transform = transforms.offset(i);

                    let mut local = rotor((*transform).local.rot);
                    let mut world = rotor((*transform).world.rot);

                    // Apply the rotation
                    let rotation = Rotor3::from_euler_angles(
                        example.z * delta,
                        example.x * delta,
                        example.y * delta,
                    );
                    local = local * rotation;
                    world = world * rotation;

                    // Update the transform with the new values
                    (*transform).local.rot = quaternion(local);
                    (*transform).world.rot = quaternion(world);
                    (*transform).version += 1;

                    updated_entities.push(*(*current_array).entities.offset(i));
                }
            }

            // Notify that the transform components on the given entities have changed
            (*self.entity_api).notify(
                ctx,
                (*(*data).engine).super_.components[1],
                updated_entities.as_ptr(),
                updated_entities.len() as u32,
            );
        }
    }
}

extern "C" fn component_category() -> *const c_char {
    const_cstr!("Samples").as_ptr()
}

const RUST_EXAMPLE_ENGINE: Identifier = tm_identifier!("tm_rust_example_engine");
const RUST_EXAMPLE_COMPONENT: Identifier = tm_identifier!("tm_rust_example_component");

fn rotor(input: Vec4T) -> Rotor3 {
    Rotor3::from_quaternion_array([input.x, input.y, input.z, input.w])
}

fn quaternion(input: Rotor3) -> Vec4T {
    let array = input.into_quaternion_array();
    Vec4T {
        x: array[0],
        y: array[1],
        z: array[2],
        w: array[3],
    }
}
