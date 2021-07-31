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
pub const TM_RUNNER_SETTINGS_API_NAME: &'static [u8; 23usize] = b"tm_runner_settings_api\0";
pub const TM_SIMULATE_CONTEXT_API_NAME: &'static [u8; 24usize] = b"tm_simulate_context_api\0";
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
pub struct SimulateContextO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct SimulateContextApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            allocator: *mut AllocatorI,
            entity_ctx: *mut EntityContextO,
        ) -> *mut SimulateContextO,
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(simulate_ctx: *mut SimulateContextO)>,
    pub camera: ::std::option::Option<
        unsafe extern "C" fn(simulate_ctx: *const SimulateContextO) -> EntityT,
    >,
    pub set_camera: ::std::option::Option<
        unsafe extern "C" fn(simulate_ctx: *mut SimulateContextO, camera: EntityT),
    >,
}

// Extensions generated by machinery-generator

use const_cstr::{const_cstr, ConstCStr};

use crate::foundation::*;
use crate::plugins::entity::*;

impl RunnerSettingsApi {
    pub unsafe fn create_truth_types(&self, tt: *mut TheTruthO) {
        self.create_truth_types.unwrap()(tt)
    }
}

impl crate::Api for RunnerSettingsApi {
    const NAME: ConstCStr = const_cstr!("tm_runner_settings_api");
}

impl SimulateContextApi {
    pub unsafe fn create(
        &self,
        allocator: *mut AllocatorI,
        entity_ctx: *mut EntityContextO,
    ) -> *mut SimulateContextO {
        self.create.unwrap()(allocator, entity_ctx)
    }

    pub unsafe fn destroy(&self, simulate_ctx: *mut SimulateContextO) {
        self.destroy.unwrap()(simulate_ctx)
    }

    pub unsafe fn camera(&self, simulate_ctx: *const SimulateContextO) -> EntityT {
        self.camera.unwrap()(simulate_ctx)
    }

    pub unsafe fn set_camera(&self, simulate_ctx: *mut SimulateContextO, camera: EntityT) {
        self.set_camera.unwrap()(simulate_ctx, camera)
    }
}

impl crate::Api for SimulateContextApi {
    const NAME: ConstCStr = const_cstr!("tm_simulate_context_api");
}

pub const TM_TT_TYPE_HASH__RUNNER_SETTINGS: StrhashT = StrhashT {
    u64_: 13458645607848451798u64,
};
pub const TM_ENTITY_BB__SIMULATE_CONTEXT: StrhashT = StrhashT {
    u64_: 8463886947231016997u64,
};
