#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const __SAL_H_VERSION: u32 = 180000000;
pub const __bool_true_false_are_defined: u32 = 1;
pub const TM_TT_TYPE__RUNNER_SETTINGS: &'static [u8; 19usize] = b"tm_runner_settings\0";
pub const TM_TT_TYPE__SIMULATION_ENTRY_SETTINGS: &'static [u8; 29usize] =
    b"tm_simulation_entry_settings\0";
pub const TM_TT_TYPE__SIMULATION_ENTRY_COMPONENT: &'static [u8; 30usize] =
    b"tm_simulation_entry_component\0";
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
pub type __vcrt_bool = bool;
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize);
}
extern "C" {
    pub static mut __security_cookie: usize;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union TtIdTBindgenTy1 {
    pub u64_: u64,
    pub __bindgen_anon_1: TtIdTBindgenTy1BindgenTy1,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Default, Copy, Clone)]
pub struct TtIdTBindgenTy1BindgenTy1 {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
impl TtIdTBindgenTy1BindgenTy1 {
    #[inline]
    pub fn type_(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 10u8) as u64) }
    }
    #[inline]
    pub fn set_type(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub fn generation(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 22u8) as u64) }
    }
    #[inline]
    pub fn set_generation(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(10usize, 22u8, val as u64)
        }
    }
    #[inline]
    pub fn index(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(32usize, 32u8) as u64) }
    }
    #[inline]
    pub fn set_index(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(32usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        type_: u64,
        generation: u64,
        index: u64,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 10u8, {
            let type_: u64 = unsafe { ::std::mem::transmute(type_) };
            type_ as u64
        });
        __bindgen_bitfield_unit.set(10usize, 22u8, {
            let generation: u64 = unsafe { ::std::mem::transmute(generation) };
            generation as u64
        });
        __bindgen_bitfield_unit.set(32usize, 32u8, {
            let index: u64 = unsafe { ::std::mem::transmute(index) };
            index as u64
        });
        __bindgen_bitfield_unit
    }
}
impl Default for TtIdTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_TT_PROP__RUNNER_SETTINGS__PROJECT: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__RUNNER_SETTINGS__WORLD_ENTITY: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__RUNNER_SETTINGS__SHADERS_DIR: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__RUNNER_SETTINGS__PLUGINS_DIR: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__RUNNER_SETTINGS__WINDOW_TITLE: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__RUNNER_SETTINGS__RESOLUTION: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__RUNNER_SETTINGS__FULLSCREEN: ::std::os::raw::c_int = 6;
pub type _bindgen_ty_1 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RunnerSettingsApi {
    pub create_truth_types: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union EntityT {
    pub __bindgen_anon_1: EntityTBindgenTy1,
    pub u64_: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct EntityTBindgenTy1 {
    pub index: u32,
    pub generation: u32,
}
impl Default for EntityT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NetworkNode {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MixerO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysxSceneSettingsT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SimulationO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct SimulationUpdateParamsT {
    pub update_when_hidden: bool,
    pub enable_ecs_editor_components: bool,
    pub enable_ecs_editor_engines: bool,
    pub passive_gamestate: bool,
    pub running_in_editor: bool,
    pub _padding_34: [::std::os::raw::c_char; 3usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SimulationNetworkParamsT {
    pub node_type_name: *const ::std::os::raw::c_char,
    pub port: u16,
    pub _padding_44: [::std::os::raw::c_char; 6usize],
}
impl Default for SimulationNetworkParamsT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SimulationPhysxParamsT {
    pub editor_mode: bool,
    pub _padding_53: [::std::os::raw::c_char; 7usize],
    pub settings: *mut PhysxSceneSettingsT,
}
impl Default for SimulationPhysxParamsT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SimulationParamsT {
    pub update: SimulationUpdateParamsT,
    pub network: SimulationNetworkParamsT,
    pub physx: SimulationPhysxParamsT,
}
impl Default for SimulationParamsT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct SimulationRenderingParamsT {
    pub ui: *mut UiO,
    pub display_scale_factor: f32,
    pub custom_ui_scale_factor: f32,
    pub content_rect: RectT,
    pub native_resolution: bool,
    pub _padding_72: [::std::os::raw::c_char; 7usize],
}
impl Default for SimulationRenderingParamsT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SimulationStateT {
    pub data: *mut u8,
    pub size: u32,
    pub _padding_81: [::std::os::raw::c_char; 4usize],
}
impl Default for SimulationStateT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct SimulationApi {
    pub all_simulations:
        ::std::option::Option<unsafe extern "C" fn(count: *mut u32) -> *mut *mut SimulationO>,
    pub single_simulation: ::std::option::Option<unsafe extern "C" fn() -> *mut SimulationO>,
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            parent_allocator: *mut AllocatorI,
            network: *mut NetworkO,
            mixer: *mut MixerO,
        ) -> *mut SimulationO,
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(simulate_ctx: *mut SimulationO)>,
    pub setup: ::std::option::Option<
        unsafe extern "C" fn(
            sim: *mut SimulationO,
            tt: *mut TheTruthO,
            asset_root: TtIdT,
            params: SimulationParamsT,
        ),
    >,
    pub setup_with_root_entity: ::std::option::Option<
        unsafe extern "C" fn(
            sim: *mut SimulationO,
            tt: *mut TheTruthO,
            asset_root: TtIdT,
            root_entity: TtIdT,
            params: SimulationParamsT,
        ),
    >,
    pub create_root_entity_from_asset: ::std::option::Option<
        unsafe extern "C" fn(sim: *mut SimulationO, entity_asset: TtIdT) -> EntityT,
    >,
    pub tick: ::std::option::Option<
        unsafe extern "C" fn(
            sim: *mut SimulationO,
            rendering_params: *mut SimulationRenderingParamsT,
            dt: f64,
        ),
    >,
    pub render_args: ::std::option::Option<
        unsafe extern "C" fn(sim: *mut SimulationO, render_args: *mut ViewerRenderArgsT),
    >,
    pub force_initialize: ::std::option::Option<unsafe extern "C" fn(sim: *mut SimulationO)>,
    pub set_camera: ::std::option::Option<
        unsafe extern "C" fn(simulate_sim: *mut SimulationO, camera: EntityT),
    >,
    pub camera: ::std::option::Option<unsafe extern "C" fn(sim: *const SimulationO) -> EntityT>,
    pub default_camera:
        ::std::option::Option<unsafe extern "C" fn(sim: *mut SimulationO) -> EntityT>,
    pub reset_default_camera:
        ::std::option::Option<unsafe extern "C" fn(sim: *mut SimulationO) -> EntityT>,
    pub entity_ctx:
        ::std::option::Option<unsafe extern "C" fn(sim: *const SimulationO) -> *mut EntityContextO>,
    pub network_node:
        ::std::option::Option<unsafe extern "C" fn(sim: *const SimulationO) -> *mut NetworkNodeO>,
    pub pause: ::std::option::Option<unsafe extern "C" fn(sim: *mut SimulationO)>,
    pub unpause: ::std::option::Option<unsafe extern "C" fn(sim: *mut SimulationO)>,
    pub is_paused: ::std::option::Option<unsafe extern "C" fn(sim: *mut SimulationO) -> bool>,
    pub params:
        ::std::option::Option<unsafe extern "C" fn(sim: *mut SimulationO) -> SimulationParamsT>,
    pub save_state: ::std::option::Option<
        unsafe extern "C" fn(sim: *mut SimulationO, allocator: *mut AllocatorI) -> SimulationStateT,
    >,
    pub load_state:
        ::std::option::Option<unsafe extern "C" fn(sim: *mut SimulationO, state: SimulationStateT)>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysxSceneO {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct SimulationFrameArgsT {
    pub commands: *mut EntityCommandsO,
    pub dt: f32,
    pub dt_unscaled: f32,
    pub time: f64,
    pub time_unscaled: f64,
    pub running_in_editor: bool,
    pub _padding_56: [::std::os::raw::c_char; 7usize],
    pub ui: *mut UiO,
    pub uistyle: *mut UiStyleT,
    pub rect: RectT,
    pub physx_scene: *mut PhysxSceneO,
}
impl Default for SimulationFrameArgsT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct SimulationStartArgsT {
    pub allocator: *mut AllocatorI,
    pub tt: *mut TheTruthO,
    pub entity_ctx: *mut EntityContextO,
    pub simulation_ctx: *mut SimulationO,
    pub asset_root: TtIdT,
    pub settings_id: TtIdT,
    pub commands: *mut EntityCommandsO,
    pub running_in_editor: bool,
    pub _padding_102: [::std::os::raw::c_char; 7usize],
}
impl Default for SimulationStartArgsT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SimulationStateO {
    _unused: [u8; 0],
}
pub const TM_MAX_DEPENDENCIES_FOR_SIMULATION_ENTRY: ::std::os::raw::c_int = 16;
pub type _bindgen_ty_2 = ::std::os::raw::c_int;
#[repr(C)]
pub struct SimulationEntryI {
    pub id: StrhashT,
    pub display_name: *const ::std::os::raw::c_char,
    pub settings_type_hash: StrhashT,
    pub start: ::std::option::Option<
        unsafe extern "C" fn(args: *mut SimulationStartArgsT) -> *mut SimulationStateO,
    >,
    pub stop: ::std::option::Option<
        unsafe extern "C" fn(state: *mut SimulationStateO, commands: *mut EntityCommandsO),
    >,
    pub tick: ::std::option::Option<
        unsafe extern "C" fn(state: *mut SimulationStateO, args: *mut SimulationFrameArgsT),
    >,
    pub hot_reload: ::std::option::Option<
        unsafe extern "C" fn(state: *mut SimulationStateO, commands: *mut EntityCommandsO),
    >,
    pub before_me: [StrhashT; 16usize],
    pub after_me: [StrhashT; 16usize],
    pub phase: StrhashT,
}
impl Default for SimulationEntryI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_TT_PROP__SIMULATION_ENTRY_SETTINGS__SETTING: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_3 = ::std::os::raw::c_int;
pub const TM_TT_PROP__SIMULATION_ENTRY_COMPONENT__IDS: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__SIMULATION_ENTRY_COMPONENT__SETTINGS: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_4 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct NetworkNodeO {
    pub _address: u8,
}

// Extensions generated by machinery-generator

use const_cstr::{const_cstr, ConstCStr};

use crate::foundation::VersionT;

use crate::foundation::*;
use crate::plugins::entity::*;
use crate::plugins::ui::*;

impl RunnerSettingsApi {
    pub unsafe fn create_truth_types(&self, tt: *mut TheTruthO) {
        self.create_truth_types.unwrap()(tt)
    }
}

impl crate::Api for RunnerSettingsApi {
    const NAME: ConstCStr = const_cstr!("tm_runner_settings_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl SimulationApi {
    pub unsafe fn all_simulations(&self, count: *mut u32) -> *mut *mut SimulationO {
        self.all_simulations.unwrap()(count)
    }

    pub unsafe fn single_simulation(&self) -> *mut SimulationO {
        self.single_simulation.unwrap()()
    }

    pub unsafe fn create(
        &self,
        parent_allocator: *mut AllocatorI,
        network: *mut NetworkO,
        mixer: *mut MixerO,
    ) -> *mut SimulationO {
        self.create.unwrap()(parent_allocator, network, mixer)
    }

    pub unsafe fn destroy(&self, simulate_ctx: *mut SimulationO) {
        self.destroy.unwrap()(simulate_ctx)
    }

    pub unsafe fn setup(
        &self,
        sim: *mut SimulationO,
        tt: *mut TheTruthO,
        asset_root: TtIdT,
        params: SimulationParamsT,
    ) {
        self.setup.unwrap()(sim, tt, asset_root, params)
    }

    pub unsafe fn setup_with_root_entity(
        &self,
        sim: *mut SimulationO,
        tt: *mut TheTruthO,
        asset_root: TtIdT,
        root_entity: TtIdT,
        params: SimulationParamsT,
    ) {
        self.setup_with_root_entity.unwrap()(sim, tt, asset_root, root_entity, params)
    }

    pub unsafe fn create_root_entity_from_asset(
        &self,
        sim: *mut SimulationO,
        entity_asset: TtIdT,
    ) -> EntityT {
        self.create_root_entity_from_asset.unwrap()(sim, entity_asset)
    }

    pub unsafe fn tick(
        &self,
        sim: *mut SimulationO,
        rendering_params: *mut SimulationRenderingParamsT,
        dt: f64,
    ) {
        self.tick.unwrap()(sim, rendering_params, dt)
    }

    pub unsafe fn render_args(&self, sim: *mut SimulationO, render_args: *mut ViewerRenderArgsT) {
        self.render_args.unwrap()(sim, render_args)
    }

    pub unsafe fn force_initialize(&self, sim: *mut SimulationO) {
        self.force_initialize.unwrap()(sim)
    }

    pub unsafe fn set_camera(&self, simulate_sim: *mut SimulationO, camera: EntityT) {
        self.set_camera.unwrap()(simulate_sim, camera)
    }

    pub unsafe fn camera(&self, sim: *const SimulationO) -> EntityT {
        self.camera.unwrap()(sim)
    }

    pub unsafe fn default_camera(&self, sim: *mut SimulationO) -> EntityT {
        self.default_camera.unwrap()(sim)
    }

    pub unsafe fn reset_default_camera(&self, sim: *mut SimulationO) -> EntityT {
        self.reset_default_camera.unwrap()(sim)
    }

    pub unsafe fn entity_ctx(&self, sim: *const SimulationO) -> *mut EntityContextO {
        self.entity_ctx.unwrap()(sim)
    }

    pub unsafe fn network_node(&self, sim: *const SimulationO) -> *mut NetworkNodeO {
        self.network_node.unwrap()(sim)
    }

    pub unsafe fn pause(&self, sim: *mut SimulationO) {
        self.pause.unwrap()(sim)
    }

    pub unsafe fn unpause(&self, sim: *mut SimulationO) {
        self.unpause.unwrap()(sim)
    }

    pub unsafe fn is_paused(&self, sim: *mut SimulationO) -> bool {
        self.is_paused.unwrap()(sim)
    }

    pub unsafe fn params(&self, sim: *mut SimulationO) -> SimulationParamsT {
        self.params.unwrap()(sim)
    }

    pub unsafe fn save_state(
        &self,
        sim: *mut SimulationO,
        allocator: *mut AllocatorI,
    ) -> SimulationStateT {
        self.save_state.unwrap()(sim, allocator)
    }

    pub unsafe fn load_state(&self, sim: *mut SimulationO, state: SimulationStateT) {
        self.load_state.unwrap()(sim, state)
    }
}

impl crate::Api for SimulationApi {
    const NAME: ConstCStr = const_cstr!("tm_simulation_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

pub const TM_TT_TYPE_HASH__RUNNER_SETTINGS: StrhashT = StrhashT {
    u64_: 13458645607848451798u64,
};
pub const TM_ENTITY_BB__SIMULATION: StrhashT = StrhashT {
    u64_: 13615359465719222455u64,
};
pub const TM_ENTITY_BB__SIMULATION_LOADING_FROM_GAMESTATE: StrhashT = StrhashT {
    u64_: 14379022613120087788u64,
};
pub const TM_TT_TYPE_HASH__SIMULATION_ENTRY_SETTINGS: StrhashT = StrhashT {
    u64_: 7106079933671888072u64,
};
pub const TM_TT_TYPE_HASH__SIMULATION_ENTRY_COMPONENT: StrhashT = StrhashT {
    u64_: 10769537077919379344u64,
};
pub const TM_RUNNER_SETTINGS_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_SIMULATION_ENTRY_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_SIMULATION_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
