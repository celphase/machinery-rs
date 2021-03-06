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
pub const _VCRT_COMPILER_PREPROCESSOR: u32 = 1;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_NODISCARD: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub const TM_USE_STRHASH_TYPE: u32 = 1;
pub const TM_PAGE_SIZE: u32 = 4096;
pub const TM_JSON_ERROR_STRING_LENGTH: u32 = 79;
pub const TM_DIRSEP: &'static [u8; 2usize] = b"/\0";
pub const TM_MEMORY_TRACKER_SCOPE__NONE: u32 = 4294967295;
pub const TM_DIRECTORY_SEPARATOR: u8 = 47u8;
pub const TM_TT_TYPE__PLUGIN: &'static [u8; 10usize] = b"tm_plugin\0";
pub const SHA1_DIGEST_SIZE: u32 = 20;
pub const TM_TT_TYPE__ANYTHING: &'static [u8; 12usize] = b"tm_anything\0";
pub const TM_THE_TRUTH_PROPERTY_NAME_LENGTH: u32 = 63;
pub const TM_TT_TYPE__ASSET_ROOT: &'static [u8; 14usize] = b"tm_asset_root\0";
pub const TM_TT_TYPE__ASSET: &'static [u8; 9usize] = b"tm_asset\0";
pub const TM_TT_TYPE__ASSET_DIRECTORY: &'static [u8; 19usize] = b"tm_asset_directory\0";
pub const TM_TT_TYPE__ASSET_LABEL: &'static [u8; 15usize] = b"tm_asset_label\0";
pub const TM_TT_TYPE__ASSET_THUMBNAIL: &'static [u8; 19usize] = b"tm_asset_thumbnail\0";
pub const TM_TT_TYPE__BOOL: &'static [u8; 8usize] = b"tm_bool\0";
pub const TM_TT_TYPE__UINT32_T: &'static [u8; 12usize] = b"tm_uint32_t\0";
pub const TM_TT_TYPE__UINT64_T: &'static [u8; 12usize] = b"tm_uint64_t\0";
pub const TM_TT_TYPE__FLOAT: &'static [u8; 9usize] = b"tm_float\0";
pub const TM_TT_TYPE__DOUBLE: &'static [u8; 10usize] = b"tm_double\0";
pub const TM_TT_TYPE__STRING: &'static [u8; 10usize] = b"tm_string\0";
pub const TM_TT_TYPE__VEC2: &'static [u8; 10usize] = b"tm_vec2_t\0";
pub const TM_TT_TYPE__VEC3: &'static [u8; 10usize] = b"tm_vec3_t\0";
pub const TM_TT_TYPE__VEC4: &'static [u8; 10usize] = b"tm_vec4_t\0";
pub const TM_TT_TYPE__POSITION: &'static [u8; 12usize] = b"tm_position\0";
pub const TM_TT_TYPE__ROTATION: &'static [u8; 12usize] = b"tm_rotation\0";
pub const TM_TT_TYPE__SCALE: &'static [u8; 9usize] = b"tm_scale\0";
pub const TM_TT_TYPE__COLOR_RGB: &'static [u8; 13usize] = b"tm_color_rgb\0";
pub const TM_TT_TYPE__COLOR_RGBA: &'static [u8; 14usize] = b"tm_color_rgba\0";
pub const TM_TT_TYPE__RECT: &'static [u8; 10usize] = b"tm_rect_t\0";
pub const TM_TT_TYPE__UUID: &'static [u8; 10usize] = b"tm_uuid_t\0";
pub const TM_TT_TYPE__VISIBILITY_FLAG: &'static [u8; 19usize] = b"tm_visibility_flag\0";
pub type va_list = *mut ::std::os::raw::c_char;
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
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_longlong;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulonglong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_int;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_uint;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct Vec2T {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct Vec3T {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct Vec4T {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct Mat44T {
    pub xx: f32,
    pub xy: f32,
    pub xz: f32,
    pub xw: f32,
    pub yx: f32,
    pub yy: f32,
    pub yz: f32,
    pub yw: f32,
    pub zx: f32,
    pub zy: f32,
    pub zz: f32,
    pub zw: f32,
    pub wx: f32,
    pub wy: f32,
    pub wz: f32,
    pub ww: f32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct TransformT {
    pub pos: Vec3T,
    pub rot: Vec4T,
    pub scl: Vec3T,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RectT {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StrT {
    pub data: *const ::std::os::raw::c_char,
    pub size: u32,
    pub null_terminated: u32,
}
impl Default for StrT {
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
pub struct ClockO {
    pub opaque: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct UuidT {
    pub a: u64,
    pub b: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ColorSrgbT {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct TtTypeT {
    pub u64_: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TtIdT {
    pub __bindgen_anon_1: TtIdTBindgenTy1,
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
impl Default for TtIdT {
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
pub struct TtUndoScopeT {
    pub u64_: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct VersionT {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct StrhashT {
    pub u64_: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AllocatorO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AllocatorI {
    pub inst: *mut AllocatorO,
    pub mem_scope: u32,
    pub _padding_16: [::std::os::raw::c_char; 4usize],
    pub realloc: ::std::option::Option<
        unsafe extern "C" fn(
            a: *mut AllocatorI,
            ptr: *mut ::std::os::raw::c_void,
            old_size: u64,
            new_size: u64,
            file: *const ::std::os::raw::c_char,
            line: u32,
        ) -> *mut ::std::os::raw::c_void,
    >,
}
impl Default for AllocatorI {
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
pub struct AllocatorStatisticsT {
    pub system_allocation_count: u64,
    pub system_allocated_bytes: u64,
    pub vm_reserved: u64,
    pub vm_committed: u64,
    pub system_churn_allocation_count: u64,
    pub system_churn_allocated_bytes: u64,
    pub vm_churn_committed: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AllocatorApi {
    pub system: *mut AllocatorI,
    pub end_of_page: *mut AllocatorI,
    pub vm: *mut AllocatorI,
    pub statistics: *mut AllocatorStatisticsT,
    pub create_child: ::std::option::Option<
        unsafe extern "C" fn(
            parent: *const AllocatorI,
            desc: *const ::std::os::raw::c_char,
        ) -> AllocatorI,
    >,
    pub destroy_child: ::std::option::Option<unsafe extern "C" fn(child: *const AllocatorI)>,
    pub destroy_child_allowing_leaks: ::std::option::Option<
        unsafe extern "C" fn(child: *const AllocatorI, max_leaked_bytes: u64),
    >,
    pub create_leaky_root_scope: ::std::option::Option<
        unsafe extern "C" fn(
            parent: *const AllocatorI,
            desc: *const ::std::os::raw::c_char,
        ) -> AllocatorI,
    >,
    pub create_fixed_vm: ::std::option::Option<
        unsafe extern "C" fn(reserve_size: u64, mem_scope: u32) -> AllocatorI,
    >,
}
impl Default for AllocatorApi {
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
pub struct ApiRegistryListenerI {
    pub ud: *mut ::std::os::raw::c_void,
    pub add_implementation: ::std::option::Option<
        unsafe extern "C" fn(
            ud: *mut ::std::os::raw::c_void,
            name: *const ::std::os::raw::c_char,
            version: VersionT,
            implementation: *const ::std::os::raw::c_void,
        ),
    >,
}
impl Default for ApiRegistryListenerI {
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
pub struct ApiRegistryApi {
    pub api_registry_version: ::std::option::Option<unsafe extern "C" fn() -> VersionT>,
    pub set: ::std::option::Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            version: VersionT,
            api: *const ::std::os::raw::c_void,
            bytes: u32,
        ),
    >,
    pub remove: ::std::option::Option<unsafe extern "C" fn(api: *const ::std::os::raw::c_void)>,
    pub get: ::std::option::Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            version: VersionT,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_optional: ::std::option::Option<
        unsafe extern "C" fn(
            api: *mut *mut ::std::os::raw::c_void,
            name: *const ::std::os::raw::c_char,
            version: VersionT,
        ),
    >,
    pub version:
        ::std::option::Option<unsafe extern "C" fn(api: *mut ::std::os::raw::c_void) -> VersionT>,
    pub add_implementation: ::std::option::Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            version: VersionT,
            implementation: *const ::std::os::raw::c_void,
        ),
    >,
    pub remove_implementation: ::std::option::Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            version: VersionT,
            implementation: *const ::std::os::raw::c_void,
        ),
    >,
    pub implementations: ::std::option::Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            version: VersionT,
        ) -> *mut *mut ::std::os::raw::c_void,
    >,
    pub num_implementations: ::std::option::Option<
        unsafe extern "C" fn(name: *const ::std::os::raw::c_char, version: VersionT) -> u32,
    >,
    pub first_implementation: ::std::option::Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            version: VersionT,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub single_implementation: ::std::option::Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            version: VersionT,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub add_listener:
        ::std::option::Option<unsafe extern "C" fn(listener: *const ApiRegistryListenerI)>,
    pub static_variable: ::std::option::Option<
        unsafe extern "C" fn(
            id: StrhashT,
            size: u32,
            file: *const ::std::os::raw::c_char,
            line: u32,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub begin_context:
        ::std::option::Option<unsafe extern "C" fn(name: *const ::std::os::raw::c_char)>,
    pub end_context:
        ::std::option::Option<unsafe extern "C" fn(name: *const ::std::os::raw::c_char)>,
    pub disable_apis_missing_dependencies: ::std::option::Option<unsafe extern "C" fn()>,
    pub available_versions: ::std::option::Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            ta: *mut TempAllocatorI,
        ) -> *mut VersionT,
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UiO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ColorSpaceDescT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ApplicationO {
    _unused: [u8; 0],
}
pub type ApplicationModalF = ::std::option::Option<
    unsafe extern "C" fn(
        data: *mut ::std::os::raw::c_void,
        ui: *mut UiO,
        rect: RectT,
        font: *const Draw2dFontT,
        font_scale: f32,
        delta_time: f32,
    ) -> bool,
>;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ApplicationApi {
    pub application: ::std::option::Option<unsafe extern "C" fn() -> *mut ApplicationO>,
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            argc: ::std::os::raw::c_int,
            argv: *mut *mut ::std::os::raw::c_char,
        ) -> *mut ApplicationO,
    >,
    pub tick: ::std::option::Option<unsafe extern "C" fn(app: *mut ApplicationO) -> bool>,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(app: *mut ApplicationO)>,
    pub set_modal: ::std::option::Option<
        unsafe extern "C" fn(
            app: *mut ApplicationO,
            f: ApplicationModalF,
            data: *mut ::std::os::raw::c_void,
        ),
    >,
    pub is_modal: ::std::option::Option<unsafe extern "C" fn(app: *const ApplicationO) -> bool>,
    pub asset_root: ::std::option::Option<unsafe extern "C" fn(app: *const ApplicationO) -> TtIdT>,
    pub load_core: ::std::option::Option<unsafe extern "C" fn(app: *const ApplicationO) -> bool>,
    pub update_core: ::std::option::Option<
        unsafe extern "C" fn(app: *const ApplicationO, include_skipped: bool) -> bool,
    >,
    pub exit: ::std::option::Option<unsafe extern "C" fn(app: *mut ApplicationO)>,
    pub set_cursor_hidden:
        ::std::option::Option<unsafe extern "C" fn(app: *mut ApplicationO, hidden: bool)>,
    pub viewer_manager:
        ::std::option::Option<unsafe extern "C" fn(app: *mut ApplicationO) -> *mut ViewerManagerO>,
    pub default_render_pipeline_api: ::std::option::Option<
        unsafe extern "C" fn(app: *mut ApplicationO) -> *mut RenderPipelineVt,
    >,
    pub custom_ui_scale_factor:
        ::std::option::Option<unsafe extern "C" fn(app: *mut ApplicationO) -> f32>,
    pub display_scale_factor:
        ::std::option::Option<unsafe extern "C" fn(app: *mut ApplicationO, ui: *mut UiO) -> f32>,
    pub data_dir: ::std::option::Option<
        unsafe extern "C" fn(app: *mut ApplicationO) -> *const ::std::os::raw::c_char,
    >,
    pub color_space: ::std::option::Option<
        unsafe extern "C" fn(
            app: *const ApplicationO,
            color_space: *mut ColorSpaceDescT,
            format: *mut u32,
        ),
    >,
    pub network:
        ::std::option::Option<unsafe extern "C" fn(app: *mut ApplicationO) -> *mut NetworkO>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TheTruthO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AssetDatabaseConfigT {
    pub allocator: *mut AllocatorI,
    pub fs: *mut OsFileSystemApi,
    pub file_io: *mut OsFileIoApi,
}
impl Default for AssetDatabaseConfigT {
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
pub struct AssetDabaseSavedItemT {
    pub uuid: UuidT,
    pub is_directory: bool,
    pub _padding_30: [::std::os::raw::c_char; 7usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AssetDatabaseO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct AssetDatabaseApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            file: *const ::std::os::raw::c_char,
            tt: *mut TheTruthO,
            config: *const AssetDatabaseConfigT,
        ) -> *mut AssetDatabaseO,
    >,
    pub open: ::std::option::Option<
        unsafe extern "C" fn(
            file: *const ::std::os::raw::c_char,
            tt: *mut TheTruthO,
            config: *const AssetDatabaseConfigT,
        ) -> *mut AssetDatabaseO,
    >,
    pub close: ::std::option::Option<unsafe extern "C" fn(db: *mut AssetDatabaseO)>,
    pub save_modified:
        ::std::option::Option<unsafe extern "C" fn(db: *mut AssetDatabaseO, asset_root: TtIdT)>,
    pub save_modified_except: ::std::option::Option<
        unsafe extern "C" fn(
            db: *mut AssetDatabaseO,
            asset_root: TtIdT,
            ignore: *mut TtIdT,
            num_ignore: u32,
        ),
    >,
    pub save_asset:
        ::std::option::Option<unsafe extern "C" fn(db: *mut AssetDatabaseO, asset: TtIdT)>,
    pub delete_asset:
        ::std::option::Option<unsafe extern "C" fn(db: *mut AssetDatabaseO, asset: TtIdT)>,
    pub revert_asset: ::std::option::Option<
        unsafe extern "C" fn(
            db: *mut AssetDatabaseO,
            asset: TtIdT,
            undo_scope: TtUndoScopeT,
        ) -> bool,
    >,
    pub load: ::std::option::Option<
        unsafe extern "C" fn(db: *mut AssetDatabaseO, load_fraction: *mut f32) -> TtIdT,
    >,
    pub saved_name: ::std::option::Option<
        unsafe extern "C" fn(db: *mut AssetDatabaseO, item: UuidT) -> *const ::std::os::raw::c_char,
    >,
    pub saved_directory:
        ::std::option::Option<unsafe extern "C" fn(db: *mut AssetDatabaseO, item: UuidT) -> UuidT>,
    pub saved_version:
        ::std::option::Option<unsafe extern "C" fn(db: *mut AssetDatabaseO, asset: UuidT) -> u64>,
    pub all_saved_items: ::std::option::Option<
        unsafe extern "C" fn(
            db: *mut AssetDatabaseO,
            ta: *mut TempAllocatorI,
        ) -> *mut AssetDabaseSavedItemT,
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AssetIoImport {
    pub allocator: *mut AllocatorI,
    pub tt: *mut TheTruthO,
    pub reimport_into: TtIdT,
    pub target_dir: TtIdT,
    pub asset_root: TtIdT,
    pub ui: *mut UiO,
    pub asset_browser: TtIdT,
    pub asset_browser_version_at_start: u64,
    pub undo_scope: TtUndoScopeT,
}
impl Default for AssetIoImport {
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
pub struct AssetIoI {
    pub inst: *mut AssetIoO,
    pub enabled: ::std::option::Option<unsafe extern "C" fn(inst: *mut AssetIoO) -> bool>,
    pub can_import: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut AssetIoO, extension: *const ::std::os::raw::c_char) -> bool,
    >,
    pub can_reimport: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut AssetIoO, tt: *mut TheTruthO, asset: TtIdT) -> bool,
    >,
    pub importer_extensions_string: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetIoO,
            output: *mut *mut ::std::os::raw::c_char,
            ta: *mut TempAllocatorI,
            separator: *const ::std::os::raw::c_char,
        ),
    >,
    pub importer_description_string: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetIoO,
            output: *mut *mut ::std::os::raw::c_char,
            ta: *mut TempAllocatorI,
            separator: *const ::std::os::raw::c_char,
        ),
    >,
    pub import_asset: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetIoO,
            file: *const ::std::os::raw::c_char,
            import: *const AssetIoImport,
        ) -> u64,
    >,
    pub can_export: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut AssetIoO, tt: *mut TheTruthO, asset: TtIdT) -> bool,
    >,
    pub exporter_extension: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetIoO,
            tt: *mut TheTruthO,
            asset: TtIdT,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub export_asset: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetIoO,
            file: *const ::std::os::raw::c_char,
            tt: *mut TheTruthO,
            asset_root: TtIdT,
            asset: TtIdT,
        ),
    >,
}
impl Default for AssetIoI {
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
pub struct AssetIoApi {
    pub add_asset_io: ::std::option::Option<unsafe extern "C" fn(loader: *mut AssetIoI)>,
    pub remove_asset_io: ::std::option::Option<unsafe extern "C" fn(loader: *mut AssetIoI)>,
    pub importer: ::std::option::Option<
        unsafe extern "C" fn(extension: *const ::std::os::raw::c_char) -> *mut AssetIoI,
    >,
    pub reimporter: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, asset: TtIdT) -> *mut AssetIoI,
    >,
    pub exporter: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, asset: TtIdT) -> *mut AssetIoI,
    >,
    pub io_interfaces:
        ::std::option::Option<unsafe extern "C" fn(interfaces: *mut *mut AssetIoI) -> u32>,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct Base64Api {
    pub encoded_size: ::std::option::Option<unsafe extern "C" fn(raw_size: u64) -> u64>,
    pub encode: ::std::option::Option<
        unsafe extern "C" fn(
            encoded: *mut ::std::os::raw::c_char,
            raw: *const u8,
            raw_size: u64,
        ) -> u64,
    >,
    pub decoded_size: ::std::option::Option<
        unsafe extern "C" fn(encoded: *const ::std::os::raw::c_char, encoded_size: u64) -> u64,
    >,
    pub decode: ::std::option::Option<
        unsafe extern "C" fn(
            raw: *mut u8,
            encoded: *const ::std::os::raw::c_char,
            encoded_size: u64,
        ) -> u64,
    >,
}
pub const TM_BOUNDING_VOLUME_TYPE_SPHERE: BoundingVolumeType = 0;
pub const TM_BOUNDING_VOLUME_TYPE_BOX: BoundingVolumeType = 1;
pub type BoundingVolumeType = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct BoundingVolumeSphereT {
    pub visibility_mask: u64,
    pub culling_disabled: bool,
    pub _padding_18: [::std::os::raw::c_char; 3usize],
    pub position: Vec3T,
    pub radius: f32,
    pub _padding_21: [::std::os::raw::c_char; 4usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct BoundingVolumeBoxT {
    pub visibility_mask: u64,
    pub culling_disabled: bool,
    pub _padding_29: [::std::os::raw::c_char; 3usize],
    pub tm: Mat44T,
    pub min: Vec3T,
    pub max: Vec3T,
    pub _padding_33: [::std::os::raw::c_char; 4usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct BuddyAllocatorRawApi {
    pub init:
        ::std::option::Option<unsafe extern "C" fn(buffer: *mut u8, size: u32, block_size: u32)>,
    pub realloc: ::std::option::Option<
        unsafe extern "C" fn(
            buffer: *mut u8,
            ptr: *mut ::std::os::raw::c_void,
            old_size: u64,
            new_size: u64,
        ) -> *mut ::std::os::raw::c_void,
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct BuddyAllocatorApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            backing: *mut AllocatorI,
            initial_size: u32,
            block_size: u32,
        ) -> *mut AllocatorI,
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(a: *mut AllocatorI)>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BuffersO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BuffersI {
    pub inst: *mut BuffersO,
    pub allocate: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut BuffersO,
            size: u64,
            initialize: *const ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub add: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut BuffersO,
            data: *const ::std::os::raw::c_void,
            size: u64,
            hash: u64,
        ) -> u32,
    >,
    pub retain: ::std::option::Option<unsafe extern "C" fn(inst: *mut BuffersO, id: u32)>,
    pub release: ::std::option::Option<unsafe extern "C" fn(inst: *mut BuffersO, id: u32)>,
    pub get: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *const BuffersO,
            id: u32,
            size: *mut u64,
        ) -> *const ::std::os::raw::c_void,
    >,
    pub size: ::std::option::Option<unsafe extern "C" fn(inst: *const BuffersO, id: u32) -> u64>,
    pub hash: ::std::option::Option<unsafe extern "C" fn(inst: *const BuffersO, id: u32) -> u64>,
    pub lookup:
        ::std::option::Option<unsafe extern "C" fn(inst: *const BuffersO, hash: u64) -> u32>,
    pub debug__refcount:
        ::std::option::Option<unsafe extern "C" fn(inst: *const BuffersO, id: u32) -> u32>,
}
impl Default for BuffersI {
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
pub struct StreamableBuffersI {
    pub super_: BuffersI,
    pub map: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut BuffersO,
            path: *const ::std::os::raw::c_char,
            offset: u64,
            size: u64,
            hash: u64,
        ) -> u32,
    >,
    pub map_database: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut BuffersO,
            hash: u64,
            size: u64,
            file: *const FileO,
            page_size: u32,
            page_header_size: u32,
            first_page: u32,
        ) -> u32,
    >,
    pub is_mapped:
        ::std::option::Option<unsafe extern "C" fn(inst: *const BuffersO, id: u32) -> bool>,
    pub is_loaded:
        ::std::option::Option<unsafe extern "C" fn(inst: *const BuffersO, id: u32) -> bool>,
    pub unload: ::std::option::Option<unsafe extern "C" fn(inst: *mut BuffersO, id: u32)>,
    pub save: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut BuffersO,
            id: u32,
            path: *const ::std::os::raw::c_char,
            offset: u64,
        ),
    >,
    pub background_load_all:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut BuffersO, percentage: *mut f32)>,
    pub ensure_all_loaded: ::std::option::Option<unsafe extern "C" fn(inst: *mut BuffersO)>,
    pub set_io:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut BuffersO, io: *mut OsFileIoApi)>,
}
impl Default for StreamableBuffersI {
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
pub struct BuffersApi {
    pub create: ::std::option::Option<unsafe extern "C" fn(a: *mut AllocatorI) -> *mut BuffersI>,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(i: *mut BuffersI)>,
    pub create_streamable: ::std::option::Option<
        unsafe extern "C" fn(a: *mut AllocatorI, io: *mut OsFileIoApi) -> *mut StreamableBuffersI,
    >,
    pub destroy_streamable: ::std::option::Option<unsafe extern "C" fn(i: *mut StreamableBuffersI)>,
}
pub const TM_BUFFER_COMPONENT_TYPE_FLOAT: BufferComponentType = 0;
pub const TM_BUFFER_COMPONENT_TYPE_NORMALIZED_INTEGER: BufferComponentType = 1;
pub const TM_BUFFER_COMPONENT_TYPE_INTEGER: BufferComponentType = 2;
pub const TM_BUFFER_COMPONENT_TYPE_DEPTH_STENCIL: BufferComponentType = 3;
pub type BufferComponentType = ::std::os::raw::c_int;
pub const TM_BUFFER_COMPRESSION_FORMAT_BC1_RGB: BufferCompressionFormat = 0;
pub const TM_BUFFER_COMPRESSION_FORMAT_BC1_RGBA: BufferCompressionFormat = 1;
pub const TM_BUFFER_COMPRESSION_FORMAT_BC2: BufferCompressionFormat = 2;
pub const TM_BUFFER_COMPRESSION_FORMAT_BC3: BufferCompressionFormat = 3;
pub const TM_BUFFER_COMPRESSION_FORMAT_BC4: BufferCompressionFormat = 4;
pub const TM_BUFFER_COMPRESSION_FORMAT_BC5: BufferCompressionFormat = 5;
pub const TM_BUFFER_COMPRESSION_FORMAT_BC6H_U: BufferCompressionFormat = 6;
pub const TM_BUFFER_COMPRESSION_FORMAT_BC6H_S: BufferCompressionFormat = 7;
pub const TM_BUFFER_COMPRESSION_FORMAT_BC7: BufferCompressionFormat = 8;
pub const TM_BUFFER_COMPRESSION_FORMAT_ETC2_RGB: BufferCompressionFormat = 9;
pub const TM_BUFFER_COMPRESSION_FORMAT_ETC2_RGBA: BufferCompressionFormat = 10;
pub const TM_BUFFER_COMPRESSION_FORMAT_MAX_FORMATS: BufferCompressionFormat = 11;
pub type BufferCompressionFormat = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct BufferFormatApi {
    pub encode_uncompressed_format: ::std::option::Option<
        unsafe extern "C" fn(
            component_type: BufferComponentType,
            sign: bool,
            bits_x: u8,
            bits_y: u8,
            bits_z: u8,
            bits_w: u8,
        ) -> u32,
    >,
    pub encode_compressed_format: ::std::option::Option<
        unsafe extern "C" fn(compression_type: BufferCompressionFormat) -> u32,
    >,
    pub is_compressed: ::std::option::Option<unsafe extern "C" fn(format: u32) -> bool>,
    pub decode_uncompressed: ::std::option::Option<
        unsafe extern "C" fn(
            format: u32,
            component_type: *mut BufferComponentType,
            sign: *mut bool,
            bits_x: *mut u8,
            bits_y: *mut u8,
            bits_z: *mut u8,
            bits_w: *mut u8,
        ) -> bool,
    >,
    pub decode_compression_format: ::std::option::Option<
        unsafe extern "C" fn(format: u32, compression_format: *mut BufferCompressionFormat) -> bool,
    >,
    pub bits_per_element: ::std::option::Option<unsafe extern "C" fn(format: u32) -> u32>,
    pub num_components: ::std::option::Option<unsafe extern "C" fn(format: u32) -> u32>,
    pub human_readable: ::std::option::Option<
        unsafe extern "C" fn(format: u32, ta: *mut TempAllocatorI) -> *const ::std::os::raw::c_char,
    >,
}
pub const TM_CAMERA_TRANSFORM_DEFAULT: CameraTransform = 0;
pub const TM_CAMERA_TRANSFORM_EYE_LEFT: CameraTransform = 1;
pub const TM_CAMERA_TRANSFORM_EYE_RIGHT: CameraTransform = 2;
pub const TM_CAMERA_TRANSFORM_MAX_TRANSFORMS: CameraTransform = 3;
pub type CameraTransform = ::std::os::raw::c_int;
pub const TM_CAMERA_MODE_PERSPECTIVE: CameraMode = 0;
pub const TM_CAMERA_MODE_ORTHOGRAPHIC: CameraMode = 1;
pub type CameraMode = ::std::os::raw::c_int;
pub const TM_CAMERA_FRUSTUM_PLANE_LEFT: CameraFrustumPlanes = 0;
pub const TM_CAMERA_FRUSTUM_PLANE_RIGHT: CameraFrustumPlanes = 1;
pub const TM_CAMERA_FRUSTUM_PLANE_BOTTOM: CameraFrustumPlanes = 2;
pub const TM_CAMERA_FRUSTUM_PLANE_TOP: CameraFrustumPlanes = 3;
pub const TM_CAMERA_FRUSTUM_PLANE_NEAR: CameraFrustumPlanes = 4;
pub const TM_CAMERA_FRUSTUM_PLANE_FAR: CameraFrustumPlanes = 5;
pub const TM_CAMERA_FRUSTUM_PLANE_MAX_PLANES: CameraFrustumPlanes = 6;
pub type CameraFrustumPlanes = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CameraSettingsT {
    pub mode: CameraMode,
    pub near_plane: f32,
    pub far_plane: f32,
    pub __bindgen_anon_1: CameraSettingsTBindgenTy1,
    pub shutter_speed: f32,
    pub aperture: f32,
    pub iso: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CameraSettingsTBindgenTy1 {
    pub vertical_fov: f32,
    pub box_height: f32,
}
impl Default for CameraSettingsTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for CameraSettingsT {
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
pub struct CameraT {
    pub projection: [Mat44T; 3usize],
    pub view: [Mat44T; 3usize],
    pub settings: CameraSettingsT,
}
impl Default for CameraT {
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
pub struct CameraApi {
    pub view_from_transform: ::std::option::Option<
        unsafe extern "C" fn(view: *mut Mat44T, tm: *const TransformT) -> *mut Mat44T,
    >,
    pub view_from_lookin: ::std::option::Option<
        unsafe extern "C" fn(
            view: *mut Mat44T,
            position: Vec3T,
            forward: Vec3T,
            up: Vec3T,
        ) -> *mut Mat44T,
    >,
    pub transform_from_view: ::std::option::Option<
        unsafe extern "C" fn(tm: *mut TransformT, view: *const Mat44T) -> *mut TransformT,
    >,
    pub projection_from_frustum: ::std::option::Option<
        unsafe extern "C" fn(
            proj: *mut Mat44T,
            left: f32,
            right: f32,
            bottom: f32,
            top: f32,
            near: f32,
            far: f32,
        ) -> *mut Mat44T,
    >,
    pub matrices_from_eyes: ::std::option::Option<
        unsafe extern "C" fn(
            camera: *mut CameraT,
            head_tm: *const Mat44T,
            head_to_left_eye: *const Mat44T,
            head_to_right_eye: *const Mat44T,
            left_eye_left: f32,
            left_eye_right: f32,
            left_eye_top: f32,
            left_eye_bottom: f32,
            right_eye_left: f32,
            right_eye_right: f32,
            right_eye_top: f32,
            right_eye_bottom: f32,
        ),
    >,
    pub projection_from_fov: ::std::option::Option<
        unsafe extern "C" fn(
            proj: *mut Mat44T,
            near_plane: f32,
            far_plane: f32,
            vertical_fov: f32,
            aspect: f32,
        ) -> *mut Mat44T,
    >,
    pub orthographic_from_frustum: ::std::option::Option<
        unsafe extern "C" fn(
            proj: *mut Mat44T,
            left: f32,
            right: f32,
            bottom: f32,
            top: f32,
            near: f32,
            far: f32,
        ) -> *mut Mat44T,
    >,
    pub orthographics_from_dimensions: ::std::option::Option<
        unsafe extern "C" fn(
            proj: *mut Mat44T,
            near_plane: f32,
            far_plane: f32,
            width: f32,
            height: f32,
        ) -> *mut Mat44T,
    >,
    pub projection_from_camera: ::std::option::Option<
        unsafe extern "C" fn(
            camera: *mut CameraT,
            transform: CameraTransform,
            aspect: f32,
        ) -> *mut Mat44T,
    >,
    pub update_free_flight:
        ::std::option::Option<unsafe extern "C" fn(tm: *mut TransformT, t: Vec3T, r: Vec2T)>,
    pub update_pan: ::std::option::Option<
        unsafe extern "C" fn(tm: *mut TransformT, focus_position: *mut Vec3T, pan: Vec2T),
    >,
    pub update_maya: ::std::option::Option<
        unsafe extern "C" fn(tm: *mut TransformT, focus_position: Vec3T, zoom: f32, rot: Vec2T),
    >,
    pub world_to_screen: ::std::option::Option<
        unsafe extern "C" fn(
            camera: *const CameraT,
            transform: CameraTransform,
            viewport: RectT,
            world: *const Vec3T,
            screen: *mut Vec3T,
            n: u32,
        ) -> *mut Vec3T,
    >,
    pub screen_to_world: ::std::option::Option<
        unsafe extern "C" fn(
            camera: *const CameraT,
            transform: CameraTransform,
            viewport: RectT,
            screen: *const Vec3T,
            world: *mut Vec3T,
            n: u32,
        ) -> *mut Vec3T,
    >,
    pub meters_per_pixel: ::std::option::Option<
        unsafe extern "C" fn(distance: f32, vertical_fov: f32, viewport_height: f32) -> f32,
    >,
    pub default_camera_settings:
        ::std::option::Option<unsafe extern "C" fn() -> *const CameraSettingsT>,
    pub frustum_planes_from_view_projection: ::std::option::Option<
        unsafe extern "C" fn(
            view: *const Mat44T,
            projection: *const Mat44T,
            frustum_planes: *mut Vec4T,
        ),
    >,
}
pub const TM_COLLABORATION_STATUS_DISCONNECTED: CollaborationStatus = 0;
pub const TM_COLLABORATION_STATUS_HOST_STARTING: CollaborationStatus = 1;
pub const TM_COLLABORATION_STATUS_HOST: CollaborationStatus = 2;
pub const TM_COLLABORATION_STATUS_CONNECTING: CollaborationStatus = 3;
pub const TM_COLLABORATION_STATUS_CLIENT: CollaborationStatus = 4;
pub const TM_COLLABORATION_STATUS_ERROR: CollaborationStatus = 5;
pub type CollaborationStatus = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CollaborationConfigI {
    pub tt: *mut TheTruthO,
    pub ud: *mut ::std::os::raw::c_void,
    pub host_init: ::std::option::Option<unsafe extern "C" fn(ud: *mut ::std::os::raw::c_void)>,
    pub client_init: ::std::option::Option<unsafe extern "C" fn(ud: *mut ::std::os::raw::c_void)>,
    pub host_save_state: ::std::option::Option<
        unsafe extern "C" fn(
            ud: *mut ::std::os::raw::c_void,
            carray: *mut *mut ::std::os::raw::c_char,
            a: *mut AllocatorI,
        ),
    >,
    pub client_reset_truth: ::std::option::Option<
        unsafe extern "C" fn(ud: *mut ::std::os::raw::c_void) -> *mut TheTruthO,
    >,
    pub client_decompress_state: ::std::option::Option<
        unsafe extern "C" fn(
            app_ud: *mut ::std::os::raw::c_void,
            state: *const ::std::os::raw::c_char,
            size: u64,
            allocator: *mut AllocatorI,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub client_buffer_hashes: ::std::option::Option<
        unsafe extern "C" fn(
            ud: *mut ::std::os::raw::c_void,
            state: *const ::std::os::raw::c_char,
            size: u64,
            count: *mut u64,
        ) -> *const u64,
    >,
    pub client_load_state: ::std::option::Option<
        unsafe extern "C" fn(
            ud: *mut ::std::os::raw::c_void,
            state: *const ::std::os::raw::c_char,
            size: u64,
        ) -> bool,
    >,
    pub cache_dir: *const ::std::os::raw::c_char,
}
impl Default for CollaborationConfigI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_COLLABORATION_SESSION_TYPE_NONE: CollaborationSessionType = 0;
pub const TM_COLLABORATION_SESSION_TYPE_HOST: CollaborationSessionType = 1;
pub const TM_COLLABORATION_SESSION_TYPE_CLIENT: CollaborationSessionType = 2;
pub type CollaborationSessionType = ::std::os::raw::c_int;
pub const TM_COLLABORATION_SESSION_STATUS_DISCONNECTED: CollaborationSessionStatus = 0;
pub const TM_COLLABORATION_SESSION_STATUS_CONNECTING: CollaborationSessionStatus = 1;
pub const TM_COLLABORATION_SESSION_STATUS_CONNECTED: CollaborationSessionStatus = 2;
pub const TM_COLLABORATION_SESSION_STATUS_ERROR: CollaborationSessionStatus = 3;
pub type CollaborationSessionStatus = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CollaborationSessionO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CollaborationSessionI {
    pub inst: *mut CollaborationSessionO,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(inst: *mut CollaborationSessionO)>,
    pub architecture:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut CollaborationSessionO) -> StrhashT>,
    pub host_id:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut CollaborationSessionO) -> u64>,
    pub get_client_request: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut CollaborationSessionO, client_id: *mut u64) -> bool,
    >,
    pub accept_client: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut CollaborationSessionO, client_id: u64),
    >,
    pub send: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut CollaborationSessionO,
            client_id: u64,
            buf: *const u8,
            size: u64,
        ),
    >,
    pub flush: ::std::option::Option<unsafe extern "C" fn(inst: *mut CollaborationSessionO)>,
    pub update: ::std::option::Option<unsafe extern "C" fn(inst: *mut CollaborationSessionO)>,
    pub get_package_data: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut CollaborationSessionO,
            client_id: u64,
            data: *mut u8,
            size: *mut u64,
        ) -> bool,
    >,
    pub close_client: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut CollaborationSessionO, client_id: u64),
    >,
    pub send_ping: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut CollaborationSessionO, client_id: u64),
    >,
    pub type_: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut CollaborationSessionO) -> CollaborationSessionType,
    >,
    pub status: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut CollaborationSessionO) -> CollaborationSessionStatus,
    >,
    pub is_client_alive: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut CollaborationSessionO, client_id: u64) -> bool,
    >,
    pub receive_progress: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *const CollaborationSessionO,
            bytes: *mut u64,
            total: *mut u64,
        ) -> bool,
    >,
    pub status_message: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut CollaborationSessionO,
            buf: *mut ::std::os::raw::c_char,
            max_size: u32,
        ),
    >,
}
impl Default for CollaborationSessionI {
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
pub struct CollaborationO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct CollaborationApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            a: *mut AllocatorI,
            config: *const CollaborationConfigI,
        ) -> *mut CollaborationO,
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(coll: *mut CollaborationO)>,
    pub status: ::std::option::Option<
        unsafe extern "C" fn(coll: *const CollaborationO) -> CollaborationStatus,
    >,
    pub is_downloading: ::std::option::Option<
        unsafe extern "C" fn(coll: *const CollaborationO, bytes: *mut u64, total: *mut u64) -> bool,
    >,
    pub set_session: ::std::option::Option<
        unsafe extern "C" fn(coll: *mut CollaborationO, session: *const CollaborationSessionI),
    >,
    pub session: ::std::option::Option<
        unsafe extern "C" fn(coll: *mut CollaborationO) -> *mut CollaborationSessionI,
    >,
    pub disconnect: ::std::option::Option<unsafe extern "C" fn(coll: *mut CollaborationO)>,
    pub update: ::std::option::Option<unsafe extern "C" fn(coll: *mut CollaborationO)>,
    pub handle: ::std::option::Option<
        unsafe extern "C" fn(coll: *const CollaborationO) -> *const ::std::os::raw::c_char,
    >,
    pub set_handle: ::std::option::Option<
        unsafe extern "C" fn(c: *mut CollaborationO, handle: *mut ::std::os::raw::c_char),
    >,
    pub host_handle: ::std::option::Option<
        unsafe extern "C" fn(c: *const CollaborationO) -> *const ::std::os::raw::c_char,
    >,
    pub num_clients:
        ::std::option::Option<unsafe extern "C" fn(coll: *const CollaborationO) -> u32>,
    pub client_handle: ::std::option::Option<
        unsafe extern "C" fn(coll: *const CollaborationO, i: u32) -> *const ::std::os::raw::c_char,
    >,
    pub all_handles: ::std::option::Option<
        unsafe extern "C" fn(
            coll: *const CollaborationO,
            ta: *mut TempAllocatorI,
        ) -> *mut *const ::std::os::raw::c_char,
    >,
    pub send_chat: ::std::option::Option<
        unsafe extern "C" fn(coll: *mut CollaborationO, msg: *const ::std::os::raw::c_char),
    >,
    pub num_chat_messages:
        ::std::option::Option<unsafe extern "C" fn(coll: *const CollaborationO) -> u32>,
    pub chat_message: ::std::option::Option<
        unsafe extern "C" fn(
            coll: *const CollaborationO,
            i: u32,
            sender: *mut *const ::std::os::raw::c_char,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub resynchronize_state:
        ::std::option::Option<unsafe extern "C" fn(coll: *mut CollaborationO, tt: *mut TheTruthO)>,
    pub send_test_packages:
        ::std::option::Option<unsafe extern "C" fn(coll: *mut CollaborationO, size: u64, num: u32)>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CollaborationP2pO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CollaborationDiscoveredHostT {
    pub address: *const SocketAddressT,
    pub name: *const ::std::os::raw::c_char,
}
impl Default for CollaborationDiscoveredHostT {
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
pub struct CollaborationP2pApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            coll: *mut CollaborationO,
            allocator: *mut AllocatorI,
        ) -> *mut CollaborationP2pO,
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(coll_p2p: *mut CollaborationP2pO)>,
    pub update: ::std::option::Option<unsafe extern "C" fn(coll_p2p: *mut CollaborationP2pO)>,
    pub host: ::std::option::Option<
        unsafe extern "C" fn(coll_p2p: *mut CollaborationP2pO, port: u32, use_upnp: bool),
    >,
    pub connect: ::std::option::Option<
        unsafe extern "C" fn(coll_p2p: *mut CollaborationP2pO, address: *const SocketAddressT),
    >,
    pub discovered_lan_hosts: ::std::option::Option<
        unsafe extern "C" fn(
            coll_p2p: *const CollaborationP2pO,
            hosts: *mut CollaborationDiscoveredHostT,
            max_hosts: u32,
        ) -> u32,
    >,
}
pub const TM_CONFIG_TYPE_NULL: ConfigType = 0;
pub const TM_CONFIG_TYPE_FALSE: ConfigType = 1;
pub const TM_CONFIG_TYPE_TRUE: ConfigType = 2;
pub const TM_CONFIG_TYPE_NUMBER: ConfigType = 3;
pub const TM_CONFIG_TYPE_STRING: ConfigType = 4;
pub const TM_CONFIG_TYPE_ARRAY: ConfigType = 5;
pub const TM_CONFIG_TYPE_OBJECT: ConfigType = 6;
pub type ConfigType = ::std::os::raw::c_int;
#[repr(C)]
#[repr(align(4))]
#[derive(Default, Copy, Clone)]
pub struct ConfigItemT {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
impl ConfigItemT {
    #[inline]
    pub fn type_(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn offset(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 29u8) as u32) }
    }
    #[inline]
    pub fn set_offset(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 29u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(type_: u32, offset: u32) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 3u8, {
            let type_: u32 = unsafe { ::std::mem::transmute(type_) };
            type_ as u64
        });
        __bindgen_bitfield_unit.set(3usize, 29u8, {
            let offset: u32 = unsafe { ::std::mem::transmute(offset) };
            offset as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConfigO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConfigI {
    pub inst: *mut ConfigO,
    pub root: ::std::option::Option<unsafe extern "C" fn(inst: *mut ConfigO) -> ConfigItemT>,
    pub to_number:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut ConfigO, item: ConfigItemT) -> f64>,
    pub to_string: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ConfigO,
            item: ConfigItemT,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub to_array: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ConfigO,
            item: ConfigItemT,
            items: *mut *mut ConfigItemT,
        ) -> u32,
    >,
    pub to_object: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ConfigO,
            item: ConfigItemT,
            keys: *mut *mut ConfigItemT,
            values: *mut *mut ConfigItemT,
        ) -> u32,
    >,
    pub array_count:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut ConfigO, array: ConfigItemT) -> u32>,
    pub array_get: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ConfigO, array: ConfigItemT, index: u32) -> ConfigItemT,
    >,
    pub object_get: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ConfigO,
            object: ConfigItemT,
            key_hash: StrhashT,
        ) -> ConfigItemT,
    >,
    pub add_number:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut ConfigO, n: f64) -> ConfigItemT>,
    pub add_string: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ConfigO, s: *const ::std::os::raw::c_char) -> ConfigItemT,
    >,
    pub add_array: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ConfigO,
            items: *const ConfigItemT,
            size: u32,
        ) -> ConfigItemT,
    >,
    pub add_object: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ConfigO,
            keys: *const ConfigItemT,
            values: *const ConfigItemT,
            size: u32,
        ) -> ConfigItemT,
    >,
    pub add_object_with_string_keys: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ConfigO,
            keys: *mut *const ::std::os::raw::c_char,
            values: *const ConfigItemT,
            size: u32,
        ) -> ConfigItemT,
    >,
    pub array_set: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ConfigO, array: ConfigItemT, i: u32, item: ConfigItemT),
    >,
    pub array_push: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ConfigO, array: ConfigItemT, item: ConfigItemT),
    >,
    pub object_update: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ConfigO,
            object: ConfigItemT,
            key_hash: StrhashT,
            value: ConfigItemT,
        ),
    >,
    pub object_add: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ConfigO,
            object: ConfigItemT,
            key: *const ::std::os::raw::c_char,
            value: ConfigItemT,
        ),
    >,
    pub set_root:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut ConfigO, root: ConfigItemT)>,
    pub copy: ::std::option::Option<unsafe extern "C" fn(dst: *mut ConfigO, src: *mut ConfigO)>,
    pub allocator:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut ConfigO) -> *mut AllocatorI>,
    pub validate_object: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ConfigO,
            object: ConfigItemT,
            object_display_name: *const ::std::os::raw::c_char,
            valid_child_keys: *mut *const ::std::os::raw::c_char,
            num_valid_child_keys: u32,
        ) -> bool,
    >,
}
impl Default for ConfigI {
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
pub struct ConfigApi {
    pub create: ::std::option::Option<unsafe extern "C" fn(a: *mut AllocatorI) -> *mut ConfigI>,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(cdi: *mut ConfigI)>,
    pub c_null: ConfigItemT,
    pub c_false: ConfigItemT,
    pub c_true: ConfigItemT,
    pub _padding_189: [::std::os::raw::c_char; 4usize],
}
pub const TM_CORE_CREATE_POLICY__CREATE: CoreCreatePolicy = 0;
pub const TM_CORE_CREATE_POLICY__REMOVE: CoreCreatePolicy = 1;
pub const TM_CORE_CREATE_POLICY__IGNORE: CoreCreatePolicy = 2;
pub type CoreCreatePolicy = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoreAssetI {
    pub core_id: StrhashT,
    pub path: *const ::std::os::raw::c_char,
    pub create_policy: CoreCreatePolicy,
    pub _padding_59: [::std::os::raw::c_char; 4usize],
    pub version: u64,
    pub user_data: *const ::std::os::raw::c_void,
    pub create: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, user_data: *const ::std::os::raw::c_void) -> TtIdT,
    >,
    pub update: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            data_id: TtIdT,
            old_version: u64,
            user_data: *const ::std::os::raw::c_void,
        ),
    >,
    pub on_change: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            data_id: TtIdT,
            user_data: *const ::std::os::raw::c_void,
        ),
    >,
}
impl Default for CoreAssetI {
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
pub struct CoreUpdateResultT {
    pub created_assets: *mut TtIdT,
    pub removed_assets: *mut TtIdT,
    pub updated_assets: *mut TtIdT,
}
impl Default for CoreUpdateResultT {
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
pub struct CoreUpdatableAssetsT {
    pub to_create: *mut *mut CoreAssetI,
    pub to_remove: *mut *mut CoreAssetI,
    pub to_update: *mut *mut CoreAssetI,
}
impl Default for CoreUpdatableAssetsT {
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
pub struct CoreApi {
    pub create:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO, asset_root_id: TtIdT)>,
    pub query_updatable: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            asset_root_id: TtIdT,
            include_skipped: bool,
            ta: *mut TempAllocatorI,
        ) -> CoreUpdatableAssetsT,
    >,
    pub update: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            asset_root_id: TtIdT,
            to_skip: *mut *mut CoreAssetI,
            num_to_skip: u32,
            include_skipped: bool,
            ta: *mut TempAllocatorI,
        ) -> CoreUpdateResultT,
    >,
    pub locate_asset: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, asset_root_id: TtIdT, core_id: StrhashT) -> TtIdT,
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoreImporterAssetT {
    pub id: StrhashT,
    pub version: u64,
    pub path: *const ::std::os::raw::c_char,
    pub create_policy: u32,
    pub _padding_50: [::std::os::raw::c_char; 4usize],
}
impl Default for CoreImporterAssetT {
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
pub struct CoreImporterStateO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct CoreImporterApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            a: *mut AllocatorI,
            user_tt: *mut TheTruthO,
            user_asset_root: TtIdT,
            core_project_path: *const ::std::os::raw::c_char,
            output_path: *const ::std::os::raw::c_char,
        ) -> *mut CoreImporterStateO,
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(state: *mut CoreImporterStateO)>,
    pub register_assets: ::std::option::Option<
        unsafe extern "C" fn(
            state: *mut CoreImporterStateO,
            to_register: *const CoreImporterAssetT,
            num_to_register: u32,
        ),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CrashRecoveryCanRecoverResultT {
    pub data: *const ::std::os::raw::c_char,
    pub data_bytes: u64,
    pub project: *const ::std::os::raw::c_char,
    pub root: *const UuidT,
}
impl Default for CrashRecoveryCanRecoverResultT {
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
pub struct CrashRecoveryO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct CrashRecoveryApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            a: *mut AllocatorI,
            recovery_path: *const ::std::os::raw::c_char,
        ) -> *mut CrashRecoveryO,
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(cr: *mut CrashRecoveryO)>,
    pub start_recording: ::std::option::Option<
        unsafe extern "C" fn(
            cr: *mut CrashRecoveryO,
            project: *const ::std::os::raw::c_char,
            tt: *mut TheTruthO,
            root: TtIdT,
        ),
    >,
    pub stop_recording: ::std::option::Option<unsafe extern "C" fn(cr: *mut CrashRecoveryO)>,
    pub update: ::std::option::Option<unsafe extern "C" fn(cr: *mut CrashRecoveryO)>,
    pub can_recover: ::std::option::Option<
        unsafe extern "C" fn(
            cr: *mut CrashRecoveryO,
            ta: *mut TempAllocatorI,
        ) -> CrashRecoveryCanRecoverResultT,
    >,
    pub recover: ::std::option::Option<
        unsafe extern "C" fn(
            cr: *mut CrashRecoveryO,
            rd: CrashRecoveryCanRecoverResultT,
            tt: *mut TheTruthO,
        ),
    >,
    pub delete_physical_file: ::std::option::Option<unsafe extern "C" fn(cr: *mut CrashRecoveryO)>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ErrorO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ErrorI {
    pub inst: *mut ErrorO,
    pub errorf: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ErrorO,
            file: *const ::std::os::raw::c_char,
            line: u32,
            format: *const ::std::os::raw::c_char,
            ...
        ),
    >,
    pub fatal: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ErrorO,
            file: *const ::std::os::raw::c_char,
            line: u32,
            format: *const ::std::os::raw::c_char,
            ...
        ),
    >,
}
impl Default for ErrorI {
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
pub struct ErrorRecordT {
    pub ta: *mut TempAllocatorI,
    pub errors: *mut *mut ::std::os::raw::c_char,
    pub backing: *mut ErrorI,
}
impl Default for ErrorRecordT {
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
pub struct ErrorApi {
    pub log: *mut ErrorI,
    pub def: *mut ErrorI,
    pub create_record_handler:
        ::std::option::Option<unsafe extern "C" fn(mem: *mut ErrorRecordT) -> ErrorI>,
}
impl Default for ErrorApi {
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
pub struct FeatureFlagsApi {
    pub enabled: ::std::option::Option<unsafe extern "C" fn(flag: StrhashT) -> bool>,
    pub set_enabled: ::std::option::Option<unsafe extern "C" fn(flag: StrhashT, enabled: bool)>,
    pub all_enabled:
        ::std::option::Option<unsafe extern "C" fn(count: *mut u32) -> *const StrhashT>,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct GitIgnoreApi {
    pub match_: ::std::option::Option<
        unsafe extern "C" fn(
            patterns: *const ::std::os::raw::c_char,
            path: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
}
pub const TM_IMAGE_TYPE_1D: ImageType = 0;
pub const TM_IMAGE_TYPE_2D: ImageType = 1;
pub const TM_IMAGE_TYPE_3D: ImageType = 2;
pub const TM_IMAGE_TYPE_CUBE: ImageType = 3;
pub type ImageType = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ImageT {
    pub type_: u32,
    pub pixel_format: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mip_levels: u32,
    pub layers: u32,
    pub size: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageArchiveO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageArchiveI {
    pub inst: *mut ImageArchiveO,
    pub read: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ImageArchiveO,
            buffer: *mut ::std::os::raw::c_void,
            offset: u64,
            size: u32,
        ) -> u32,
    >,
    pub size: ::std::option::Option<unsafe extern "C" fn(inst: *mut ImageArchiveO) -> u64>,
}
impl Default for ImageArchiveI {
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
pub struct ImageLoaderO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageLoaderI {
    pub inst: *mut ImageLoaderO,
    pub extensions_string: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ImageLoaderO,
            output: *mut *mut ::std::os::raw::c_char,
            ta: *mut TempAllocatorI,
            separator: *const ::std::os::raw::c_char,
        ),
    >,
    pub description_string: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ImageLoaderO,
            output: *mut *mut ::std::os::raw::c_char,
            ta: *mut TempAllocatorI,
            separator: *const ::std::os::raw::c_char,
        ),
    >,
    pub support_from_archive: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ImageLoaderO, image_archive: *mut ImageArchiveI) -> bool,
    >,
    pub support_from_extension: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ImageLoaderO,
            extension: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub load_image: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ImageLoaderO,
            image_archive: *mut ImageArchiveI,
            image: *mut ImageT,
            bits: *mut u8,
        ) -> bool,
    >,
}
impl Default for ImageLoaderI {
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
pub struct ImageLoaderApi {
    pub add_loader: ::std::option::Option<unsafe extern "C" fn(loader: *mut ImageLoaderI)>,
    pub remove_loader: ::std::option::Option<unsafe extern "C" fn(loader: *mut ImageLoaderI)>,
    pub loader_from_archive: ::std::option::Option<
        unsafe extern "C" fn(image_archive: *mut ImageArchiveI) -> *mut ImageLoaderI,
    >,
    pub loader_from_extension: ::std::option::Option<
        unsafe extern "C" fn(extension: *const ::std::os::raw::c_char) -> *mut ImageLoaderI,
    >,
    pub loaders:
        ::std::option::Option<unsafe extern "C" fn(loaders: *mut *mut ImageLoaderI) -> u32>,
}
pub const TM_INPUT_CONTROLLER_TYPE_NONE: InputControllerType = 0;
pub const TM_INPUT_CONTROLLER_TYPE_KEYBOARD: InputControllerType = 1;
pub const TM_INPUT_CONTROLLER_TYPE_MOUSE: InputControllerType = 2;
pub const TM_INPUT_CONTROLLER_TYPE_GAMEPAD: InputControllerType = 3;
pub const TM_INPUT_CONTROLLER_TYPE_TOUCH: InputControllerType = 4;
pub const TM_INPUT_CONTROLLER_TYPE_PEN: InputControllerType = 5;
pub const TM_INPUT_CONTROLLER_TYPE_OTHER: InputControllerType = -1;
pub type InputControllerType = ::std::os::raw::c_int;
pub const TM_INPUT_MOUSE_ITEM_NONE: InputMouseItem = 0;
pub const TM_INPUT_MOUSE_ITEM_BUTTON_LEFT: InputMouseItem = 1;
pub const TM_INPUT_MOUSE_ITEM_BUTTON_RIGHT: InputMouseItem = 2;
pub const TM_INPUT_MOUSE_ITEM_BUTTON_MIDDLE: InputMouseItem = 3;
pub const TM_INPUT_MOUSE_ITEM_BUTTON_4: InputMouseItem = 4;
pub const TM_INPUT_MOUSE_ITEM_BUTTON_5: InputMouseItem = 5;
pub const TM_INPUT_MOUSE_ITEM_WHEEL: InputMouseItem = 6;
pub const TM_INPUT_MOUSE_ITEM_MOVE: InputMouseItem = 7;
pub const TM_INPUT_MOUSE_ITEM_POSITION: InputMouseItem = 8;
pub const TM_INPUT_MOUSE_ITEM_COUNT: InputMouseItem = 9;
pub type InputMouseItem = ::std::os::raw::c_int;
pub const TM_INPUT_PEN_ITEM_NONE: InputPenItem = 0;
pub const TM_INPUT_PEN_ITEM_BUTTON_1: InputPenItem = 1;
pub const TM_INPUT_PEN_ITEM_BUTTON_2: InputPenItem = 2;
pub const TM_INPUT_PEN_ITEM_BUTTON_3: InputPenItem = 3;
pub const TM_INPUT_PEN_ITEM_BUTTON_4: InputPenItem = 4;
pub const TM_INPUT_PEN_ITEM_BUTTON_5: InputPenItem = 5;
pub const TM_INPUT_PEN_ITEM_WHEEL: InputPenItem = 6;
pub const TM_INPUT_PEN_ITEM_POSITION: InputPenItem = 7;
pub const TM_INPUT_PEN_ITEM_PRESSURE: InputPenItem = 8;
pub const TM_INPUT_PEN_ITEM_ROTATION: InputPenItem = 9;
pub const TM_INPUT_PEN_ITEM_TILT: InputPenItem = 10;
pub const TM_INPUT_PEN_ITEM_INVERTED: InputPenItem = 11;
pub const TM_INPUT_PEN_ITEM_ERASER: InputPenItem = 12;
pub const TM_INPUT_PEN_ITEM_COUNT: InputPenItem = 13;
pub type InputPenItem = ::std::os::raw::c_int;
pub const TM_INPUT_TOUCH_ITEM_NONE: InputTouchItem = 0;
pub const TM_INPUT_TOUCH_ITEM_TOUCH: InputTouchItem = 1;
pub const TM_INPUT_TOUCH_ITEM_POSITION: InputTouchItem = 2;
pub const TM_INPUT_TOUCH_ITEM_PRESSURE: InputTouchItem = 3;
pub const TM_INPUT_TOUCH_ITEM_ORIENTATION: InputTouchItem = 4;
pub const TM_INPUT_TOUCH_ITEM_COUNT: InputTouchItem = 5;
pub type InputTouchItem = ::std::os::raw::c_int;
pub const TM_INPUT_KEYBOARD_ITEM_NONE: InputKeyboardItem = 0;
pub const TM_INPUT_KEYBOARD_ITEM_LBUTTON: InputKeyboardItem = 1;
pub const TM_INPUT_KEYBOARD_ITEM_RBUTTON: InputKeyboardItem = 2;
pub const TM_INPUT_KEYBOARD_ITEM_CANCEL: InputKeyboardItem = 3;
pub const TM_INPUT_KEYBOARD_ITEM_MBUTTON: InputKeyboardItem = 4;
pub const TM_INPUT_KEYBOARD_ITEM_XBUTTON1: InputKeyboardItem = 5;
pub const TM_INPUT_KEYBOARD_ITEM_XBUTTON2: InputKeyboardItem = 6;
pub const TM_INPUT_KEYBOARD_ITEM_BACKSPACE: InputKeyboardItem = 8;
pub const TM_INPUT_KEYBOARD_ITEM_TAB: InputKeyboardItem = 9;
pub const TM_INPUT_KEYBOARD_ITEM_CLEAR: InputKeyboardItem = 12;
pub const TM_INPUT_KEYBOARD_ITEM_ENTER: InputKeyboardItem = 13;
pub const TM_INPUT_KEYBOARD_ITEM_SHIFT: InputKeyboardItem = 16;
pub const TM_INPUT_KEYBOARD_ITEM_CONTROL: InputKeyboardItem = 17;
pub const TM_INPUT_KEYBOARD_ITEM_MENU: InputKeyboardItem = 18;
pub const TM_INPUT_KEYBOARD_ITEM_PAUSE: InputKeyboardItem = 19;
pub const TM_INPUT_KEYBOARD_ITEM_CAPSLOCK: InputKeyboardItem = 20;
pub const TM_INPUT_KEYBOARD_ITEM_KANA: InputKeyboardItem = 21;
pub const TM_INPUT_KEYBOARD_ITEM_JUNJA: InputKeyboardItem = 23;
pub const TM_INPUT_KEYBOARD_ITEM_FINAL: InputKeyboardItem = 24;
pub const TM_INPUT_KEYBOARD_ITEM_HANJA: InputKeyboardItem = 25;
pub const TM_INPUT_KEYBOARD_ITEM_ESCAPE: InputKeyboardItem = 27;
pub const TM_INPUT_KEYBOARD_ITEM_CONVERT: InputKeyboardItem = 28;
pub const TM_INPUT_KEYBOARD_ITEM_NONCONVERT: InputKeyboardItem = 29;
pub const TM_INPUT_KEYBOARD_ITEM_ACCEPT: InputKeyboardItem = 30;
pub const TM_INPUT_KEYBOARD_ITEM_MODECHANGE: InputKeyboardItem = 31;
pub const TM_INPUT_KEYBOARD_ITEM_SPACE: InputKeyboardItem = 32;
pub const TM_INPUT_KEYBOARD_ITEM_PAGEUP: InputKeyboardItem = 33;
pub const TM_INPUT_KEYBOARD_ITEM_PAGEDOWN: InputKeyboardItem = 34;
pub const TM_INPUT_KEYBOARD_ITEM_END: InputKeyboardItem = 35;
pub const TM_INPUT_KEYBOARD_ITEM_HOME: InputKeyboardItem = 36;
pub const TM_INPUT_KEYBOARD_ITEM_LEFT: InputKeyboardItem = 37;
pub const TM_INPUT_KEYBOARD_ITEM_UP: InputKeyboardItem = 38;
pub const TM_INPUT_KEYBOARD_ITEM_RIGHT: InputKeyboardItem = 39;
pub const TM_INPUT_KEYBOARD_ITEM_DOWN: InputKeyboardItem = 40;
pub const TM_INPUT_KEYBOARD_ITEM_SELECT: InputKeyboardItem = 41;
pub const TM_INPUT_KEYBOARD_ITEM_PRINT: InputKeyboardItem = 42;
pub const TM_INPUT_KEYBOARD_ITEM_EXECUTE: InputKeyboardItem = 43;
pub const TM_INPUT_KEYBOARD_ITEM_PRINTSCREEN: InputKeyboardItem = 44;
pub const TM_INPUT_KEYBOARD_ITEM_INSERT: InputKeyboardItem = 45;
pub const TM_INPUT_KEYBOARD_ITEM_DELETE: InputKeyboardItem = 46;
pub const TM_INPUT_KEYBOARD_ITEM_HELP: InputKeyboardItem = 47;
pub const TM_INPUT_KEYBOARD_ITEM_0: InputKeyboardItem = 48;
pub const TM_INPUT_KEYBOARD_ITEM_1: InputKeyboardItem = 49;
pub const TM_INPUT_KEYBOARD_ITEM_2: InputKeyboardItem = 50;
pub const TM_INPUT_KEYBOARD_ITEM_3: InputKeyboardItem = 51;
pub const TM_INPUT_KEYBOARD_ITEM_4: InputKeyboardItem = 52;
pub const TM_INPUT_KEYBOARD_ITEM_5: InputKeyboardItem = 53;
pub const TM_INPUT_KEYBOARD_ITEM_6: InputKeyboardItem = 54;
pub const TM_INPUT_KEYBOARD_ITEM_7: InputKeyboardItem = 55;
pub const TM_INPUT_KEYBOARD_ITEM_8: InputKeyboardItem = 56;
pub const TM_INPUT_KEYBOARD_ITEM_9: InputKeyboardItem = 57;
pub const TM_INPUT_KEYBOARD_ITEM_A: InputKeyboardItem = 65;
pub const TM_INPUT_KEYBOARD_ITEM_B: InputKeyboardItem = 66;
pub const TM_INPUT_KEYBOARD_ITEM_C: InputKeyboardItem = 67;
pub const TM_INPUT_KEYBOARD_ITEM_D: InputKeyboardItem = 68;
pub const TM_INPUT_KEYBOARD_ITEM_E: InputKeyboardItem = 69;
pub const TM_INPUT_KEYBOARD_ITEM_F: InputKeyboardItem = 70;
pub const TM_INPUT_KEYBOARD_ITEM_G: InputKeyboardItem = 71;
pub const TM_INPUT_KEYBOARD_ITEM_H: InputKeyboardItem = 72;
pub const TM_INPUT_KEYBOARD_ITEM_I: InputKeyboardItem = 73;
pub const TM_INPUT_KEYBOARD_ITEM_J: InputKeyboardItem = 74;
pub const TM_INPUT_KEYBOARD_ITEM_K: InputKeyboardItem = 75;
pub const TM_INPUT_KEYBOARD_ITEM_L: InputKeyboardItem = 76;
pub const TM_INPUT_KEYBOARD_ITEM_M: InputKeyboardItem = 77;
pub const TM_INPUT_KEYBOARD_ITEM_N: InputKeyboardItem = 78;
pub const TM_INPUT_KEYBOARD_ITEM_O: InputKeyboardItem = 79;
pub const TM_INPUT_KEYBOARD_ITEM_P: InputKeyboardItem = 80;
pub const TM_INPUT_KEYBOARD_ITEM_Q: InputKeyboardItem = 81;
pub const TM_INPUT_KEYBOARD_ITEM_R: InputKeyboardItem = 82;
pub const TM_INPUT_KEYBOARD_ITEM_S: InputKeyboardItem = 83;
pub const TM_INPUT_KEYBOARD_ITEM_T: InputKeyboardItem = 84;
pub const TM_INPUT_KEYBOARD_ITEM_U: InputKeyboardItem = 85;
pub const TM_INPUT_KEYBOARD_ITEM_V: InputKeyboardItem = 86;
pub const TM_INPUT_KEYBOARD_ITEM_W: InputKeyboardItem = 87;
pub const TM_INPUT_KEYBOARD_ITEM_X: InputKeyboardItem = 88;
pub const TM_INPUT_KEYBOARD_ITEM_Y: InputKeyboardItem = 89;
pub const TM_INPUT_KEYBOARD_ITEM_Z: InputKeyboardItem = 90;
pub const TM_INPUT_KEYBOARD_ITEM_LWIN: InputKeyboardItem = 91;
pub const TM_INPUT_KEYBOARD_ITEM_RWIN: InputKeyboardItem = 92;
pub const TM_INPUT_KEYBOARD_ITEM_APPS: InputKeyboardItem = 93;
pub const TM_INPUT_KEYBOARD_ITEM_SLEEP: InputKeyboardItem = 95;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPAD0: InputKeyboardItem = 96;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPAD1: InputKeyboardItem = 97;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPAD2: InputKeyboardItem = 98;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPAD3: InputKeyboardItem = 99;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPAD4: InputKeyboardItem = 100;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPAD5: InputKeyboardItem = 101;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPAD6: InputKeyboardItem = 102;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPAD7: InputKeyboardItem = 103;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPAD8: InputKeyboardItem = 104;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPAD9: InputKeyboardItem = 105;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPADASTERISK: InputKeyboardItem = 106;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPADPLUS: InputKeyboardItem = 107;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPADENTER: InputKeyboardItem = 108;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPADMINUS: InputKeyboardItem = 109;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPADDOT: InputKeyboardItem = 110;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPADSLASH: InputKeyboardItem = 111;
pub const TM_INPUT_KEYBOARD_ITEM_F1: InputKeyboardItem = 112;
pub const TM_INPUT_KEYBOARD_ITEM_F2: InputKeyboardItem = 113;
pub const TM_INPUT_KEYBOARD_ITEM_F3: InputKeyboardItem = 114;
pub const TM_INPUT_KEYBOARD_ITEM_F4: InputKeyboardItem = 115;
pub const TM_INPUT_KEYBOARD_ITEM_F5: InputKeyboardItem = 116;
pub const TM_INPUT_KEYBOARD_ITEM_F6: InputKeyboardItem = 117;
pub const TM_INPUT_KEYBOARD_ITEM_F7: InputKeyboardItem = 118;
pub const TM_INPUT_KEYBOARD_ITEM_F8: InputKeyboardItem = 119;
pub const TM_INPUT_KEYBOARD_ITEM_F9: InputKeyboardItem = 120;
pub const TM_INPUT_KEYBOARD_ITEM_F10: InputKeyboardItem = 121;
pub const TM_INPUT_KEYBOARD_ITEM_F11: InputKeyboardItem = 122;
pub const TM_INPUT_KEYBOARD_ITEM_F12: InputKeyboardItem = 123;
pub const TM_INPUT_KEYBOARD_ITEM_F13: InputKeyboardItem = 124;
pub const TM_INPUT_KEYBOARD_ITEM_F14: InputKeyboardItem = 125;
pub const TM_INPUT_KEYBOARD_ITEM_F15: InputKeyboardItem = 126;
pub const TM_INPUT_KEYBOARD_ITEM_F16: InputKeyboardItem = 127;
pub const TM_INPUT_KEYBOARD_ITEM_F17: InputKeyboardItem = 128;
pub const TM_INPUT_KEYBOARD_ITEM_F18: InputKeyboardItem = 129;
pub const TM_INPUT_KEYBOARD_ITEM_F19: InputKeyboardItem = 130;
pub const TM_INPUT_KEYBOARD_ITEM_F20: InputKeyboardItem = 131;
pub const TM_INPUT_KEYBOARD_ITEM_F21: InputKeyboardItem = 132;
pub const TM_INPUT_KEYBOARD_ITEM_F22: InputKeyboardItem = 133;
pub const TM_INPUT_KEYBOARD_ITEM_F23: InputKeyboardItem = 134;
pub const TM_INPUT_KEYBOARD_ITEM_F24: InputKeyboardItem = 135;
pub const TM_INPUT_KEYBOARD_ITEM_NAVIGATION_VIEW: InputKeyboardItem = 136;
pub const TM_INPUT_KEYBOARD_ITEM_NAVIGATION_MENU: InputKeyboardItem = 137;
pub const TM_INPUT_KEYBOARD_ITEM_NAVIGATION_UP: InputKeyboardItem = 138;
pub const TM_INPUT_KEYBOARD_ITEM_NAVIGATION_DOWN: InputKeyboardItem = 139;
pub const TM_INPUT_KEYBOARD_ITEM_NAVIGATION_LEFT: InputKeyboardItem = 140;
pub const TM_INPUT_KEYBOARD_ITEM_NAVIGATION_RIGHT: InputKeyboardItem = 141;
pub const TM_INPUT_KEYBOARD_ITEM_NAVIGATION_ACCEPT: InputKeyboardItem = 142;
pub const TM_INPUT_KEYBOARD_ITEM_NAVIGATION_CANCEL: InputKeyboardItem = 143;
pub const TM_INPUT_KEYBOARD_ITEM_NUMLOCK: InputKeyboardItem = 144;
pub const TM_INPUT_KEYBOARD_ITEM_SCROLLLOCK: InputKeyboardItem = 145;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPADEQUAL: InputKeyboardItem = 146;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_FJ_MASSHOU: InputKeyboardItem = 147;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_FJ_TOUROKU: InputKeyboardItem = 148;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_FJ_LOYA: InputKeyboardItem = 149;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_FJ_ROYA: InputKeyboardItem = 150;
pub const TM_INPUT_KEYBOARD_ITEM_LEFTSHIFT: InputKeyboardItem = 160;
pub const TM_INPUT_KEYBOARD_ITEM_RIGHTSHIFT: InputKeyboardItem = 161;
pub const TM_INPUT_KEYBOARD_ITEM_LEFTCONTROL: InputKeyboardItem = 162;
pub const TM_INPUT_KEYBOARD_ITEM_RIGHTCONTROL: InputKeyboardItem = 163;
pub const TM_INPUT_KEYBOARD_ITEM_LEFTALT: InputKeyboardItem = 164;
pub const TM_INPUT_KEYBOARD_ITEM_RIGHTALT: InputKeyboardItem = 165;
pub const TM_INPUT_KEYBOARD_ITEM_BROWSER_BACK: InputKeyboardItem = 166;
pub const TM_INPUT_KEYBOARD_ITEM_BROWSER_FORWARD: InputKeyboardItem = 167;
pub const TM_INPUT_KEYBOARD_ITEM_BROWSER_REFRESH: InputKeyboardItem = 168;
pub const TM_INPUT_KEYBOARD_ITEM_BROWSER_STOP: InputKeyboardItem = 169;
pub const TM_INPUT_KEYBOARD_ITEM_BROWSER_SEARCH: InputKeyboardItem = 170;
pub const TM_INPUT_KEYBOARD_ITEM_BROWSER_FAVORITES: InputKeyboardItem = 171;
pub const TM_INPUT_KEYBOARD_ITEM_BROWSER_HOME: InputKeyboardItem = 172;
pub const TM_INPUT_KEYBOARD_ITEM_VOLUME_MUTE: InputKeyboardItem = 173;
pub const TM_INPUT_KEYBOARD_ITEM_VOLUME_DOWN: InputKeyboardItem = 174;
pub const TM_INPUT_KEYBOARD_ITEM_VOLUME_UP: InputKeyboardItem = 175;
pub const TM_INPUT_KEYBOARD_ITEM_MEDIA_NEXT_TRACK: InputKeyboardItem = 176;
pub const TM_INPUT_KEYBOARD_ITEM_MEDIA_PREV_TRACK: InputKeyboardItem = 177;
pub const TM_INPUT_KEYBOARD_ITEM_MEDIA_STOP: InputKeyboardItem = 178;
pub const TM_INPUT_KEYBOARD_ITEM_MEDIA_PLAY_PAUSE: InputKeyboardItem = 179;
pub const TM_INPUT_KEYBOARD_ITEM_LAUNCH_MAIL: InputKeyboardItem = 180;
pub const TM_INPUT_KEYBOARD_ITEM_LAUNCH_MEDIA_SELECT: InputKeyboardItem = 181;
pub const TM_INPUT_KEYBOARD_ITEM_LAUNCH_APP1: InputKeyboardItem = 182;
pub const TM_INPUT_KEYBOARD_ITEM_LAUNCH_APP2: InputKeyboardItem = 183;
pub const TM_INPUT_KEYBOARD_ITEM_SEMICOLON: InputKeyboardItem = 186;
pub const TM_INPUT_KEYBOARD_ITEM_EQUAL: InputKeyboardItem = 187;
pub const TM_INPUT_KEYBOARD_ITEM_COMMA: InputKeyboardItem = 188;
pub const TM_INPUT_KEYBOARD_ITEM_MINUS: InputKeyboardItem = 189;
pub const TM_INPUT_KEYBOARD_ITEM_DOT: InputKeyboardItem = 190;
pub const TM_INPUT_KEYBOARD_ITEM_SLASH: InputKeyboardItem = 191;
pub const TM_INPUT_KEYBOARD_ITEM_GRAVE: InputKeyboardItem = 192;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_A: InputKeyboardItem = 195;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_B: InputKeyboardItem = 196;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_X: InputKeyboardItem = 197;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_Y: InputKeyboardItem = 198;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_RIGHT_SHOULDER: InputKeyboardItem = 199;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_LEFT_SHOULDER: InputKeyboardItem = 200;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_LEFT_TRIGGER: InputKeyboardItem = 201;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_RIGHT_TRIGGER: InputKeyboardItem = 202;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_DPAD_UP: InputKeyboardItem = 203;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_DPAD_DOWN: InputKeyboardItem = 204;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_DPAD_LEFT: InputKeyboardItem = 205;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_DPAD_RIGHT: InputKeyboardItem = 206;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_MENU: InputKeyboardItem = 207;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_VIEW: InputKeyboardItem = 208;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_LEFT_THUMBSTICK_BUTTON: InputKeyboardItem = 209;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: InputKeyboardItem = 210;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_LEFT_THUMBSTICK_UP: InputKeyboardItem = 211;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_LEFT_THUMBSTICK_DOWN: InputKeyboardItem = 212;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_LEFT_THUMBSTICK_RIGHT: InputKeyboardItem = 213;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_LEFT_THUMBSTICK_LEFT: InputKeyboardItem = 214;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_RIGHT_THUMBSTICK_UP: InputKeyboardItem = 215;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_RIGHT_THUMBSTICK_DOWN: InputKeyboardItem = 216;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: InputKeyboardItem = 217;
pub const TM_INPUT_KEYBOARD_ITEM_GAMEPAD_RIGHT_THUMBSTICK_LEFT: InputKeyboardItem = 218;
pub const TM_INPUT_KEYBOARD_ITEM_LEFTBRACE: InputKeyboardItem = 219;
pub const TM_INPUT_KEYBOARD_ITEM_BACKSLASH: InputKeyboardItem = 220;
pub const TM_INPUT_KEYBOARD_ITEM_RIGHTBRACE: InputKeyboardItem = 221;
pub const TM_INPUT_KEYBOARD_ITEM_APOSTROPHE: InputKeyboardItem = 222;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_8: InputKeyboardItem = 223;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_AX: InputKeyboardItem = 225;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_102: InputKeyboardItem = 226;
pub const TM_INPUT_KEYBOARD_ITEM_ICO_HELP: InputKeyboardItem = 227;
pub const TM_INPUT_KEYBOARD_ITEM_ICO_00: InputKeyboardItem = 228;
pub const TM_INPUT_KEYBOARD_ITEM_PROCESSKEY: InputKeyboardItem = 229;
pub const TM_INPUT_KEYBOARD_ITEM_ICO_CLEAR: InputKeyboardItem = 230;
pub const TM_INPUT_KEYBOARD_ITEM_PACKET: InputKeyboardItem = 231;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_RESET: InputKeyboardItem = 233;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_JUMP: InputKeyboardItem = 234;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_PA1: InputKeyboardItem = 235;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_PA2: InputKeyboardItem = 236;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_PA3: InputKeyboardItem = 237;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_WSCTRL: InputKeyboardItem = 238;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_CUSEL: InputKeyboardItem = 239;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_ATTN: InputKeyboardItem = 240;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_FINISH: InputKeyboardItem = 241;
pub const TM_INPUT_KEYBOARD_ITEM_COPY: InputKeyboardItem = 242;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_AUTO: InputKeyboardItem = 243;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_ENLW: InputKeyboardItem = 244;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_BACKTAB: InputKeyboardItem = 245;
pub const TM_INPUT_KEYBOARD_ITEM_ATTN: InputKeyboardItem = 246;
pub const TM_INPUT_KEYBOARD_ITEM_CRSEL: InputKeyboardItem = 247;
pub const TM_INPUT_KEYBOARD_ITEM_EXSEL: InputKeyboardItem = 248;
pub const TM_INPUT_KEYBOARD_ITEM_EREOF: InputKeyboardItem = 249;
pub const TM_INPUT_KEYBOARD_ITEM_PLAY: InputKeyboardItem = 250;
pub const TM_INPUT_KEYBOARD_ITEM_ZOOM: InputKeyboardItem = 251;
pub const TM_INPUT_KEYBOARD_ITEM_NONAME: InputKeyboardItem = 252;
pub const TM_INPUT_KEYBOARD_ITEM_PA1: InputKeyboardItem = 253;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_CLEAR: InputKeyboardItem = 254;
pub const TM_INPUT_KEYBOARD_ITEM_KANJI: InputKeyboardItem = 25;
pub const TM_INPUT_KEYBOARD_ITEM_OEM_FJ_JISHO: InputKeyboardItem = 146;
pub const TM_INPUT_KEYBOARD_ITEM_HASHTILDE: InputKeyboardItem = 255;
pub const TM_INPUT_KEYBOARD_ITEM_102ND: InputKeyboardItem = 256;
pub const TM_INPUT_KEYBOARD_ITEM_COMPOSE: InputKeyboardItem = 257;
pub const TM_INPUT_KEYBOARD_ITEM_POWER: InputKeyboardItem = 258;
pub const TM_INPUT_KEYBOARD_ITEM_OPEN: InputKeyboardItem = 259;
pub const TM_INPUT_KEYBOARD_ITEM_PROPS: InputKeyboardItem = 260;
pub const TM_INPUT_KEYBOARD_ITEM_FRONT: InputKeyboardItem = 261;
pub const TM_INPUT_KEYBOARD_ITEM_STOP: InputKeyboardItem = 262;
pub const TM_INPUT_KEYBOARD_ITEM_AGAIN: InputKeyboardItem = 263;
pub const TM_INPUT_KEYBOARD_ITEM_UNDO: InputKeyboardItem = 264;
pub const TM_INPUT_KEYBOARD_ITEM_CUT: InputKeyboardItem = 265;
pub const TM_INPUT_KEYBOARD_ITEM_PASTE: InputKeyboardItem = 266;
pub const TM_INPUT_KEYBOARD_ITEM_FIND: InputKeyboardItem = 267;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPADCOMMA: InputKeyboardItem = 268;
pub const TM_INPUT_KEYBOARD_ITEM_RO: InputKeyboardItem = 269;
pub const TM_INPUT_KEYBOARD_ITEM_KATAKANAHIRAGANA: InputKeyboardItem = 270;
pub const TM_INPUT_KEYBOARD_ITEM_YEN: InputKeyboardItem = 271;
pub const TM_INPUT_KEYBOARD_ITEM_HENKAN: InputKeyboardItem = 272;
pub const TM_INPUT_KEYBOARD_ITEM_MUHENKAN: InputKeyboardItem = 273;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPADJPCOMMA: InputKeyboardItem = 274;
pub const TM_INPUT_KEYBOARD_ITEM_INTERNATIONAL_7: InputKeyboardItem = 275;
pub const TM_INPUT_KEYBOARD_ITEM_INTERNATIONAL_8: InputKeyboardItem = 276;
pub const TM_INPUT_KEYBOARD_ITEM_INTERNATIONAL_9: InputKeyboardItem = 277;
pub const TM_INPUT_KEYBOARD_ITEM_HANGEUL: InputKeyboardItem = 278;
pub const TM_INPUT_KEYBOARD_ITEM_KATAKANA: InputKeyboardItem = 279;
pub const TM_INPUT_KEYBOARD_ITEM_HIRAGANA: InputKeyboardItem = 280;
pub const TM_INPUT_KEYBOARD_ITEM_ZENKAKUHANKAKU: InputKeyboardItem = 281;
pub const TM_INPUT_KEYBOARD_ITEM_LANG_6: InputKeyboardItem = 282;
pub const TM_INPUT_KEYBOARD_ITEM_LANG_7: InputKeyboardItem = 283;
pub const TM_INPUT_KEYBOARD_ITEM_LANG_8: InputKeyboardItem = 284;
pub const TM_INPUT_KEYBOARD_ITEM_LANG_9: InputKeyboardItem = 285;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPADLEFTPAREN: InputKeyboardItem = 286;
pub const TM_INPUT_KEYBOARD_ITEM_NUMPADRIGHTPAREN: InputKeyboardItem = 287;
pub const TM_INPUT_KEYBOARD_ITEM_LEFTMETA: InputKeyboardItem = 288;
pub const TM_INPUT_KEYBOARD_ITEM_RIGHTMETA: InputKeyboardItem = 289;
pub const TM_INPUT_KEYBOARD_ITEM_MEDIA_EJECT: InputKeyboardItem = 290;
pub const TM_INPUT_KEYBOARD_ITEM_MEDIA_VOLUME_UP: InputKeyboardItem = 291;
pub const TM_INPUT_KEYBOARD_ITEM_MEDIA_VOLUME_DOWN: InputKeyboardItem = 292;
pub const TM_INPUT_KEYBOARD_ITEM_MEDIA_MUTE: InputKeyboardItem = 293;
pub const TM_INPUT_KEYBOARD_ITEM_BROWSER_WWW: InputKeyboardItem = 294;
pub const TM_INPUT_KEYBOARD_ITEM_BROWSER_SCROLLUP: InputKeyboardItem = 295;
pub const TM_INPUT_KEYBOARD_ITEM_BROWSER_SCROLLDOWN: InputKeyboardItem = 296;
pub const TM_INPUT_KEYBOARD_ITEM_BROWSER_EDIT: InputKeyboardItem = 297;
pub const TM_INPUT_KEYBOARD_ITEM_BROWSER_SLEEP: InputKeyboardItem = 298;
pub const TM_INPUT_KEYBOARD_ITEM_BROWSER_COFFEE: InputKeyboardItem = 299;
pub const TM_INPUT_KEYBOARD_ITEM_BROWSER_CALC: InputKeyboardItem = 300;
pub const TM_INPUT_KEYBOARD_ITEM_COUNT: InputKeyboardItem = 301;
pub type InputKeyboardItem = ::std::os::raw::c_int;
pub const TM_INPUT_GAMEPAD_ITEM_NONE: InputGamepadItem = 0;
pub const TM_INPUT_GAMEPAD_ITEM_BUTTON_A: InputGamepadItem = 1;
pub const TM_INPUT_GAMEPAD_ITEM_BUTTON_B: InputGamepadItem = 2;
pub const TM_INPUT_GAMEPAD_ITEM_BUTTON_X: InputGamepadItem = 3;
pub const TM_INPUT_GAMEPAD_ITEM_BUTTON_Y: InputGamepadItem = 4;
pub const TM_INPUT_GAMEPAD_ITEM_BUTTON_LEFT_SHOULDER: InputGamepadItem = 5;
pub const TM_INPUT_GAMEPAD_ITEM_BUTTON_RIGHT_SHOULDER: InputGamepadItem = 6;
pub const TM_INPUT_GAMEPAD_ITEM_BUTTON_BACK: InputGamepadItem = 7;
pub const TM_INPUT_GAMEPAD_ITEM_BUTTON_START: InputGamepadItem = 8;
pub const TM_INPUT_GAMEPAD_ITEM_DPAD_LEFT: InputGamepadItem = 9;
pub const TM_INPUT_GAMEPAD_ITEM_DPAD_RIGHT: InputGamepadItem = 10;
pub const TM_INPUT_GAMEPAD_ITEM_DPAD_UP: InputGamepadItem = 11;
pub const TM_INPUT_GAMEPAD_ITEM_DPAD_DOWN: InputGamepadItem = 12;
pub const TM_INPUT_GAMEPAD_BUTTON_LEFT_THUMB: InputGamepadItem = 13;
pub const TM_INPUT_GAMEPAD_BUTTON_RIGHT_THUMB: InputGamepadItem = 14;
pub const TM_INPUT_GAMEPAD_ITEM_BUTTON_LEFT_TRIGGER: InputGamepadItem = 15;
pub const TM_INPUT_GAMEPAD_ITEM_BUTTON_RIGHT_TRIGGER: InputGamepadItem = 16;
pub const TM_INPUT_GAMEPAD_ITEM_LEFT_STICK: InputGamepadItem = 17;
pub const TM_INPUT_GAMEPAD_ITEM_RIGHT_STICK: InputGamepadItem = 18;
pub const TM_INPUT_GAMEPAD_ITEM_COUNT: InputGamepadItem = 19;
pub type InputGamepadItem = ::std::os::raw::c_int;
pub const TM_INPUT_EVENT_TYPE_NONE: InputEventType = 0;
pub const TM_INPUT_EVENT_TYPE_DATA_CHANGE: InputEventType = 1;
pub const TM_INPUT_EVENT_TYPE_TEXT: InputEventType = 2;
pub type InputEventType = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InputItemT {
    pub id: u64,
    pub name: *const ::std::os::raw::c_char,
    pub components: u32,
    pub _padding_421: [::std::os::raw::c_char; 4usize],
}
impl Default for InputItemT {
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
pub struct InputDataT {
    pub __bindgen_anon_1: InputDataTBindgenTy1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union InputDataTBindgenTy1 {
    pub f: Vec4T,
    pub codepoint: u32,
}
impl Default for InputDataTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for InputDataT {
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
pub struct InputEventT {
    pub time: u64,
    pub source: *mut InputSourceI,
    pub controller_id: u64,
    pub item_id: u64,
    pub type_: u64,
    pub data: InputDataT,
    pub extra: *mut ::std::os::raw::c_void,
}
impl Default for InputEventT {
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
pub struct InputSourceI {
    pub controller_name: *const ::std::os::raw::c_char,
    pub controller_type: u32,
    pub _padding_479: [::std::os::raw::c_char; 4usize],
    pub controllers: ::std::option::Option<unsafe extern "C" fn(ids: *mut *mut u64) -> u32>,
    pub items: ::std::option::Option<unsafe extern "C" fn(items: *mut *mut InputItemT) -> u32>,
    pub events: ::std::option::Option<
        unsafe extern "C" fn(start: u64, events: *mut InputEventT, buffer_size: u64) -> u64,
    >,
    pub state:
        ::std::option::Option<unsafe extern "C" fn(controller: u64, item: u64) -> InputDataT>,
}
impl Default for InputSourceI {
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
pub struct InputApi {
    pub add_source: ::std::option::Option<unsafe extern "C" fn(source: *mut InputSourceI)>,
    pub remove_source: ::std::option::Option<unsafe extern "C" fn(source: *mut InputSourceI)>,
    pub sources:
        ::std::option::Option<unsafe extern "C" fn(sources: *mut *mut *mut InputSourceI) -> u32>,
    pub events: ::std::option::Option<
        unsafe extern "C" fn(start: u64, events: *mut InputEventT, buffer_size: u64) -> u64,
    >,
    pub keyboard_item_names:
        ::std::option::Option<unsafe extern "C" fn() -> *mut *const ::std::os::raw::c_char>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IntegrationTestConfigT {
    pub config: *mut ConfigI,
    pub object: ConfigItemT,
    pub _padding_33: [::std::os::raw::c_char; 4usize],
}
impl Default for IntegrationTestConfigT {
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
pub struct IntegrationTestRunnerO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IntegrationTestRunnerI {
    pub inst: *mut IntegrationTestRunnerO,
    pub context: StrhashT,
    pub config: IntegrationTestConfigT,
    pub app: *mut ApplicationO,
    pub wait: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut IntegrationTestRunnerO, sec: f32, id: u64) -> bool,
    >,
    pub record: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut IntegrationTestRunnerO,
            pass: bool,
            test_str: *const ::std::os::raw::c_char,
            file: *const ::std::os::raw::c_char,
            line: u32,
        ) -> bool,
    >,
    pub expect_error: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut IntegrationTestRunnerO,
            err: *const ::std::os::raw::c_char,
            file: *const ::std::os::raw::c_char,
            line: u32,
        ),
    >,
    pub has_errors: ::std::option::Option<unsafe extern "C" fn() -> bool>,
    pub num_errors: ::std::option::Option<unsafe extern "C" fn() -> u32>,
}
impl Default for IntegrationTestRunnerI {
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
pub struct IntegrationTestI {
    pub name: *const ::std::os::raw::c_char,
    pub path_config_json_file: *const ::std::os::raw::c_char,
    pub config_key: StrhashT,
    pub context: StrhashT,
    pub tick: ::std::option::Option<unsafe extern "C" fn(arg1: *mut IntegrationTestRunnerI)>,
}
impl Default for IntegrationTestI {
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
pub struct AtomicCounterO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JobdeclT {
    pub task: ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void)>,
    pub data: *mut ::std::os::raw::c_void,
    pub pin_thread_handle: u32,
    pub _padding_45: [::std::os::raw::c_char; 4usize],
}
impl Default for JobdeclT {
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
pub struct JobSystemApi {
    pub run_jobs: ::std::option::Option<
        unsafe extern "C" fn(jobs: *mut JobdeclT, num_jobs: u32) -> *mut AtomicCounterO,
    >,
    pub run_jobs_and_auto_free_counter:
        ::std::option::Option<unsafe extern "C" fn(jobs: *mut JobdeclT, num_jobs: u32)>,
    pub wait_for_counter:
        ::std::option::Option<unsafe extern "C" fn(counter: *mut AtomicCounterO, value: u32)>,
    pub wait_for_counter_and_free:
        ::std::option::Option<unsafe extern "C" fn(counter: *mut AtomicCounterO)>,
    pub wait_for_counter_and_free_no_fiber:
        ::std::option::Option<unsafe extern "C" fn(counter: *mut AtomicCounterO)>,
    pub pin_thread_handle:
        ::std::option::Option<unsafe extern "C" fn(worker_thread_index: u32) -> u32>,
    pub num_worker_threads: ::std::option::Option<unsafe extern "C" fn() -> u32>,
}
pub const TM_JSON_PARSE_EXT_ALLOW_UNQUOTED_KEYS: JsonParseExt = 1;
pub const TM_JSON_PARSE_EXT_ALLOW_COMMENTS: JsonParseExt = 2;
pub const TM_JSON_PARSE_EXT_IMPLICIT_ROOT_OBJECT: JsonParseExt = 4;
pub const TM_JSON_PARSE_EXT_OPTIONAL_COMMAS: JsonParseExt = 8;
pub const TM_JSON_PARSE_EXT_EQUALS_FOR_COLON: JsonParseExt = 16;
pub const TM_JSON_PARSE_EXT_LUA_QUOTING: JsonParseExt = 32;
pub type JsonParseExt = ::std::os::raw::c_int;
pub const TM_JSON_GENERATE_EXT_PREFER_UNQUOTED_KEYS: JsonGenerateExt = 1;
pub const TM_JSON_GENERATE_EXT_IMPLICIT_ROOT_OBJECT: JsonGenerateExt = 4;
pub const TM_JSON_GENERATE_EXT_NO_COMMAS: JsonGenerateExt = 8;
pub const TM_JSON_GENERATE_EXT_USE_EQUALS_INSTEAD_OF_COLON: JsonGenerateExt = 16;
pub const TM_JSON_GENERATE_EXT_USE_LUA_QUOTING: JsonGenerateExt = 32;
pub const TM_JSON_GENERATE_INLINE_SMALL_ARRAYS: JsonGenerateExt = 64;
pub const TM_JSON_GENERATE_INLINE_SMALL_OBJECTS: JsonGenerateExt = 128;
pub const TM_JSON_GENERATE_INDENT_WITH_TABS: JsonGenerateExt = 256;
pub type JsonGenerateExt = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct JsonLineInfoT {
    pub config_item: u32,
    pub line_number: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JsonParseInfoT {
    pub success: bool,
    pub error: [::std::os::raw::c_char; 80usize],
    pub _padding_128: [::std::os::raw::c_char; 3usize],
    pub num_line_info: u32,
    pub allocated_line_info: u32,
    pub line_info: [JsonLineInfoT; 1usize],
}
impl Default for JsonParseInfoT {
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
pub struct JsonGenerateT {
    pub s: *mut ::std::os::raw::c_char,
    pub len: u32,
    pub allocated: u32,
}
impl Default for JsonGenerateT {
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
pub struct JsonApi {
    pub parse: ::std::option::Option<
        unsafe extern "C" fn(
            s: *const ::std::os::raw::c_char,
            config: *mut ConfigI,
            extensions: JsonParseExt,
            error: *mut ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub parse_with_line_info: ::std::option::Option<
        unsafe extern "C" fn(
            s: *const ::std::os::raw::c_char,
            config: *mut ConfigI,
            extensions: JsonParseExt,
            ta: *mut TempAllocatorI,
        ) -> *mut JsonParseInfoT,
    >,
    pub line_number: ::std::option::Option<
        unsafe extern "C" fn(pi: *const JsonParseInfoT, item: *const ConfigItemT) -> u32,
    >,
    pub generate: ::std::option::Option<
        unsafe extern "C" fn(
            config: *mut ConfigI,
            flags: JsonGenerateExt,
            ta: *mut TempAllocatorI,
        ) -> JsonGenerateT,
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LocalizerStringsT {
    pub num_strings: u32,
    pub stride_bytes: u32,
    pub strings: *const *const ::std::os::raw::c_char,
}
impl Default for LocalizerStringsT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type LocalizerStringsI =
    ::std::option::Option<unsafe extern "C" fn(language: StrhashT) -> LocalizerStringsT>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LocalizerO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LocalizerI {
    pub inst: *mut LocalizerO,
    pub localize: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut LocalizerO,
            s: *const ::std::os::raw::c_char,
            context: *const ::std::os::raw::c_char,
        ) -> *const ::std::os::raw::c_char,
    >,
}
impl Default for LocalizerI {
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
pub struct LocalizerApi {
    pub def: *mut *mut LocalizerI,
    pub passthrough: *mut LocalizerI,
}
impl Default for LocalizerApi {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_LOG_TYPE_INFO: LogType = 0;
pub const TM_LOG_TYPE_DEBUG: LogType = 1;
pub const TM_LOG_TYPE_ERROR: LogType = 2;
pub type LogType = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LoggerO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LoggerI {
    pub inst: *mut LoggerO,
    pub log: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut LoggerO,
            log_type: LogType,
            msg: *const ::std::os::raw::c_char,
        ),
    >,
}
impl Default for LoggerI {
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
pub struct LoggerApi {
    pub add_logger: ::std::option::Option<unsafe extern "C" fn(logger: *const LoggerI)>,
    pub remove_logger: ::std::option::Option<unsafe extern "C" fn(logger: *const LoggerI)>,
    pub print: ::std::option::Option<
        unsafe extern "C" fn(log_type: LogType, msg: *const ::std::os::raw::c_char),
    >,
    pub printf: ::std::option::Option<
        unsafe extern "C" fn(
            log_type: LogType,
            format: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int,
    >,
    pub default_logger: *mut LoggerI,
}
impl Default for LoggerApi {
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
pub struct Lz4Api {
    pub compress: ::std::option::Option<
        unsafe extern "C" fn(
            src: *const ::std::os::raw::c_char,
            src_size: u32,
            dst: *mut ::std::os::raw::c_char,
            dst_capacity: u32,
        ) -> u32,
    >,
    pub decompress: ::std::option::Option<
        unsafe extern "C" fn(
            src: *const ::std::os::raw::c_char,
            src_size: u32,
            dst: *mut ::std::os::raw::c_char,
            dst_capacity: u32,
        ) -> u32,
    >,
    pub compress_bound: ::std::option::Option<unsafe extern "C" fn(src_size: u32) -> u32>,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct MathApi {
    pub mat44_multiply: ::std::option::Option<
        unsafe extern "C" fn(res: *mut Mat44T, lhs: *const Mat44T, rhs: *const Mat44T),
    >,
    pub mat44_inverse:
        ::std::option::Option<unsafe extern "C" fn(res: *mut Mat44T, m: *const Mat44T)>,
    pub mat44_determinant: ::std::option::Option<unsafe extern "C" fn(m: *const Mat44T) -> f32>,
    pub mat44_determinant33: ::std::option::Option<unsafe extern "C" fn(m: *const Mat44T) -> f32>,
    pub mat44_to_quaternion: ::std::option::Option<unsafe extern "C" fn(m: *const Mat44T) -> Vec4T>,
    pub mat44_to_translation_quaternion_scale: ::std::option::Option<
        unsafe extern "C" fn(t: *mut Vec3T, r: *mut Vec4T, s: *mut Vec3T, m: *const Mat44T),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryTrackerScopeDataT {
    pub desc: *const ::std::os::raw::c_char,
    pub allocated_bytes: u64,
    pub allocation_count: u64,
    pub parent: u32,
    pub num_children: u32,
    pub tracing_enabled: bool,
    pub _padding_44: [::std::os::raw::c_char; 3usize],
    pub trace_count: u32,
}
impl Default for MemoryTrackerScopeDataT {
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
pub struct MemoryTrackerTraceDataT {
    pub file: *const ::std::os::raw::c_char,
    pub line: u32,
    pub scope: u32,
    pub allocated_bytes: u64,
    pub allocation_count: u64,
}
impl Default for MemoryTrackerTraceDataT {
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
pub struct MemoryTrackerApi {
    pub check_for_leaked_scopes: ::std::option::Option<unsafe extern "C" fn()>,
    pub create_scope: ::std::option::Option<
        unsafe extern "C" fn(desc: *const ::std::os::raw::c_char, parent_scope: u32) -> u32,
    >,
    pub destroy_scope: ::std::option::Option<unsafe extern "C" fn(s: u32)>,
    pub destroy_scope_allowing_leaks:
        ::std::option::Option<unsafe extern "C" fn(scope: u32, max_leaked_bytes: u64)>,
    pub record_realloc: ::std::option::Option<
        unsafe extern "C" fn(
            old_ptr: *mut ::std::os::raw::c_void,
            old_size: u64,
            new_ptr: *mut ::std::os::raw::c_void,
            new_size: u64,
            file: *const ::std::os::raw::c_char,
            line: u32,
            scope: u32,
        ),
    >,
    pub allocated_bytes: ::std::option::Option<unsafe extern "C" fn(scope: u32) -> u64>,
    pub allocation_count: ::std::option::Option<unsafe extern "C" fn(scope: u32) -> u64>,
    pub set_scope_tracing: ::std::option::Option<unsafe extern "C" fn(scope: u32, enabled: bool)>,
    pub scope_data_snapshot: ::std::option::Option<
        unsafe extern "C" fn(ta: *mut TempAllocatorI) -> *mut MemoryTrackerScopeDataT,
    >,
    pub trace_data_snapshot: ::std::option::Option<
        unsafe extern "C" fn(ta: *mut TempAllocatorI) -> *mut MemoryTrackerTraceDataT,
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct OsVirtualMemoryApi {
    pub map: ::std::option::Option<unsafe extern "C" fn(size: u64) -> *mut ::std::os::raw::c_void>,
    pub unmap:
        ::std::option::Option<unsafe extern "C" fn(p: *mut ::std::os::raw::c_void, size: u64)>,
    pub reserve:
        ::std::option::Option<unsafe extern "C" fn(size: u64) -> *mut ::std::os::raw::c_void>,
    pub commit:
        ::std::option::Option<unsafe extern "C" fn(p: *mut ::std::os::raw::c_void, size: u64)>,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct FileO {
    pub handle: u64,
    pub valid: bool,
    pub _padding_53: [::std::os::raw::c_char; 7usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct FileTimeO {
    pub opaque: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct OsFileIoApi {
    pub open_input:
        ::std::option::Option<unsafe extern "C" fn(path: *const ::std::os::raw::c_char) -> FileO>,
    pub open_output:
        ::std::option::Option<unsafe extern "C" fn(path: *const ::std::os::raw::c_char) -> FileO>,
    pub open_append:
        ::std::option::Option<unsafe extern "C" fn(path: *const ::std::os::raw::c_char) -> FileO>,
    pub set_position: ::std::option::Option<unsafe extern "C" fn(file: FileO, pos: u64)>,
    pub size: ::std::option::Option<unsafe extern "C" fn(file: FileO) -> u64>,
    pub read: ::std::option::Option<
        unsafe extern "C" fn(file: FileO, buffer: *mut ::std::os::raw::c_void, size: u64) -> i64,
    >,
    pub write: ::std::option::Option<
        unsafe extern "C" fn(file: FileO, buffer: *const ::std::os::raw::c_void, size: u64) -> bool,
    >,
    pub read_at: ::std::option::Option<
        unsafe extern "C" fn(
            file: FileO,
            offset: u64,
            buffer: *mut ::std::os::raw::c_void,
            size: u64,
        ) -> i64,
    >,
    pub write_at: ::std::option::Option<
        unsafe extern "C" fn(
            file: FileO,
            offset: u64,
            buffer: *const ::std::os::raw::c_void,
            size: u64,
        ) -> bool,
    >,
    pub set_last_modified_time:
        ::std::option::Option<unsafe extern "C" fn(file: FileO, time: FileTimeO)>,
    pub close: ::std::option::Option<unsafe extern "C" fn(file: FileO)>,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct FileStatT {
    pub exists: bool,
    pub is_directory: bool,
    pub _padding_125: [::std::os::raw::c_char; 6usize],
    pub last_modified_time: FileTimeO,
    pub size: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct StringsT {
    pub count: u32,
    pub bytes: u32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct FileSystemWatcherO {
    pub opaque: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FileSystemDetailedWatcherO {
    _unused: [u8; 0],
}
pub const TM_FILE_SYSTEM_CHANGE_ACTION__ADDED: FileSystemChangeAction = 0;
pub const TM_FILE_SYSTEM_CHANGE_ACTION__REMOVED: FileSystemChangeAction = 1;
pub const TM_FILE_SYSTEM_CHANGE_ACTION__MODIFIED: FileSystemChangeAction = 2;
pub type FileSystemChangeAction = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FileSystemChangeT {
    pub relative_path: *const ::std::os::raw::c_char,
    pub action: FileSystemChangeAction,
    pub _padding_186: [::std::os::raw::c_char; 4usize],
}
impl Default for FileSystemChangeT {
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
pub struct OsFileSystemApi {
    pub stat: ::std::option::Option<
        unsafe extern "C" fn(path: *const ::std::os::raw::c_char) -> FileStatT,
    >,
    pub directory_entries: ::std::option::Option<
        unsafe extern "C" fn(
            path: *const ::std::os::raw::c_char,
            ta: *mut TempAllocatorI,
        ) -> *mut StringsT,
    >,
    pub make_directory:
        ::std::option::Option<unsafe extern "C" fn(path: *const ::std::os::raw::c_char) -> bool>,
    pub remove_file:
        ::std::option::Option<unsafe extern "C" fn(path: *const ::std::os::raw::c_char) -> bool>,
    pub remove_directory:
        ::std::option::Option<unsafe extern "C" fn(path: *const ::std::os::raw::c_char) -> bool>,
    pub rename: ::std::option::Option<
        unsafe extern "C" fn(
            old_name: *const ::std::os::raw::c_char,
            new_name: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub copy_file: ::std::option::Option<
        unsafe extern "C" fn(
            from: *const ::std::os::raw::c_char,
            to: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub getcwd: ::std::option::Option<
        unsafe extern "C" fn(ta: *mut TempAllocatorI) -> *const ::std::os::raw::c_char,
    >,
    pub chdir:
        ::std::option::Option<unsafe extern "C" fn(path: *const ::std::os::raw::c_char) -> bool>,
    pub is_absolute:
        ::std::option::Option<unsafe extern "C" fn(path: *const ::std::os::raw::c_char) -> bool>,
    pub absolute: ::std::option::Option<
        unsafe extern "C" fn(
            path: *const ::std::os::raw::c_char,
            ta: *mut TempAllocatorI,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub temp_directory: ::std::option::Option<
        unsafe extern "C" fn(ta: *mut TempAllocatorI) -> *const ::std::os::raw::c_char,
    >,
    pub create_watcher: ::std::option::Option<
        unsafe extern "C" fn(subtree_path: *const ::std::os::raw::c_char) -> FileSystemWatcherO,
    >,
    pub any_changes:
        ::std::option::Option<unsafe extern "C" fn(watcher: FileSystemWatcherO) -> bool>,
    pub destroy_watcher: ::std::option::Option<unsafe extern "C" fn(watcher: FileSystemWatcherO)>,
    pub create_detailed_watcher: ::std::option::Option<
        unsafe extern "C" fn(
            subtree_path: *const ::std::os::raw::c_char,
        ) -> *mut FileSystemDetailedWatcherO,
    >,
    pub detailed_changes: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut FileSystemDetailedWatcherO,
            ta: *mut TempAllocatorI,
        ) -> *mut FileSystemChangeT,
    >,
    pub destroy_detailed_watcher:
        ::std::option::Option<unsafe extern "C" fn(watcher: *mut FileSystemDetailedWatcherO)>,
    pub app_folder: ::std::option::Option<
        unsafe extern "C" fn(ta: *mut TempAllocatorI) -> *const ::std::os::raw::c_char,
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct DllO {
    pub handle: u64,
    pub valid: bool,
    pub _padding_282: [::std::os::raw::c_char; 7usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct OsDllApi {
    pub open:
        ::std::option::Option<unsafe extern "C" fn(path: *const ::std::os::raw::c_char) -> DllO>,
    pub get:
        ::std::option::Option<unsafe extern "C" fn(path: *const ::std::os::raw::c_char) -> DllO>,
    pub sym: ::std::option::Option<
        unsafe extern "C" fn(
            handle: DllO,
            name: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub close: ::std::option::Option<unsafe extern "C" fn(handle: DllO)>,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct SocketO {
    pub handle: u64,
    pub valid: bool,
    pub _padding_313: [::std::os::raw::c_char; 7usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SocketAddressT {
    pub __bindgen_anon_1: SocketAddressTBindgenTy1,
    pub port: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SocketAddressTBindgenTy1 {
    pub ip: u32,
    pub ip_byte: [u8; 4usize],
}
impl Default for SocketAddressTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for SocketAddressT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_OS_SOCKET_TYPE_TCP: OsSocketType = 1;
pub const TM_OS_SOCKET_TYPE_UDP: OsSocketType = 2;
pub type OsSocketType = ::std::os::raw::c_int;
pub const TM_OS_SOCKET_ERROR_WOULD_BLOCK: OsSocketError = -1000;
pub const TM_OS_SOCKET_ERROR_CLOSED: OsSocketError = -1001;
pub const TM_OS_SOCKET_ERROR_INVALID: OsSocketError = -1002;
pub const TM_OS_SOCKET_ERROR_OTHER: OsSocketError = -1;
pub type OsSocketError = ::std::os::raw::c_int;
pub const TM_OS_SOCKET_CONNECT_PENDING: OsSocketConnect = 0;
pub const TM_OS_SOCKET_CONNECT_ESTABLISHED: OsSocketConnect = 1;
pub const TM_OS_SOCKET_CONNECT_FAILED: OsSocketConnect = 2;
pub type OsSocketConnect = ::std::os::raw::c_int;
pub const TM_OS_SOCKET_GETADDRINFO_IN_PROGRESS: OsSocketGetaddrinfo = 0;
pub const TM_OS_SOCKET_GETADDRINFO_SUCCESS: OsSocketGetaddrinfo = 1;
pub const TM_OS_SOCKET_GETADDRINFO_ERROR: OsSocketGetaddrinfo = 2;
pub type OsSocketGetaddrinfo = ::std::os::raw::c_int;
pub const TM_SOCKET_OPTION__NODELAY: OsSocketOption = 0;
pub const TM_SOCKET_OPTION__NONBLOCK: OsSocketOption = 1;
pub type OsSocketOption = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct OsSocketApi {
    pub init: ::std::option::Option<unsafe extern "C" fn()>,
    pub shutdown: ::std::option::Option<unsafe extern "C" fn()>,
    pub socket: ::std::option::Option<unsafe extern "C" fn(type_: OsSocketType) -> SocketO>,
    pub set_option: ::std::option::Option<
        unsafe extern "C" fn(socket: SocketO, option: OsSocketOption, enabled: bool),
    >,
    pub bind: ::std::option::Option<
        unsafe extern "C" fn(socket: SocketO, address: SocketAddressT) -> bool,
    >,
    pub getsockname: ::std::option::Option<
        unsafe extern "C" fn(socket: SocketO, address: *mut SocketAddressT) -> bool,
    >,
    pub listen:
        ::std::option::Option<unsafe extern "C" fn(socket: SocketO, queue_size: u32) -> bool>,
    pub accept: ::std::option::Option<
        unsafe extern "C" fn(socket: SocketO, address: *mut SocketAddressT) -> SocketO,
    >,
    pub connect: ::std::option::Option<
        unsafe extern "C" fn(socket: SocketO, target: SocketAddressT) -> OsSocketConnect,
    >,
    pub send: ::std::option::Option<
        unsafe extern "C" fn(
            socket: SocketO,
            buffer: *const ::std::os::raw::c_void,
            size: u32,
        ) -> i32,
    >,
    pub recv: ::std::option::Option<
        unsafe extern "C" fn(
            socket: SocketO,
            buffer: *mut ::std::os::raw::c_void,
            size: u32,
        ) -> i32,
    >,
    pub sendto: ::std::option::Option<
        unsafe extern "C" fn(
            socket: SocketO,
            buffer: *const ::std::os::raw::c_void,
            size: u32,
            target: SocketAddressT,
        ) -> i32,
    >,
    pub recvfrom: ::std::option::Option<
        unsafe extern "C" fn(
            socket: SocketO,
            buffer: *mut ::std::os::raw::c_void,
            size: u32,
            source: *mut SocketAddressT,
        ) -> i32,
    >,
    pub close: ::std::option::Option<unsafe extern "C" fn(socket: SocketO) -> bool>,
    pub getaddrinfo: ::std::option::Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            service: *const ::std::os::raw::c_char,
            addresses: *mut SocketAddressT,
            size: u32,
        ) -> u32,
    >,
    pub getaddrinfo_async: ::std::option::Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            service: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub getaddrinfo_result: ::std::option::Option<
        unsafe extern "C" fn(
            query: *mut ::std::os::raw::c_void,
            addresses: *mut SocketAddressT,
            count: *mut u32,
        ) -> OsSocketGetaddrinfo,
    >,
}
pub const TM_OS_THREAD__PRIORITY__LOWEST: OsThreadPriority = 0;
pub const TM_OS_THREAD__PRIOIRTY__LOW: OsThreadPriority = 1;
pub const TM_OS_THREAD__PRIORITY__NORMAL: OsThreadPriority = 2;
pub const TM_OS_THREAD__PRIORITY__HIGH: OsThreadPriority = 3;
pub const TM_OS_THREAD__PRIORITY__HIGHEST: OsThreadPriority = 4;
pub const TM_OS_THREAD__PRIORITY__TIME_CRITICAL: OsThreadPriority = 5;
pub type OsThreadPriority = ::std::os::raw::c_int;
pub type ThreadEntryF =
    ::std::option::Option<unsafe extern "C" fn(user_data: *mut ::std::os::raw::c_void)>;
pub type FiberEntryF =
    ::std::option::Option<unsafe extern "C" fn(user_data: *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct CriticalSectionO {
    pub opaque: [u64; 8usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct SemaphoreO {
    pub opaque: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ThreadO {
    pub opaque: [u64; 2usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct FiberO {
    pub opaque: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct OsThreadApi {
    pub create_critical_section:
        ::std::option::Option<unsafe extern "C" fn(cs: *mut CriticalSectionO)>,
    pub enter_critical_section:
        ::std::option::Option<unsafe extern "C" fn(cs: *mut CriticalSectionO)>,
    pub leave_critical_section:
        ::std::option::Option<unsafe extern "C" fn(cs: *mut CriticalSectionO)>,
    pub destroy_critical_section:
        ::std::option::Option<unsafe extern "C" fn(cs: *mut CriticalSectionO)>,
    pub create_semaphore:
        ::std::option::Option<unsafe extern "C" fn(initial_count: u32) -> SemaphoreO>,
    pub semaphore_add: ::std::option::Option<unsafe extern "C" fn(sem: SemaphoreO, count: u32)>,
    pub semaphore_wait: ::std::option::Option<unsafe extern "C" fn(sem: SemaphoreO)>,
    pub semaphore_poll: ::std::option::Option<unsafe extern "C" fn(sem: SemaphoreO) -> bool>,
    pub destroy_semaphore: ::std::option::Option<unsafe extern "C" fn(sem: SemaphoreO)>,
    pub thread_id: ::std::option::Option<unsafe extern "C" fn() -> u32>,
    pub processor_id: ::std::option::Option<unsafe extern "C" fn() -> u32>,
    pub create_thread: ::std::option::Option<
        unsafe extern "C" fn(
            entry: ThreadEntryF,
            user_data: *mut ::std::os::raw::c_void,
            stack_size: u32,
            debug_name: *const ::std::os::raw::c_char,
        ) -> ThreadO,
    >,
    pub set_thread_priority:
        ::std::option::Option<unsafe extern "C" fn(thread: ThreadO, priority: OsThreadPriority)>,
    pub wait_for_thread: ::std::option::Option<unsafe extern "C" fn(thread: ThreadO)>,
    pub thread_id_from_tm_thread:
        ::std::option::Option<unsafe extern "C" fn(thread: ThreadO) -> u32>,
    pub convert_thread_to_fiber: ::std::option::Option<
        unsafe extern "C" fn(user_data: *mut ::std::os::raw::c_void) -> FiberO,
    >,
    pub convert_fiber_to_thread: ::std::option::Option<unsafe extern "C" fn()>,
    pub create_fiber: ::std::option::Option<
        unsafe extern "C" fn(
            entry: FiberEntryF,
            user_data: *mut ::std::os::raw::c_void,
            stack_size: u32,
        ) -> FiberO,
    >,
    pub destroy_fiber: ::std::option::Option<unsafe extern "C" fn(fiber: FiberO)>,
    pub switch_to_fiber: ::std::option::Option<unsafe extern "C" fn(fiber: FiberO)>,
    pub fiber_user_data:
        ::std::option::Option<unsafe extern "C" fn() -> *mut ::std::os::raw::c_void>,
    pub yield_processor: ::std::option::Option<unsafe extern "C" fn()>,
    pub sleep: ::std::option::Option<unsafe extern "C" fn(seconds: f64)>,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct OsTimeApi {
    pub now: ::std::option::Option<unsafe extern "C" fn() -> ClockO>,
    pub delta: ::std::option::Option<unsafe extern "C" fn(to: ClockO, from: ClockO) -> f64>,
    pub add: ::std::option::Option<unsafe extern "C" fn(from: ClockO, delta: f64) -> ClockO>,
    pub file_time_now: ::std::option::Option<unsafe extern "C" fn() -> FileTimeO>,
    pub file_time_delta:
        ::std::option::Option<unsafe extern "C" fn(to: FileTimeO, from: FileTimeO) -> f64>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsDialogsOpenT {
    pub extensions: *const ::std::os::raw::c_char,
    pub description: *const ::std::os::raw::c_char,
    pub allow_multi_select: bool,
    pub _padding_710: [::std::os::raw::c_char; 7usize],
}
impl Default for OsDialogsOpenT {
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
pub struct OsDialogsOpenResT {
    pub num_files: u32,
    pub _padding_718: [::std::os::raw::c_char; 4usize],
    pub files: *mut *mut ::std::os::raw::c_char,
}
impl Default for OsDialogsOpenResT {
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
pub struct OsDialogsSaveT {
    pub default_name: *const ::std::os::raw::c_char,
}
impl Default for OsDialogsSaveT {
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
pub struct OsDialogsApi {
    pub open: ::std::option::Option<
        unsafe extern "C" fn(
            s: *const OsDialogsOpenT,
            ta: *mut TempAllocatorI,
        ) -> OsDialogsOpenResT,
    >,
    pub open_folder: ::std::option::Option<
        unsafe extern "C" fn(ta: *mut TempAllocatorI) -> *mut ::std::os::raw::c_char,
    >,
    pub save: ::std::option::Option<
        unsafe extern "C" fn(
            s: *const OsDialogsSaveT,
            ta: *mut TempAllocatorI,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub message_box: ::std::option::Option<
        unsafe extern "C" fn(
            title: *const ::std::os::raw::c_char,
            text: *const ::std::os::raw::c_char,
        ),
    >,
    pub show_count: ::std::option::Option<unsafe extern "C" fn() -> u64>,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct OsInfoApi {
    pub num_logical_processors: ::std::option::Option<unsafe extern "C" fn() -> u32>,
    pub avx_support: ::std::option::Option<unsafe extern "C" fn() -> bool>,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct OsDebuggerApi {
    pub is_debugger_present: ::std::option::Option<unsafe extern "C" fn() -> bool>,
    pub debug_break: ::std::option::Option<unsafe extern "C" fn()>,
    pub print_stack_trace: ::std::option::Option<unsafe extern "C" fn()>,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct OsSystemApi {
    pub open_url: ::std::option::Option<unsafe extern "C" fn(url: *const ::std::os::raw::c_char)>,
    pub open_file:
        ::std::option::Option<unsafe extern "C" fn(file: *const ::std::os::raw::c_char) -> bool>,
    pub exe_path: ::std::option::Option<
        unsafe extern "C" fn(
            argv_0: *const ::std::os::raw::c_char,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub execute: ::std::option::Option<
        unsafe extern "C" fn(command: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
    pub execute_in_background: ::std::option::Option<
        unsafe extern "C" fn(command: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
    pub execute_stdout: ::std::option::Option<
        unsafe extern "C" fn(
            command: *const ::std::os::raw::c_char,
            timeout_ms: u32,
            ta: *mut TempAllocatorI,
            exit_code: *mut ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char,
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsApi {
    pub virtual_memory: *mut OsVirtualMemoryApi,
    pub file_io: *mut OsFileIoApi,
    pub file_system: *mut OsFileSystemApi,
    pub dll: *mut OsDllApi,
    pub socket: *mut OsSocketApi,
    pub thread: *mut OsThreadApi,
    pub time: *mut OsTimeApi,
    pub dialogs: *mut OsDialogsApi,
    pub info: *mut OsInfoApi,
    pub debugger: *mut OsDebuggerApi,
    pub system: *mut OsSystemApi,
}
impl Default for OsApi {
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
pub struct PathApi {
    pub extension: ::std::option::Option<unsafe extern "C" fn(path: StrT) -> StrT>,
    pub extension_cstr: ::std::option::Option<
        unsafe extern "C" fn(path: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char,
    >,
    pub strip_extension: ::std::option::Option<unsafe extern "C" fn(path: StrT) -> StrT>,
    pub base: ::std::option::Option<unsafe extern "C" fn(path: StrT) -> StrT>,
    pub base_cstr: ::std::option::Option<
        unsafe extern "C" fn(path: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char,
    >,
    pub directory: ::std::option::Option<unsafe extern "C" fn(path: StrT) -> StrT>,
    pub pop_last: ::std::option::Option<unsafe extern "C" fn(path: *mut StrT) -> StrT>,
    pub pop_first: ::std::option::Option<unsafe extern "C" fn(path: *mut StrT) -> StrT>,
    pub split_all: ::std::option::Option<
        unsafe extern "C" fn(path: StrT, ta: *mut TempAllocatorI) -> *mut StrT,
    >,
    pub join: ::std::option::Option<
        unsafe extern "C" fn(a: StrT, b: StrT, ta: *mut TempAllocatorI) -> StrT,
    >,
}
pub type PluginLoadF =
    ::std::option::Option<unsafe extern "C" fn(reg: *mut ApiRegistryApi, load: bool)>;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct PluginsApi {
    pub load: ::std::option::Option<
        unsafe extern "C" fn(path: *const ::std::os::raw::c_char, hot_reload: bool) -> u64,
    >,
    pub unload: ::std::option::Option<unsafe extern "C" fn(plugin: u64)>,
    pub reload: ::std::option::Option<unsafe extern "C" fn(plugin: u64)>,
    pub set_path: ::std::option::Option<
        unsafe extern "C" fn(plugin: u64, path: *const ::std::os::raw::c_char),
    >,
    pub check_hot_reload: ::std::option::Option<unsafe extern "C" fn() -> bool>,
    pub reload_count: ::std::option::Option<unsafe extern "C" fn() -> u64>,
    pub enumerate: ::std::option::Option<
        unsafe extern "C" fn(
            directory: *const ::std::os::raw::c_char,
            ta: *mut TempAllocatorI,
        ) -> *mut *const ::std::os::raw::c_char,
    >,
    pub plugin_dllpath: ::std::option::Option<
        unsafe extern "C" fn(
            ta: *mut TempAllocatorI,
            exe: *const ::std::os::raw::c_char,
            name: *const ::std::os::raw::c_char,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub app_dllpath: ::std::option::Option<
        unsafe extern "C" fn(
            ta: *mut TempAllocatorI,
            exe: *const ::std::os::raw::c_char,
            name: *const ::std::os::raw::c_char,
        ) -> *const ::std::os::raw::c_char,
    >,
}
pub const TM_TT_PROP__PLUGIN__IMPORT_ON_CHANGE: TM_TT_PROP__PLUGIN = 0;
pub const TM_TT_PROP__PLUGIN__WINDOWS_DLL_IMPORT_PATH: TM_TT_PROP__PLUGIN = 1;
pub const TM_TT_PROP__PLUGIN__WINDOWS_DLL: TM_TT_PROP__PLUGIN = 2;
pub const TM_TT_PROP__PLUGIN__LINUX_DLL_IMPORT_PATH: TM_TT_PROP__PLUGIN = 3;
pub const TM_TT_PROP__PLUGIN__LINUX_DLL: TM_TT_PROP__PLUGIN = 4;
pub const TM_TT_PROP__PLUGIN__IS_HELPER: TM_TT_PROP__PLUGIN = 5;
pub type TM_TT_PROP__PLUGIN = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct PluginAssetsApi {
    pub any_plugins: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO) -> bool>,
    pub init_truth:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO, allow_code_execution: bool)>,
    pub shutdown_truth: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
    pub update_truth: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
    pub allow_code_execution:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO) -> bool>,
    pub set_allow_code_execution:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO, allow_code_execution: bool)>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PluginInitI {
    pub inst: *mut PluginO,
    pub init:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut PluginO, allocator: *mut AllocatorI)>,
}
impl Default for PluginInitI {
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
pub struct PluginShutdownI {
    pub inst: *mut PluginO,
    pub shutdown: ::std::option::Option<unsafe extern "C" fn(inst: *mut PluginO)>,
}
impl Default for PluginShutdownI {
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
pub struct PluginTickI {
    pub inst: *mut PluginO,
    pub tick: ::std::option::Option<unsafe extern "C" fn(inst: *mut PluginO, dt: f32)>,
}
impl Default for PluginTickI {
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
pub struct PluginReloadI {
    pub inst: *mut PluginO,
    pub reload: ::std::option::Option<unsafe extern "C" fn(inst: *mut PluginO)>,
}
impl Default for PluginReloadI {
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
pub struct PluginSetTheTruthI {
    pub inst: *mut PluginO,
    pub set_the_truth:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut PluginO, tt: *mut TheTruthO)>,
}
impl Default for PluginSetTheTruthI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_PROFILER_EVENT_TYPE_BEGIN: ProfilerEventType = 0;
pub const TM_PROFILER_EVENT_TYPE_END: ProfilerEventType = 1;
pub const TM_PROFILER_EVENT_TYPE_INSTANT: ProfilerEventType = 2;
pub const TM_PROFILER_EVENT_TYPE_START: ProfilerEventType = 3;
pub const TM_PROFILER_EVENT_TYPE_FINISH: ProfilerEventType = 4;
pub const TM_PROFILER_EVENT_TYPE_FIBER_SWITCH: ProfilerEventType = 5;
pub type ProfilerEventType = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProfilerEventT {
    pub type_: u32,
    pub thread_id: u32,
    pub time_stamp: u64,
    pub __bindgen_anon_1: ProfilerEventTBindgenTy1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ProfilerEventTBindgenTy1 {
    pub __bindgen_anon_1: ProfilerEventTBindgenTy1BindgenTy1,
    pub __bindgen_anon_2: ProfilerEventTBindgenTy1BindgenTy2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProfilerEventTBindgenTy1BindgenTy1 {
    pub id: u32,
    pub _padding_51: [::std::os::raw::c_char; 4usize],
    pub category: *const ::std::os::raw::c_char,
    pub name: *const ::std::os::raw::c_char,
    pub object: *const ::std::os::raw::c_char,
}
impl Default for ProfilerEventTBindgenTy1BindgenTy1 {
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
pub struct ProfilerEventTBindgenTy1BindgenTy2 {
    pub from_fiber: u32,
    pub to_fiber: u32,
}
impl Default for ProfilerEventTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for ProfilerEventT {
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
pub struct ProfilerBufferT {
    pub total_recorded: u64,
    pub start: [u64; 2usize],
    pub events: [*const ProfilerEventT; 2usize],
    pub count: [u32; 2usize],
}
impl Default for ProfilerBufferT {
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
pub struct ProfilerApi {
    pub init: ::std::option::Option<
        unsafe extern "C" fn(allocator: *mut AllocatorI, event_buffer_size: u32),
    >,
    pub shutdown: ::std::option::Option<unsafe extern "C" fn()>,
    pub enabled: *mut bool,
    pub begin: ::std::option::Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            category: *const ::std::os::raw::c_char,
            object: *const ::std::os::raw::c_char,
        ) -> u64,
    >,
    pub end: ::std::option::Option<unsafe extern "C" fn(begin_id: u64)>,
    pub instant: ::std::option::Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            category: *const ::std::os::raw::c_char,
            object: *const ::std::os::raw::c_char,
        ),
    >,
    pub start: ::std::option::Option<
        unsafe extern "C" fn(
            name: *const ::std::os::raw::c_char,
            category: *const ::std::os::raw::c_char,
            object: *const ::std::os::raw::c_char,
        ) -> u64,
    >,
    pub finish: ::std::option::Option<unsafe extern "C" fn(start_id: u64)>,
    pub intern: ::std::option::Option<
        unsafe extern "C" fn(s: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char,
    >,
    pub fiber_switch: ::std::option::Option<unsafe extern "C" fn(from_fiber: u32, to_fiber: u32)>,
    pub submit:
        ::std::option::Option<unsafe extern "C" fn(events: *mut ProfilerEventT, count: u32)>,
    pub copy: ::std::option::Option<
        unsafe extern "C" fn(
            dest: *mut ProfilerEventT,
            start: u64,
            count: u32,
            actual_start: *mut u64,
            actual_count: *mut u32,
        ),
    >,
    pub buffer: ::std::option::Option<unsafe extern "C" fn() -> ProfilerBufferT>,
}
impl Default for ProfilerApi {
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
pub struct TaskProgressT {
    pub num_tasks: u32,
    pub task_index: u32,
    pub text: *const ::std::os::raw::c_char,
    pub fraction: f32,
    pub _padding_32: [::std::os::raw::c_char; 4usize],
}
impl Default for TaskProgressT {
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
pub struct TaskStatusT {
    pub num_tasks: u32,
    pub _padding_40: [::std::os::raw::c_char; 4usize],
    pub task_id: *mut u64,
    pub text: *mut *const ::std::os::raw::c_char,
    pub fraction: *mut f32,
}
impl Default for TaskStatusT {
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
pub struct ProgressReportApi {
    pub create:
        ::std::option::Option<unsafe extern "C" fn(a: *mut AllocatorI, task_display_time: f32)>,
    pub destroy: ::std::option::Option<unsafe extern "C" fn()>,
    pub update: ::std::option::Option<
        unsafe extern "C" fn(dt: f64, ta: *mut TempAllocatorI) -> TaskProgressT,
    >,
    pub status: ::std::option::Option<unsafe extern "C" fn(ta: *mut TempAllocatorI) -> TaskStatusT>,
    pub idle: ::std::option::Option<unsafe extern "C" fn() -> bool>,
    pub set_task_progress: ::std::option::Option<
        unsafe extern "C" fn(task: u64, text: *const ::std::os::raw::c_char, fraction: f32),
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RandomApi {
    pub next: ::std::option::Option<unsafe extern "C" fn() -> u64>,
    pub next_n: ::std::option::Option<unsafe extern "C" fn(res: *mut u64, n: u32)>,
    pub seed_new_state: ::std::option::Option<unsafe extern "C" fn(s: *mut u64)>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RuntimeDataT {
    pub version: u64,
    pub data: *mut ::std::os::raw::c_void,
}
impl Default for RuntimeDataT {
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
pub struct RuntimeDataIoI {
    pub load: ::std::option::Option<
        unsafe extern "C" fn(
            io_context: *mut ::std::os::raw::c_void,
            tt: *mut TheTruthO,
            id: TtIdT,
            result_runtime_data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub unload: ::std::option::Option<
        unsafe extern "C" fn(
            io_context: *mut ::std::os::raw::c_void,
            runtime_data: *mut ::std::os::raw::c_void,
        ),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RuntimeDataRepositoryO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RuntimeDataRepositoryApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            allocator: *mut AllocatorI,
            tt: *mut TheTruthO,
            type_: TtTypeT,
            runtime_data_size: u32,
            io_interface: *mut RuntimeDataIoI,
        ) -> *mut RuntimeDataRepositoryO,
    >,
    pub destroy: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RuntimeDataRepositoryO,
            io_context: *mut ::std::os::raw::c_void,
        ),
    >,
    pub lookup: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RuntimeDataRepositoryO,
            id: TtIdT,
            wanted_version: u64,
            io_context: *mut ::std::os::raw::c_void,
        ) -> RuntimeDataT,
    >,
    pub garbage_collect: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RuntimeDataRepositoryO,
            io_context: *mut ::std::os::raw::c_void,
        ),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SHA1_CTX {
    pub state: [u32; 5usize],
    pub count: [u32; 2usize],
    pub buffer: [u8; 64usize],
}
impl Default for SHA1_CTX {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type SprintfPrinter = ::std::option::Option<
    unsafe extern "C" fn(
        buf: *mut ::std::os::raw::c_char,
        count: ::std::os::raw::c_int,
        type_: StrT,
        args: StrT,
        data: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct SprintfApi {
    pub print_unsafe: ::std::option::Option<
        unsafe extern "C" fn(
            buf: *mut ::std::os::raw::c_char,
            fmt: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int,
    >,
    pub print: ::std::option::Option<
        unsafe extern "C" fn(
            buf: *mut ::std::os::raw::c_char,
            count: ::std::os::raw::c_int,
            fmt: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int,
    >,
    pub vprint_unsafe: ::std::option::Option<
        unsafe extern "C" fn(
            buf: *mut ::std::os::raw::c_char,
            fmt: *const ::std::os::raw::c_char,
            va: va_list,
        ) -> ::std::os::raw::c_int,
    >,
    pub vprint: ::std::option::Option<
        unsafe extern "C" fn(
            buf: *mut ::std::os::raw::c_char,
            count: ::std::os::raw::c_int,
            fmt: *const ::std::os::raw::c_char,
            va: va_list,
        ) -> ::std::os::raw::c_int,
    >,
    pub add_printer: ::std::option::Option<
        unsafe extern "C" fn(name: *const ::std::os::raw::c_char, printer: SprintfPrinter),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetStrhashT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct StringApi {
    pub find_unique_name: ::std::option::Option<
        unsafe extern "C" fn(
            taken_names: *mut SetStrhashT,
            ignore_case: bool,
            desired_name: *const ::std::os::raw::c_char,
            separator: *const ::std::os::raw::c_char,
            ta: *mut TempAllocatorI,
        ) -> *const ::std::os::raw::c_char,
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StringRepositoryI {
    pub inst: *mut StringRepositoryO,
    pub add: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut StringRepositoryO,
            s: *const ::std::os::raw::c_char,
        ) -> StrhashT,
    >,
    pub add_str: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut StringRepositoryO, s: StrT) -> StrhashT,
    >,
    pub retain:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut StringRepositoryO, hash: StrhashT)>,
    pub remove:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut StringRepositoryO, hash: StrhashT)>,
    pub lookup: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut StringRepositoryO,
            hash: StrhashT,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub intern: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut StringRepositoryO,
            s: *const ::std::os::raw::c_char,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub intern_str:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut StringRepositoryO, s: StrT) -> StrT>,
}
impl Default for StringRepositoryI {
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
pub struct StringRepositoryApi {
    pub create:
        ::std::option::Option<unsafe extern "C" fn(a: *mut AllocatorI) -> *mut StringRepositoryI>,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(i: *mut StringRepositoryI)>,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct TaskSystemApi {
    pub run_task: ::std::option::Option<
        unsafe extern "C" fn(
            f: ::std::option::Option<
                unsafe extern "C" fn(data: *mut ::std::os::raw::c_void, id: u64),
            >,
            data: *mut ::std::os::raw::c_void,
            debug_name: *const ::std::os::raw::c_char,
        ) -> u64,
    >,
    pub is_task_done: ::std::option::Option<unsafe extern "C" fn(id: u64) -> bool>,
    pub is_task_done_else_assist: ::std::option::Option<unsafe extern "C" fn(id: u64) -> bool>,
    pub cancel_task: ::std::option::Option<unsafe extern "C" fn(id: u64)>,
    pub is_task_canceled: ::std::option::Option<unsafe extern "C" fn(id: u64) -> bool>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TempAllocatorI {
    pub inst: *mut TempAllocatorO,
    pub realloc: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut TempAllocatorO,
            ptr: *mut ::std::os::raw::c_void,
            old_size: u64,
            new_size: u64,
        ) -> *mut ::std::os::raw::c_void,
    >,
}
impl Default for TempAllocatorI {
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
pub struct TempAllocator1024O {
    pub buffer: [::std::os::raw::c_char; 1024usize],
}
impl Default for TempAllocator1024O {
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
pub struct TempAllocatorStatisticsT {
    pub temp_allocation_blocks: u64,
    pub temp_allocation_bytes: u64,
    pub frame_allocation_blocks: u64,
    pub frame_allocation_bytes: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TempAllocatorApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(backing: *mut AllocatorI) -> *mut TempAllocatorI,
    >,
    pub create_in_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            buffer: *mut ::std::os::raw::c_char,
            size: u64,
            backing: *mut AllocatorI,
        ) -> *mut TempAllocatorI,
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(ta: *mut TempAllocatorI)>,
    pub allocator:
        ::std::option::Option<unsafe extern "C" fn(a: *mut AllocatorI, ta: *mut TempAllocatorI)>,
    pub frame_alloc:
        ::std::option::Option<unsafe extern "C" fn(size: u64) -> *mut ::std::os::raw::c_void>,
    pub frame_allocator: ::std::option::Option<unsafe extern "C" fn() -> *mut AllocatorI>,
    pub tick_frame: ::std::option::Option<unsafe extern "C" fn()>,
    pub printf: ::std::option::Option<
        unsafe extern "C" fn(
            ta: *mut TempAllocatorI,
            format: *const ::std::os::raw::c_char,
            ...
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub vprintf: ::std::option::Option<
        unsafe extern "C" fn(
            ta: *mut TempAllocatorI,
            format: *const ::std::os::raw::c_char,
            args: va_list,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub frame_printf: ::std::option::Option<
        unsafe extern "C" fn(
            format: *const ::std::os::raw::c_char,
            ...
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub frame_vprintf: ::std::option::Option<
        unsafe extern "C" fn(
            format: *const ::std::os::raw::c_char,
            args: va_list,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub statistics: *mut TempAllocatorStatisticsT,
}
impl Default for TempAllocatorApi {
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
pub struct HashIdToIdT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetT {
    _unused: [u8; 0],
}
pub const TM_THE_TRUTH_PROPERTY_TYPE_NONE: TheTruthPropertyType = 0;
pub const TM_THE_TRUTH_PROPERTY_TYPE_BOOL: TheTruthPropertyType = 1;
pub const TM_THE_TRUTH_PROPERTY_TYPE_UINT32_T: TheTruthPropertyType = 2;
pub const TM_THE_TRUTH_PROPERTY_TYPE_UINT64_T: TheTruthPropertyType = 3;
pub const TM_THE_TRUTH_PROPERTY_TYPE_FLOAT: TheTruthPropertyType = 4;
pub const TM_THE_TRUTH_PROPERTY_TYPE_DOUBLE: TheTruthPropertyType = 5;
pub const TM_THE_TRUTH_PROPERTY_TYPE_STRING: TheTruthPropertyType = 6;
pub const TM_THE_TRUTH_PROPERTY_TYPE_BUFFER: TheTruthPropertyType = 7;
pub const TM_THE_TRUTH_PROPERTY_TYPE_REFERENCE: TheTruthPropertyType = 8;
pub const TM_THE_TRUTH_PROPERTY_TYPE_SUBOBJECT: TheTruthPropertyType = 9;
pub const TM_THE_TRUTH_PROPERTY_TYPE_REFERENCE_SET: TheTruthPropertyType = 10;
pub const TM_THE_TRUTH_PROPERTY_TYPE_SUBOBJECT_SET: TheTruthPropertyType = 11;
pub const TM_THE_TRUTH_NUM_PROPERTY_TYPES: TheTruthPropertyType = 12;
pub type TheTruthPropertyType = ::std::os::raw::c_int;
pub const TM_THE_TRUTH__EDITOR__DEFAULT: TheTruthEditor = 0;
pub const TM_THE_TRUTH__EDITOR__HIDDEN: TheTruthEditor = 1;
pub const TM_THE_TRUTH__EDITOR__UINT32_T__ENUM: TheTruthEditor = 2;
pub const TM_THE_TRUTH__EDITOR__STRING__OPEN_PATH: TheTruthEditor = 3;
pub const TM_THE_TRUTH__EDITOR__STRING__SAVE_PATH: TheTruthEditor = 4;
pub type TheTruthEditor = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TheTruthEditorEnumT {
    pub count: u32,
    pub _padding_357: [::std::os::raw::c_char; 4usize],
    pub names: *const *const ::std::os::raw::c_char,
    pub tooltips: *const *const ::std::os::raw::c_char,
}
impl Default for TheTruthEditorEnumT {
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
pub struct TheTruthEditorStringOpenPathT {
    pub extensions: *const ::std::os::raw::c_char,
    pub description: *const ::std::os::raw::c_char,
}
impl Default for TheTruthEditorStringOpenPathT {
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
pub struct TheTruthEditorStringSavePathT {
    pub default_file_name: *const ::std::os::raw::c_char,
}
impl Default for TheTruthEditorStringSavePathT {
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
pub struct TheTruthObjectO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TheTruthPropertyDefinitionT {
    pub name: *const ::std::os::raw::c_char,
    pub type_: TheTruthPropertyType,
    pub editor: u32,
    pub __bindgen_anon_1: TheTruthPropertyDefinitionTBindgenTy1,
    pub type_hash: StrhashT,
    pub allow_other_types: bool,
    pub _padding_451: [::std::os::raw::c_char; 7usize],
    pub buffer_extension: *const ::std::os::raw::c_char,
    pub buffer_extension_f: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object: TtIdT,
            property: u32,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub tooltip: *const ::std::os::raw::c_char,
    pub not_serialized: bool,
    pub _padding_469: [::std::os::raw::c_char; 7usize],
    pub ui_name: *const ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union TheTruthPropertyDefinitionTBindgenTy1 {
    pub enum_editor: TheTruthEditorEnumT,
    pub string_open_path_editor: TheTruthEditorStringOpenPathT,
    pub string_save_path_editor: TheTruthEditorStringSavePathT,
}
impl Default for TheTruthPropertyDefinitionTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for TheTruthPropertyDefinitionT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type TheTruthCreateTypesI = ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>;
pub type TheTruthDestroyedI = ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>;
pub const TM_THE_TRUTH_CREATE_TYPES_NONE: TheTruthCreateTypes = 0;
pub const TM_THE_TRUTH_CREATE_TYPES_ALL: TheTruthCreateTypes = 1;
pub type TheTruthCreateTypes = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TheTruthGetTypesWithAspectT {
    pub type_: TtTypeT,
    pub data: *mut ::std::os::raw::c_void,
}
impl Default for TheTruthGetTypesWithAspectT {
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
pub struct TheTruthGetAspectsT {
    pub id: StrhashT,
    pub data: *mut ::std::os::raw::c_void,
}
impl Default for TheTruthGetAspectsT {
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
pub struct TheTruthLocalSetT {
    pub num_added: u32,
    pub _padding_524: [::std::os::raw::c_char; 4usize],
    pub added: *const TtIdT,
    pub num_removed: u32,
    pub _padding_529: [::std::os::raw::c_char; 4usize],
    pub removed: *const TtIdT,
    pub num_instantiated: u32,
    pub _padding_534: [::std::os::raw::c_char; 4usize],
    pub instantiated: *const TtIdT,
}
impl Default for TheTruthLocalSetT {
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
pub struct TheTruthSetLocalSubobjectSetT {
    pub num_added: u32,
    pub _padding_543: [::std::os::raw::c_char; 4usize],
    pub added: *mut *mut TheTruthObjectO,
    pub num_removed: u32,
    pub _padding_548: [::std::os::raw::c_char; 4usize],
    pub removed: *const TtIdT,
    pub num_instantiated: u32,
    pub _padding_553: [::std::os::raw::c_char; 4usize],
    pub instantiated: *mut *mut TheTruthObjectO,
}
impl Default for TheTruthSetLocalSubobjectSetT {
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
pub struct TheTruthChangedObjectsT {
    pub overflow: bool,
    pub _padding_563: [::std::os::raw::c_char; 3usize],
    pub num_objects: u32,
    pub objects: *mut TtIdT,
    pub version: u64,
}
impl Default for TheTruthChangedObjectsT {
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
pub struct HashU64ToIdT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TheTruthInteropContextT {
    pub to_tt: *mut TheTruthO,
    pub from_tt: *mut TheTruthO,
    pub type_lookup: *mut Hash32T,
    pub property_lookup: *mut Hash32T,
    pub buffer_lookup: *mut Hash32T,
    pub object_lookup: *mut HashU64ToIdT,
}
impl Default for TheTruthInteropContextT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_TT_PROTOTYPE_RELATION_ADDED: TheTruthPrototypeRelation = 0;
pub const TM_TT_PROTOTYPE_RELATION_ASSET: TheTruthPrototypeRelation = 1;
pub const TM_TT_PROTOTYPE_RELATION_INHERITED: TheTruthPrototypeRelation = 2;
pub const TM_TT_PROTOTYPE_RELATION_INSTANTIATED: TheTruthPrototypeRelation = 3;
pub const TM_TT_PROTOTYPE_RELATION_REMOVED: TheTruthPrototypeRelation = 4;
pub const TM_TT_PROTOTYPE_RELATION_NONE: TheTruthPrototypeRelation = 5;
pub type TheTruthPrototypeRelation = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TtBufferT {
    pub id: u32,
    pub _padding_627: [::std::os::raw::c_char; 4usize],
    pub size: u64,
    pub data: *const ::std::os::raw::c_void,
    pub hash: u64,
}
impl Default for TtBufferT {
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
pub struct TtPropValueT {
    pub type_: TheTruthPropertyType,
    pub _padding_643: [::std::os::raw::c_char; 4usize],
    pub __bindgen_anon_1: TtPropValueTBindgenTy1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union TtPropValueTBindgenTy1 {
    pub b: bool,
    pub u32_: u32,
    pub u64_: u64,
    pub f32_: f32,
    pub f64_: f64,
    pub string: *const ::std::os::raw::c_char,
    pub buffer: TtBufferT,
    pub object: TtIdT,
    pub set: *const TtIdT,
}
impl Default for TtPropValueTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for TtPropValueT {
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
pub struct TtUndoActionT {
    pub before: *const TheTruthObjectO,
    pub after: *const TheTruthObjectO,
}
impl Default for TtUndoActionT {
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
pub struct TtSerializeOptionsT {
    pub serialize_buffers_as_hashes: bool,
    pub skip_type_index: bool,
    pub _padding_684: [::std::os::raw::c_char; 6usize],
    pub types: *mut SetT,
}
impl Default for TtSerializeOptionsT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_THE_TRUTH_MAX_PROPERTIES: ::std::os::raw::c_int = 64;
pub type _bindgen_ty_1 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TheTruthSerializedTypeInfoT {
    pub runtime_type: TtTypeT,
    pub properties_differ: bool,
    pub _padding_706: [::std::os::raw::c_char; 3usize],
    pub num_properties: u32,
    pub serialized_property_to_runtime_property: [u32; 64usize],
    pub serialized_property_type: [u32; 64usize],
}
impl Default for TheTruthSerializedTypeInfoT {
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
pub struct TtDeserializeOptionsT {
    pub buffers_preloaded: bool,
    pub skip_type_index: bool,
    pub _padding_731: [::std::os::raw::c_char; 6usize],
    pub type_infos: *const TheTruthSerializedTypeInfoT,
}
impl Default for TtDeserializeOptionsT {
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
pub struct TtSerializeChangesOptionsT {
    pub no_header: bool,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TtDeserializeChangesOptionsT {
    pub header: *const ::std::os::raw::c_char,
}
impl Default for TtDeserializeChangesOptionsT {
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
pub struct TtMemoryUseT {
    pub resident: u64,
    pub unloaded: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct TheTruthApi {
    pub allocator:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO) -> *mut AllocatorI>,
    pub buffers: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO) -> *mut BuffersI>,
    pub streamable_buffers:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO) -> *mut StreamableBuffersI>,
    pub create_object_type: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            name: *const ::std::os::raw::c_char,
            properties: *const TheTruthPropertyDefinitionT,
            num_properties: u32,
        ) -> TtTypeT,
    >,
    pub set_default_object: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, object_type: TtTypeT, object: TtIdT),
    >,
    pub set_default_object_to_create_subobjects:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO, object_type: TtTypeT)>,
    pub default_object: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, object_type: TtTypeT) -> TtIdT,
    >,
    pub is_default: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> bool,
    >,
    pub set_aspect: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object_type: TtTypeT,
            aspect: StrhashT,
            data: *const ::std::os::raw::c_void,
        ),
    >,
    pub set_default_aspect: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            aspect: StrhashT,
            data: *const ::std::os::raw::c_void,
        ),
    >,
    pub set_property_aspect: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object_type: TtTypeT,
            property: u32,
            aspect: StrhashT,
            data: *const ::std::os::raw::c_void,
        ),
    >,
    pub reload_aspects: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
    pub object_type_from_name_hash: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, name_hash: StrhashT) -> TtTypeT,
    >,
    pub optional_object_type_from_name_hash: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, name_hash: StrhashT) -> TtTypeT,
    >,
    pub num_types: ::std::option::Option<unsafe extern "C" fn(tt: *const TheTruthO) -> u32>,
    pub type_name: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object_type: TtTypeT,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub type_name_hash: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, object_type: TtTypeT) -> StrhashT,
    >,
    pub num_properties: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, object_type: TtTypeT) -> u32,
    >,
    pub properties: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object_type: TtTypeT,
        ) -> *const TheTruthPropertyDefinitionT,
    >,
    pub find_property: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object_type: TtTypeT,
            name_hash: StrhashT,
            type_: TheTruthPropertyType,
            res: *mut u32,
        ) -> bool,
    >,
    pub property_index: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, type_: TtTypeT, name_hash: StrhashT) -> u32,
    >,
    pub has_property: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, type_: TtTypeT, name_hash: StrhashT) -> u32,
    >,
    pub get_aspect: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object_type: TtTypeT,
            aspect: StrhashT,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_types_with_aspect: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            aspect: StrhashT,
            ta: *mut TempAllocatorI,
        ) -> *mut TheTruthGetTypesWithAspectT,
    >,
    pub get_aspects: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object_type: TtTypeT,
        ) -> *const TheTruthGetAspectsT,
    >,
    pub get_property_aspect: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object_type: TtTypeT,
            property: u32,
            aspect: StrhashT,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub all_objects_of_type: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object_type: TtTypeT,
            ta: *mut TempAllocatorI,
        ) -> *mut TtIdT,
    >,
    pub create_undo_scope: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            name: *const ::std::os::raw::c_char,
        ) -> TtUndoScopeT,
    >,
    pub create_thread_safe_undo_scope: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            name: *const ::std::os::raw::c_char,
        ) -> TtUndoScopeT,
    >,
    pub undo_scope_name: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            scope: TtUndoScopeT,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub undo_scope_objects: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            scope: TtUndoScopeT,
            ta: *mut TempAllocatorI,
        ) -> *mut TtIdT,
    >,
    pub undo_scope_actions: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            scope: TtUndoScopeT,
            ta: *mut TempAllocatorI,
        ) -> *mut TtUndoActionT,
    >,
    pub undo: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO, scope: TtUndoScopeT)>,
    pub redo: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO, scope: TtUndoScopeT)>,
    pub create_object_of_type: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, type_: TtTypeT, undo_scope: TtUndoScopeT) -> TtIdT,
    >,
    pub create_object_of_hash: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            type_name_hash: StrhashT,
            undo_scope: TtUndoScopeT,
        ) -> TtIdT,
    >,
    pub create_object_from_prototype: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            prototype: TtIdT,
            undo_scope: TtUndoScopeT,
        ) -> TtIdT,
    >,
    pub clone_object: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, object: TtIdT, undo_scope: TtUndoScopeT) -> TtIdT,
    >,
    pub instantiate_subobject: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            undo_scope: TtUndoScopeT,
        ) -> TtIdT,
    >,
    pub remove_instantiated_subobject: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub instantiate_subobject_from_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            subobject: TtIdT,
            undo_scope: TtUndoScopeT,
        ) -> TtIdT,
    >,
    pub remove_instantiated_subobject_from_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            subobject: TtIdT,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub add_instantiated_subobject_back_to_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            subobject: TtIdT,
        ),
    >,
    pub id: ::std::option::Option<unsafe extern "C" fn(obj: *const TheTruthObjectO) -> TtIdT>,
    pub destroy_object: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, object: TtIdT, undo_scope: TtUndoScopeT),
    >,
    pub destroy_objects: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: *const TtIdT,
            n: u32,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub garbage_collect: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
    pub is_alive:
        ::std::option::Option<unsafe extern "C" fn(tt: *const TheTruthO, object: TtIdT) -> bool>,
    pub interop_ensure_compatibility:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut TheTruthInteropContextT)>,
    pub interop_clone_object: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut TheTruthInteropContextT, object: TtIdT) -> TtIdT,
    >,
    pub deep_clone_assets: ::std::option::Option<
        unsafe extern "C" fn(
            to_tt: *mut TheTruthO,
            from_tt: *const TheTruthO,
            assets: *const TtIdT,
            n: u32,
            undo_scope: TtUndoScopeT,
            ta: *mut TempAllocatorI,
        ) -> *mut TtIdT,
    >,
    pub uuid:
        ::std::option::Option<unsafe extern "C" fn(tt: *const TheTruthO, object: TtIdT) -> UuidT>,
    pub read: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, object: TtIdT) -> *const TheTruthObjectO,
    >,
    pub get_bool: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> bool,
    >,
    pub get_uint32_t: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> u32,
    >,
    pub get_uint64_t: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> u64,
    >,
    pub get_float: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> f32,
    >,
    pub get_double: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> f64,
    >,
    pub get_string: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub get_string_hash: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> StrhashT,
    >,
    pub get_str: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> StrT,
    >,
    pub get_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> TtBufferT,
    >,
    pub get_buffer_id: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> u32,
    >,
    pub get_reference: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> TtIdT,
    >,
    pub get_subobject: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> TtIdT,
    >,
    pub get_property_value: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
            ta: *mut TempAllocatorI,
        ) -> TtPropValueT,
    >,
    pub property_value_equal:
        ::std::option::Option<unsafe extern "C" fn(a: TtPropValueT, b: TtPropValueT) -> bool>,
    pub get_reference_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
            ta: *mut TempAllocatorI,
        ) -> *const TtIdT,
    >,
    pub get_subobject_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
            ta: *mut TempAllocatorI,
        ) -> *const TtIdT,
    >,
    pub get_reference_set_size: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> u64,
    >,
    pub get_subobject_set_size: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> u64,
    >,
    pub get_subobject_set_type: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> TtTypeT,
    >,
    pub get_subobject_set_locally_removed: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
            ta: *mut TempAllocatorI,
        ) -> *const TtIdT,
    >,
    pub find_subobject_of_type: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
            type_: TtTypeT,
        ) -> TtIdT,
    >,
    pub is_subobject_of: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
            subobject: TtIdT,
        ) -> bool,
    >,
    pub is_in_reference_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
            object: TtIdT,
        ) -> bool,
    >,
    pub property_index_of_subobject: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, object: TtIdT, subobject: TtIdT) -> u32,
    >,
    pub write: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, object: TtIdT) -> *mut TheTruthObjectO,
    >,
    pub commit: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub commit_range: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut *mut TheTruthObjectO,
            n: u32,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub retarget_write: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, obj: *mut TheTruthObjectO, object: TtIdT),
    >,
    pub try_write: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: TtIdT,
            original: *mut *const TheTruthObjectO,
        ) -> *mut TheTruthObjectO,
    >,
    pub try_commit: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            original: *const TheTruthObjectO,
            undo_scope: TtUndoScopeT,
        ) -> bool,
    >,
    pub set_bool: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            value: bool,
        ),
    >,
    pub set_uint32_t: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            value: u32,
        ),
    >,
    pub set_uint64_t: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            value: u64,
        ),
    >,
    pub set_float: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            value: f32,
        ),
    >,
    pub set_double: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            value: f64,
        ),
    >,
    pub set_string: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            value: *const ::std::os::raw::c_char,
        ),
    >,
    pub set_str: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            value: StrT,
        ),
    >,
    pub set_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            value: u32,
        ),
    >,
    pub set_buffer_content: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            p: *mut ::std::os::raw::c_void,
            size: u64,
        ),
    >,
    pub set_reference: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            value: TtIdT,
        ),
    >,
    pub set_subobject: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            value: *mut TheTruthObjectO,
        ),
    >,
    pub set_subobject_id: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            value: TtIdT,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub set_property_value: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            value: TtPropValueT,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub clear: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, obj: *mut TheTruthObjectO, property: u32),
    >,
    pub clear_object:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO, obj: *mut TheTruthObjectO)>,
    pub propagate_property: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: TtIdT,
            property: u32,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub propagate_property_except: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: TtIdT,
            property: u32,
            skip: *const TtIdT,
            num_skip: u32,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub propagate_property_subobject: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: TtIdT,
            property: u32,
            subobject: TtIdT,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub propagate_object: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, object: TtIdT, undo_scope: TtUndoScopeT),
    >,
    pub propagate_object_except: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: TtIdT,
            skip: *const TtIdT,
            num_skip: u32,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub add_to_reference_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            items: *const TtIdT,
            count: u32,
        ),
    >,
    pub remove_from_reference_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            items: *const TtIdT,
            count: u32,
        ),
    >,
    pub clear_reference_set: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, obj: *mut TheTruthObjectO, property: u32),
    >,
    pub remove_from_prototype_reference_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            items: *const TtIdT,
            count: u32,
        ),
    >,
    pub cancel_remove_from_prototype_reference_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            items: *const TtIdT,
            count: u32,
        ),
    >,
    pub add_to_subobject_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            items: *mut *mut TheTruthObjectO,
            count: u32,
        ),
    >,
    pub add_to_subobject_set_id: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            items: *const TtIdT,
            count: u32,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub remove_from_subobject_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            items: *const TtIdT,
            count: u32,
        ),
    >,
    pub clear_subobject_set: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, obj: *mut TheTruthObjectO, property: u32),
    >,
    pub remove_from_prototype_subobject_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            items: *const TtIdT,
            count: u32,
        ),
    >,
    pub cancel_remove_from_prototype_subobject_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            items: *const TtIdT,
            count: u32,
        ),
    >,
    pub prototype:
        ::std::option::Option<unsafe extern "C" fn(tt: *const TheTruthO, object: TtIdT) -> TtIdT>,
    pub owner:
        ::std::option::Option<unsafe extern "C" fn(tt: *const TheTruthO, object: TtIdT) -> TtIdT>,
    pub is_currently_owner_of: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, object: TtIdT, subobject: TtIdT) -> bool,
    >,
    pub is_overridden: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> bool,
    >,
    pub has_data: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> bool,
    >,
    pub prototype_relation: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            parent: TtIdT,
            property: u32,
            object: TtIdT,
        ) -> TheTruthPrototypeRelation,
    >,
    pub version:
        ::std::option::Option<unsafe extern "C" fn(tt: *const TheTruthO, object: TtIdT) -> u32>,
    pub changed_objects: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            type_: TtTypeT,
            since_version: u64,
            ta: *mut TempAllocatorI,
        ) -> TheTruthChangedObjectsT,
    >,
    pub request_changelog: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO) -> u64>,
    pub relinquish_changelog:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO, h: u64)>,
    pub disable_changelog_start_scope:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
    pub disable_changelog_end_scope:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
    pub changelog_size: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO) -> u64>,
    pub serialize: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            o: TtIdT,
            carray: *mut *mut ::std::os::raw::c_char,
            a: *mut AllocatorI,
            opt: *const TtSerializeOptionsT,
        ),
    >,
    pub deserialize: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            buffer: *mut *const ::std::os::raw::c_char,
            opt: *const TtDeserializeOptionsT,
        ) -> TtIdT,
    >,
    pub buffer_hashes: ::std::option::Option<
        unsafe extern "C" fn(
            buffer: *mut *const ::std::os::raw::c_char,
            count: *mut u64,
        ) -> *const u64,
    >,
    pub deserialize_from_file: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, file: *const ::std::os::raw::c_char) -> TtIdT,
    >,
    pub migration_ids: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, n: *mut u32) -> *mut StrhashT,
    >,
    pub serialize_changes_header: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            carray: *mut *mut ::std::os::raw::c_char,
            a: *mut AllocatorI,
        ),
    >,
    pub serialize_changes: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            begin: u64,
            end: u64,
            carray: *mut *mut ::std::os::raw::c_char,
            a: *mut AllocatorI,
            opt: *const TtSerializeChangesOptionsT,
        ),
    >,
    pub deserialize_changes: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            buffer: *mut *const ::std::os::raw::c_char,
            opt: *const TtDeserializeChangesOptionsT,
        ),
    >,
    pub serialize_patch: ::std::option::Option<
        unsafe extern "C" fn(
            from_tt: *mut TheTruthO,
            from_o: TtIdT,
            to_tt: *mut TheTruthO,
            to_o: TtIdT,
            carray: *mut *mut ::std::os::raw::c_char,
            a: *mut AllocatorI,
        ),
    >,
    pub deserialize_patch: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, buffer: *mut *const ::std::os::raw::c_char),
    >,
    pub deserialize_patch_from_file: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, file: *const ::std::os::raw::c_char),
    >,
    pub serialize_type: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            type_: u32,
            ta: *mut TempAllocatorI,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub deserialize_type: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            buf: *mut *const ::std::os::raw::c_char,
            type_info: *mut TheTruthSerializedTypeInfoT,
        ),
    >,
    pub memory_use: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, id: TtIdT, buffers: *mut SetT) -> TtMemoryUseT,
    >,
    pub add_properties: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            type_: TtTypeT,
            properties: *const TheTruthPropertyDefinitionT,
            num_properties: u32,
        ),
    >,
    pub resolve_or_create_placeholder: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            uuid: UuidT,
            type_: TtTypeT,
            default_initialize: bool,
        ) -> TtIdT,
    >,
    pub resolve_or_fail: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, uuid: UuidT, type_: TtTypeT) -> TtIdT,
    >,
    pub set_uuid:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO, id: TtIdT, uuid: UuidT)>,
    pub set_prototype: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, obj: *mut TheTruthObjectO, id: TtIdT),
    >,
    pub detach_from_prototype: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            id: TtIdT,
            lookup: *mut HashIdToIdT,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub detach_all_instances: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, id: TtIdT, undo_scope: TtUndoScopeT),
    >,
    pub get_local_reference_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> TheTruthLocalSetT,
    >,
    pub get_local_subobject_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            property: u32,
        ) -> TheTruthLocalSetT,
    >,
    pub set_local_reference_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            set: TheTruthLocalSetT,
        ),
    >,
    pub set_local_subobject_set: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            property: u32,
            set: TheTruthSetLocalSubobjectSetT,
        ),
    >,
    pub string_repository:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO) -> *mut StringRepositoryI>,
    pub set_migration_ids:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO, ids: *mut StrhashT, n: u32)>,
    pub set_properties_to_default: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, obj: *mut TheTruthObjectO, mask: u64),
    >,
    pub instantiate_subobjects_recursively: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, object: TtIdT, undo_scope: TtUndoScopeT),
    >,
    pub quick_set_properties: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, undo_scope: TtUndoScopeT, id: TtIdT, ...),
    >,
    pub quick_create_object: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            undo_scope: TtUndoScopeT,
            type_hash: StrhashT,
            ...
        ) -> TtIdT,
    >,
    pub quick_get_property: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, id: TtIdT, prop_1: u32, ...) -> TtPropValueT,
    >,
    pub copy_properties_by_name: ::std::option::Option<
        unsafe extern "C" fn(t: *mut TheTruthO, to: TtIdT, from: TtIdT, undo_scope: TtUndoScopeT),
    >,
    pub internal__make_into_pseudo_object_with_owner: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, obj: *mut TheTruthObjectO, owner: TtIdT),
    >,
    pub internal__convert_pseudo_object_back_to_regular_object:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO, obj: *mut TheTruthObjectO)>,
    pub debug_inspect: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, object: TtIdT) -> *const ::std::os::raw::c_char,
    >,
    pub internal__detect_overlapping_writes:
        ::std::option::Option<unsafe extern "C" fn(tt: *const TheTruthO) -> bool>,
    pub create: ::std::option::Option<
        unsafe extern "C" fn(a: *mut AllocatorI, types: TheTruthCreateTypes) -> *mut TheTruthO,
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
}
pub const TM_TT_PROP__ASSET_ROOT__ASSETS: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__ASSET_ROOT__DIRECTORIES: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_2 = ::std::os::raw::c_int;
pub const TM_TT_PROP__ASSET__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__ASSET__DIRECTORY: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__ASSET__UUID_LABELS: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__ASSET__OBJECT: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__ASSET__THUMBNAIL: ::std::os::raw::c_int = 4;
pub type _bindgen_ty_3 = ::std::os::raw::c_int;
pub const TM_TT_PROP__ASSET_DIRECTORY__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__ASSET_DIRECTORY__PARENT: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_4 = ::std::os::raw::c_int;
pub const TM_TT_PROP__ASSET_LABEL__UUID: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_5 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct AssetLabelUuidT {
    pub uuid: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AssetLabelT {
    pub uuid: AssetLabelUuidT,
    pub name: *const ::std::os::raw::c_char,
    pub icon: u32,
    pub icon_color: u32,
}
impl Default for AssetLabelT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type TtAssetsFileExtensionAspectI = ::std::os::raw::c_char;
pub const TM_TT_PROP__ASSET_THUMBNAIL__THUMBNAIL: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__ASSET_THUMBNAIL__VALIDITY_HASH: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_6 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TtAssetsBufferWriteT {
    pub id: u32,
    pub _padding_126: [::std::os::raw::c_char; 4usize],
    pub hash: u64,
    pub ext: *const ::std::os::raw::c_char,
}
impl Default for TtAssetsBufferWriteT {
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
pub struct TtAssetsBufferT {
    pub object_id: TtIdT,
    pub property_index: u32,
    pub _padding_143: [::std::os::raw::c_char; 4usize],
    pub hash: u64,
}
impl Default for TtAssetsBufferT {
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
pub struct SavedTruthDataO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct TheTruthAssetsApi {
    pub get_asset_path: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            asset: TtIdT,
            path: *mut ::std::os::raw::c_char,
            n: u32,
        ) -> u32,
    >,
    pub get_directory_path: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            directory: TtIdT,
            path: *mut ::std::os::raw::c_char,
            n: u32,
        ) -> u32,
    >,
    pub get_asset_path_with_extension: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            asset: TtIdT,
            path: *mut ::std::os::raw::c_char,
            n: u32,
        ) -> u32,
    >,
    pub asset_from_path: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            asset_root: TtIdT,
            path: *const ::std::os::raw::c_char,
        ) -> TtIdT,
    >,
    pub asset_from_path_with_type: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            asset_root: TtIdT,
            path: *const ::std::os::raw::c_char,
            type_: TtTypeT,
        ) -> TtIdT,
    >,
    pub asset_object_from_path: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            asset_root: TtIdT,
            path: *const ::std::os::raw::c_char,
        ) -> TtIdT,
    >,
    pub asset_object_from_path_with_type: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            asset_root: TtIdT,
            path: *const ::std::os::raw::c_char,
            type_: TtTypeT,
        ) -> TtIdT,
    >,
    pub directory_from_path: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            asset_root: TtIdT,
            path: *const ::std::os::raw::c_char,
        ) -> TtIdT,
    >,
    pub find_subdirectory_by_name: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            asset_root: TtIdT,
            parent_dir: TtIdT,
            subdir_name: *const ::std::os::raw::c_char,
        ) -> TtIdT,
    >,
    pub unique_asset_name: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            asset_root: TtIdT,
            asset_r: *const TheTruthObjectO,
            desired_name: *const ::std::os::raw::c_char,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub unique_directory_name: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            asset_root: TtIdT,
            directory_r: *const TheTruthObjectO,
            desired_name: *const ::std::os::raw::c_char,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub object_asset_name: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, object: TtIdT) -> *const ::std::os::raw::c_char,
    >,
    pub find_directory_settings: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            relative_to: TtIdT,
            target_type_hash: StrhashT,
            asset_root: TtIdT,
        ) -> TtIdT,
    >,
    pub object_to_config: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object: TtIdT,
            config: *mut ConfigI,
            buffers: *mut *mut TtAssetsBufferWriteT,
            buffers_ta: *mut TempAllocatorI,
            save_uuid: bool,
        ),
    >,
    pub create_object_from_config: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            config: *mut ConfigI,
            buffers: *mut *mut TtAssetsBufferT,
            buffers_ta: *mut TempAllocatorI,
        ) -> TtIdT,
    >,
    pub read_object_from_config: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            config: *mut ConfigI,
            buffers: *mut *mut TtAssetsBufferT,
            buffers_ta: *mut TempAllocatorI,
            id: TtIdT,
        ),
    >,
    pub save_to_directory: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            asset_root: TtIdT,
            dir: *const ::std::os::raw::c_char,
            ignore: *mut TtIdT,
            num_ignore: u32,
            old_std: *mut SavedTruthDataO,
            allocator: *mut AllocatorI,
        ) -> *mut SavedTruthDataO,
    >,
    pub load_from_directory: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            dir: *const ::std::os::raw::c_char,
            allocator: *mut AllocatorI,
            asset_root: *mut TtIdT,
            error: *mut ErrorI,
        ) -> *mut SavedTruthDataO,
    >,
    pub current_truth_data: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            asset_root: TtIdT,
            allocator: *mut AllocatorI,
        ) -> *mut SavedTruthDataO,
    >,
    pub revert_asset: ::std::option::Option<
        unsafe extern "C" fn(
            sd: *mut SavedTruthDataO,
            asset: TtIdT,
            undo_scope: TtUndoScopeT,
        ) -> bool,
    >,
    pub save_asset:
        ::std::option::Option<unsafe extern "C" fn(sd: *mut SavedTruthDataO, asset: TtIdT) -> bool>,
    pub saved_name: ::std::option::Option<
        unsafe extern "C" fn(
            sd: *mut SavedTruthDataO,
            item: TtIdT,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub saved_directory:
        ::std::option::Option<unsafe extern "C" fn(sd: *mut SavedTruthDataO, item: TtIdT) -> TtIdT>,
    pub saved_version:
        ::std::option::Option<unsafe extern "C" fn(sd: *mut SavedTruthDataO, item: TtIdT) -> u64>,
    pub all_saved_items: ::std::option::Option<
        unsafe extern "C" fn(sd: *mut SavedTruthDataO, ta: *mut TempAllocatorI) -> *mut TtIdT,
    >,
    pub free_saved_data: ::std::option::Option<unsafe extern "C" fn(sd: *mut SavedTruthDataO)>,
    pub set_mock_file_system: ::std::option::Option<
        unsafe extern "C" fn(fs: *mut OsFileSystemApi, file_io: *mut OsFileIoApi),
    >,
    pub any_disk_changes: ::std::option::Option<
        unsafe extern "C" fn(sd: *mut SavedTruthDataO, dir: *const ::std::os::raw::c_char) -> bool,
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TheTruthMigrationO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TheTruthMigrationI {
    pub inst: *mut TheTruthMigrationO,
    pub id: StrhashT,
    pub num_prerequisites: u32,
    pub _padding_81: [::std::os::raw::c_char; 4usize],
    pub prerequisites: *mut StrhashT,
    pub migrate: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut TheTruthMigrationO, tt: *mut TheTruthO) -> bool,
    >,
}
impl Default for TheTruthMigrationI {
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
pub struct TheTruthMigrationApi {
    pub migrate: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO) -> bool>,
}
pub const TM_TT_PROP__VEC2__X: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__VEC2__Y: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_7 = ::std::os::raw::c_int;
pub const TM_TT_PROP__VEC3__X: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__VEC3__Y: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__VEC3__Z: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_8 = ::std::os::raw::c_int;
pub const TM_TT_PROP__VEC4__X: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__VEC4__Y: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__VEC4__Z: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__VEC4__W: ::std::os::raw::c_int = 3;
pub type _bindgen_ty_9 = ::std::os::raw::c_int;
pub const TM_TT_PROP__POSITION__X: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__POSITION__Y: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__POSITION__Z: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_10 = ::std::os::raw::c_int;
pub const TM_TT_PROP__ROTATION__X: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__ROTATION__Y: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__ROTATION__Z: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__ROTATION__W: ::std::os::raw::c_int = 3;
pub type _bindgen_ty_11 = ::std::os::raw::c_int;
pub const TM_TT_PROP__SCALE__X: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__SCALE__Y: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__SCALE__Z: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_12 = ::std::os::raw::c_int;
pub const TM_TT_PROP__COLOR_RGB__R: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__COLOR_RGB__G: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__COLOR_RGB__B: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_13 = ::std::os::raw::c_int;
pub const TM_TT_PROP__COLOR_RGBA__R: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__COLOR_RGBA__G: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__COLOR_RGBA__B: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__COLOR_RGBA__A: ::std::os::raw::c_int = 3;
pub type _bindgen_ty_14 = ::std::os::raw::c_int;
pub const TM_TT_PROP__RECT__X: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__RECT__Y: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__RECT__W: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__RECT__H: ::std::os::raw::c_int = 3;
pub type _bindgen_ty_15 = ::std::os::raw::c_int;
pub const TM_TT_PROP__UUID_A: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__UUID_B: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_16 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct TheTruthCommonTypesApi {
    pub create_common_types: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
    pub get_vec2: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object: *const TheTruthObjectO,
            property: u32,
        ) -> Vec2T,
    >,
    pub get_vec3: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object: *const TheTruthObjectO,
            property: u32,
        ) -> Vec3T,
    >,
    pub get_vec4: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object: *const TheTruthObjectO,
            property: u32,
        ) -> Vec4T,
    >,
    pub get_rect: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object: *const TheTruthObjectO,
            property: u32,
        ) -> RectT,
    >,
    pub get_position: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object: *const TheTruthObjectO,
            property: u32,
        ) -> Vec3T,
    >,
    pub get_rotation: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object: *const TheTruthObjectO,
            property: u32,
        ) -> Vec4T,
    >,
    pub get_scale: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object: *const TheTruthObjectO,
            property: u32,
        ) -> Vec3T,
    >,
    pub get_color_rgb: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object: *const TheTruthObjectO,
            property: u32,
        ) -> Vec3T,
    >,
    pub get_color_rgba: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object: *const TheTruthObjectO,
            property: u32,
        ) -> Vec4T,
    >,
    pub get_color_srgb: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object: *const TheTruthObjectO,
            property: u32,
        ) -> ColorSrgbT,
    >,
    pub get_color_srgba: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object: *const TheTruthObjectO,
            property: u32,
        ) -> ColorSrgbT,
    >,
    pub get_uuid: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object: *const TheTruthObjectO,
            property: u32,
        ) -> UuidT,
    >,
    pub set_vec2: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: *mut TheTruthObjectO,
            property: u32,
            vec2: Vec2T,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub set_vec3: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: *mut TheTruthObjectO,
            property: u32,
            vec3: Vec3T,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub set_vec4: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: *mut TheTruthObjectO,
            property: u32,
            vec4: Vec4T,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub set_rect: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: *mut TheTruthObjectO,
            property: u32,
            rect: RectT,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub set_position: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: *mut TheTruthObjectO,
            property: u32,
            vec3: Vec3T,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub set_rotation: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: *mut TheTruthObjectO,
            property: u32,
            vec4: Vec4T,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub set_scale: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: *mut TheTruthObjectO,
            property: u32,
            vec3: Vec3T,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub set_color_rgb: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: *mut TheTruthObjectO,
            property: u32,
            vec3: Vec3T,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub set_color_rgba: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: *mut TheTruthObjectO,
            property: u32,
            vec4: Vec4T,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub set_color_srgb: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: *mut TheTruthObjectO,
            property: u32,
            col: ColorSrgbT,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub set_color_srgba: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: *mut TheTruthObjectO,
            property: u32,
            col: ColorSrgbT,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub set_uuid: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            object: *mut TheTruthObjectO,
            property: u32,
            uuid: UuidT,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub read_floats: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            obj: *const TheTruthObjectO,
            res: *mut f32,
            n: u32,
        ) -> *mut f32,
    >,
    pub write_floats: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            obj: *mut TheTruthObjectO,
            values: *const f32,
            n: u32,
        ),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UndoStackO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UndoStackI {
    pub inst: *mut UndoStackO,
    pub add: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut UndoStackO, tt: *mut TheTruthO, scope: TtUndoScopeT),
    >,
    pub add_to_document: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut UndoStackO,
            tt: *mut TheTruthO,
            scope: TtUndoScopeT,
            document_object: TtIdT,
        ),
    >,
}
impl Default for UndoStackI {
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
pub struct UnicodeApi {
    pub is_valid:
        ::std::option::Option<unsafe extern "C" fn(utf8: *const ::std::os::raw::c_char) -> bool>,
    pub truncate: ::std::option::Option<unsafe extern "C" fn(utf8: *mut ::std::os::raw::c_char)>,
    pub utf8_encode: ::std::option::Option<
        unsafe extern "C" fn(
            utf8: *mut ::std::os::raw::c_char,
            codepoint: u32,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub utf8_decode: ::std::option::Option<
        unsafe extern "C" fn(utf8: *mut *const ::std::os::raw::c_char) -> u32,
    >,
    pub utf8_num_codepoints:
        ::std::option::Option<unsafe extern "C" fn(utf8: *const ::std::os::raw::c_char) -> u32>,
    pub utf8_decode_n: ::std::option::Option<
        unsafe extern "C" fn(
            codepoints: *mut u32,
            n: u32,
            utf8: *const ::std::os::raw::c_char,
        ) -> u32,
    >,
    pub utf8_to_utf32: ::std::option::Option<
        unsafe extern "C" fn(
            utf8: *const ::std::os::raw::c_char,
            ta: *mut TempAllocatorI,
        ) -> *mut u32,
    >,
    pub utf8_to_utf32_n: ::std::option::Option<
        unsafe extern "C" fn(
            utf8: *const ::std::os::raw::c_char,
            n: u32,
            ta: *mut TempAllocatorI,
        ) -> *mut u32,
    >,
    pub utf32_to_utf8: ::std::option::Option<
        unsafe extern "C" fn(
            utf32: *const u32,
            ta: *mut TempAllocatorI,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub utf32_to_utf8_n: ::std::option::Option<
        unsafe extern "C" fn(
            utf32: *const u32,
            n: u32,
            ta: *mut TempAllocatorI,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub utf16_encode:
        ::std::option::Option<unsafe extern "C" fn(utf16: *mut u16, codepoint: u32) -> *mut u16>,
    pub utf16_decode: ::std::option::Option<unsafe extern "C" fn(utf16: *mut *const u16) -> u32>,
    pub utf8_to_utf16: ::std::option::Option<
        unsafe extern "C" fn(
            utf8: *const ::std::os::raw::c_char,
            ta: *mut TempAllocatorI,
        ) -> *mut u16,
    >,
    pub utf8_to_utf16_n: ::std::option::Option<
        unsafe extern "C" fn(
            utf8: *const ::std::os::raw::c_char,
            n: u32,
            ta: *mut TempAllocatorI,
        ) -> *mut u16,
    >,
    pub utf16_to_utf8: ::std::option::Option<
        unsafe extern "C" fn(
            utf16: *const u16,
            ta: *mut TempAllocatorI,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub utf16_to_utf8_n: ::std::option::Option<
        unsafe extern "C" fn(
            utf16: *const u16,
            n: u32,
            ta: *mut TempAllocatorI,
        ) -> *mut ::std::os::raw::c_char,
    >,
}
pub const TM_UNICODE__HORIZONTAL_ELLIPSIS: ::std::os::raw::c_int = 8230;
pub const TM_UNICODE__ARROW_UPWARDS: ::std::os::raw::c_int = 8593;
pub const TM_UNICODE__ARROW_DOWNWARDS: ::std::os::raw::c_int = 8595;
pub const TM_UNICODE__COPYRIGHT: ::std::os::raw::c_int = 169;
pub const TM_UNICODE__CROSS_MARK: ::std::os::raw::c_int = 10060;
pub const TM_UNICODE__DOUBLE_PRIME: ::std::os::raw::c_int = 8243;
pub type _bindgen_ty_17 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UnitTestRunnerO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UnitTestRunnerI {
    pub inst: *mut UnitTestRunnerO,
    pub test_file: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut UnitTestRunnerO,
            name: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub test_custom: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut UnitTestRunnerO,
            name: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub test_disk: ::std::option::Option<unsafe extern "C" fn(inst: *mut UnitTestRunnerO) -> bool>,
    pub test_network:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut UnitTestRunnerO) -> bool>,
    pub test_slow_paths:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut UnitTestRunnerO) -> bool>,
    pub record: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut UnitTestRunnerO,
            pass: bool,
            test_str: *const ::std::os::raw::c_char,
            file: *const ::std::os::raw::c_char,
            line: u32,
        ) -> bool,
    >,
    pub expect_error: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut UnitTestRunnerO,
            err: *const ::std::os::raw::c_char,
            file: *const ::std::os::raw::c_char,
            line: u32,
        ),
    >,
}
impl Default for UnitTestRunnerI {
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
pub struct UnitTestI {
    pub name: *const ::std::os::raw::c_char,
    pub test:
        ::std::option::Option<unsafe extern "C" fn(tr: *mut UnitTestRunnerI, a: *mut AllocatorI)>,
}
impl Default for UnitTestI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_VISIBILITY_FLAG_OPT_IN: VisibilityFlagBehaviour = 0;
pub const TM_VISIBILITY_FLAG_OPT_OUT: VisibilityFlagBehaviour = 1;
pub type VisibilityFlagBehaviour = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VisibilityFlagDescI {
    pub name: StrhashT,
    pub description: *const ::std::os::raw::c_char,
    pub tooltip: *const ::std::os::raw::c_char,
    pub uuid: u32,
    pub behaviour: VisibilityFlagBehaviour,
}
impl Default for VisibilityFlagDescI {
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
pub struct VisibilityFlagEditorI {
    pub flag: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VisibilityContextO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct VisibilityFlagsApi {
    pub create_context: ::std::option::Option<
        unsafe extern "C" fn(allocator: *mut AllocatorI, name: StrhashT) -> *mut VisibilityContextO,
    >,
    pub context_name:
        ::std::option::Option<unsafe extern "C" fn(context: *mut VisibilityContextO) -> StrhashT>,
    pub destroy_context:
        ::std::option::Option<unsafe extern "C" fn(context: *mut VisibilityContextO)>,
    pub register_visibility_flag: ::std::option::Option<
        unsafe extern "C" fn(
            context: *mut VisibilityContextO,
            desc: *const VisibilityFlagDescI,
        ) -> u64,
    >,
    pub name_from_visibility_flag: ::std::option::Option<
        unsafe extern "C" fn(context: *mut VisibilityContextO, visibility_flag: u64) -> StrhashT,
    >,
    pub visibility_flag_from_name: ::std::option::Option<
        unsafe extern "C" fn(context: *mut VisibilityContextO, name: StrhashT) -> u64,
    >,
    pub unregister_visibility_flag:
        ::std::option::Option<unsafe extern "C" fn(context: *mut VisibilityContextO, uuid: u32)>,
    pub enumerate_flags: ::std::option::Option<
        unsafe extern "C" fn(
            context: *mut VisibilityContextO,
            flags: *mut VisibilityFlagDescI,
            num_flags: *mut u32,
        ),
    >,
    pub build_visibility_mask: ::std::option::Option<
        unsafe extern "C" fn(
            context: *mut VisibilityContextO,
            uuids: *const u32,
            num_uuids: u32,
        ) -> u64,
    >,
}
pub const TM_TT_PROP__VISIBILITY_FLAG__UUID: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_18 = ::std::os::raw::c_int;
pub const TM_WEB_SOCKET_OPCODE_CONTINUATION: WebSocketOpcode = 0;
pub const TM_WEB_SOCKET_OPCODE_TEXT: WebSocketOpcode = 1;
pub const TM_WEB_SOCKET_OPCODE_BINARY: WebSocketOpcode = 2;
pub const TM_WEB_SOCKET_OPCODE_CLOSE: WebSocketOpcode = 8;
pub const TM_WEB_SOCKET_OPCODE_PING: WebSocketOpcode = 9;
pub const TM_WEB_SOCKET_OPCODE_PONG: WebSocketOpcode = 10;
pub type WebSocketOpcode = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct WebSocketProtocolApi {
    pub make_client_handshake: ::std::option::Option<
        unsafe extern "C" fn(
            buffer: *mut ::std::os::raw::c_char,
            size: u32,
            host: *const ::std::os::raw::c_char,
            port: u32,
            request: *const ::std::os::raw::c_char,
            key: *const ::std::os::raw::c_char,
        ),
    >,
    pub make_server_handshake: ::std::option::Option<
        unsafe extern "C" fn(
            buffer: *mut ::std::os::raw::c_char,
            size: u32,
            key: *const ::std::os::raw::c_char,
            key_size: u32,
        ),
    >,
    pub make_frame_header: ::std::option::Option<
        unsafe extern "C" fn(buffer: *mut u8, fin: bool, opcode: u8, size: u64, mask: u32) -> u32,
    >,
    pub parse_frame_header: ::std::option::Option<
        unsafe extern "C" fn(
            buffer: *mut u8,
            buf_size: u64,
            fin: *mut bool,
            opcode: *mut u8,
            size: *mut u64,
            mask: *mut u32,
        ) -> u32,
    >,
    pub mask_data: ::std::option::Option<
        unsafe extern "C" fn(data: *mut u8, size: u64, offset: u64, mask: u32),
    >,
    pub mask_segmented_buffer: ::std::option::Option<
        unsafe extern "C" fn(seg: *mut SegmentedBufferT, size: u64, mask: u32),
    >,
}
pub const TM_HTTP_REQUEST_STATUS_IN_PROGRESS: HttpRequestStatus = 0;
pub const TM_HTTP_REQUEST_STATUS_SUCCESS: HttpRequestStatus = 1;
pub const TM_HTTP_REQUEST_STATUS_ERROR: HttpRequestStatus = 2;
pub const TM_HTTP_REQUEST_STATUS_CLOSED: HttpRequestStatus = 3;
pub type HttpRequestStatus = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HttpRequestT {
    pub id: u64,
    pub request_target: *const ::std::os::raw::c_char,
}
impl Default for HttpRequestT {
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
pub struct SegmentedBufferT {
    pub data: *mut ::std::os::raw::c_void,
    pub size: u64,
    pub next: *mut SegmentedBufferT,
}
impl Default for SegmentedBufferT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_WEB_SOCKET_PSEUDO_OPCODE_OPEN: WebSocketPseudoOpcode = 16;
pub const TM_WEB_SOCKET_PSEUDO_OPCODE_ERROR: WebSocketPseudoOpcode = 17;
pub type WebSocketPseudoOpcode = ::std::os::raw::c_int;
pub const TM_WEB_SOCKET_STATUS_CONNECTING: WebSocketStatus = 0;
pub const TM_WEB_SOCKET_STATUS_OPEN: WebSocketStatus = 1;
pub const TM_WEB_SOCKET_STATUS_CLOSING: WebSocketStatus = 2;
pub const TM_WEB_SOCKET_STATUS_CLOSED: WebSocketStatus = 3;
pub const TM_WEB_SOCKET_STATUS_ERROR: WebSocketStatus = 4;
pub type WebSocketStatus = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WebSocketRequestT {
    pub id: u64,
    pub request_target: *const ::std::os::raw::c_char,
}
impl Default for WebSocketRequestT {
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
pub struct WebSocketEventT {
    pub opcode: u32,
    pub _padding_107: [::std::os::raw::c_char; 4usize],
    pub payload_size: u64,
    pub payload: SegmentedBufferT,
    pub payload_reference: *mut ::std::os::raw::c_void,
}
impl Default for WebSocketEventT {
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
pub struct WebTalkerO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct WebTalkerApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            a: *mut AllocatorI,
            socket: *mut OsSocketApi,
            port: u32,
        ) -> *mut WebTalkerO,
    >,
    pub listening_address: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut WebTalkerO, address: *mut SocketAddressT) -> bool,
    >,
    pub create_no_server: ::std::option::Option<
        unsafe extern "C" fn(a: *mut AllocatorI, socket: *mut OsSocketApi) -> *mut WebTalkerO,
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut WebTalkerO)>,
    pub receive: ::std::option::Option<unsafe extern "C" fn(inst: *mut WebTalkerO)>,
    pub send: ::std::option::Option<unsafe extern "C" fn(inst: *mut WebTalkerO)>,
    pub http_get_requests: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut WebTalkerO,
            buffer: *mut HttpRequestT,
            capacity: u32,
        ) -> u32,
    >,
    pub http_respond_raw: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut WebTalkerO,
            id: u64,
            response: *const ::std::os::raw::c_char,
        ),
    >,
    pub http_respond_html: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut WebTalkerO, id: u64, html: *const ::std::os::raw::c_char),
    >,
    pub http_request: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut WebTalkerO,
            address: SocketAddressT,
            headers: *const ::std::os::raw::c_char,
        ) -> u64,
    >,
    pub http_request_status: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut WebTalkerO, id: u64) -> HttpRequestStatus,
    >,
    pub http_response: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut WebTalkerO, id: u64, size: *mut u64) -> SegmentedBufferT,
    >,
    pub http_close: ::std::option::Option<unsafe extern "C" fn(inst: *mut WebTalkerO, id: u64)>,
    pub ws_connect: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut WebTalkerO,
            address: SocketAddressT,
            host: *const ::std::os::raw::c_char,
            request: *const ::std::os::raw::c_char,
        ) -> u64,
    >,
    pub ws_get_requests: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut WebTalkerO,
            buffer: *mut WebSocketRequestT,
            capacity: u32,
        ) -> u32,
    >,
    pub ws_status: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut WebTalkerO, id: u64) -> WebSocketStatus,
    >,
    pub ws_recv_progress: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut WebTalkerO,
            id: u64,
            bytes: *mut u64,
            total: *mut u64,
        ) -> bool,
    >,
    pub ws_get_events: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut WebTalkerO,
            id: u64,
            buffer: *mut WebSocketEventT,
            capacity: u32,
        ) -> u32,
    >,
    pub ws_send_text_frame: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut WebTalkerO, id: u64, s: *const ::std::os::raw::c_char),
    >,
    pub ws_send_binary_frame: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut WebTalkerO, id: u64, data: *const u8, size: u64),
    >,
    pub ws_send_segmented_binary_frame: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut WebTalkerO,
            id: u64,
            buf: *const SegmentedBufferT,
            size: u64,
        ),
    >,
    pub ws_send_ping: ::std::option::Option<unsafe extern "C" fn(inst: *mut WebTalkerO, id: u64)>,
    pub ws_close: ::std::option::Option<unsafe extern "C" fn(inst: *mut WebTalkerO, id: u64)>,
    pub copy_segmented_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            buffer: *mut ::std::os::raw::c_void,
            segments: *const SegmentedBufferT,
            size: u64,
        ),
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ViewerManagerO {
    pub _address: u8,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RenderPipelineVt {
    pub _address: u8,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct NetworkO {
    pub _address: u8,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct AssetIoO {
    pub _address: u8,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct PluginO {
    pub _address: u8,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct StringRepositoryO {
    pub _address: u8,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct TempAllocatorO {
    pub _address: u8,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct Hash32T {
    pub _address: u8,
}

// Extensions generated by machinery-generator

use const_cstr::{const_cstr, ConstCStr};

use crate::plugins::ui::Draw2dFontT;

impl AllocatorApi {
    pub unsafe fn create_child(
        &self,
        parent: *const AllocatorI,
        desc: *const ::std::os::raw::c_char,
    ) -> AllocatorI {
        self.create_child.unwrap()(parent, desc)
    }

    pub unsafe fn destroy_child(&self, child: *const AllocatorI) {
        self.destroy_child.unwrap()(child)
    }

    pub unsafe fn destroy_child_allowing_leaks(
        &self,
        child: *const AllocatorI,
        max_leaked_bytes: u64,
    ) {
        self.destroy_child_allowing_leaks.unwrap()(child, max_leaked_bytes)
    }

    pub unsafe fn create_leaky_root_scope(
        &self,
        parent: *const AllocatorI,
        desc: *const ::std::os::raw::c_char,
    ) -> AllocatorI {
        self.create_leaky_root_scope.unwrap()(parent, desc)
    }

    pub unsafe fn create_fixed_vm(&self, reserve_size: u64, mem_scope: u32) -> AllocatorI {
        self.create_fixed_vm.unwrap()(reserve_size, mem_scope)
    }
}

impl crate::Api for AllocatorApi {
    const NAME: ConstCStr = const_cstr!("tm_allocator_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl ApiRegistryApi {
    pub unsafe fn api_registry_version(&self) -> VersionT {
        self.api_registry_version.unwrap()()
    }

    pub unsafe fn set(
        &self,
        name: *const ::std::os::raw::c_char,
        version: VersionT,
        api: *const ::std::os::raw::c_void,
        bytes: u32,
    ) {
        self.set.unwrap()(name, version, api, bytes)
    }

    pub unsafe fn remove(&self, api: *const ::std::os::raw::c_void) {
        self.remove.unwrap()(api)
    }

    pub unsafe fn get(
        &self,
        name: *const ::std::os::raw::c_char,
        version: VersionT,
    ) -> *mut ::std::os::raw::c_void {
        self.get.unwrap()(name, version)
    }

    pub unsafe fn get_optional(
        &self,
        api: *mut *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        version: VersionT,
    ) {
        self.get_optional.unwrap()(api, name, version)
    }

    pub unsafe fn version(&self, api: *mut ::std::os::raw::c_void) -> VersionT {
        self.version.unwrap()(api)
    }

    pub unsafe fn add_implementation(
        &self,
        name: *const ::std::os::raw::c_char,
        version: VersionT,
        implementation: *const ::std::os::raw::c_void,
    ) {
        self.add_implementation.unwrap()(name, version, implementation)
    }

    pub unsafe fn remove_implementation(
        &self,
        name: *const ::std::os::raw::c_char,
        version: VersionT,
        implementation: *const ::std::os::raw::c_void,
    ) {
        self.remove_implementation.unwrap()(name, version, implementation)
    }

    pub unsafe fn implementations(
        &self,
        name: *const ::std::os::raw::c_char,
        version: VersionT,
    ) -> *mut *mut ::std::os::raw::c_void {
        self.implementations.unwrap()(name, version)
    }

    pub unsafe fn num_implementations(
        &self,
        name: *const ::std::os::raw::c_char,
        version: VersionT,
    ) -> u32 {
        self.num_implementations.unwrap()(name, version)
    }

    pub unsafe fn first_implementation(
        &self,
        name: *const ::std::os::raw::c_char,
        version: VersionT,
    ) -> *mut ::std::os::raw::c_void {
        self.first_implementation.unwrap()(name, version)
    }

    pub unsafe fn single_implementation(
        &self,
        name: *const ::std::os::raw::c_char,
        version: VersionT,
    ) -> *mut ::std::os::raw::c_void {
        self.single_implementation.unwrap()(name, version)
    }

    pub unsafe fn add_listener(&self, listener: *const ApiRegistryListenerI) {
        self.add_listener.unwrap()(listener)
    }

    pub unsafe fn static_variable(
        &self,
        id: StrhashT,
        size: u32,
        file: *const ::std::os::raw::c_char,
        line: u32,
    ) -> *mut ::std::os::raw::c_void {
        self.static_variable.unwrap()(id, size, file, line)
    }

    pub unsafe fn begin_context(&self, name: *const ::std::os::raw::c_char) {
        self.begin_context.unwrap()(name)
    }

    pub unsafe fn end_context(&self, name: *const ::std::os::raw::c_char) {
        self.end_context.unwrap()(name)
    }

    pub unsafe fn disable_apis_missing_dependencies(&self) {
        self.disable_apis_missing_dependencies.unwrap()()
    }

    pub unsafe fn available_versions(
        &self,
        name: *const ::std::os::raw::c_char,
        ta: *mut TempAllocatorI,
    ) -> *mut VersionT {
        self.available_versions.unwrap()(name, ta)
    }
}

impl crate::Api for ApiRegistryApi {
    const NAME: ConstCStr = const_cstr!("tm_api_registry_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl ApplicationApi {
    pub unsafe fn application(&self) -> *mut ApplicationO {
        self.application.unwrap()()
    }

    pub unsafe fn create(
        &self,
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ApplicationO {
        self.create.unwrap()(argc, argv)
    }

    pub unsafe fn tick(&self, app: *mut ApplicationO) -> bool {
        self.tick.unwrap()(app)
    }

    pub unsafe fn destroy(&self, app: *mut ApplicationO) {
        self.destroy.unwrap()(app)
    }

    pub unsafe fn set_modal(
        &self,
        app: *mut ApplicationO,
        f: ApplicationModalF,
        data: *mut ::std::os::raw::c_void,
    ) {
        self.set_modal.unwrap()(app, f, data)
    }

    pub unsafe fn is_modal(&self, app: *const ApplicationO) -> bool {
        self.is_modal.unwrap()(app)
    }

    pub unsafe fn asset_root(&self, app: *const ApplicationO) -> TtIdT {
        self.asset_root.unwrap()(app)
    }

    pub unsafe fn load_core(&self, app: *const ApplicationO) -> bool {
        self.load_core.unwrap()(app)
    }

    pub unsafe fn update_core(&self, app: *const ApplicationO, include_skipped: bool) -> bool {
        self.update_core.unwrap()(app, include_skipped)
    }

    pub unsafe fn exit(&self, app: *mut ApplicationO) {
        self.exit.unwrap()(app)
    }

    pub unsafe fn set_cursor_hidden(&self, app: *mut ApplicationO, hidden: bool) {
        self.set_cursor_hidden.unwrap()(app, hidden)
    }

    pub unsafe fn viewer_manager(&self, app: *mut ApplicationO) -> *mut ViewerManagerO {
        self.viewer_manager.unwrap()(app)
    }

    pub unsafe fn default_render_pipeline_api(
        &self,
        app: *mut ApplicationO,
    ) -> *mut RenderPipelineVt {
        self.default_render_pipeline_api.unwrap()(app)
    }

    pub unsafe fn custom_ui_scale_factor(&self, app: *mut ApplicationO) -> f32 {
        self.custom_ui_scale_factor.unwrap()(app)
    }

    pub unsafe fn display_scale_factor(&self, app: *mut ApplicationO, ui: *mut UiO) -> f32 {
        self.display_scale_factor.unwrap()(app, ui)
    }

    pub unsafe fn data_dir(&self, app: *mut ApplicationO) -> *const ::std::os::raw::c_char {
        self.data_dir.unwrap()(app)
    }

    pub unsafe fn color_space(
        &self,
        app: *const ApplicationO,
        color_space: *mut ColorSpaceDescT,
        format: *mut u32,
    ) {
        self.color_space.unwrap()(app, color_space, format)
    }

    pub unsafe fn network(&self, app: *mut ApplicationO) -> *mut NetworkO {
        self.network.unwrap()(app)
    }
}

impl crate::Api for ApplicationApi {
    const NAME: ConstCStr = const_cstr!("tm_application_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 1u32,
        patch: 0u32,
    };
}

impl AssetDatabaseApi {
    pub unsafe fn create(
        &self,
        file: *const ::std::os::raw::c_char,
        tt: *mut TheTruthO,
        config: *const AssetDatabaseConfigT,
    ) -> *mut AssetDatabaseO {
        self.create.unwrap()(file, tt, config)
    }

    pub unsafe fn open(
        &self,
        file: *const ::std::os::raw::c_char,
        tt: *mut TheTruthO,
        config: *const AssetDatabaseConfigT,
    ) -> *mut AssetDatabaseO {
        self.open.unwrap()(file, tt, config)
    }

    pub unsafe fn close(&self, db: *mut AssetDatabaseO) {
        self.close.unwrap()(db)
    }

    pub unsafe fn save_modified(&self, db: *mut AssetDatabaseO, asset_root: TtIdT) {
        self.save_modified.unwrap()(db, asset_root)
    }

    pub unsafe fn save_modified_except(
        &self,
        db: *mut AssetDatabaseO,
        asset_root: TtIdT,
        ignore: *mut TtIdT,
        num_ignore: u32,
    ) {
        self.save_modified_except.unwrap()(db, asset_root, ignore, num_ignore)
    }

    pub unsafe fn save_asset(&self, db: *mut AssetDatabaseO, asset: TtIdT) {
        self.save_asset.unwrap()(db, asset)
    }

    pub unsafe fn delete_asset(&self, db: *mut AssetDatabaseO, asset: TtIdT) {
        self.delete_asset.unwrap()(db, asset)
    }

    pub unsafe fn revert_asset(
        &self,
        db: *mut AssetDatabaseO,
        asset: TtIdT,
        undo_scope: TtUndoScopeT,
    ) -> bool {
        self.revert_asset.unwrap()(db, asset, undo_scope)
    }

    pub unsafe fn load(&self, db: *mut AssetDatabaseO, load_fraction: *mut f32) -> TtIdT {
        self.load.unwrap()(db, load_fraction)
    }

    pub unsafe fn saved_name(
        &self,
        db: *mut AssetDatabaseO,
        item: UuidT,
    ) -> *const ::std::os::raw::c_char {
        self.saved_name.unwrap()(db, item)
    }

    pub unsafe fn saved_directory(&self, db: *mut AssetDatabaseO, item: UuidT) -> UuidT {
        self.saved_directory.unwrap()(db, item)
    }

    pub unsafe fn saved_version(&self, db: *mut AssetDatabaseO, asset: UuidT) -> u64 {
        self.saved_version.unwrap()(db, asset)
    }

    pub unsafe fn all_saved_items(
        &self,
        db: *mut AssetDatabaseO,
        ta: *mut TempAllocatorI,
    ) -> *mut AssetDabaseSavedItemT {
        self.all_saved_items.unwrap()(db, ta)
    }
}

impl crate::Api for AssetDatabaseApi {
    const NAME: ConstCStr = const_cstr!("tm_asset_database_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl AssetIoApi {
    pub unsafe fn add_asset_io(&self, loader: *mut AssetIoI) {
        self.add_asset_io.unwrap()(loader)
    }

    pub unsafe fn remove_asset_io(&self, loader: *mut AssetIoI) {
        self.remove_asset_io.unwrap()(loader)
    }

    pub unsafe fn importer(&self, extension: *const ::std::os::raw::c_char) -> *mut AssetIoI {
        self.importer.unwrap()(extension)
    }

    pub unsafe fn reimporter(&self, tt: *mut TheTruthO, asset: TtIdT) -> *mut AssetIoI {
        self.reimporter.unwrap()(tt, asset)
    }

    pub unsafe fn exporter(&self, tt: *mut TheTruthO, asset: TtIdT) -> *mut AssetIoI {
        self.exporter.unwrap()(tt, asset)
    }

    pub unsafe fn io_interfaces(&self, interfaces: *mut *mut AssetIoI) -> u32 {
        self.io_interfaces.unwrap()(interfaces)
    }
}

impl crate::Api for AssetIoApi {
    const NAME: ConstCStr = const_cstr!("tm_asset_io_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl Base64Api {
    pub unsafe fn encoded_size(&self, raw_size: u64) -> u64 {
        self.encoded_size.unwrap()(raw_size)
    }

    pub unsafe fn encode(
        &self,
        encoded: *mut ::std::os::raw::c_char,
        raw: *const u8,
        raw_size: u64,
    ) -> u64 {
        self.encode.unwrap()(encoded, raw, raw_size)
    }

    pub unsafe fn decoded_size(
        &self,
        encoded: *const ::std::os::raw::c_char,
        encoded_size: u64,
    ) -> u64 {
        self.decoded_size.unwrap()(encoded, encoded_size)
    }

    pub unsafe fn decode(
        &self,
        raw: *mut u8,
        encoded: *const ::std::os::raw::c_char,
        encoded_size: u64,
    ) -> u64 {
        self.decode.unwrap()(raw, encoded, encoded_size)
    }
}

impl BuddyAllocatorRawApi {
    pub unsafe fn init(&self, buffer: *mut u8, size: u32, block_size: u32) {
        self.init.unwrap()(buffer, size, block_size)
    }

    pub unsafe fn realloc(
        &self,
        buffer: *mut u8,
        ptr: *mut ::std::os::raw::c_void,
        old_size: u64,
        new_size: u64,
    ) -> *mut ::std::os::raw::c_void {
        self.realloc.unwrap()(buffer, ptr, old_size, new_size)
    }
}

impl crate::Api for BuddyAllocatorRawApi {
    const NAME: ConstCStr = const_cstr!("tm_buddy_allocator_raw_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl BuddyAllocatorApi {
    pub unsafe fn create(
        &self,
        backing: *mut AllocatorI,
        initial_size: u32,
        block_size: u32,
    ) -> *mut AllocatorI {
        self.create.unwrap()(backing, initial_size, block_size)
    }

    pub unsafe fn destroy(&self, a: *mut AllocatorI) {
        self.destroy.unwrap()(a)
    }
}

impl crate::Api for BuddyAllocatorApi {
    const NAME: ConstCStr = const_cstr!("tm_buddy_allocator_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl BuffersApi {
    pub unsafe fn create(&self, a: *mut AllocatorI) -> *mut BuffersI {
        self.create.unwrap()(a)
    }

    pub unsafe fn destroy(&self, i: *mut BuffersI) {
        self.destroy.unwrap()(i)
    }

    pub unsafe fn create_streamable(
        &self,
        a: *mut AllocatorI,
        io: *mut OsFileIoApi,
    ) -> *mut StreamableBuffersI {
        self.create_streamable.unwrap()(a, io)
    }

    pub unsafe fn destroy_streamable(&self, i: *mut StreamableBuffersI) {
        self.destroy_streamable.unwrap()(i)
    }
}

impl crate::Api for BuffersApi {
    const NAME: ConstCStr = const_cstr!("tm_buffers_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl BufferFormatApi {
    pub unsafe fn encode_uncompressed_format(
        &self,
        component_type: BufferComponentType,
        sign: bool,
        bits_x: u8,
        bits_y: u8,
        bits_z: u8,
        bits_w: u8,
    ) -> u32 {
        self.encode_uncompressed_format.unwrap()(
            component_type,
            sign,
            bits_x,
            bits_y,
            bits_z,
            bits_w,
        )
    }

    pub unsafe fn encode_compressed_format(
        &self,
        compression_type: BufferCompressionFormat,
    ) -> u32 {
        self.encode_compressed_format.unwrap()(compression_type)
    }

    pub unsafe fn is_compressed(&self, format: u32) -> bool {
        self.is_compressed.unwrap()(format)
    }

    pub unsafe fn decode_uncompressed(
        &self,
        format: u32,
        component_type: *mut BufferComponentType,
        sign: *mut bool,
        bits_x: *mut u8,
        bits_y: *mut u8,
        bits_z: *mut u8,
        bits_w: *mut u8,
    ) -> bool {
        self.decode_uncompressed.unwrap()(
            format,
            component_type,
            sign,
            bits_x,
            bits_y,
            bits_z,
            bits_w,
        )
    }

    pub unsafe fn decode_compression_format(
        &self,
        format: u32,
        compression_format: *mut BufferCompressionFormat,
    ) -> bool {
        self.decode_compression_format.unwrap()(format, compression_format)
    }

    pub unsafe fn bits_per_element(&self, format: u32) -> u32 {
        self.bits_per_element.unwrap()(format)
    }

    pub unsafe fn num_components(&self, format: u32) -> u32 {
        self.num_components.unwrap()(format)
    }

    pub unsafe fn human_readable(
        &self,
        format: u32,
        ta: *mut TempAllocatorI,
    ) -> *const ::std::os::raw::c_char {
        self.human_readable.unwrap()(format, ta)
    }
}

impl crate::Api for BufferFormatApi {
    const NAME: ConstCStr = const_cstr!("tm_buffer_format_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl CameraApi {
    pub unsafe fn view_from_transform(
        &self,
        view: *mut Mat44T,
        tm: *const TransformT,
    ) -> *mut Mat44T {
        self.view_from_transform.unwrap()(view, tm)
    }

    pub unsafe fn view_from_lookin(
        &self,
        view: *mut Mat44T,
        position: Vec3T,
        forward: Vec3T,
        up: Vec3T,
    ) -> *mut Mat44T {
        self.view_from_lookin.unwrap()(view, position, forward, up)
    }

    pub unsafe fn transform_from_view(
        &self,
        tm: *mut TransformT,
        view: *const Mat44T,
    ) -> *mut TransformT {
        self.transform_from_view.unwrap()(tm, view)
    }

    pub unsafe fn projection_from_frustum(
        &self,
        proj: *mut Mat44T,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> *mut Mat44T {
        self.projection_from_frustum.unwrap()(proj, left, right, bottom, top, near, far)
    }

    pub unsafe fn matrices_from_eyes(
        &self,
        camera: *mut CameraT,
        head_tm: *const Mat44T,
        head_to_left_eye: *const Mat44T,
        head_to_right_eye: *const Mat44T,
        left_eye_left: f32,
        left_eye_right: f32,
        left_eye_top: f32,
        left_eye_bottom: f32,
        right_eye_left: f32,
        right_eye_right: f32,
        right_eye_top: f32,
        right_eye_bottom: f32,
    ) {
        self.matrices_from_eyes.unwrap()(
            camera,
            head_tm,
            head_to_left_eye,
            head_to_right_eye,
            left_eye_left,
            left_eye_right,
            left_eye_top,
            left_eye_bottom,
            right_eye_left,
            right_eye_right,
            right_eye_top,
            right_eye_bottom,
        )
    }

    pub unsafe fn projection_from_fov(
        &self,
        proj: *mut Mat44T,
        near_plane: f32,
        far_plane: f32,
        vertical_fov: f32,
        aspect: f32,
    ) -> *mut Mat44T {
        self.projection_from_fov.unwrap()(proj, near_plane, far_plane, vertical_fov, aspect)
    }

    pub unsafe fn orthographic_from_frustum(
        &self,
        proj: *mut Mat44T,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> *mut Mat44T {
        self.orthographic_from_frustum.unwrap()(proj, left, right, bottom, top, near, far)
    }

    pub unsafe fn orthographics_from_dimensions(
        &self,
        proj: *mut Mat44T,
        near_plane: f32,
        far_plane: f32,
        width: f32,
        height: f32,
    ) -> *mut Mat44T {
        self.orthographics_from_dimensions.unwrap()(proj, near_plane, far_plane, width, height)
    }

    pub unsafe fn projection_from_camera(
        &self,
        camera: *mut CameraT,
        transform: CameraTransform,
        aspect: f32,
    ) -> *mut Mat44T {
        self.projection_from_camera.unwrap()(camera, transform, aspect)
    }

    pub unsafe fn update_free_flight(&self, tm: *mut TransformT, t: Vec3T, r: Vec2T) {
        self.update_free_flight.unwrap()(tm, t, r)
    }

    pub unsafe fn update_pan(&self, tm: *mut TransformT, focus_position: *mut Vec3T, pan: Vec2T) {
        self.update_pan.unwrap()(tm, focus_position, pan)
    }

    pub unsafe fn update_maya(
        &self,
        tm: *mut TransformT,
        focus_position: Vec3T,
        zoom: f32,
        rot: Vec2T,
    ) {
        self.update_maya.unwrap()(tm, focus_position, zoom, rot)
    }

    pub unsafe fn world_to_screen(
        &self,
        camera: *const CameraT,
        transform: CameraTransform,
        viewport: RectT,
        world: *const Vec3T,
        screen: *mut Vec3T,
        n: u32,
    ) -> *mut Vec3T {
        self.world_to_screen.unwrap()(camera, transform, viewport, world, screen, n)
    }

    pub unsafe fn screen_to_world(
        &self,
        camera: *const CameraT,
        transform: CameraTransform,
        viewport: RectT,
        screen: *const Vec3T,
        world: *mut Vec3T,
        n: u32,
    ) -> *mut Vec3T {
        self.screen_to_world.unwrap()(camera, transform, viewport, screen, world, n)
    }

    pub unsafe fn meters_per_pixel(
        &self,
        distance: f32,
        vertical_fov: f32,
        viewport_height: f32,
    ) -> f32 {
        self.meters_per_pixel.unwrap()(distance, vertical_fov, viewport_height)
    }

    pub unsafe fn default_camera_settings(&self) -> *const CameraSettingsT {
        self.default_camera_settings.unwrap()()
    }

    pub unsafe fn frustum_planes_from_view_projection(
        &self,
        view: *const Mat44T,
        projection: *const Mat44T,
        frustum_planes: *mut Vec4T,
    ) {
        self.frustum_planes_from_view_projection.unwrap()(view, projection, frustum_planes)
    }
}

impl crate::Api for CameraApi {
    const NAME: ConstCStr = const_cstr!("tm_camera_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl CollaborationApi {
    pub unsafe fn create(
        &self,
        a: *mut AllocatorI,
        config: *const CollaborationConfigI,
    ) -> *mut CollaborationO {
        self.create.unwrap()(a, config)
    }

    pub unsafe fn destroy(&self, coll: *mut CollaborationO) {
        self.destroy.unwrap()(coll)
    }

    pub unsafe fn status(&self, coll: *const CollaborationO) -> CollaborationStatus {
        self.status.unwrap()(coll)
    }

    pub unsafe fn is_downloading(
        &self,
        coll: *const CollaborationO,
        bytes: *mut u64,
        total: *mut u64,
    ) -> bool {
        self.is_downloading.unwrap()(coll, bytes, total)
    }

    pub unsafe fn set_session(
        &self,
        coll: *mut CollaborationO,
        session: *const CollaborationSessionI,
    ) {
        self.set_session.unwrap()(coll, session)
    }

    pub unsafe fn session(&self, coll: *mut CollaborationO) -> *mut CollaborationSessionI {
        self.session.unwrap()(coll)
    }

    pub unsafe fn disconnect(&self, coll: *mut CollaborationO) {
        self.disconnect.unwrap()(coll)
    }

    pub unsafe fn update(&self, coll: *mut CollaborationO) {
        self.update.unwrap()(coll)
    }

    pub unsafe fn handle(&self, coll: *const CollaborationO) -> *const ::std::os::raw::c_char {
        self.handle.unwrap()(coll)
    }

    pub unsafe fn set_handle(&self, c: *mut CollaborationO, handle: *mut ::std::os::raw::c_char) {
        self.set_handle.unwrap()(c, handle)
    }

    pub unsafe fn host_handle(&self, c: *const CollaborationO) -> *const ::std::os::raw::c_char {
        self.host_handle.unwrap()(c)
    }

    pub unsafe fn num_clients(&self, coll: *const CollaborationO) -> u32 {
        self.num_clients.unwrap()(coll)
    }

    pub unsafe fn client_handle(
        &self,
        coll: *const CollaborationO,
        i: u32,
    ) -> *const ::std::os::raw::c_char {
        self.client_handle.unwrap()(coll, i)
    }

    pub unsafe fn all_handles(
        &self,
        coll: *const CollaborationO,
        ta: *mut TempAllocatorI,
    ) -> *mut *const ::std::os::raw::c_char {
        self.all_handles.unwrap()(coll, ta)
    }

    pub unsafe fn send_chat(&self, coll: *mut CollaborationO, msg: *const ::std::os::raw::c_char) {
        self.send_chat.unwrap()(coll, msg)
    }

    pub unsafe fn num_chat_messages(&self, coll: *const CollaborationO) -> u32 {
        self.num_chat_messages.unwrap()(coll)
    }

    pub unsafe fn chat_message(
        &self,
        coll: *const CollaborationO,
        i: u32,
        sender: *mut *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char {
        self.chat_message.unwrap()(coll, i, sender)
    }

    pub unsafe fn resynchronize_state(&self, coll: *mut CollaborationO, tt: *mut TheTruthO) {
        self.resynchronize_state.unwrap()(coll, tt)
    }

    pub unsafe fn send_test_packages(&self, coll: *mut CollaborationO, size: u64, num: u32) {
        self.send_test_packages.unwrap()(coll, size, num)
    }
}

impl crate::Api for CollaborationApi {
    const NAME: ConstCStr = const_cstr!("tm_collaboration_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl CollaborationP2pApi {
    pub unsafe fn create(
        &self,
        coll: *mut CollaborationO,
        allocator: *mut AllocatorI,
    ) -> *mut CollaborationP2pO {
        self.create.unwrap()(coll, allocator)
    }

    pub unsafe fn destroy(&self, coll_p2p: *mut CollaborationP2pO) {
        self.destroy.unwrap()(coll_p2p)
    }

    pub unsafe fn update(&self, coll_p2p: *mut CollaborationP2pO) {
        self.update.unwrap()(coll_p2p)
    }

    pub unsafe fn host(&self, coll_p2p: *mut CollaborationP2pO, port: u32, use_upnp: bool) {
        self.host.unwrap()(coll_p2p, port, use_upnp)
    }

    pub unsafe fn connect(&self, coll_p2p: *mut CollaborationP2pO, address: *const SocketAddressT) {
        self.connect.unwrap()(coll_p2p, address)
    }

    pub unsafe fn discovered_lan_hosts(
        &self,
        coll_p2p: *const CollaborationP2pO,
        hosts: *mut CollaborationDiscoveredHostT,
        max_hosts: u32,
    ) -> u32 {
        self.discovered_lan_hosts.unwrap()(coll_p2p, hosts, max_hosts)
    }
}

impl crate::Api for CollaborationP2pApi {
    const NAME: ConstCStr = const_cstr!("tm_collaboration_p2p_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl ConfigApi {
    pub unsafe fn create(&self, a: *mut AllocatorI) -> *mut ConfigI {
        self.create.unwrap()(a)
    }

    pub unsafe fn destroy(&self, cdi: *mut ConfigI) {
        self.destroy.unwrap()(cdi)
    }
}

impl crate::Api for ConfigApi {
    const NAME: ConstCStr = const_cstr!("tm_config_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl CoreApi {
    pub unsafe fn create(&self, tt: *mut TheTruthO, asset_root_id: TtIdT) {
        self.create.unwrap()(tt, asset_root_id)
    }

    pub unsafe fn query_updatable(
        &self,
        tt: *mut TheTruthO,
        asset_root_id: TtIdT,
        include_skipped: bool,
        ta: *mut TempAllocatorI,
    ) -> CoreUpdatableAssetsT {
        self.query_updatable.unwrap()(tt, asset_root_id, include_skipped, ta)
    }

    pub unsafe fn update(
        &self,
        tt: *mut TheTruthO,
        asset_root_id: TtIdT,
        to_skip: *mut *mut CoreAssetI,
        num_to_skip: u32,
        include_skipped: bool,
        ta: *mut TempAllocatorI,
    ) -> CoreUpdateResultT {
        self.update.unwrap()(tt, asset_root_id, to_skip, num_to_skip, include_skipped, ta)
    }

    pub unsafe fn locate_asset(
        &self,
        tt: *mut TheTruthO,
        asset_root_id: TtIdT,
        core_id: StrhashT,
    ) -> TtIdT {
        self.locate_asset.unwrap()(tt, asset_root_id, core_id)
    }
}

impl crate::Api for CoreApi {
    const NAME: ConstCStr = const_cstr!("tm_core_api");
    const VERSION: VersionT = VersionT {
        major: 2u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl CoreImporterApi {
    pub unsafe fn create(
        &self,
        a: *mut AllocatorI,
        user_tt: *mut TheTruthO,
        user_asset_root: TtIdT,
        core_project_path: *const ::std::os::raw::c_char,
        output_path: *const ::std::os::raw::c_char,
    ) -> *mut CoreImporterStateO {
        self.create.unwrap()(a, user_tt, user_asset_root, core_project_path, output_path)
    }

    pub unsafe fn destroy(&self, state: *mut CoreImporterStateO) {
        self.destroy.unwrap()(state)
    }

    pub unsafe fn register_assets(
        &self,
        state: *mut CoreImporterStateO,
        to_register: *const CoreImporterAssetT,
        num_to_register: u32,
    ) {
        self.register_assets.unwrap()(state, to_register, num_to_register)
    }
}

impl crate::Api for CoreImporterApi {
    const NAME: ConstCStr = const_cstr!("tm_core_importer_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl CrashRecoveryApi {
    pub unsafe fn create(
        &self,
        a: *mut AllocatorI,
        recovery_path: *const ::std::os::raw::c_char,
    ) -> *mut CrashRecoveryO {
        self.create.unwrap()(a, recovery_path)
    }

    pub unsafe fn destroy(&self, cr: *mut CrashRecoveryO) {
        self.destroy.unwrap()(cr)
    }

    pub unsafe fn start_recording(
        &self,
        cr: *mut CrashRecoveryO,
        project: *const ::std::os::raw::c_char,
        tt: *mut TheTruthO,
        root: TtIdT,
    ) {
        self.start_recording.unwrap()(cr, project, tt, root)
    }

    pub unsafe fn stop_recording(&self, cr: *mut CrashRecoveryO) {
        self.stop_recording.unwrap()(cr)
    }

    pub unsafe fn update(&self, cr: *mut CrashRecoveryO) {
        self.update.unwrap()(cr)
    }

    pub unsafe fn can_recover(
        &self,
        cr: *mut CrashRecoveryO,
        ta: *mut TempAllocatorI,
    ) -> CrashRecoveryCanRecoverResultT {
        self.can_recover.unwrap()(cr, ta)
    }

    pub unsafe fn recover(
        &self,
        cr: *mut CrashRecoveryO,
        rd: CrashRecoveryCanRecoverResultT,
        tt: *mut TheTruthO,
    ) {
        self.recover.unwrap()(cr, rd, tt)
    }

    pub unsafe fn delete_physical_file(&self, cr: *mut CrashRecoveryO) {
        self.delete_physical_file.unwrap()(cr)
    }
}

impl crate::Api for CrashRecoveryApi {
    const NAME: ConstCStr = const_cstr!("tm_crash_recovery_api");
    const VERSION: VersionT = VersionT {
        major: 2u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl ErrorApi {
    pub unsafe fn create_record_handler(&self, mem: *mut ErrorRecordT) -> ErrorI {
        self.create_record_handler.unwrap()(mem)
    }
}

impl crate::Api for ErrorApi {
    const NAME: ConstCStr = const_cstr!("tm_error_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl FeatureFlagsApi {
    pub unsafe fn enabled(&self, flag: StrhashT) -> bool {
        self.enabled.unwrap()(flag)
    }

    pub unsafe fn set_enabled(&self, flag: StrhashT, enabled: bool) {
        self.set_enabled.unwrap()(flag, enabled)
    }

    pub unsafe fn all_enabled(&self, count: *mut u32) -> *const StrhashT {
        self.all_enabled.unwrap()(count)
    }
}

impl crate::Api for FeatureFlagsApi {
    const NAME: ConstCStr = const_cstr!("tm_feature_flags_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl GitIgnoreApi {
    pub unsafe fn match_(
        &self,
        patterns: *const ::std::os::raw::c_char,
        path: *const ::std::os::raw::c_char,
    ) -> bool {
        self.match_.unwrap()(patterns, path)
    }
}

impl crate::Api for GitIgnoreApi {
    const NAME: ConstCStr = const_cstr!("tm_git_ignore_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl ImageLoaderApi {
    pub unsafe fn add_loader(&self, loader: *mut ImageLoaderI) {
        self.add_loader.unwrap()(loader)
    }

    pub unsafe fn remove_loader(&self, loader: *mut ImageLoaderI) {
        self.remove_loader.unwrap()(loader)
    }

    pub unsafe fn loader_from_archive(
        &self,
        image_archive: *mut ImageArchiveI,
    ) -> *mut ImageLoaderI {
        self.loader_from_archive.unwrap()(image_archive)
    }

    pub unsafe fn loader_from_extension(
        &self,
        extension: *const ::std::os::raw::c_char,
    ) -> *mut ImageLoaderI {
        self.loader_from_extension.unwrap()(extension)
    }

    pub unsafe fn loaders(&self, loaders: *mut *mut ImageLoaderI) -> u32 {
        self.loaders.unwrap()(loaders)
    }
}

impl crate::Api for ImageLoaderApi {
    const NAME: ConstCStr = const_cstr!("tm_image_loader_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl InputApi {
    pub unsafe fn add_source(&self, source: *mut InputSourceI) {
        self.add_source.unwrap()(source)
    }

    pub unsafe fn remove_source(&self, source: *mut InputSourceI) {
        self.remove_source.unwrap()(source)
    }

    pub unsafe fn sources(&self, sources: *mut *mut *mut InputSourceI) -> u32 {
        self.sources.unwrap()(sources)
    }

    pub unsafe fn events(&self, start: u64, events: *mut InputEventT, buffer_size: u64) -> u64 {
        self.events.unwrap()(start, events, buffer_size)
    }

    pub unsafe fn keyboard_item_names(&self) -> *mut *const ::std::os::raw::c_char {
        self.keyboard_item_names.unwrap()()
    }
}

impl crate::Api for InputApi {
    const NAME: ConstCStr = const_cstr!("tm_input_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl JobSystemApi {
    pub unsafe fn run_jobs(&self, jobs: *mut JobdeclT, num_jobs: u32) -> *mut AtomicCounterO {
        self.run_jobs.unwrap()(jobs, num_jobs)
    }

    pub unsafe fn run_jobs_and_auto_free_counter(&self, jobs: *mut JobdeclT, num_jobs: u32) {
        self.run_jobs_and_auto_free_counter.unwrap()(jobs, num_jobs)
    }

    pub unsafe fn wait_for_counter(&self, counter: *mut AtomicCounterO, value: u32) {
        self.wait_for_counter.unwrap()(counter, value)
    }

    pub unsafe fn wait_for_counter_and_free(&self, counter: *mut AtomicCounterO) {
        self.wait_for_counter_and_free.unwrap()(counter)
    }

    pub unsafe fn wait_for_counter_and_free_no_fiber(&self, counter: *mut AtomicCounterO) {
        self.wait_for_counter_and_free_no_fiber.unwrap()(counter)
    }

    pub unsafe fn pin_thread_handle(&self, worker_thread_index: u32) -> u32 {
        self.pin_thread_handle.unwrap()(worker_thread_index)
    }

    pub unsafe fn num_worker_threads(&self) -> u32 {
        self.num_worker_threads.unwrap()()
    }
}

impl crate::Api for JobSystemApi {
    const NAME: ConstCStr = const_cstr!("tm_job_system_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl JsonApi {
    pub unsafe fn parse(
        &self,
        s: *const ::std::os::raw::c_char,
        config: *mut ConfigI,
        extensions: JsonParseExt,
        error: *mut ::std::os::raw::c_char,
    ) -> bool {
        self.parse.unwrap()(s, config, extensions, error)
    }

    pub unsafe fn parse_with_line_info(
        &self,
        s: *const ::std::os::raw::c_char,
        config: *mut ConfigI,
        extensions: JsonParseExt,
        ta: *mut TempAllocatorI,
    ) -> *mut JsonParseInfoT {
        self.parse_with_line_info.unwrap()(s, config, extensions, ta)
    }

    pub unsafe fn line_number(&self, pi: *const JsonParseInfoT, item: *const ConfigItemT) -> u32 {
        self.line_number.unwrap()(pi, item)
    }

    pub unsafe fn generate(
        &self,
        config: *mut ConfigI,
        flags: JsonGenerateExt,
        ta: *mut TempAllocatorI,
    ) -> JsonGenerateT {
        self.generate.unwrap()(config, flags, ta)
    }
}

impl crate::Api for JsonApi {
    const NAME: ConstCStr = const_cstr!("tm_json_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl LocalizerApi {}

impl crate::Api for LocalizerApi {
    const NAME: ConstCStr = const_cstr!("tm_localizer_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl LoggerApi {
    pub unsafe fn add_logger(&self, logger: *const LoggerI) {
        self.add_logger.unwrap()(logger)
    }

    pub unsafe fn remove_logger(&self, logger: *const LoggerI) {
        self.remove_logger.unwrap()(logger)
    }

    pub unsafe fn print(&self, log_type: LogType, msg: *const ::std::os::raw::c_char) {
        self.print.unwrap()(log_type, msg)
    }

    pub unsafe fn printf(
        &self,
        log_type: LogType,
        format: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        self.printf.unwrap()(log_type, format)
    }
}

impl crate::Api for LoggerApi {
    const NAME: ConstCStr = const_cstr!("tm_logger_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl Lz4Api {
    pub unsafe fn compress(
        &self,
        src: *const ::std::os::raw::c_char,
        src_size: u32,
        dst: *mut ::std::os::raw::c_char,
        dst_capacity: u32,
    ) -> u32 {
        self.compress.unwrap()(src, src_size, dst, dst_capacity)
    }

    pub unsafe fn decompress(
        &self,
        src: *const ::std::os::raw::c_char,
        src_size: u32,
        dst: *mut ::std::os::raw::c_char,
        dst_capacity: u32,
    ) -> u32 {
        self.decompress.unwrap()(src, src_size, dst, dst_capacity)
    }

    pub unsafe fn compress_bound(&self, src_size: u32) -> u32 {
        self.compress_bound.unwrap()(src_size)
    }
}

impl crate::Api for Lz4Api {
    const NAME: ConstCStr = const_cstr!("tm_lz4_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl MathApi {
    pub unsafe fn mat44_multiply(&self, res: *mut Mat44T, lhs: *const Mat44T, rhs: *const Mat44T) {
        self.mat44_multiply.unwrap()(res, lhs, rhs)
    }

    pub unsafe fn mat44_inverse(&self, res: *mut Mat44T, m: *const Mat44T) {
        self.mat44_inverse.unwrap()(res, m)
    }

    pub unsafe fn mat44_determinant(&self, m: *const Mat44T) -> f32 {
        self.mat44_determinant.unwrap()(m)
    }

    pub unsafe fn mat44_determinant33(&self, m: *const Mat44T) -> f32 {
        self.mat44_determinant33.unwrap()(m)
    }

    pub unsafe fn mat44_to_quaternion(&self, m: *const Mat44T) -> Vec4T {
        self.mat44_to_quaternion.unwrap()(m)
    }

    pub unsafe fn mat44_to_translation_quaternion_scale(
        &self,
        t: *mut Vec3T,
        r: *mut Vec4T,
        s: *mut Vec3T,
        m: *const Mat44T,
    ) {
        self.mat44_to_translation_quaternion_scale.unwrap()(t, r, s, m)
    }
}

impl crate::Api for MathApi {
    const NAME: ConstCStr = const_cstr!("tm_math_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl MemoryTrackerApi {
    pub unsafe fn check_for_leaked_scopes(&self) {
        self.check_for_leaked_scopes.unwrap()()
    }

    pub unsafe fn create_scope(
        &self,
        desc: *const ::std::os::raw::c_char,
        parent_scope: u32,
    ) -> u32 {
        self.create_scope.unwrap()(desc, parent_scope)
    }

    pub unsafe fn destroy_scope(&self, s: u32) {
        self.destroy_scope.unwrap()(s)
    }

    pub unsafe fn destroy_scope_allowing_leaks(&self, scope: u32, max_leaked_bytes: u64) {
        self.destroy_scope_allowing_leaks.unwrap()(scope, max_leaked_bytes)
    }

    pub unsafe fn record_realloc(
        &self,
        old_ptr: *mut ::std::os::raw::c_void,
        old_size: u64,
        new_ptr: *mut ::std::os::raw::c_void,
        new_size: u64,
        file: *const ::std::os::raw::c_char,
        line: u32,
        scope: u32,
    ) {
        self.record_realloc.unwrap()(old_ptr, old_size, new_ptr, new_size, file, line, scope)
    }

    pub unsafe fn allocated_bytes(&self, scope: u32) -> u64 {
        self.allocated_bytes.unwrap()(scope)
    }

    pub unsafe fn allocation_count(&self, scope: u32) -> u64 {
        self.allocation_count.unwrap()(scope)
    }

    pub unsafe fn set_scope_tracing(&self, scope: u32, enabled: bool) {
        self.set_scope_tracing.unwrap()(scope, enabled)
    }

    pub unsafe fn scope_data_snapshot(
        &self,
        ta: *mut TempAllocatorI,
    ) -> *mut MemoryTrackerScopeDataT {
        self.scope_data_snapshot.unwrap()(ta)
    }

    pub unsafe fn trace_data_snapshot(
        &self,
        ta: *mut TempAllocatorI,
    ) -> *mut MemoryTrackerTraceDataT {
        self.trace_data_snapshot.unwrap()(ta)
    }
}

impl crate::Api for MemoryTrackerApi {
    const NAME: ConstCStr = const_cstr!("tm_memory_tracker_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl OsVirtualMemoryApi {
    pub unsafe fn map(&self, size: u64) -> *mut ::std::os::raw::c_void {
        self.map.unwrap()(size)
    }

    pub unsafe fn unmap(&self, p: *mut ::std::os::raw::c_void, size: u64) {
        self.unmap.unwrap()(p, size)
    }

    pub unsafe fn reserve(&self, size: u64) -> *mut ::std::os::raw::c_void {
        self.reserve.unwrap()(size)
    }

    pub unsafe fn commit(&self, p: *mut ::std::os::raw::c_void, size: u64) {
        self.commit.unwrap()(p, size)
    }
}

impl crate::Api for OsVirtualMemoryApi {
    const NAME: ConstCStr = const_cstr!("tm_os_virtual_memory_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl OsFileIoApi {
    pub unsafe fn open_input(&self, path: *const ::std::os::raw::c_char) -> FileO {
        self.open_input.unwrap()(path)
    }

    pub unsafe fn open_output(&self, path: *const ::std::os::raw::c_char) -> FileO {
        self.open_output.unwrap()(path)
    }

    pub unsafe fn open_append(&self, path: *const ::std::os::raw::c_char) -> FileO {
        self.open_append.unwrap()(path)
    }

    pub unsafe fn set_position(&self, file: FileO, pos: u64) {
        self.set_position.unwrap()(file, pos)
    }

    pub unsafe fn size(&self, file: FileO) -> u64 {
        self.size.unwrap()(file)
    }

    pub unsafe fn read(&self, file: FileO, buffer: *mut ::std::os::raw::c_void, size: u64) -> i64 {
        self.read.unwrap()(file, buffer, size)
    }

    pub unsafe fn write(
        &self,
        file: FileO,
        buffer: *const ::std::os::raw::c_void,
        size: u64,
    ) -> bool {
        self.write.unwrap()(file, buffer, size)
    }

    pub unsafe fn read_at(
        &self,
        file: FileO,
        offset: u64,
        buffer: *mut ::std::os::raw::c_void,
        size: u64,
    ) -> i64 {
        self.read_at.unwrap()(file, offset, buffer, size)
    }

    pub unsafe fn write_at(
        &self,
        file: FileO,
        offset: u64,
        buffer: *const ::std::os::raw::c_void,
        size: u64,
    ) -> bool {
        self.write_at.unwrap()(file, offset, buffer, size)
    }

    pub unsafe fn set_last_modified_time(&self, file: FileO, time: FileTimeO) {
        self.set_last_modified_time.unwrap()(file, time)
    }

    pub unsafe fn close(&self, file: FileO) {
        self.close.unwrap()(file)
    }
}

impl crate::Api for OsFileIoApi {
    const NAME: ConstCStr = const_cstr!("tm_os_file_io_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl OsFileSystemApi {
    pub unsafe fn stat(&self, path: *const ::std::os::raw::c_char) -> FileStatT {
        self.stat.unwrap()(path)
    }

    pub unsafe fn directory_entries(
        &self,
        path: *const ::std::os::raw::c_char,
        ta: *mut TempAllocatorI,
    ) -> *mut StringsT {
        self.directory_entries.unwrap()(path, ta)
    }

    pub unsafe fn make_directory(&self, path: *const ::std::os::raw::c_char) -> bool {
        self.make_directory.unwrap()(path)
    }

    pub unsafe fn remove_file(&self, path: *const ::std::os::raw::c_char) -> bool {
        self.remove_file.unwrap()(path)
    }

    pub unsafe fn remove_directory(&self, path: *const ::std::os::raw::c_char) -> bool {
        self.remove_directory.unwrap()(path)
    }

    pub unsafe fn rename(
        &self,
        old_name: *const ::std::os::raw::c_char,
        new_name: *const ::std::os::raw::c_char,
    ) -> bool {
        self.rename.unwrap()(old_name, new_name)
    }

    pub unsafe fn copy_file(
        &self,
        from: *const ::std::os::raw::c_char,
        to: *const ::std::os::raw::c_char,
    ) -> bool {
        self.copy_file.unwrap()(from, to)
    }

    pub unsafe fn getcwd(&self, ta: *mut TempAllocatorI) -> *const ::std::os::raw::c_char {
        self.getcwd.unwrap()(ta)
    }

    pub unsafe fn chdir(&self, path: *const ::std::os::raw::c_char) -> bool {
        self.chdir.unwrap()(path)
    }

    pub unsafe fn is_absolute(&self, path: *const ::std::os::raw::c_char) -> bool {
        self.is_absolute.unwrap()(path)
    }

    pub unsafe fn absolute(
        &self,
        path: *const ::std::os::raw::c_char,
        ta: *mut TempAllocatorI,
    ) -> *const ::std::os::raw::c_char {
        self.absolute.unwrap()(path, ta)
    }

    pub unsafe fn temp_directory(&self, ta: *mut TempAllocatorI) -> *const ::std::os::raw::c_char {
        self.temp_directory.unwrap()(ta)
    }

    pub unsafe fn create_watcher(
        &self,
        subtree_path: *const ::std::os::raw::c_char,
    ) -> FileSystemWatcherO {
        self.create_watcher.unwrap()(subtree_path)
    }

    pub unsafe fn any_changes(&self, watcher: FileSystemWatcherO) -> bool {
        self.any_changes.unwrap()(watcher)
    }

    pub unsafe fn destroy_watcher(&self, watcher: FileSystemWatcherO) {
        self.destroy_watcher.unwrap()(watcher)
    }

    pub unsafe fn create_detailed_watcher(
        &self,
        subtree_path: *const ::std::os::raw::c_char,
    ) -> *mut FileSystemDetailedWatcherO {
        self.create_detailed_watcher.unwrap()(subtree_path)
    }

    pub unsafe fn detailed_changes(
        &self,
        arg1: *mut FileSystemDetailedWatcherO,
        ta: *mut TempAllocatorI,
    ) -> *mut FileSystemChangeT {
        self.detailed_changes.unwrap()(arg1, ta)
    }

    pub unsafe fn destroy_detailed_watcher(&self, watcher: *mut FileSystemDetailedWatcherO) {
        self.destroy_detailed_watcher.unwrap()(watcher)
    }

    pub unsafe fn app_folder(&self, ta: *mut TempAllocatorI) -> *const ::std::os::raw::c_char {
        self.app_folder.unwrap()(ta)
    }
}

impl crate::Api for OsFileSystemApi {
    const NAME: ConstCStr = const_cstr!("tm_os_file_system_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl OsDllApi {
    pub unsafe fn open(&self, path: *const ::std::os::raw::c_char) -> DllO {
        self.open.unwrap()(path)
    }

    pub unsafe fn get(&self, path: *const ::std::os::raw::c_char) -> DllO {
        self.get.unwrap()(path)
    }

    pub unsafe fn sym(
        &self,
        handle: DllO,
        name: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void {
        self.sym.unwrap()(handle, name)
    }

    pub unsafe fn close(&self, handle: DllO) {
        self.close.unwrap()(handle)
    }
}

impl OsSocketApi {
    pub unsafe fn init(&self) {
        self.init.unwrap()()
    }

    pub unsafe fn shutdown(&self) {
        self.shutdown.unwrap()()
    }

    pub unsafe fn socket(&self, type_: OsSocketType) -> SocketO {
        self.socket.unwrap()(type_)
    }

    pub unsafe fn set_option(&self, socket: SocketO, option: OsSocketOption, enabled: bool) {
        self.set_option.unwrap()(socket, option, enabled)
    }

    pub unsafe fn bind(&self, socket: SocketO, address: SocketAddressT) -> bool {
        self.bind.unwrap()(socket, address)
    }

    pub unsafe fn getsockname(&self, socket: SocketO, address: *mut SocketAddressT) -> bool {
        self.getsockname.unwrap()(socket, address)
    }

    pub unsafe fn listen(&self, socket: SocketO, queue_size: u32) -> bool {
        self.listen.unwrap()(socket, queue_size)
    }

    pub unsafe fn accept(&self, socket: SocketO, address: *mut SocketAddressT) -> SocketO {
        self.accept.unwrap()(socket, address)
    }

    pub unsafe fn connect(&self, socket: SocketO, target: SocketAddressT) -> OsSocketConnect {
        self.connect.unwrap()(socket, target)
    }

    pub unsafe fn send(
        &self,
        socket: SocketO,
        buffer: *const ::std::os::raw::c_void,
        size: u32,
    ) -> i32 {
        self.send.unwrap()(socket, buffer, size)
    }

    pub unsafe fn recv(
        &self,
        socket: SocketO,
        buffer: *mut ::std::os::raw::c_void,
        size: u32,
    ) -> i32 {
        self.recv.unwrap()(socket, buffer, size)
    }

    pub unsafe fn sendto(
        &self,
        socket: SocketO,
        buffer: *const ::std::os::raw::c_void,
        size: u32,
        target: SocketAddressT,
    ) -> i32 {
        self.sendto.unwrap()(socket, buffer, size, target)
    }

    pub unsafe fn recvfrom(
        &self,
        socket: SocketO,
        buffer: *mut ::std::os::raw::c_void,
        size: u32,
        source: *mut SocketAddressT,
    ) -> i32 {
        self.recvfrom.unwrap()(socket, buffer, size, source)
    }

    pub unsafe fn close(&self, socket: SocketO) -> bool {
        self.close.unwrap()(socket)
    }

    pub unsafe fn getaddrinfo(
        &self,
        name: *const ::std::os::raw::c_char,
        service: *const ::std::os::raw::c_char,
        addresses: *mut SocketAddressT,
        size: u32,
    ) -> u32 {
        self.getaddrinfo.unwrap()(name, service, addresses, size)
    }

    pub unsafe fn getaddrinfo_async(
        &self,
        name: *const ::std::os::raw::c_char,
        service: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void {
        self.getaddrinfo_async.unwrap()(name, service)
    }

    pub unsafe fn getaddrinfo_result(
        &self,
        query: *mut ::std::os::raw::c_void,
        addresses: *mut SocketAddressT,
        count: *mut u32,
    ) -> OsSocketGetaddrinfo {
        self.getaddrinfo_result.unwrap()(query, addresses, count)
    }
}

impl crate::Api for OsSocketApi {
    const NAME: ConstCStr = const_cstr!("tm_os_socket_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl OsThreadApi {
    pub unsafe fn create_critical_section(&self, cs: *mut CriticalSectionO) {
        self.create_critical_section.unwrap()(cs)
    }

    pub unsafe fn enter_critical_section(&self, cs: *mut CriticalSectionO) {
        self.enter_critical_section.unwrap()(cs)
    }

    pub unsafe fn leave_critical_section(&self, cs: *mut CriticalSectionO) {
        self.leave_critical_section.unwrap()(cs)
    }

    pub unsafe fn destroy_critical_section(&self, cs: *mut CriticalSectionO) {
        self.destroy_critical_section.unwrap()(cs)
    }

    pub unsafe fn create_semaphore(&self, initial_count: u32) -> SemaphoreO {
        self.create_semaphore.unwrap()(initial_count)
    }

    pub unsafe fn semaphore_add(&self, sem: SemaphoreO, count: u32) {
        self.semaphore_add.unwrap()(sem, count)
    }

    pub unsafe fn semaphore_wait(&self, sem: SemaphoreO) {
        self.semaphore_wait.unwrap()(sem)
    }

    pub unsafe fn semaphore_poll(&self, sem: SemaphoreO) -> bool {
        self.semaphore_poll.unwrap()(sem)
    }

    pub unsafe fn destroy_semaphore(&self, sem: SemaphoreO) {
        self.destroy_semaphore.unwrap()(sem)
    }

    pub unsafe fn thread_id(&self) -> u32 {
        self.thread_id.unwrap()()
    }

    pub unsafe fn processor_id(&self) -> u32 {
        self.processor_id.unwrap()()
    }

    pub unsafe fn create_thread(
        &self,
        entry: ThreadEntryF,
        user_data: *mut ::std::os::raw::c_void,
        stack_size: u32,
        debug_name: *const ::std::os::raw::c_char,
    ) -> ThreadO {
        self.create_thread.unwrap()(entry, user_data, stack_size, debug_name)
    }

    pub unsafe fn set_thread_priority(&self, thread: ThreadO, priority: OsThreadPriority) {
        self.set_thread_priority.unwrap()(thread, priority)
    }

    pub unsafe fn wait_for_thread(&self, thread: ThreadO) {
        self.wait_for_thread.unwrap()(thread)
    }

    pub unsafe fn thread_id_from_tm_thread(&self, thread: ThreadO) -> u32 {
        self.thread_id_from_tm_thread.unwrap()(thread)
    }

    pub unsafe fn convert_thread_to_fiber(&self, user_data: *mut ::std::os::raw::c_void) -> FiberO {
        self.convert_thread_to_fiber.unwrap()(user_data)
    }

    pub unsafe fn convert_fiber_to_thread(&self) {
        self.convert_fiber_to_thread.unwrap()()
    }

    pub unsafe fn create_fiber(
        &self,
        entry: FiberEntryF,
        user_data: *mut ::std::os::raw::c_void,
        stack_size: u32,
    ) -> FiberO {
        self.create_fiber.unwrap()(entry, user_data, stack_size)
    }

    pub unsafe fn destroy_fiber(&self, fiber: FiberO) {
        self.destroy_fiber.unwrap()(fiber)
    }

    pub unsafe fn switch_to_fiber(&self, fiber: FiberO) {
        self.switch_to_fiber.unwrap()(fiber)
    }

    pub unsafe fn fiber_user_data(&self) -> *mut ::std::os::raw::c_void {
        self.fiber_user_data.unwrap()()
    }

    pub unsafe fn yield_processor(&self) {
        self.yield_processor.unwrap()()
    }

    pub unsafe fn sleep(&self, seconds: f64) {
        self.sleep.unwrap()(seconds)
    }
}

impl OsTimeApi {
    pub unsafe fn now(&self) -> ClockO {
        self.now.unwrap()()
    }

    pub unsafe fn delta(&self, to: ClockO, from: ClockO) -> f64 {
        self.delta.unwrap()(to, from)
    }

    pub unsafe fn add(&self, from: ClockO, delta: f64) -> ClockO {
        self.add.unwrap()(from, delta)
    }

    pub unsafe fn file_time_now(&self) -> FileTimeO {
        self.file_time_now.unwrap()()
    }

    pub unsafe fn file_time_delta(&self, to: FileTimeO, from: FileTimeO) -> f64 {
        self.file_time_delta.unwrap()(to, from)
    }
}

impl OsDialogsApi {
    pub unsafe fn open(
        &self,
        s: *const OsDialogsOpenT,
        ta: *mut TempAllocatorI,
    ) -> OsDialogsOpenResT {
        self.open.unwrap()(s, ta)
    }

    pub unsafe fn open_folder(&self, ta: *mut TempAllocatorI) -> *mut ::std::os::raw::c_char {
        self.open_folder.unwrap()(ta)
    }

    pub unsafe fn save(
        &self,
        s: *const OsDialogsSaveT,
        ta: *mut TempAllocatorI,
    ) -> *mut ::std::os::raw::c_char {
        self.save.unwrap()(s, ta)
    }

    pub unsafe fn message_box(
        &self,
        title: *const ::std::os::raw::c_char,
        text: *const ::std::os::raw::c_char,
    ) {
        self.message_box.unwrap()(title, text)
    }

    pub unsafe fn show_count(&self) -> u64 {
        self.show_count.unwrap()()
    }
}

impl OsInfoApi {
    pub unsafe fn num_logical_processors(&self) -> u32 {
        self.num_logical_processors.unwrap()()
    }

    pub unsafe fn avx_support(&self) -> bool {
        self.avx_support.unwrap()()
    }
}

impl OsDebuggerApi {
    pub unsafe fn is_debugger_present(&self) -> bool {
        self.is_debugger_present.unwrap()()
    }

    pub unsafe fn debug_break(&self) {
        self.debug_break.unwrap()()
    }

    pub unsafe fn print_stack_trace(&self) {
        self.print_stack_trace.unwrap()()
    }
}

impl OsSystemApi {
    pub unsafe fn open_url(&self, url: *const ::std::os::raw::c_char) {
        self.open_url.unwrap()(url)
    }

    pub unsafe fn open_file(&self, file: *const ::std::os::raw::c_char) -> bool {
        self.open_file.unwrap()(file)
    }

    pub unsafe fn exe_path(
        &self,
        argv_0: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char {
        self.exe_path.unwrap()(argv_0)
    }

    pub unsafe fn execute(&self, command: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int {
        self.execute.unwrap()(command)
    }

    pub unsafe fn execute_in_background(
        &self,
        command: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        self.execute_in_background.unwrap()(command)
    }

    pub unsafe fn execute_stdout(
        &self,
        command: *const ::std::os::raw::c_char,
        timeout_ms: u32,
        ta: *mut TempAllocatorI,
        exit_code: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char {
        self.execute_stdout.unwrap()(command, timeout_ms, ta, exit_code)
    }
}

impl OsApi {}

impl crate::Api for OsApi {
    const NAME: ConstCStr = const_cstr!("tm_os_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl PathApi {
    pub unsafe fn extension(&self, path: StrT) -> StrT {
        self.extension.unwrap()(path)
    }

    pub unsafe fn extension_cstr(
        &self,
        path: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char {
        self.extension_cstr.unwrap()(path)
    }

    pub unsafe fn strip_extension(&self, path: StrT) -> StrT {
        self.strip_extension.unwrap()(path)
    }

    pub unsafe fn base(&self, path: StrT) -> StrT {
        self.base.unwrap()(path)
    }

    pub unsafe fn base_cstr(
        &self,
        path: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char {
        self.base_cstr.unwrap()(path)
    }

    pub unsafe fn directory(&self, path: StrT) -> StrT {
        self.directory.unwrap()(path)
    }

    pub unsafe fn pop_last(&self, path: *mut StrT) -> StrT {
        self.pop_last.unwrap()(path)
    }

    pub unsafe fn pop_first(&self, path: *mut StrT) -> StrT {
        self.pop_first.unwrap()(path)
    }

    pub unsafe fn split_all(&self, path: StrT, ta: *mut TempAllocatorI) -> *mut StrT {
        self.split_all.unwrap()(path, ta)
    }

    pub unsafe fn join(&self, a: StrT, b: StrT, ta: *mut TempAllocatorI) -> StrT {
        self.join.unwrap()(a, b, ta)
    }
}

impl crate::Api for PathApi {
    const NAME: ConstCStr = const_cstr!("tm_path_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl PluginsApi {
    pub unsafe fn load(&self, path: *const ::std::os::raw::c_char, hot_reload: bool) -> u64 {
        self.load.unwrap()(path, hot_reload)
    }

    pub unsafe fn unload(&self, plugin: u64) {
        self.unload.unwrap()(plugin)
    }

    pub unsafe fn reload(&self, plugin: u64) {
        self.reload.unwrap()(plugin)
    }

    pub unsafe fn set_path(&self, plugin: u64, path: *const ::std::os::raw::c_char) {
        self.set_path.unwrap()(plugin, path)
    }

    pub unsafe fn check_hot_reload(&self) -> bool {
        self.check_hot_reload.unwrap()()
    }

    pub unsafe fn reload_count(&self) -> u64 {
        self.reload_count.unwrap()()
    }

    pub unsafe fn enumerate(
        &self,
        directory: *const ::std::os::raw::c_char,
        ta: *mut TempAllocatorI,
    ) -> *mut *const ::std::os::raw::c_char {
        self.enumerate.unwrap()(directory, ta)
    }

    pub unsafe fn plugin_dllpath(
        &self,
        ta: *mut TempAllocatorI,
        exe: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char {
        self.plugin_dllpath.unwrap()(ta, exe, name)
    }

    pub unsafe fn app_dllpath(
        &self,
        ta: *mut TempAllocatorI,
        exe: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char {
        self.app_dllpath.unwrap()(ta, exe, name)
    }
}

impl crate::Api for PluginsApi {
    const NAME: ConstCStr = const_cstr!("tm_plugins_api");
    const VERSION: VersionT = VersionT {
        major: 3u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl PluginAssetsApi {
    pub unsafe fn any_plugins(&self, tt: *mut TheTruthO) -> bool {
        self.any_plugins.unwrap()(tt)
    }

    pub unsafe fn init_truth(&self, tt: *mut TheTruthO, allow_code_execution: bool) {
        self.init_truth.unwrap()(tt, allow_code_execution)
    }

    pub unsafe fn shutdown_truth(&self, tt: *mut TheTruthO) {
        self.shutdown_truth.unwrap()(tt)
    }

    pub unsafe fn update_truth(&self, tt: *mut TheTruthO) {
        self.update_truth.unwrap()(tt)
    }

    pub unsafe fn allow_code_execution(&self, tt: *mut TheTruthO) -> bool {
        self.allow_code_execution.unwrap()(tt)
    }

    pub unsafe fn set_allow_code_execution(&self, tt: *mut TheTruthO, allow_code_execution: bool) {
        self.set_allow_code_execution.unwrap()(tt, allow_code_execution)
    }
}

impl crate::Api for PluginAssetsApi {
    const NAME: ConstCStr = const_cstr!("tm_plugin_assets_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl ProfilerApi {
    pub unsafe fn init(&self, allocator: *mut AllocatorI, event_buffer_size: u32) {
        self.init.unwrap()(allocator, event_buffer_size)
    }

    pub unsafe fn shutdown(&self) {
        self.shutdown.unwrap()()
    }

    pub unsafe fn begin(
        &self,
        name: *const ::std::os::raw::c_char,
        category: *const ::std::os::raw::c_char,
        object: *const ::std::os::raw::c_char,
    ) -> u64 {
        self.begin.unwrap()(name, category, object)
    }

    pub unsafe fn end(&self, begin_id: u64) {
        self.end.unwrap()(begin_id)
    }

    pub unsafe fn instant(
        &self,
        name: *const ::std::os::raw::c_char,
        category: *const ::std::os::raw::c_char,
        object: *const ::std::os::raw::c_char,
    ) {
        self.instant.unwrap()(name, category, object)
    }

    pub unsafe fn start(
        &self,
        name: *const ::std::os::raw::c_char,
        category: *const ::std::os::raw::c_char,
        object: *const ::std::os::raw::c_char,
    ) -> u64 {
        self.start.unwrap()(name, category, object)
    }

    pub unsafe fn finish(&self, start_id: u64) {
        self.finish.unwrap()(start_id)
    }

    pub unsafe fn intern(&self, s: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char {
        self.intern.unwrap()(s)
    }

    pub unsafe fn fiber_switch(&self, from_fiber: u32, to_fiber: u32) {
        self.fiber_switch.unwrap()(from_fiber, to_fiber)
    }

    pub unsafe fn submit(&self, events: *mut ProfilerEventT, count: u32) {
        self.submit.unwrap()(events, count)
    }

    pub unsafe fn copy(
        &self,
        dest: *mut ProfilerEventT,
        start: u64,
        count: u32,
        actual_start: *mut u64,
        actual_count: *mut u32,
    ) {
        self.copy.unwrap()(dest, start, count, actual_start, actual_count)
    }

    pub unsafe fn buffer(&self) -> ProfilerBufferT {
        self.buffer.unwrap()()
    }
}

impl crate::Api for ProfilerApi {
    const NAME: ConstCStr = const_cstr!("tm_profiler_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl ProgressReportApi {
    pub unsafe fn create(&self, a: *mut AllocatorI, task_display_time: f32) {
        self.create.unwrap()(a, task_display_time)
    }

    pub unsafe fn destroy(&self) {
        self.destroy.unwrap()()
    }

    pub unsafe fn update(&self, dt: f64, ta: *mut TempAllocatorI) -> TaskProgressT {
        self.update.unwrap()(dt, ta)
    }

    pub unsafe fn status(&self, ta: *mut TempAllocatorI) -> TaskStatusT {
        self.status.unwrap()(ta)
    }

    pub unsafe fn idle(&self) -> bool {
        self.idle.unwrap()()
    }

    pub unsafe fn set_task_progress(
        &self,
        task: u64,
        text: *const ::std::os::raw::c_char,
        fraction: f32,
    ) {
        self.set_task_progress.unwrap()(task, text, fraction)
    }
}

impl crate::Api for ProgressReportApi {
    const NAME: ConstCStr = const_cstr!("tm_progress_report_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl RandomApi {
    pub unsafe fn next(&self) -> u64 {
        self.next.unwrap()()
    }

    pub unsafe fn next_n(&self, res: *mut u64, n: u32) {
        self.next_n.unwrap()(res, n)
    }

    pub unsafe fn seed_new_state(&self, s: *mut u64) {
        self.seed_new_state.unwrap()(s)
    }
}

impl crate::Api for RandomApi {
    const NAME: ConstCStr = const_cstr!("tm_random_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl RuntimeDataRepositoryApi {
    pub unsafe fn create(
        &self,
        allocator: *mut AllocatorI,
        tt: *mut TheTruthO,
        type_: TtTypeT,
        runtime_data_size: u32,
        io_interface: *mut RuntimeDataIoI,
    ) -> *mut RuntimeDataRepositoryO {
        self.create.unwrap()(allocator, tt, type_, runtime_data_size, io_interface)
    }

    pub unsafe fn destroy(
        &self,
        inst: *mut RuntimeDataRepositoryO,
        io_context: *mut ::std::os::raw::c_void,
    ) {
        self.destroy.unwrap()(inst, io_context)
    }

    pub unsafe fn lookup(
        &self,
        inst: *mut RuntimeDataRepositoryO,
        id: TtIdT,
        wanted_version: u64,
        io_context: *mut ::std::os::raw::c_void,
    ) -> RuntimeDataT {
        self.lookup.unwrap()(inst, id, wanted_version, io_context)
    }

    pub unsafe fn garbage_collect(
        &self,
        inst: *mut RuntimeDataRepositoryO,
        io_context: *mut ::std::os::raw::c_void,
    ) {
        self.garbage_collect.unwrap()(inst, io_context)
    }
}

impl crate::Api for RuntimeDataRepositoryApi {
    const NAME: ConstCStr = const_cstr!("tm_runtime_data_repository_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl SprintfApi {
    pub unsafe fn print_unsafe(
        &self,
        buf: *mut ::std::os::raw::c_char,
        fmt: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        self.print_unsafe.unwrap()(buf, fmt)
    }

    pub unsafe fn print(
        &self,
        buf: *mut ::std::os::raw::c_char,
        count: ::std::os::raw::c_int,
        fmt: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        self.print.unwrap()(buf, count, fmt)
    }

    pub unsafe fn vprint_unsafe(
        &self,
        buf: *mut ::std::os::raw::c_char,
        fmt: *const ::std::os::raw::c_char,
        va: va_list,
    ) -> ::std::os::raw::c_int {
        self.vprint_unsafe.unwrap()(buf, fmt, va)
    }

    pub unsafe fn vprint(
        &self,
        buf: *mut ::std::os::raw::c_char,
        count: ::std::os::raw::c_int,
        fmt: *const ::std::os::raw::c_char,
        va: va_list,
    ) -> ::std::os::raw::c_int {
        self.vprint.unwrap()(buf, count, fmt, va)
    }

    pub unsafe fn add_printer(&self, name: *const ::std::os::raw::c_char, printer: SprintfPrinter) {
        self.add_printer.unwrap()(name, printer)
    }
}

impl crate::Api for SprintfApi {
    const NAME: ConstCStr = const_cstr!("tm_sprintf_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl StringApi {
    pub unsafe fn find_unique_name(
        &self,
        taken_names: *mut SetStrhashT,
        ignore_case: bool,
        desired_name: *const ::std::os::raw::c_char,
        separator: *const ::std::os::raw::c_char,
        ta: *mut TempAllocatorI,
    ) -> *const ::std::os::raw::c_char {
        self.find_unique_name.unwrap()(taken_names, ignore_case, desired_name, separator, ta)
    }
}

impl crate::Api for StringApi {
    const NAME: ConstCStr = const_cstr!("tm_string_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl StringRepositoryApi {
    pub unsafe fn create(&self, a: *mut AllocatorI) -> *mut StringRepositoryI {
        self.create.unwrap()(a)
    }

    pub unsafe fn destroy(&self, i: *mut StringRepositoryI) {
        self.destroy.unwrap()(i)
    }
}

impl crate::Api for StringRepositoryApi {
    const NAME: ConstCStr = const_cstr!("tm_string_repository_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl TaskSystemApi {
    pub unsafe fn run_task(
        &self,
        f: ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void, id: u64)>,
        data: *mut ::std::os::raw::c_void,
        debug_name: *const ::std::os::raw::c_char,
    ) -> u64 {
        self.run_task.unwrap()(f, data, debug_name)
    }

    pub unsafe fn is_task_done(&self, id: u64) -> bool {
        self.is_task_done.unwrap()(id)
    }

    pub unsafe fn is_task_done_else_assist(&self, id: u64) -> bool {
        self.is_task_done_else_assist.unwrap()(id)
    }

    pub unsafe fn cancel_task(&self, id: u64) {
        self.cancel_task.unwrap()(id)
    }

    pub unsafe fn is_task_canceled(&self, id: u64) -> bool {
        self.is_task_canceled.unwrap()(id)
    }
}

impl crate::Api for TaskSystemApi {
    const NAME: ConstCStr = const_cstr!("tm_task_system_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl TempAllocatorApi {
    pub unsafe fn create(&self, backing: *mut AllocatorI) -> *mut TempAllocatorI {
        self.create.unwrap()(backing)
    }

    pub unsafe fn create_in_buffer(
        &self,
        buffer: *mut ::std::os::raw::c_char,
        size: u64,
        backing: *mut AllocatorI,
    ) -> *mut TempAllocatorI {
        self.create_in_buffer.unwrap()(buffer, size, backing)
    }

    pub unsafe fn destroy(&self, ta: *mut TempAllocatorI) {
        self.destroy.unwrap()(ta)
    }

    pub unsafe fn allocator(&self, a: *mut AllocatorI, ta: *mut TempAllocatorI) {
        self.allocator.unwrap()(a, ta)
    }

    pub unsafe fn frame_alloc(&self, size: u64) -> *mut ::std::os::raw::c_void {
        self.frame_alloc.unwrap()(size)
    }

    pub unsafe fn frame_allocator(&self) -> *mut AllocatorI {
        self.frame_allocator.unwrap()()
    }

    pub unsafe fn tick_frame(&self) {
        self.tick_frame.unwrap()()
    }

    pub unsafe fn printf(
        &self,
        ta: *mut TempAllocatorI,
        format: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char {
        self.printf.unwrap()(ta, format)
    }

    pub unsafe fn vprintf(
        &self,
        ta: *mut TempAllocatorI,
        format: *const ::std::os::raw::c_char,
        args: va_list,
    ) -> *mut ::std::os::raw::c_char {
        self.vprintf.unwrap()(ta, format, args)
    }

    pub unsafe fn frame_printf(
        &self,
        format: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char {
        self.frame_printf.unwrap()(format)
    }

    pub unsafe fn frame_vprintf(
        &self,
        format: *const ::std::os::raw::c_char,
        args: va_list,
    ) -> *mut ::std::os::raw::c_char {
        self.frame_vprintf.unwrap()(format, args)
    }
}

impl crate::Api for TempAllocatorApi {
    const NAME: ConstCStr = const_cstr!("tm_temp_allocator_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl TheTruthApi {
    pub unsafe fn allocator(&self, tt: *mut TheTruthO) -> *mut AllocatorI {
        self.allocator.unwrap()(tt)
    }

    pub unsafe fn buffers(&self, tt: *mut TheTruthO) -> *mut BuffersI {
        self.buffers.unwrap()(tt)
    }

    pub unsafe fn streamable_buffers(&self, tt: *mut TheTruthO) -> *mut StreamableBuffersI {
        self.streamable_buffers.unwrap()(tt)
    }

    pub unsafe fn create_object_type(
        &self,
        tt: *mut TheTruthO,
        name: *const ::std::os::raw::c_char,
        properties: *const TheTruthPropertyDefinitionT,
        num_properties: u32,
    ) -> TtTypeT {
        self.create_object_type.unwrap()(tt, name, properties, num_properties)
    }

    pub unsafe fn set_default_object(
        &self,
        tt: *mut TheTruthO,
        object_type: TtTypeT,
        object: TtIdT,
    ) {
        self.set_default_object.unwrap()(tt, object_type, object)
    }

    pub unsafe fn set_default_object_to_create_subobjects(
        &self,
        tt: *mut TheTruthO,
        object_type: TtTypeT,
    ) {
        self.set_default_object_to_create_subobjects.unwrap()(tt, object_type)
    }

    pub unsafe fn default_object(&self, tt: *const TheTruthO, object_type: TtTypeT) -> TtIdT {
        self.default_object.unwrap()(tt, object_type)
    }

    pub unsafe fn is_default(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> bool {
        self.is_default.unwrap()(tt, obj, property)
    }

    pub unsafe fn set_aspect(
        &self,
        tt: *mut TheTruthO,
        object_type: TtTypeT,
        aspect: StrhashT,
        data: *const ::std::os::raw::c_void,
    ) {
        self.set_aspect.unwrap()(tt, object_type, aspect, data)
    }

    pub unsafe fn set_default_aspect(
        &self,
        tt: *mut TheTruthO,
        aspect: StrhashT,
        data: *const ::std::os::raw::c_void,
    ) {
        self.set_default_aspect.unwrap()(tt, aspect, data)
    }

    pub unsafe fn set_property_aspect(
        &self,
        tt: *mut TheTruthO,
        object_type: TtTypeT,
        property: u32,
        aspect: StrhashT,
        data: *const ::std::os::raw::c_void,
    ) {
        self.set_property_aspect.unwrap()(tt, object_type, property, aspect, data)
    }

    pub unsafe fn reload_aspects(&self, tt: *mut TheTruthO) {
        self.reload_aspects.unwrap()(tt)
    }

    pub unsafe fn object_type_from_name_hash(
        &self,
        tt: *const TheTruthO,
        name_hash: StrhashT,
    ) -> TtTypeT {
        self.object_type_from_name_hash.unwrap()(tt, name_hash)
    }

    pub unsafe fn optional_object_type_from_name_hash(
        &self,
        tt: *const TheTruthO,
        name_hash: StrhashT,
    ) -> TtTypeT {
        self.optional_object_type_from_name_hash.unwrap()(tt, name_hash)
    }

    pub unsafe fn num_types(&self, tt: *const TheTruthO) -> u32 {
        self.num_types.unwrap()(tt)
    }

    pub unsafe fn type_name(
        &self,
        tt: *const TheTruthO,
        object_type: TtTypeT,
    ) -> *const ::std::os::raw::c_char {
        self.type_name.unwrap()(tt, object_type)
    }

    pub unsafe fn type_name_hash(&self, tt: *const TheTruthO, object_type: TtTypeT) -> StrhashT {
        self.type_name_hash.unwrap()(tt, object_type)
    }

    pub unsafe fn num_properties(&self, tt: *const TheTruthO, object_type: TtTypeT) -> u32 {
        self.num_properties.unwrap()(tt, object_type)
    }

    pub unsafe fn properties(
        &self,
        tt: *const TheTruthO,
        object_type: TtTypeT,
    ) -> *const TheTruthPropertyDefinitionT {
        self.properties.unwrap()(tt, object_type)
    }

    pub unsafe fn find_property(
        &self,
        tt: *const TheTruthO,
        object_type: TtTypeT,
        name_hash: StrhashT,
        type_: TheTruthPropertyType,
        res: *mut u32,
    ) -> bool {
        self.find_property.unwrap()(tt, object_type, name_hash, type_, res)
    }

    pub unsafe fn property_index(
        &self,
        tt: *const TheTruthO,
        type_: TtTypeT,
        name_hash: StrhashT,
    ) -> u32 {
        self.property_index.unwrap()(tt, type_, name_hash)
    }

    pub unsafe fn has_property(
        &self,
        tt: *const TheTruthO,
        type_: TtTypeT,
        name_hash: StrhashT,
    ) -> u32 {
        self.has_property.unwrap()(tt, type_, name_hash)
    }

    pub unsafe fn get_aspect(
        &self,
        tt: *const TheTruthO,
        object_type: TtTypeT,
        aspect: StrhashT,
    ) -> *mut ::std::os::raw::c_void {
        self.get_aspect.unwrap()(tt, object_type, aspect)
    }

    pub unsafe fn get_types_with_aspect(
        &self,
        tt: *const TheTruthO,
        aspect: StrhashT,
        ta: *mut TempAllocatorI,
    ) -> *mut TheTruthGetTypesWithAspectT {
        self.get_types_with_aspect.unwrap()(tt, aspect, ta)
    }

    pub unsafe fn get_aspects(
        &self,
        tt: *const TheTruthO,
        object_type: TtTypeT,
    ) -> *const TheTruthGetAspectsT {
        self.get_aspects.unwrap()(tt, object_type)
    }

    pub unsafe fn get_property_aspect(
        &self,
        tt: *const TheTruthO,
        object_type: TtTypeT,
        property: u32,
        aspect: StrhashT,
    ) -> *mut ::std::os::raw::c_void {
        self.get_property_aspect.unwrap()(tt, object_type, property, aspect)
    }

    pub unsafe fn all_objects_of_type(
        &self,
        tt: *const TheTruthO,
        object_type: TtTypeT,
        ta: *mut TempAllocatorI,
    ) -> *mut TtIdT {
        self.all_objects_of_type.unwrap()(tt, object_type, ta)
    }

    pub unsafe fn create_undo_scope(
        &self,
        tt: *mut TheTruthO,
        name: *const ::std::os::raw::c_char,
    ) -> TtUndoScopeT {
        self.create_undo_scope.unwrap()(tt, name)
    }

    pub unsafe fn create_thread_safe_undo_scope(
        &self,
        tt: *mut TheTruthO,
        name: *const ::std::os::raw::c_char,
    ) -> TtUndoScopeT {
        self.create_thread_safe_undo_scope.unwrap()(tt, name)
    }

    pub unsafe fn undo_scope_name(
        &self,
        tt: *mut TheTruthO,
        scope: TtUndoScopeT,
    ) -> *const ::std::os::raw::c_char {
        self.undo_scope_name.unwrap()(tt, scope)
    }

    pub unsafe fn undo_scope_objects(
        &self,
        tt: *mut TheTruthO,
        scope: TtUndoScopeT,
        ta: *mut TempAllocatorI,
    ) -> *mut TtIdT {
        self.undo_scope_objects.unwrap()(tt, scope, ta)
    }

    pub unsafe fn undo_scope_actions(
        &self,
        tt: *mut TheTruthO,
        scope: TtUndoScopeT,
        ta: *mut TempAllocatorI,
    ) -> *mut TtUndoActionT {
        self.undo_scope_actions.unwrap()(tt, scope, ta)
    }

    pub unsafe fn undo(&self, tt: *mut TheTruthO, scope: TtUndoScopeT) {
        self.undo.unwrap()(tt, scope)
    }

    pub unsafe fn redo(&self, tt: *mut TheTruthO, scope: TtUndoScopeT) {
        self.redo.unwrap()(tt, scope)
    }

    pub unsafe fn create_object_of_type(
        &self,
        tt: *mut TheTruthO,
        type_: TtTypeT,
        undo_scope: TtUndoScopeT,
    ) -> TtIdT {
        self.create_object_of_type.unwrap()(tt, type_, undo_scope)
    }

    pub unsafe fn create_object_of_hash(
        &self,
        tt: *mut TheTruthO,
        type_name_hash: StrhashT,
        undo_scope: TtUndoScopeT,
    ) -> TtIdT {
        self.create_object_of_hash.unwrap()(tt, type_name_hash, undo_scope)
    }

    pub unsafe fn create_object_from_prototype(
        &self,
        tt: *mut TheTruthO,
        prototype: TtIdT,
        undo_scope: TtUndoScopeT,
    ) -> TtIdT {
        self.create_object_from_prototype.unwrap()(tt, prototype, undo_scope)
    }

    pub unsafe fn clone_object(
        &self,
        tt: *mut TheTruthO,
        object: TtIdT,
        undo_scope: TtUndoScopeT,
    ) -> TtIdT {
        self.clone_object.unwrap()(tt, object, undo_scope)
    }

    pub unsafe fn instantiate_subobject(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        undo_scope: TtUndoScopeT,
    ) -> TtIdT {
        self.instantiate_subobject.unwrap()(tt, obj, property, undo_scope)
    }

    pub unsafe fn remove_instantiated_subobject(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        undo_scope: TtUndoScopeT,
    ) {
        self.remove_instantiated_subobject.unwrap()(tt, obj, property, undo_scope)
    }

    pub unsafe fn instantiate_subobject_from_set(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        subobject: TtIdT,
        undo_scope: TtUndoScopeT,
    ) -> TtIdT {
        self.instantiate_subobject_from_set.unwrap()(tt, obj, property, subobject, undo_scope)
    }

    pub unsafe fn remove_instantiated_subobject_from_set(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        subobject: TtIdT,
        undo_scope: TtUndoScopeT,
    ) {
        self.remove_instantiated_subobject_from_set.unwrap()(
            tt, obj, property, subobject, undo_scope,
        )
    }

    pub unsafe fn add_instantiated_subobject_back_to_set(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        subobject: TtIdT,
    ) {
        self.add_instantiated_subobject_back_to_set.unwrap()(tt, obj, property, subobject)
    }

    pub unsafe fn id(&self, obj: *const TheTruthObjectO) -> TtIdT {
        self.id.unwrap()(obj)
    }

    pub unsafe fn destroy_object(
        &self,
        tt: *mut TheTruthO,
        object: TtIdT,
        undo_scope: TtUndoScopeT,
    ) {
        self.destroy_object.unwrap()(tt, object, undo_scope)
    }

    pub unsafe fn destroy_objects(
        &self,
        tt: *mut TheTruthO,
        object: *const TtIdT,
        n: u32,
        undo_scope: TtUndoScopeT,
    ) {
        self.destroy_objects.unwrap()(tt, object, n, undo_scope)
    }

    pub unsafe fn garbage_collect(&self, tt: *mut TheTruthO) {
        self.garbage_collect.unwrap()(tt)
    }

    pub unsafe fn is_alive(&self, tt: *const TheTruthO, object: TtIdT) -> bool {
        self.is_alive.unwrap()(tt, object)
    }

    pub unsafe fn interop_ensure_compatibility(&self, ctx: *mut TheTruthInteropContextT) {
        self.interop_ensure_compatibility.unwrap()(ctx)
    }

    pub unsafe fn interop_clone_object(
        &self,
        ctx: *mut TheTruthInteropContextT,
        object: TtIdT,
    ) -> TtIdT {
        self.interop_clone_object.unwrap()(ctx, object)
    }

    pub unsafe fn deep_clone_assets(
        &self,
        to_tt: *mut TheTruthO,
        from_tt: *const TheTruthO,
        assets: *const TtIdT,
        n: u32,
        undo_scope: TtUndoScopeT,
        ta: *mut TempAllocatorI,
    ) -> *mut TtIdT {
        self.deep_clone_assets.unwrap()(to_tt, from_tt, assets, n, undo_scope, ta)
    }

    pub unsafe fn uuid(&self, tt: *const TheTruthO, object: TtIdT) -> UuidT {
        self.uuid.unwrap()(tt, object)
    }

    pub unsafe fn read(&self, tt: *const TheTruthO, object: TtIdT) -> *const TheTruthObjectO {
        self.read.unwrap()(tt, object)
    }

    pub unsafe fn get_bool(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> bool {
        self.get_bool.unwrap()(tt, obj, property)
    }

    pub unsafe fn get_uint32_t(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> u32 {
        self.get_uint32_t.unwrap()(tt, obj, property)
    }

    pub unsafe fn get_uint64_t(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> u64 {
        self.get_uint64_t.unwrap()(tt, obj, property)
    }

    pub unsafe fn get_float(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> f32 {
        self.get_float.unwrap()(tt, obj, property)
    }

    pub unsafe fn get_double(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> f64 {
        self.get_double.unwrap()(tt, obj, property)
    }

    pub unsafe fn get_string(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> *const ::std::os::raw::c_char {
        self.get_string.unwrap()(tt, obj, property)
    }

    pub unsafe fn get_string_hash(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> StrhashT {
        self.get_string_hash.unwrap()(tt, obj, property)
    }

    pub unsafe fn get_str(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> StrT {
        self.get_str.unwrap()(tt, obj, property)
    }

    pub unsafe fn get_buffer(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> TtBufferT {
        self.get_buffer.unwrap()(tt, obj, property)
    }

    pub unsafe fn get_buffer_id(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> u32 {
        self.get_buffer_id.unwrap()(tt, obj, property)
    }

    pub unsafe fn get_reference(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> TtIdT {
        self.get_reference.unwrap()(tt, obj, property)
    }

    pub unsafe fn get_subobject(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> TtIdT {
        self.get_subobject.unwrap()(tt, obj, property)
    }

    pub unsafe fn get_property_value(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
        ta: *mut TempAllocatorI,
    ) -> TtPropValueT {
        self.get_property_value.unwrap()(tt, obj, property, ta)
    }

    pub unsafe fn property_value_equal(&self, a: TtPropValueT, b: TtPropValueT) -> bool {
        self.property_value_equal.unwrap()(a, b)
    }

    pub unsafe fn get_reference_set(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
        ta: *mut TempAllocatorI,
    ) -> *const TtIdT {
        self.get_reference_set.unwrap()(tt, obj, property, ta)
    }

    pub unsafe fn get_subobject_set(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
        ta: *mut TempAllocatorI,
    ) -> *const TtIdT {
        self.get_subobject_set.unwrap()(tt, obj, property, ta)
    }

    pub unsafe fn get_reference_set_size(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> u64 {
        self.get_reference_set_size.unwrap()(tt, obj, property)
    }

    pub unsafe fn get_subobject_set_size(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> u64 {
        self.get_subobject_set_size.unwrap()(tt, obj, property)
    }

    pub unsafe fn get_subobject_set_type(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> TtTypeT {
        self.get_subobject_set_type.unwrap()(tt, obj, property)
    }

    pub unsafe fn get_subobject_set_locally_removed(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
        ta: *mut TempAllocatorI,
    ) -> *const TtIdT {
        self.get_subobject_set_locally_removed.unwrap()(tt, obj, property, ta)
    }

    pub unsafe fn find_subobject_of_type(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
        type_: TtTypeT,
    ) -> TtIdT {
        self.find_subobject_of_type.unwrap()(tt, obj, property, type_)
    }

    pub unsafe fn is_subobject_of(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
        subobject: TtIdT,
    ) -> bool {
        self.is_subobject_of.unwrap()(tt, obj, property, subobject)
    }

    pub unsafe fn is_in_reference_set(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
        object: TtIdT,
    ) -> bool {
        self.is_in_reference_set.unwrap()(tt, obj, property, object)
    }

    pub unsafe fn property_index_of_subobject(
        &self,
        tt: *const TheTruthO,
        object: TtIdT,
        subobject: TtIdT,
    ) -> u32 {
        self.property_index_of_subobject.unwrap()(tt, object, subobject)
    }

    pub unsafe fn write(&self, tt: *mut TheTruthO, object: TtIdT) -> *mut TheTruthObjectO {
        self.write.unwrap()(tt, object)
    }

    pub unsafe fn commit(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        undo_scope: TtUndoScopeT,
    ) {
        self.commit.unwrap()(tt, obj, undo_scope)
    }

    pub unsafe fn commit_range(
        &self,
        tt: *mut TheTruthO,
        obj: *mut *mut TheTruthObjectO,
        n: u32,
        undo_scope: TtUndoScopeT,
    ) {
        self.commit_range.unwrap()(tt, obj, n, undo_scope)
    }

    pub unsafe fn retarget_write(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        object: TtIdT,
    ) {
        self.retarget_write.unwrap()(tt, obj, object)
    }

    pub unsafe fn try_write(
        &self,
        tt: *mut TheTruthO,
        object: TtIdT,
        original: *mut *const TheTruthObjectO,
    ) -> *mut TheTruthObjectO {
        self.try_write.unwrap()(tt, object, original)
    }

    pub unsafe fn try_commit(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        original: *const TheTruthObjectO,
        undo_scope: TtUndoScopeT,
    ) -> bool {
        self.try_commit.unwrap()(tt, obj, original, undo_scope)
    }

    pub unsafe fn set_bool(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        value: bool,
    ) {
        self.set_bool.unwrap()(tt, obj, property, value)
    }

    pub unsafe fn set_uint32_t(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        value: u32,
    ) {
        self.set_uint32_t.unwrap()(tt, obj, property, value)
    }

    pub unsafe fn set_uint64_t(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        value: u64,
    ) {
        self.set_uint64_t.unwrap()(tt, obj, property, value)
    }

    pub unsafe fn set_float(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        value: f32,
    ) {
        self.set_float.unwrap()(tt, obj, property, value)
    }

    pub unsafe fn set_double(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        value: f64,
    ) {
        self.set_double.unwrap()(tt, obj, property, value)
    }

    pub unsafe fn set_string(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        value: *const ::std::os::raw::c_char,
    ) {
        self.set_string.unwrap()(tt, obj, property, value)
    }

    pub unsafe fn set_str(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        value: StrT,
    ) {
        self.set_str.unwrap()(tt, obj, property, value)
    }

    pub unsafe fn set_buffer(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        value: u32,
    ) {
        self.set_buffer.unwrap()(tt, obj, property, value)
    }

    pub unsafe fn set_buffer_content(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        p: *mut ::std::os::raw::c_void,
        size: u64,
    ) {
        self.set_buffer_content.unwrap()(tt, obj, property, p, size)
    }

    pub unsafe fn set_reference(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        value: TtIdT,
    ) {
        self.set_reference.unwrap()(tt, obj, property, value)
    }

    pub unsafe fn set_subobject(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        value: *mut TheTruthObjectO,
    ) {
        self.set_subobject.unwrap()(tt, obj, property, value)
    }

    pub unsafe fn set_subobject_id(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        value: TtIdT,
        undo_scope: TtUndoScopeT,
    ) {
        self.set_subobject_id.unwrap()(tt, obj, property, value, undo_scope)
    }

    pub unsafe fn set_property_value(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        value: TtPropValueT,
        undo_scope: TtUndoScopeT,
    ) {
        self.set_property_value.unwrap()(tt, obj, property, value, undo_scope)
    }

    pub unsafe fn clear(&self, tt: *mut TheTruthO, obj: *mut TheTruthObjectO, property: u32) {
        self.clear.unwrap()(tt, obj, property)
    }

    pub unsafe fn clear_object(&self, tt: *mut TheTruthO, obj: *mut TheTruthObjectO) {
        self.clear_object.unwrap()(tt, obj)
    }

    pub unsafe fn propagate_property(
        &self,
        tt: *mut TheTruthO,
        object: TtIdT,
        property: u32,
        undo_scope: TtUndoScopeT,
    ) {
        self.propagate_property.unwrap()(tt, object, property, undo_scope)
    }

    pub unsafe fn propagate_property_except(
        &self,
        tt: *mut TheTruthO,
        object: TtIdT,
        property: u32,
        skip: *const TtIdT,
        num_skip: u32,
        undo_scope: TtUndoScopeT,
    ) {
        self.propagate_property_except.unwrap()(tt, object, property, skip, num_skip, undo_scope)
    }

    pub unsafe fn propagate_property_subobject(
        &self,
        tt: *mut TheTruthO,
        object: TtIdT,
        property: u32,
        subobject: TtIdT,
        undo_scope: TtUndoScopeT,
    ) {
        self.propagate_property_subobject.unwrap()(tt, object, property, subobject, undo_scope)
    }

    pub unsafe fn propagate_object(
        &self,
        tt: *mut TheTruthO,
        object: TtIdT,
        undo_scope: TtUndoScopeT,
    ) {
        self.propagate_object.unwrap()(tt, object, undo_scope)
    }

    pub unsafe fn propagate_object_except(
        &self,
        tt: *mut TheTruthO,
        object: TtIdT,
        skip: *const TtIdT,
        num_skip: u32,
        undo_scope: TtUndoScopeT,
    ) {
        self.propagate_object_except.unwrap()(tt, object, skip, num_skip, undo_scope)
    }

    pub unsafe fn add_to_reference_set(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        items: *const TtIdT,
        count: u32,
    ) {
        self.add_to_reference_set.unwrap()(tt, obj, property, items, count)
    }

    pub unsafe fn remove_from_reference_set(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        items: *const TtIdT,
        count: u32,
    ) {
        self.remove_from_reference_set.unwrap()(tt, obj, property, items, count)
    }

    pub unsafe fn clear_reference_set(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
    ) {
        self.clear_reference_set.unwrap()(tt, obj, property)
    }

    pub unsafe fn remove_from_prototype_reference_set(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        items: *const TtIdT,
        count: u32,
    ) {
        self.remove_from_prototype_reference_set.unwrap()(tt, obj, property, items, count)
    }

    pub unsafe fn cancel_remove_from_prototype_reference_set(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        items: *const TtIdT,
        count: u32,
    ) {
        self.cancel_remove_from_prototype_reference_set.unwrap()(tt, obj, property, items, count)
    }

    pub unsafe fn add_to_subobject_set(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        items: *mut *mut TheTruthObjectO,
        count: u32,
    ) {
        self.add_to_subobject_set.unwrap()(tt, obj, property, items, count)
    }

    pub unsafe fn add_to_subobject_set_id(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        items: *const TtIdT,
        count: u32,
        undo_scope: TtUndoScopeT,
    ) {
        self.add_to_subobject_set_id.unwrap()(tt, obj, property, items, count, undo_scope)
    }

    pub unsafe fn remove_from_subobject_set(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        items: *const TtIdT,
        count: u32,
    ) {
        self.remove_from_subobject_set.unwrap()(tt, obj, property, items, count)
    }

    pub unsafe fn clear_subobject_set(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
    ) {
        self.clear_subobject_set.unwrap()(tt, obj, property)
    }

    pub unsafe fn remove_from_prototype_subobject_set(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        items: *const TtIdT,
        count: u32,
    ) {
        self.remove_from_prototype_subobject_set.unwrap()(tt, obj, property, items, count)
    }

    pub unsafe fn cancel_remove_from_prototype_subobject_set(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        items: *const TtIdT,
        count: u32,
    ) {
        self.cancel_remove_from_prototype_subobject_set.unwrap()(tt, obj, property, items, count)
    }

    pub unsafe fn prototype(&self, tt: *const TheTruthO, object: TtIdT) -> TtIdT {
        self.prototype.unwrap()(tt, object)
    }

    pub unsafe fn owner(&self, tt: *const TheTruthO, object: TtIdT) -> TtIdT {
        self.owner.unwrap()(tt, object)
    }

    pub unsafe fn is_currently_owner_of(
        &self,
        tt: *const TheTruthO,
        object: TtIdT,
        subobject: TtIdT,
    ) -> bool {
        self.is_currently_owner_of.unwrap()(tt, object, subobject)
    }

    pub unsafe fn is_overridden(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> bool {
        self.is_overridden.unwrap()(tt, obj, property)
    }

    pub unsafe fn has_data(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> bool {
        self.has_data.unwrap()(tt, obj, property)
    }

    pub unsafe fn prototype_relation(
        &self,
        tt: *const TheTruthO,
        parent: TtIdT,
        property: u32,
        object: TtIdT,
    ) -> TheTruthPrototypeRelation {
        self.prototype_relation.unwrap()(tt, parent, property, object)
    }

    pub unsafe fn version(&self, tt: *const TheTruthO, object: TtIdT) -> u32 {
        self.version.unwrap()(tt, object)
    }

    pub unsafe fn changed_objects(
        &self,
        tt: *const TheTruthO,
        type_: TtTypeT,
        since_version: u64,
        ta: *mut TempAllocatorI,
    ) -> TheTruthChangedObjectsT {
        self.changed_objects.unwrap()(tt, type_, since_version, ta)
    }

    pub unsafe fn request_changelog(&self, tt: *mut TheTruthO) -> u64 {
        self.request_changelog.unwrap()(tt)
    }

    pub unsafe fn relinquish_changelog(&self, tt: *mut TheTruthO, h: u64) {
        self.relinquish_changelog.unwrap()(tt, h)
    }

    pub unsafe fn disable_changelog_start_scope(&self, tt: *mut TheTruthO) {
        self.disable_changelog_start_scope.unwrap()(tt)
    }

    pub unsafe fn disable_changelog_end_scope(&self, tt: *mut TheTruthO) {
        self.disable_changelog_end_scope.unwrap()(tt)
    }

    pub unsafe fn changelog_size(&self, tt: *mut TheTruthO) -> u64 {
        self.changelog_size.unwrap()(tt)
    }

    pub unsafe fn serialize(
        &self,
        tt: *mut TheTruthO,
        o: TtIdT,
        carray: *mut *mut ::std::os::raw::c_char,
        a: *mut AllocatorI,
        opt: *const TtSerializeOptionsT,
    ) {
        self.serialize.unwrap()(tt, o, carray, a, opt)
    }

    pub unsafe fn deserialize(
        &self,
        tt: *mut TheTruthO,
        buffer: *mut *const ::std::os::raw::c_char,
        opt: *const TtDeserializeOptionsT,
    ) -> TtIdT {
        self.deserialize.unwrap()(tt, buffer, opt)
    }

    pub unsafe fn buffer_hashes(
        &self,
        buffer: *mut *const ::std::os::raw::c_char,
        count: *mut u64,
    ) -> *const u64 {
        self.buffer_hashes.unwrap()(buffer, count)
    }

    pub unsafe fn deserialize_from_file(
        &self,
        tt: *mut TheTruthO,
        file: *const ::std::os::raw::c_char,
    ) -> TtIdT {
        self.deserialize_from_file.unwrap()(tt, file)
    }

    pub unsafe fn migration_ids(&self, tt: *const TheTruthO, n: *mut u32) -> *mut StrhashT {
        self.migration_ids.unwrap()(tt, n)
    }

    pub unsafe fn serialize_changes_header(
        &self,
        tt: *mut TheTruthO,
        carray: *mut *mut ::std::os::raw::c_char,
        a: *mut AllocatorI,
    ) {
        self.serialize_changes_header.unwrap()(tt, carray, a)
    }

    pub unsafe fn serialize_changes(
        &self,
        tt: *mut TheTruthO,
        begin: u64,
        end: u64,
        carray: *mut *mut ::std::os::raw::c_char,
        a: *mut AllocatorI,
        opt: *const TtSerializeChangesOptionsT,
    ) {
        self.serialize_changes.unwrap()(tt, begin, end, carray, a, opt)
    }

    pub unsafe fn deserialize_changes(
        &self,
        tt: *mut TheTruthO,
        buffer: *mut *const ::std::os::raw::c_char,
        opt: *const TtDeserializeChangesOptionsT,
    ) {
        self.deserialize_changes.unwrap()(tt, buffer, opt)
    }

    pub unsafe fn serialize_patch(
        &self,
        from_tt: *mut TheTruthO,
        from_o: TtIdT,
        to_tt: *mut TheTruthO,
        to_o: TtIdT,
        carray: *mut *mut ::std::os::raw::c_char,
        a: *mut AllocatorI,
    ) {
        self.serialize_patch.unwrap()(from_tt, from_o, to_tt, to_o, carray, a)
    }

    pub unsafe fn deserialize_patch(
        &self,
        tt: *mut TheTruthO,
        buffer: *mut *const ::std::os::raw::c_char,
    ) {
        self.deserialize_patch.unwrap()(tt, buffer)
    }

    pub unsafe fn deserialize_patch_from_file(
        &self,
        tt: *mut TheTruthO,
        file: *const ::std::os::raw::c_char,
    ) {
        self.deserialize_patch_from_file.unwrap()(tt, file)
    }

    pub unsafe fn serialize_type(
        &self,
        tt: *mut TheTruthO,
        type_: u32,
        ta: *mut TempAllocatorI,
    ) -> *mut ::std::os::raw::c_char {
        self.serialize_type.unwrap()(tt, type_, ta)
    }

    pub unsafe fn deserialize_type(
        &self,
        tt: *mut TheTruthO,
        buf: *mut *const ::std::os::raw::c_char,
        type_info: *mut TheTruthSerializedTypeInfoT,
    ) {
        self.deserialize_type.unwrap()(tt, buf, type_info)
    }

    pub unsafe fn memory_use(
        &self,
        tt: *mut TheTruthO,
        id: TtIdT,
        buffers: *mut SetT,
    ) -> TtMemoryUseT {
        self.memory_use.unwrap()(tt, id, buffers)
    }

    pub unsafe fn add_properties(
        &self,
        tt: *mut TheTruthO,
        type_: TtTypeT,
        properties: *const TheTruthPropertyDefinitionT,
        num_properties: u32,
    ) {
        self.add_properties.unwrap()(tt, type_, properties, num_properties)
    }

    pub unsafe fn resolve_or_create_placeholder(
        &self,
        tt: *mut TheTruthO,
        uuid: UuidT,
        type_: TtTypeT,
        default_initialize: bool,
    ) -> TtIdT {
        self.resolve_or_create_placeholder.unwrap()(tt, uuid, type_, default_initialize)
    }

    pub unsafe fn resolve_or_fail(&self, tt: *mut TheTruthO, uuid: UuidT, type_: TtTypeT) -> TtIdT {
        self.resolve_or_fail.unwrap()(tt, uuid, type_)
    }

    pub unsafe fn set_uuid(&self, tt: *mut TheTruthO, id: TtIdT, uuid: UuidT) {
        self.set_uuid.unwrap()(tt, id, uuid)
    }

    pub unsafe fn set_prototype(&self, tt: *mut TheTruthO, obj: *mut TheTruthObjectO, id: TtIdT) {
        self.set_prototype.unwrap()(tt, obj, id)
    }

    pub unsafe fn detach_from_prototype(
        &self,
        tt: *mut TheTruthO,
        id: TtIdT,
        lookup: *mut HashIdToIdT,
        undo_scope: TtUndoScopeT,
    ) {
        self.detach_from_prototype.unwrap()(tt, id, lookup, undo_scope)
    }

    pub unsafe fn detach_all_instances(
        &self,
        tt: *mut TheTruthO,
        id: TtIdT,
        undo_scope: TtUndoScopeT,
    ) {
        self.detach_all_instances.unwrap()(tt, id, undo_scope)
    }

    pub unsafe fn get_local_reference_set(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> TheTruthLocalSetT {
        self.get_local_reference_set.unwrap()(tt, obj, property)
    }

    pub unsafe fn get_local_subobject_set(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        property: u32,
    ) -> TheTruthLocalSetT {
        self.get_local_subobject_set.unwrap()(tt, obj, property)
    }

    pub unsafe fn set_local_reference_set(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        set: TheTruthLocalSetT,
    ) {
        self.set_local_reference_set.unwrap()(tt, obj, property, set)
    }

    pub unsafe fn set_local_subobject_set(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        property: u32,
        set: TheTruthSetLocalSubobjectSetT,
    ) {
        self.set_local_subobject_set.unwrap()(tt, obj, property, set)
    }

    pub unsafe fn string_repository(&self, tt: *mut TheTruthO) -> *mut StringRepositoryI {
        self.string_repository.unwrap()(tt)
    }

    pub unsafe fn set_migration_ids(&self, tt: *mut TheTruthO, ids: *mut StrhashT, n: u32) {
        self.set_migration_ids.unwrap()(tt, ids, n)
    }

    pub unsafe fn set_properties_to_default(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        mask: u64,
    ) {
        self.set_properties_to_default.unwrap()(tt, obj, mask)
    }

    pub unsafe fn instantiate_subobjects_recursively(
        &self,
        tt: *mut TheTruthO,
        object: TtIdT,
        undo_scope: TtUndoScopeT,
    ) {
        self.instantiate_subobjects_recursively.unwrap()(tt, object, undo_scope)
    }

    pub unsafe fn quick_set_properties(
        &self,
        tt: *mut TheTruthO,
        undo_scope: TtUndoScopeT,
        id: TtIdT,
    ) {
        self.quick_set_properties.unwrap()(tt, undo_scope, id)
    }

    pub unsafe fn quick_create_object(
        &self,
        tt: *mut TheTruthO,
        undo_scope: TtUndoScopeT,
        type_hash: StrhashT,
    ) -> TtIdT {
        self.quick_create_object.unwrap()(tt, undo_scope, type_hash)
    }

    pub unsafe fn quick_get_property(
        &self,
        tt: *const TheTruthO,
        id: TtIdT,
        prop_1: u32,
    ) -> TtPropValueT {
        self.quick_get_property.unwrap()(tt, id, prop_1)
    }

    pub unsafe fn copy_properties_by_name(
        &self,
        t: *mut TheTruthO,
        to: TtIdT,
        from: TtIdT,
        undo_scope: TtUndoScopeT,
    ) {
        self.copy_properties_by_name.unwrap()(t, to, from, undo_scope)
    }

    pub unsafe fn debug_inspect(
        &self,
        tt: *const TheTruthO,
        object: TtIdT,
    ) -> *const ::std::os::raw::c_char {
        self.debug_inspect.unwrap()(tt, object)
    }

    pub unsafe fn create(&self, a: *mut AllocatorI, types: TheTruthCreateTypes) -> *mut TheTruthO {
        self.create.unwrap()(a, types)
    }

    pub unsafe fn destroy(&self, tt: *mut TheTruthO) {
        self.destroy.unwrap()(tt)
    }
}

impl crate::Api for TheTruthApi {
    const NAME: ConstCStr = const_cstr!("tm_the_truth_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl TheTruthAssetsApi {
    pub unsafe fn get_asset_path(
        &self,
        tt: *const TheTruthO,
        asset: TtIdT,
        path: *mut ::std::os::raw::c_char,
        n: u32,
    ) -> u32 {
        self.get_asset_path.unwrap()(tt, asset, path, n)
    }

    pub unsafe fn get_directory_path(
        &self,
        tt: *const TheTruthO,
        directory: TtIdT,
        path: *mut ::std::os::raw::c_char,
        n: u32,
    ) -> u32 {
        self.get_directory_path.unwrap()(tt, directory, path, n)
    }

    pub unsafe fn get_asset_path_with_extension(
        &self,
        tt: *const TheTruthO,
        asset: TtIdT,
        path: *mut ::std::os::raw::c_char,
        n: u32,
    ) -> u32 {
        self.get_asset_path_with_extension.unwrap()(tt, asset, path, n)
    }

    pub unsafe fn asset_from_path(
        &self,
        tt: *const TheTruthO,
        asset_root: TtIdT,
        path: *const ::std::os::raw::c_char,
    ) -> TtIdT {
        self.asset_from_path.unwrap()(tt, asset_root, path)
    }

    pub unsafe fn asset_from_path_with_type(
        &self,
        tt: *const TheTruthO,
        asset_root: TtIdT,
        path: *const ::std::os::raw::c_char,
        type_: TtTypeT,
    ) -> TtIdT {
        self.asset_from_path_with_type.unwrap()(tt, asset_root, path, type_)
    }

    pub unsafe fn asset_object_from_path(
        &self,
        tt: *const TheTruthO,
        asset_root: TtIdT,
        path: *const ::std::os::raw::c_char,
    ) -> TtIdT {
        self.asset_object_from_path.unwrap()(tt, asset_root, path)
    }

    pub unsafe fn asset_object_from_path_with_type(
        &self,
        tt: *const TheTruthO,
        asset_root: TtIdT,
        path: *const ::std::os::raw::c_char,
        type_: TtTypeT,
    ) -> TtIdT {
        self.asset_object_from_path_with_type.unwrap()(tt, asset_root, path, type_)
    }

    pub unsafe fn directory_from_path(
        &self,
        tt: *const TheTruthO,
        asset_root: TtIdT,
        path: *const ::std::os::raw::c_char,
    ) -> TtIdT {
        self.directory_from_path.unwrap()(tt, asset_root, path)
    }

    pub unsafe fn find_subdirectory_by_name(
        &self,
        tt: *const TheTruthO,
        asset_root: TtIdT,
        parent_dir: TtIdT,
        subdir_name: *const ::std::os::raw::c_char,
    ) -> TtIdT {
        self.find_subdirectory_by_name.unwrap()(tt, asset_root, parent_dir, subdir_name)
    }

    pub unsafe fn unique_asset_name(
        &self,
        tt: *mut TheTruthO,
        asset_root: TtIdT,
        asset_r: *const TheTruthObjectO,
        desired_name: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char {
        self.unique_asset_name.unwrap()(tt, asset_root, asset_r, desired_name)
    }

    pub unsafe fn unique_directory_name(
        &self,
        tt: *mut TheTruthO,
        asset_root: TtIdT,
        directory_r: *const TheTruthObjectO,
        desired_name: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char {
        self.unique_directory_name.unwrap()(tt, asset_root, directory_r, desired_name)
    }

    pub unsafe fn object_asset_name(
        &self,
        tt: *mut TheTruthO,
        object: TtIdT,
    ) -> *const ::std::os::raw::c_char {
        self.object_asset_name.unwrap()(tt, object)
    }

    pub unsafe fn find_directory_settings(
        &self,
        tt: *mut TheTruthO,
        relative_to: TtIdT,
        target_type_hash: StrhashT,
        asset_root: TtIdT,
    ) -> TtIdT {
        self.find_directory_settings.unwrap()(tt, relative_to, target_type_hash, asset_root)
    }

    pub unsafe fn object_to_config(
        &self,
        tt: *const TheTruthO,
        object: TtIdT,
        config: *mut ConfigI,
        buffers: *mut *mut TtAssetsBufferWriteT,
        buffers_ta: *mut TempAllocatorI,
        save_uuid: bool,
    ) {
        self.object_to_config.unwrap()(tt, object, config, buffers, buffers_ta, save_uuid)
    }

    pub unsafe fn create_object_from_config(
        &self,
        tt: *mut TheTruthO,
        config: *mut ConfigI,
        buffers: *mut *mut TtAssetsBufferT,
        buffers_ta: *mut TempAllocatorI,
    ) -> TtIdT {
        self.create_object_from_config.unwrap()(tt, config, buffers, buffers_ta)
    }

    pub unsafe fn read_object_from_config(
        &self,
        tt: *mut TheTruthO,
        config: *mut ConfigI,
        buffers: *mut *mut TtAssetsBufferT,
        buffers_ta: *mut TempAllocatorI,
        id: TtIdT,
    ) {
        self.read_object_from_config.unwrap()(tt, config, buffers, buffers_ta, id)
    }

    pub unsafe fn save_to_directory(
        &self,
        tt: *mut TheTruthO,
        asset_root: TtIdT,
        dir: *const ::std::os::raw::c_char,
        ignore: *mut TtIdT,
        num_ignore: u32,
        old_std: *mut SavedTruthDataO,
        allocator: *mut AllocatorI,
    ) -> *mut SavedTruthDataO {
        self.save_to_directory.unwrap()(tt, asset_root, dir, ignore, num_ignore, old_std, allocator)
    }

    pub unsafe fn load_from_directory(
        &self,
        tt: *mut TheTruthO,
        dir: *const ::std::os::raw::c_char,
        allocator: *mut AllocatorI,
        asset_root: *mut TtIdT,
        error: *mut ErrorI,
    ) -> *mut SavedTruthDataO {
        self.load_from_directory.unwrap()(tt, dir, allocator, asset_root, error)
    }

    pub unsafe fn current_truth_data(
        &self,
        tt: *mut TheTruthO,
        asset_root: TtIdT,
        allocator: *mut AllocatorI,
    ) -> *mut SavedTruthDataO {
        self.current_truth_data.unwrap()(tt, asset_root, allocator)
    }

    pub unsafe fn revert_asset(
        &self,
        sd: *mut SavedTruthDataO,
        asset: TtIdT,
        undo_scope: TtUndoScopeT,
    ) -> bool {
        self.revert_asset.unwrap()(sd, asset, undo_scope)
    }

    pub unsafe fn save_asset(&self, sd: *mut SavedTruthDataO, asset: TtIdT) -> bool {
        self.save_asset.unwrap()(sd, asset)
    }

    pub unsafe fn saved_name(
        &self,
        sd: *mut SavedTruthDataO,
        item: TtIdT,
    ) -> *const ::std::os::raw::c_char {
        self.saved_name.unwrap()(sd, item)
    }

    pub unsafe fn saved_directory(&self, sd: *mut SavedTruthDataO, item: TtIdT) -> TtIdT {
        self.saved_directory.unwrap()(sd, item)
    }

    pub unsafe fn saved_version(&self, sd: *mut SavedTruthDataO, item: TtIdT) -> u64 {
        self.saved_version.unwrap()(sd, item)
    }

    pub unsafe fn all_saved_items(
        &self,
        sd: *mut SavedTruthDataO,
        ta: *mut TempAllocatorI,
    ) -> *mut TtIdT {
        self.all_saved_items.unwrap()(sd, ta)
    }

    pub unsafe fn free_saved_data(&self, sd: *mut SavedTruthDataO) {
        self.free_saved_data.unwrap()(sd)
    }

    pub unsafe fn set_mock_file_system(&self, fs: *mut OsFileSystemApi, file_io: *mut OsFileIoApi) {
        self.set_mock_file_system.unwrap()(fs, file_io)
    }

    pub unsafe fn any_disk_changes(
        &self,
        sd: *mut SavedTruthDataO,
        dir: *const ::std::os::raw::c_char,
    ) -> bool {
        self.any_disk_changes.unwrap()(sd, dir)
    }
}

impl crate::Api for TheTruthAssetsApi {
    const NAME: ConstCStr = const_cstr!("tm_the_truth_assets_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl TheTruthMigrationApi {
    pub unsafe fn migrate(&self, tt: *mut TheTruthO) -> bool {
        self.migrate.unwrap()(tt)
    }
}

impl crate::Api for TheTruthMigrationApi {
    const NAME: ConstCStr = const_cstr!("tm_the_truth_migration_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl TheTruthCommonTypesApi {
    pub unsafe fn create_common_types(&self, tt: *mut TheTruthO) {
        self.create_common_types.unwrap()(tt)
    }

    pub unsafe fn get_vec2(
        &self,
        tt: *const TheTruthO,
        object: *const TheTruthObjectO,
        property: u32,
    ) -> Vec2T {
        self.get_vec2.unwrap()(tt, object, property)
    }

    pub unsafe fn get_vec3(
        &self,
        tt: *const TheTruthO,
        object: *const TheTruthObjectO,
        property: u32,
    ) -> Vec3T {
        self.get_vec3.unwrap()(tt, object, property)
    }

    pub unsafe fn get_vec4(
        &self,
        tt: *const TheTruthO,
        object: *const TheTruthObjectO,
        property: u32,
    ) -> Vec4T {
        self.get_vec4.unwrap()(tt, object, property)
    }

    pub unsafe fn get_rect(
        &self,
        tt: *const TheTruthO,
        object: *const TheTruthObjectO,
        property: u32,
    ) -> RectT {
        self.get_rect.unwrap()(tt, object, property)
    }

    pub unsafe fn get_position(
        &self,
        tt: *const TheTruthO,
        object: *const TheTruthObjectO,
        property: u32,
    ) -> Vec3T {
        self.get_position.unwrap()(tt, object, property)
    }

    pub unsafe fn get_rotation(
        &self,
        tt: *const TheTruthO,
        object: *const TheTruthObjectO,
        property: u32,
    ) -> Vec4T {
        self.get_rotation.unwrap()(tt, object, property)
    }

    pub unsafe fn get_scale(
        &self,
        tt: *const TheTruthO,
        object: *const TheTruthObjectO,
        property: u32,
    ) -> Vec3T {
        self.get_scale.unwrap()(tt, object, property)
    }

    pub unsafe fn get_color_rgb(
        &self,
        tt: *const TheTruthO,
        object: *const TheTruthObjectO,
        property: u32,
    ) -> Vec3T {
        self.get_color_rgb.unwrap()(tt, object, property)
    }

    pub unsafe fn get_color_rgba(
        &self,
        tt: *const TheTruthO,
        object: *const TheTruthObjectO,
        property: u32,
    ) -> Vec4T {
        self.get_color_rgba.unwrap()(tt, object, property)
    }

    pub unsafe fn get_color_srgb(
        &self,
        tt: *const TheTruthO,
        object: *const TheTruthObjectO,
        property: u32,
    ) -> ColorSrgbT {
        self.get_color_srgb.unwrap()(tt, object, property)
    }

    pub unsafe fn get_color_srgba(
        &self,
        tt: *const TheTruthO,
        object: *const TheTruthObjectO,
        property: u32,
    ) -> ColorSrgbT {
        self.get_color_srgba.unwrap()(tt, object, property)
    }

    pub unsafe fn get_uuid(
        &self,
        tt: *const TheTruthO,
        object: *const TheTruthObjectO,
        property: u32,
    ) -> UuidT {
        self.get_uuid.unwrap()(tt, object, property)
    }

    pub unsafe fn set_vec2(
        &self,
        tt: *mut TheTruthO,
        object: *mut TheTruthObjectO,
        property: u32,
        vec2: Vec2T,
        undo_scope: TtUndoScopeT,
    ) {
        self.set_vec2.unwrap()(tt, object, property, vec2, undo_scope)
    }

    pub unsafe fn set_vec3(
        &self,
        tt: *mut TheTruthO,
        object: *mut TheTruthObjectO,
        property: u32,
        vec3: Vec3T,
        undo_scope: TtUndoScopeT,
    ) {
        self.set_vec3.unwrap()(tt, object, property, vec3, undo_scope)
    }

    pub unsafe fn set_vec4(
        &self,
        tt: *mut TheTruthO,
        object: *mut TheTruthObjectO,
        property: u32,
        vec4: Vec4T,
        undo_scope: TtUndoScopeT,
    ) {
        self.set_vec4.unwrap()(tt, object, property, vec4, undo_scope)
    }

    pub unsafe fn set_rect(
        &self,
        tt: *mut TheTruthO,
        object: *mut TheTruthObjectO,
        property: u32,
        rect: RectT,
        undo_scope: TtUndoScopeT,
    ) {
        self.set_rect.unwrap()(tt, object, property, rect, undo_scope)
    }

    pub unsafe fn set_position(
        &self,
        tt: *mut TheTruthO,
        object: *mut TheTruthObjectO,
        property: u32,
        vec3: Vec3T,
        undo_scope: TtUndoScopeT,
    ) {
        self.set_position.unwrap()(tt, object, property, vec3, undo_scope)
    }

    pub unsafe fn set_rotation(
        &self,
        tt: *mut TheTruthO,
        object: *mut TheTruthObjectO,
        property: u32,
        vec4: Vec4T,
        undo_scope: TtUndoScopeT,
    ) {
        self.set_rotation.unwrap()(tt, object, property, vec4, undo_scope)
    }

    pub unsafe fn set_scale(
        &self,
        tt: *mut TheTruthO,
        object: *mut TheTruthObjectO,
        property: u32,
        vec3: Vec3T,
        undo_scope: TtUndoScopeT,
    ) {
        self.set_scale.unwrap()(tt, object, property, vec3, undo_scope)
    }

    pub unsafe fn set_color_rgb(
        &self,
        tt: *mut TheTruthO,
        object: *mut TheTruthObjectO,
        property: u32,
        vec3: Vec3T,
        undo_scope: TtUndoScopeT,
    ) {
        self.set_color_rgb.unwrap()(tt, object, property, vec3, undo_scope)
    }

    pub unsafe fn set_color_rgba(
        &self,
        tt: *mut TheTruthO,
        object: *mut TheTruthObjectO,
        property: u32,
        vec4: Vec4T,
        undo_scope: TtUndoScopeT,
    ) {
        self.set_color_rgba.unwrap()(tt, object, property, vec4, undo_scope)
    }

    pub unsafe fn set_color_srgb(
        &self,
        tt: *mut TheTruthO,
        object: *mut TheTruthObjectO,
        property: u32,
        col: ColorSrgbT,
        undo_scope: TtUndoScopeT,
    ) {
        self.set_color_srgb.unwrap()(tt, object, property, col, undo_scope)
    }

    pub unsafe fn set_color_srgba(
        &self,
        tt: *mut TheTruthO,
        object: *mut TheTruthObjectO,
        property: u32,
        col: ColorSrgbT,
        undo_scope: TtUndoScopeT,
    ) {
        self.set_color_srgba.unwrap()(tt, object, property, col, undo_scope)
    }

    pub unsafe fn set_uuid(
        &self,
        tt: *mut TheTruthO,
        object: *mut TheTruthObjectO,
        property: u32,
        uuid: UuidT,
        undo_scope: TtUndoScopeT,
    ) {
        self.set_uuid.unwrap()(tt, object, property, uuid, undo_scope)
    }

    pub unsafe fn read_floats(
        &self,
        tt: *const TheTruthO,
        obj: *const TheTruthObjectO,
        res: *mut f32,
        n: u32,
    ) -> *mut f32 {
        self.read_floats.unwrap()(tt, obj, res, n)
    }

    pub unsafe fn write_floats(
        &self,
        tt: *mut TheTruthO,
        obj: *mut TheTruthObjectO,
        values: *const f32,
        n: u32,
    ) {
        self.write_floats.unwrap()(tt, obj, values, n)
    }
}

impl crate::Api for TheTruthCommonTypesApi {
    const NAME: ConstCStr = const_cstr!("tm_the_truth_common_types_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl UnicodeApi {
    pub unsafe fn is_valid(&self, utf8: *const ::std::os::raw::c_char) -> bool {
        self.is_valid.unwrap()(utf8)
    }

    pub unsafe fn truncate(&self, utf8: *mut ::std::os::raw::c_char) {
        self.truncate.unwrap()(utf8)
    }

    pub unsafe fn utf8_encode(
        &self,
        utf8: *mut ::std::os::raw::c_char,
        codepoint: u32,
    ) -> *mut ::std::os::raw::c_char {
        self.utf8_encode.unwrap()(utf8, codepoint)
    }

    pub unsafe fn utf8_decode(&self, utf8: *mut *const ::std::os::raw::c_char) -> u32 {
        self.utf8_decode.unwrap()(utf8)
    }

    pub unsafe fn utf8_num_codepoints(&self, utf8: *const ::std::os::raw::c_char) -> u32 {
        self.utf8_num_codepoints.unwrap()(utf8)
    }

    pub unsafe fn utf8_decode_n(
        &self,
        codepoints: *mut u32,
        n: u32,
        utf8: *const ::std::os::raw::c_char,
    ) -> u32 {
        self.utf8_decode_n.unwrap()(codepoints, n, utf8)
    }

    pub unsafe fn utf8_to_utf32(
        &self,
        utf8: *const ::std::os::raw::c_char,
        ta: *mut TempAllocatorI,
    ) -> *mut u32 {
        self.utf8_to_utf32.unwrap()(utf8, ta)
    }

    pub unsafe fn utf8_to_utf32_n(
        &self,
        utf8: *const ::std::os::raw::c_char,
        n: u32,
        ta: *mut TempAllocatorI,
    ) -> *mut u32 {
        self.utf8_to_utf32_n.unwrap()(utf8, n, ta)
    }

    pub unsafe fn utf32_to_utf8(
        &self,
        utf32: *const u32,
        ta: *mut TempAllocatorI,
    ) -> *mut ::std::os::raw::c_char {
        self.utf32_to_utf8.unwrap()(utf32, ta)
    }

    pub unsafe fn utf32_to_utf8_n(
        &self,
        utf32: *const u32,
        n: u32,
        ta: *mut TempAllocatorI,
    ) -> *mut ::std::os::raw::c_char {
        self.utf32_to_utf8_n.unwrap()(utf32, n, ta)
    }

    pub unsafe fn utf16_encode(&self, utf16: *mut u16, codepoint: u32) -> *mut u16 {
        self.utf16_encode.unwrap()(utf16, codepoint)
    }

    pub unsafe fn utf16_decode(&self, utf16: *mut *const u16) -> u32 {
        self.utf16_decode.unwrap()(utf16)
    }

    pub unsafe fn utf8_to_utf16(
        &self,
        utf8: *const ::std::os::raw::c_char,
        ta: *mut TempAllocatorI,
    ) -> *mut u16 {
        self.utf8_to_utf16.unwrap()(utf8, ta)
    }

    pub unsafe fn utf8_to_utf16_n(
        &self,
        utf8: *const ::std::os::raw::c_char,
        n: u32,
        ta: *mut TempAllocatorI,
    ) -> *mut u16 {
        self.utf8_to_utf16_n.unwrap()(utf8, n, ta)
    }

    pub unsafe fn utf16_to_utf8(
        &self,
        utf16: *const u16,
        ta: *mut TempAllocatorI,
    ) -> *mut ::std::os::raw::c_char {
        self.utf16_to_utf8.unwrap()(utf16, ta)
    }

    pub unsafe fn utf16_to_utf8_n(
        &self,
        utf16: *const u16,
        n: u32,
        ta: *mut TempAllocatorI,
    ) -> *mut ::std::os::raw::c_char {
        self.utf16_to_utf8_n.unwrap()(utf16, n, ta)
    }
}

impl crate::Api for UnicodeApi {
    const NAME: ConstCStr = const_cstr!("tm_unicode_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl VisibilityFlagsApi {
    pub unsafe fn create_context(
        &self,
        allocator: *mut AllocatorI,
        name: StrhashT,
    ) -> *mut VisibilityContextO {
        self.create_context.unwrap()(allocator, name)
    }

    pub unsafe fn context_name(&self, context: *mut VisibilityContextO) -> StrhashT {
        self.context_name.unwrap()(context)
    }

    pub unsafe fn destroy_context(&self, context: *mut VisibilityContextO) {
        self.destroy_context.unwrap()(context)
    }

    pub unsafe fn register_visibility_flag(
        &self,
        context: *mut VisibilityContextO,
        desc: *const VisibilityFlagDescI,
    ) -> u64 {
        self.register_visibility_flag.unwrap()(context, desc)
    }

    pub unsafe fn name_from_visibility_flag(
        &self,
        context: *mut VisibilityContextO,
        visibility_flag: u64,
    ) -> StrhashT {
        self.name_from_visibility_flag.unwrap()(context, visibility_flag)
    }

    pub unsafe fn visibility_flag_from_name(
        &self,
        context: *mut VisibilityContextO,
        name: StrhashT,
    ) -> u64 {
        self.visibility_flag_from_name.unwrap()(context, name)
    }

    pub unsafe fn unregister_visibility_flag(&self, context: *mut VisibilityContextO, uuid: u32) {
        self.unregister_visibility_flag.unwrap()(context, uuid)
    }

    pub unsafe fn enumerate_flags(
        &self,
        context: *mut VisibilityContextO,
        flags: *mut VisibilityFlagDescI,
        num_flags: *mut u32,
    ) {
        self.enumerate_flags.unwrap()(context, flags, num_flags)
    }

    pub unsafe fn build_visibility_mask(
        &self,
        context: *mut VisibilityContextO,
        uuids: *const u32,
        num_uuids: u32,
    ) -> u64 {
        self.build_visibility_mask.unwrap()(context, uuids, num_uuids)
    }
}

impl crate::Api for VisibilityFlagsApi {
    const NAME: ConstCStr = const_cstr!("tm_visibility_flags_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl WebSocketProtocolApi {
    pub unsafe fn make_client_handshake(
        &self,
        buffer: *mut ::std::os::raw::c_char,
        size: u32,
        host: *const ::std::os::raw::c_char,
        port: u32,
        request: *const ::std::os::raw::c_char,
        key: *const ::std::os::raw::c_char,
    ) {
        self.make_client_handshake.unwrap()(buffer, size, host, port, request, key)
    }

    pub unsafe fn make_server_handshake(
        &self,
        buffer: *mut ::std::os::raw::c_char,
        size: u32,
        key: *const ::std::os::raw::c_char,
        key_size: u32,
    ) {
        self.make_server_handshake.unwrap()(buffer, size, key, key_size)
    }

    pub unsafe fn make_frame_header(
        &self,
        buffer: *mut u8,
        fin: bool,
        opcode: u8,
        size: u64,
        mask: u32,
    ) -> u32 {
        self.make_frame_header.unwrap()(buffer, fin, opcode, size, mask)
    }

    pub unsafe fn parse_frame_header(
        &self,
        buffer: *mut u8,
        buf_size: u64,
        fin: *mut bool,
        opcode: *mut u8,
        size: *mut u64,
        mask: *mut u32,
    ) -> u32 {
        self.parse_frame_header.unwrap()(buffer, buf_size, fin, opcode, size, mask)
    }

    pub unsafe fn mask_data(&self, data: *mut u8, size: u64, offset: u64, mask: u32) {
        self.mask_data.unwrap()(data, size, offset, mask)
    }

    pub unsafe fn mask_segmented_buffer(&self, seg: *mut SegmentedBufferT, size: u64, mask: u32) {
        self.mask_segmented_buffer.unwrap()(seg, size, mask)
    }
}

impl crate::Api for WebSocketProtocolApi {
    const NAME: ConstCStr = const_cstr!("tm_web_socket_protocol_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl WebTalkerApi {
    pub unsafe fn create(
        &self,
        a: *mut AllocatorI,
        socket: *mut OsSocketApi,
        port: u32,
    ) -> *mut WebTalkerO {
        self.create.unwrap()(a, socket, port)
    }

    pub unsafe fn listening_address(
        &self,
        inst: *mut WebTalkerO,
        address: *mut SocketAddressT,
    ) -> bool {
        self.listening_address.unwrap()(inst, address)
    }

    pub unsafe fn create_no_server(
        &self,
        a: *mut AllocatorI,
        socket: *mut OsSocketApi,
    ) -> *mut WebTalkerO {
        self.create_no_server.unwrap()(a, socket)
    }

    pub unsafe fn destroy(&self, arg1: *mut WebTalkerO) {
        self.destroy.unwrap()(arg1)
    }

    pub unsafe fn receive(&self, inst: *mut WebTalkerO) {
        self.receive.unwrap()(inst)
    }

    pub unsafe fn send(&self, inst: *mut WebTalkerO) {
        self.send.unwrap()(inst)
    }

    pub unsafe fn http_get_requests(
        &self,
        inst: *mut WebTalkerO,
        buffer: *mut HttpRequestT,
        capacity: u32,
    ) -> u32 {
        self.http_get_requests.unwrap()(inst, buffer, capacity)
    }

    pub unsafe fn http_respond_raw(
        &self,
        inst: *mut WebTalkerO,
        id: u64,
        response: *const ::std::os::raw::c_char,
    ) {
        self.http_respond_raw.unwrap()(inst, id, response)
    }

    pub unsafe fn http_respond_html(
        &self,
        inst: *mut WebTalkerO,
        id: u64,
        html: *const ::std::os::raw::c_char,
    ) {
        self.http_respond_html.unwrap()(inst, id, html)
    }

    pub unsafe fn http_request(
        &self,
        inst: *mut WebTalkerO,
        address: SocketAddressT,
        headers: *const ::std::os::raw::c_char,
    ) -> u64 {
        self.http_request.unwrap()(inst, address, headers)
    }

    pub unsafe fn http_request_status(&self, inst: *mut WebTalkerO, id: u64) -> HttpRequestStatus {
        self.http_request_status.unwrap()(inst, id)
    }

    pub unsafe fn http_response(
        &self,
        inst: *mut WebTalkerO,
        id: u64,
        size: *mut u64,
    ) -> SegmentedBufferT {
        self.http_response.unwrap()(inst, id, size)
    }

    pub unsafe fn http_close(&self, inst: *mut WebTalkerO, id: u64) {
        self.http_close.unwrap()(inst, id)
    }

    pub unsafe fn ws_connect(
        &self,
        inst: *mut WebTalkerO,
        address: SocketAddressT,
        host: *const ::std::os::raw::c_char,
        request: *const ::std::os::raw::c_char,
    ) -> u64 {
        self.ws_connect.unwrap()(inst, address, host, request)
    }

    pub unsafe fn ws_get_requests(
        &self,
        inst: *mut WebTalkerO,
        buffer: *mut WebSocketRequestT,
        capacity: u32,
    ) -> u32 {
        self.ws_get_requests.unwrap()(inst, buffer, capacity)
    }

    pub unsafe fn ws_status(&self, inst: *mut WebTalkerO, id: u64) -> WebSocketStatus {
        self.ws_status.unwrap()(inst, id)
    }

    pub unsafe fn ws_recv_progress(
        &self,
        inst: *mut WebTalkerO,
        id: u64,
        bytes: *mut u64,
        total: *mut u64,
    ) -> bool {
        self.ws_recv_progress.unwrap()(inst, id, bytes, total)
    }

    pub unsafe fn ws_get_events(
        &self,
        inst: *mut WebTalkerO,
        id: u64,
        buffer: *mut WebSocketEventT,
        capacity: u32,
    ) -> u32 {
        self.ws_get_events.unwrap()(inst, id, buffer, capacity)
    }

    pub unsafe fn ws_send_text_frame(
        &self,
        inst: *mut WebTalkerO,
        id: u64,
        s: *const ::std::os::raw::c_char,
    ) {
        self.ws_send_text_frame.unwrap()(inst, id, s)
    }

    pub unsafe fn ws_send_binary_frame(
        &self,
        inst: *mut WebTalkerO,
        id: u64,
        data: *const u8,
        size: u64,
    ) {
        self.ws_send_binary_frame.unwrap()(inst, id, data, size)
    }

    pub unsafe fn ws_send_segmented_binary_frame(
        &self,
        inst: *mut WebTalkerO,
        id: u64,
        buf: *const SegmentedBufferT,
        size: u64,
    ) {
        self.ws_send_segmented_binary_frame.unwrap()(inst, id, buf, size)
    }

    pub unsafe fn ws_send_ping(&self, inst: *mut WebTalkerO, id: u64) {
        self.ws_send_ping.unwrap()(inst, id)
    }

    pub unsafe fn ws_close(&self, inst: *mut WebTalkerO, id: u64) {
        self.ws_close.unwrap()(inst, id)
    }

    pub unsafe fn copy_segmented_buffer(
        &self,
        buffer: *mut ::std::os::raw::c_void,
        segments: *const SegmentedBufferT,
        size: u64,
    ) {
        self.copy_segmented_buffer.unwrap()(buffer, segments, size)
    }
}

impl crate::Api for WebTalkerApi {
    const NAME: ConstCStr = const_cstr!("tm_web_talker_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

pub const TM_TYPE_HASH__BOOL: StrhashT = StrhashT {
    u64_: 16540862642162215401u64,
};
pub const TM_TYPE_HASH__UINT8_T: StrhashT = StrhashT {
    u64_: 8559580523335163012u64,
};
pub const TM_TYPE_HASH__UINT16_T: StrhashT = StrhashT {
    u64_: 15324894991619482704u64,
};
pub const TM_TYPE_HASH__UINT32_T: StrhashT = StrhashT {
    u64_: 7376399353478788036u64,
};
pub const TM_TYPE_HASH__UINT64_T: StrhashT = StrhashT {
    u64_: 18144472437779314990u64,
};
pub const TM_TYPE_HASH__FLOAT: StrhashT = StrhashT {
    u64_: 4361463899232155093u64,
};
pub const TM_TYPE_HASH__DOUBLE: StrhashT = StrhashT {
    u64_: 2716045784919460316u64,
};
pub const TM_TYPE_HASH__TM_VEC2_T: StrhashT = StrhashT {
    u64_: 6818937449469173318u64,
};
pub const TM_TYPE_HASH__TM_VEC3_T: StrhashT = StrhashT {
    u64_: 10165899445488706529u64,
};
pub const TM_TYPE_HASH__TM_VEC4_T: StrhashT = StrhashT {
    u64_: 16105198182785596086u64,
};
pub const TM_TYPE_HASH__TM_MAT44_T: StrhashT = StrhashT {
    u64_: 6274398046370934111u64,
};
pub const TM_COLLABORATION_SESSION_ARCHITECTURE__P2P: StrhashT = StrhashT {
    u64_: 3712735182291565690u64,
};
pub const TM_FEATURE_FLAG__INTERNAL_DEVELOPER_TOOLS: StrhashT = StrhashT {
    u64_: 11549283883340918408u64,
};
pub const TM_FEATURE_FLAG__UI_USES_VECTOR_FONT: StrhashT = StrhashT {
    u64_: 7354548536171183257u64,
};
pub const TM_FEATURE_FLAG__SCENE_COMMANDS: StrhashT = StrhashT {
    u64_: 4872997832008015268u64,
};
pub const TM_FEATURE_FLAG__VOXEL_ASSETS: StrhashT = StrhashT {
    u64_: 14684113058603897788u64,
};
pub const TM_FEATURE_FLAG__WATCH_WIRES: StrhashT = StrhashT {
    u64_: 6084627402685503047u64,
};
pub const TM_FEATURE_FLAG__BREAKPOINTS: StrhashT = StrhashT {
    u64_: 12171838945250017565u64,
};
pub const TM_FEATURE_FLAG__GLOBAL_ILLUMINATION: StrhashT = StrhashT {
    u64_: 14054475124752319158u64,
};
pub const TM_FEATURE_FLAG__NETWORKING: StrhashT = StrhashT {
    u64_: 14342885002789868064u64,
};
pub const TM_INTEGRATION_TEST_CONTEXT__THE_MACHINERY_EDITOR: StrhashT = StrhashT {
    u64_: 6564203205322320365u64,
};
pub const TM_LANGUAGE_ENGLISH: StrhashT = StrhashT {
    u64_: 7710216835639188562u64,
};
pub const TM_LANGUAGE_DUTCH: StrhashT = StrhashT {
    u64_: 1250082070672125555u64,
};
pub const TM_LANGUAGE_FRENCH: StrhashT = StrhashT {
    u64_: 18347935498605111012u64,
};
pub const TM_LANGUAGE_GERMAN: StrhashT = StrhashT {
    u64_: 13418971983228644420u64,
};
pub const TM_LANGUAGE_ITALIAN: StrhashT = StrhashT {
    u64_: 16355695940806680529u64,
};
pub const TM_LANGUAGE_PORTUGUESE: StrhashT = StrhashT {
    u64_: 5344833540692673681u64,
};
pub const TM_LANGUAGE_SPANISH: StrhashT = StrhashT {
    u64_: 3566965541708517290u64,
};
pub const TM_LANGUAGE_SWEDISH: StrhashT = StrhashT {
    u64_: 17821763756083510268u64,
};
pub const TM_LANGUAGE_TAGALOG: StrhashT = StrhashT {
    u64_: 15699795126974246192u64,
};
pub const TM_PSEUDO_LANGUAGE_GIBBERISH: StrhashT = StrhashT {
    u64_: 8182454343404938431u64,
};
pub const TM_PSEUDO_LANGUAGE_CONTEXT: StrhashT = StrhashT {
    u64_: 10587708927429250677u64,
};
pub const TM_TT_TYPE_HASH__PLUGIN: StrhashT = StrhashT {
    u64_: 14899577057384826377u64,
};
pub const TM_TT_TYPE_HASH__ANYTHING: StrhashT = StrhashT {
    u64_: 9891277735898990422u64,
};
pub const TM_TT_TYPE_HASH__ASSET_ROOT: StrhashT = StrhashT {
    u64_: 12427581353952698963u64,
};
pub const TM_TT_TYPE_HASH__ASSET: StrhashT = StrhashT {
    u64_: 14587460986374400352u64,
};
pub const TM_TT_TYPE_HASH__ASSET_DIRECTORY: StrhashT = StrhashT {
    u64_: 10890794697157403146u64,
};
pub const TM_TT_TYPE_HASH__ASSET_LABEL: StrhashT = StrhashT {
    u64_: 6843264115021824839u64,
};
pub const TM_TT_ASPECT__FILE_EXTENSION: StrhashT = StrhashT {
    u64_: 9567164115475830323u64,
};
pub const TM_TT_TYPE_HASH__ASSET_THUMBNAIL: StrhashT = StrhashT {
    u64_: 2983294151888264002u64,
};
pub const TM_TT_TYPE_HASH__BOOL: StrhashT = StrhashT {
    u64_: 12597635396010430865u64,
};
pub const TM_TT_TYPE_HASH__UINT32_T: StrhashT = StrhashT {
    u64_: 16929917001317266792u64,
};
pub const TM_TT_TYPE_HASH__UINT64_T: StrhashT = StrhashT {
    u64_: 12761691598820893534u64,
};
pub const TM_TT_TYPE_HASH__FLOAT: StrhashT = StrhashT {
    u64_: 7857178761304035899u64,
};
pub const TM_TT_TYPE_HASH__DOUBLE: StrhashT = StrhashT {
    u64_: 1077166915827059684u64,
};
pub const TM_TT_TYPE_HASH__STRING: StrhashT = StrhashT {
    u64_: 12126753421808361675u64,
};
pub const TM_TT_TYPE_HASH__VEC2: StrhashT = StrhashT {
    u64_: 6818937449469173318u64,
};
pub const TM_TT_TYPE_HASH__VEC3: StrhashT = StrhashT {
    u64_: 10165899445488706529u64,
};
pub const TM_TT_TYPE_HASH__VEC4: StrhashT = StrhashT {
    u64_: 16105198182785596086u64,
};
pub const TM_TT_TYPE_HASH__POSITION: StrhashT = StrhashT {
    u64_: 8802770316356633324u64,
};
pub const TM_TT_TYPE_HASH__ROTATION: StrhashT = StrhashT {
    u64_: 11876823908831844119u64,
};
pub const TM_TT_TYPE_HASH__SCALE: StrhashT = StrhashT {
    u64_: 2373204840714521648u64,
};
pub const TM_TT_TYPE_HASH__COLOR_RGB: StrhashT = StrhashT {
    u64_: 15044398031543779569u64,
};
pub const TM_TT_TYPE_HASH__COLOR_RGBA: StrhashT = StrhashT {
    u64_: 1289506825880340452u64,
};
pub const TM_TT_TYPE_HASH__RECT: StrhashT = StrhashT {
    u64_: 12027361935592340977u64,
};
pub const TM_TT_TYPE_HASH__UUID: StrhashT = StrhashT {
    u64_: 7193459200112519797u64,
};
pub const TM_TT_TYPE_HASH__VISIBILITY_FLAG: StrhashT = StrhashT {
    u64_: 5713385340038884041u64,
};
pub const TM_WEB_SOCKET_PROTOCOL_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_JOB_SYSTEM_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_CORE_IMPORTER_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_BUDDY_ALLOCATOR_RAW_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_TEMP_ALLOCATOR_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_CONFIG_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_THE_TRUTH_CREATE_TYPES_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_ERROR_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_APPLICATION_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 1u32,
    patch: 0u32,
};
pub const TM_MATH_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_PLUGIN_TICK_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_BUDDY_ALLOCATOR_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_VISIBILITY_FLAG_EDITOR_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_GIT_IGNORE_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_LOCALIZER_STRINGS_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_LOGGER_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_PATH_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_OS_VIRTUAL_MEMORY_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_CAMERA_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_SPRINTF_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_VISIBILITY_CONTEXT_O_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_CORE_ASSET_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_ASSET_DATABASE_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_TASK_SYSTEM_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_MEMORY_TRACKER_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_THE_TRUTH_DESTROYED_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_INTEGRATION_TEST_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_THE_TRUTH_COMMON_TYPES_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_STRING_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_JSON_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_CRASH_RECOVERY_API_VERSION: VersionT = VersionT {
    major: 2u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_LZ4_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_BASE_64_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_IMAGE_LOADER_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_OS_SOCKET_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_STRING_REPOSITORY_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_PLUGIN_SHUTDOWN_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_THE_TRUTH_ASSETS_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_RANDOM_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_THE_TRUTH_MIGRATION_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_UNIT_TEST_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_ASSET_LABEL_T_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_PROFILER_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_LOCALIZER_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_VISIBILITY_FLAGS_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_PLUGIN_RELOAD_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_BUFFER_FORMAT_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_ALLOCATOR_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_ASSET_IO_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_PROGRESS_REPORT_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_FEATURE_FLAGS_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_BUFFERS_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_OS_FILE_IO_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_UNICODE_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_PLUGIN_SET_THE_TRUTH_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_API_REGISTRY_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_PLUGINS_API_VERSION: VersionT = VersionT {
    major: 3u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_COLLABORATION_P2P_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_INPUT_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_IMAGE_LOADER_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_RUNTIME_DATA_REPOSITORY_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_WEB_TALKER_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_THE_TRUTH_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_PLUGIN_ASSETS_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_CORE_API_VERSION: VersionT = VersionT {
    major: 2u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_OS_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_PLUGIN_INIT_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_THE_TRUTH_MIGRATION_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_OS_FILE_SYSTEM_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_COLLABORATION_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
