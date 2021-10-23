#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(clippy::all)]

pub mod foundation;
pub mod plugins;
pub mod the_machinery;

use std::ffi::c_void;

use const_cstr::{const_cstr, ConstCStr};
use foundation::VersionT;

/// Marker trait for APIs, to get static information required for lookup and versioning.
pub trait Api {
    const NAME: ConstCStr;
    const VERSION: VersionT;
}

/// Marker trait for Interfaces, to get static information required for lookup and versioning.
pub trait Interface {
    const NAME: ConstCStr;
    const VERSION: VersionT;

    fn to_registry_ptr(&self) -> *const c_void;
}

// Manually added types, to be generated later

#[repr(transparent)]
pub struct TheTruthCreateTypesI(pub unsafe extern "C" fn(tt: *mut foundation::TheTruthO));

impl Interface for TheTruthCreateTypesI {
    const NAME: ConstCStr = const_cstr!("tm_the_truth_create_types_i");
    const VERSION: VersionT = foundation::TM_THE_TRUTH_CREATE_TYPES_I_VERSION;

    fn to_registry_ptr(&self) -> *const c_void {
        self.0 as *const c_void
    }
}

#[repr(transparent)]
pub struct EntityCreateComponentI(
    pub unsafe extern "C" fn(ctx: *mut plugins::entity::EntityContextO),
);

impl Interface for EntityCreateComponentI {
    const NAME: ConstCStr = const_cstr!("tm_entity_create_component_i");
    const VERSION: VersionT = plugins::entity::TM_ENTITY_CREATE_COMPONENT_I_VERSION;

    fn to_registry_ptr(&self) -> *const c_void {
        self.0 as *const c_void
    }
}

#[repr(transparent)]
pub struct EntityRegisterEnginesSimulationI(
    pub unsafe extern "C" fn(ctx: *mut plugins::entity::EntityContextO),
);

impl Interface for EntityRegisterEnginesSimulationI {
    const NAME: ConstCStr = const_cstr!("tm_entity_register_engines_simulation_i");
    const VERSION: VersionT = plugins::entity::TM_ENTITY_REGISTER_ENGINES_SIMULATION_I_VERSION;

    fn to_registry_ptr(&self) -> *const c_void {
        self.0 as *const c_void
    }
}
