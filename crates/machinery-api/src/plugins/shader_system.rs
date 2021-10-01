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
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
pub const __SAL_H_VERSION: u32 = 180000000;
pub const __bool_true_false_are_defined: u32 = 1;
pub const TM_TT_TYPE__SHADER_SYSTEM_REPOSITORY: &'static [u8; 28usize] =
    b"tm_shader_system_repository\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_STAGE_DECLARATION: &'static [u8; 35usize] =
    b"tm_shader_system_stage_declaration\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_DECLARATION: &'static [u8; 29usize] =
    b"tm_shader_system_declaration\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_RENDER_STATE_BLOCK: &'static [u8; 36usize] =
    b"tm_shader_system_render_state_block\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_SAMPLER_STATE_BLOCK: &'static [u8; 37usize] =
    b"tm_shader_system_sampler_state_block\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_REQUEST: &'static [u8; 25usize] = b"tm_shader_system_request\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_CONSTANT: &'static [u8; 26usize] =
    b"tm_shader_system_constant\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_RESOURCE: &'static [u8; 26usize] =
    b"tm_shader_system_resource\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_PAYLOAD: &'static [u8; 25usize] = b"tm_shader_system_payload\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_EXPORT: &'static [u8; 30usize] =
    b"tm_shader_system_stage_export\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_INCLUDE: &'static [u8; 25usize] = b"tm_shader_system_include\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_VARIATION: &'static [u8; 27usize] =
    b"tm_shader_system_variation\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_VARIATION_SYSTEM: &'static [u8; 34usize] =
    b"tm_shader_system_variation_system\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_COMPILE_CONFIGURATION: &'static [u8; 39usize] =
    b"tm_shader_system_compile_configuration\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_COMPILE_BRANCH: &'static [u8; 32usize] =
    b"tm_shader_system_compile_branch\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_PASS_CONDITION: &'static [u8; 32usize] =
    b"tm_shader_system_pass_condition\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_PASS: &'static [u8; 22usize] = b"tm_shader_system_pass\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_COMPILE_CONTEXT: &'static [u8; 33usize] =
    b"tm_shader_system_compile_context\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_COMPILED_RESULT: &'static [u8; 33usize] =
    b"tm_shader_system_compiled_result\0";
pub const TM_TT_TYPE__SHADER_SYSTEM__CONSTANT_NODE__SETTINGS: &'static [u8; 40usize] =
    b"tm_shader_system_constant_node_settings\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_CREATION_GRAPH_NODE_RESOURCE_ASPECT: &'static [u8; 53usize] =
    b"tm_shader_system_creation_graph_node_resource_aspect\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_CREATION_GRAPH_NODE_DEFINE: &'static [u8; 44usize] =
    b"tm_shader_system_creation_graph_node_define\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_CREATION_GRAPH_NODE_ACTION: &'static [u8; 44usize] =
    b"tm_shader_system_creation_graph_node_action\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_CREATION_GRAPH_NODE_CONNECTOR_SETTING_OPTION:
    &'static [u8; 62usize] = b"tm_shader_system_creation_graph_node_connector_setting_option\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_CREATION_GRAPH_NODE_CONNECTOR_SETTING: &'static [u8; 55usize] =
    b"tm_shader_system_creation_graph_node_connector_setting\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_CREATION_GRAPH_NODE_CONNECTOR_SETTINGS: &'static [u8; 56usize] =
    b"tm_shader_system_creation_graph_node_connector_settings\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_CREATION_GRAPH_NODE_CONNECTOR: &'static [u8; 47usize] =
    b"tm_shader_system_creation_graph_node_connector\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_CREATION_GRAPH_NODE_CONNECTOR_TYPE_OF: &'static [u8; 55usize] =
    b"tm_shader_system_creation_graph_node_connector_type_of\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_CREATION_GRAPH_NODE: &'static [u8; 37usize] =
    b"tm_shader_system_creation_graph_node\0";
pub const TM_TT_TYPE__SHADER_SYSTEM_CREATION_GRAPH_NODE_EVALUTAION_CONTEXT_NAME:
    &'static [u8; 61usize] = b"tm_shader_system_creation_graph_node_evaluation_context_name\0";
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
pub const TM_SHADER_CONSTANT_TYPE_BOOL: ShaderConstantType = 0;
pub const TM_SHADER_CONSTANT_TYPE_INT: ShaderConstantType = 1;
pub const TM_SHADER_CONSTANT_TYPE_UINT: ShaderConstantType = 2;
pub const TM_SHADER_CONSTANT_TYPE_HALF: ShaderConstantType = 3;
pub const TM_SHADER_CONSTANT_TYPE_FLOAT: ShaderConstantType = 4;
pub const TM_SHADER_CONSTANT_TYPE_DOUBLE: ShaderConstantType = 5;
pub const TM_SHADER_CONSTANT_TYPE_STRUCT: ShaderConstantType = 6;
pub const TM_SHADER_CONSTANT_TYPE_CONSERVATIVE_DEPTH_LESS_EQUAL: ShaderConstantType = 7;
pub const TM_SHADER_CONSTANT_TYPE_CONSERVATIVE_DEPTH_GREATER_EQUAL: ShaderConstantType = 8;
pub const TM_SHADER_CONSTANT_TYPE_DEPTH: ShaderConstantType = 9;
pub const TM_SHADER_CONSTANT_TYPE_STENCIL_REF: ShaderConstantType = 10;
pub const TM_SHADER_CONSTANT_TYPE_MAX_TYPES: ShaderConstantType = 11;
pub type ShaderConstantType = ::std::os::raw::c_int;
#[repr(C)]
pub struct ShaderConstantT {
    pub __bindgen_anon_1: ShaderConstantTBindgenTy1,
    pub type_: ShaderConstantType,
    pub __bindgen_anon_2: ShaderConstantTBindgenTy2,
    pub elements: u32,
    pub _padding_40: [::std::os::raw::c_char; 4usize],
}
#[repr(C)]
pub struct ShaderConstantTBindgenTy1 {
    pub name: __BindgenUnionField<*const ::std::os::raw::c_char>,
    pub hashed_name: __BindgenUnionField<StrhashT>,
    pub bindgen_union_field: u64,
}
impl Default for ShaderConstantTBindgenTy1 {
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
pub union ShaderConstantTBindgenTy2 {
    pub __bindgen_anon_1: ShaderConstantTBindgenTy2BindgenTy1,
    pub struct_size: u32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ShaderConstantTBindgenTy2BindgenTy1 {
    pub rows: u8,
    pub columns: u8,
}
impl Default for ShaderConstantTBindgenTy2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for ShaderConstantT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_SHADER_RESOURCE_TYPE_BUFFER: ShaderResourceType = 0;
pub const TM_SHADER_RESOURCE_TYPE_TEXTURE_1D: ShaderResourceType = 1;
pub const TM_SHADER_RESOURCE_TYPE_TEXTURE_2D: ShaderResourceType = 2;
pub const TM_SHADER_RESOURCE_TYPE_TEXTURE_3D: ShaderResourceType = 3;
pub const TM_SHADER_RESOURCE_TYPE_TEXTURE_CUBE: ShaderResourceType = 4;
pub const TM_SHADER_RESOURCE_TYPE_TEXTURE_1D_ARRAY: ShaderResourceType = 5;
pub const TM_SHADER_RESOURCE_TYPE_TEXTURE_2D_ARRAY: ShaderResourceType = 6;
pub const TM_SHADER_RESOURCE_TYPE_TEXTURE_CUBE_ARRAY: ShaderResourceType = 7;
pub const TM_SHADER_RESOURCE_TYPE_SAMPLER: ShaderResourceType = 8;
pub const TM_SHADER_RESOURCE_TYPE_SAMPLER_COMPARISON: ShaderResourceType = 9;
pub const TM_SHADER_RESOURCE_TYPE_ACCELERATION_STRUCTURE: ShaderResourceType = 10;
pub const TM_SHADER_RESOURCE_TYPE_MAX_TYPES: ShaderResourceType = 11;
pub type ShaderResourceType = ::std::os::raw::c_int;
#[repr(C)]
pub struct ShaderResourceT {
    pub __bindgen_anon_1: ShaderResourceTBindgenTy1,
    pub element_type: *const ::std::os::raw::c_char,
    pub static_resource_name: *const ::std::os::raw::c_char,
    pub static_resource: u32,
    pub elements: u32,
    pub type_: ShaderResourceType,
    pub requested_view_aspect: u8,
    pub uav: bool,
    pub _padding_74: [::std::os::raw::c_char; 2usize],
}
#[repr(C)]
pub struct ShaderResourceTBindgenTy1 {
    pub name: __BindgenUnionField<*const ::std::os::raw::c_char>,
    pub hashed_name: __BindgenUnionField<StrhashT>,
    pub bindgen_union_field: u64,
}
impl Default for ShaderResourceTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for ShaderResourceT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct ShaderPayloadT {
    pub __bindgen_anon_1: ShaderPayloadTBindgenTy1,
    pub num_fields: u32,
    pub _padding_87: [::std::os::raw::c_char; 4usize],
    pub fields: *const ShaderConstantT,
}
#[repr(C)]
pub struct ShaderPayloadTBindgenTy1 {
    pub name: __BindgenUnionField<*const ::std::os::raw::c_char>,
    pub hashed_name: __BindgenUnionField<StrhashT>,
    pub bindgen_union_field: u64,
}
impl Default for ShaderPayloadTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for ShaderPayloadT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_SHADER_SYSTEM_SEMANTIC_POSITION: ShaderSystemSemanticType = 0;
pub const TM_SHADER_SYSTEM_SEMANTIC_DISPATCH_THREAD_ID: ShaderSystemSemanticType = 1;
pub const TM_SHADER_SYSTEM_SEMANTIC_GROUP_ID: ShaderSystemSemanticType = 2;
pub const TM_SHADER_SYSTEM_SEMANTIC_GROUP_INDEX: ShaderSystemSemanticType = 3;
pub const TM_SHADER_SYSTEM_SEMANTIC_GROUP_THREAD_ID: ShaderSystemSemanticType = 4;
pub const TM_SHADER_SYSTEM_SEMANTIC_INSTANCE_ID: ShaderSystemSemanticType = 5;
pub const TM_SHADER_SYSTEM_SEMANTIC_PRIMITIVE_ID: ShaderSystemSemanticType = 6;
pub const TM_SHADER_SYSTEM_SEMANTIC_VERTEX_ID: ShaderSystemSemanticType = 7;
pub const TM_SHADER_SYSTEM_SEMANTIC_IS_FRONT_FACE_ID: ShaderSystemSemanticType = 8;
pub const TM_SHADER_SYSTEM_SEMANTIC_OUTPUT_CONTROL_POINT_ID: ShaderSystemSemanticType = 9;
pub const TM_SHADER_SYSTEM_SEMANTIC_DOMAIN_LOCATION: ShaderSystemSemanticType = 10;
pub const TM_SHADER_SYSTEM_SEMANTIC_MAX_SEMANTICS: ShaderSystemSemanticType = 11;
pub type ShaderSystemSemanticType = ::std::os::raw::c_int;
pub const TM_SHADER_INTERPOLATION_MODIFIER_LINEAR: ShaderInterpolationModifierType = 0;
pub const TM_SHADER_INTERPOLATION_MODIFIER_CENTROID: ShaderInterpolationModifierType = 1;
pub const TM_SHADER_INTERPOLATION_MODIFIER_NOINTERPOLATION: ShaderInterpolationModifierType = 2;
pub const TM_SHADER_INTERPOLATION_MODIFIER_NOPERSPECTIVE: ShaderInterpolationModifierType = 3;
pub const TM_SHADER_INTERPOLATION_MODIFIER_SAMPLE: ShaderInterpolationModifierType = 4;
pub const TM_SHADER_INTERPOLATION_MODIFIER_MAX_MODIFIERS: ShaderInterpolationModifierType = 5;
pub type ShaderInterpolationModifierType = ::std::os::raw::c_int;
#[repr(C)]
pub struct ShaderStageExportT {
    pub requested: bool,
    pub _padding_118: [::std::os::raw::c_char; 7usize],
    pub constant: ShaderConstantT,
    pub interpolation_modifier: u32,
    pub _padding_121: [::std::os::raw::c_char; 4usize],
}
impl Default for ShaderStageExportT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_SHADER_TESS_ATTRIBUTE_DOMAIN_VALUE_TRIANGLE: ShaderTessAttributeDomain = 0;
pub const TM_SHADER_TESS_ATTRIBUTE_DOMAIN_VALUE_QUAD: ShaderTessAttributeDomain = 1;
pub const TM_SHADER_TESS_ATTRIBUTE_DOMAIN_VALUE_ISOLINE: ShaderTessAttributeDomain = 2;
pub const TM_SHADER_TESS_ATTRIBUTE_DOMAIN_VALUE_MAX_VALUES: ShaderTessAttributeDomain = 3;
pub type ShaderTessAttributeDomain = ::std::os::raw::c_int;
pub const TM_SHADER_HULL_ATTRIBUTE_OUTPUT_VALUE_POINT: ShaderHullAttributeOutputTopology = 0;
pub const TM_SHADER_HULL_ATTRIBUTE_OUTPUT_VALUE_LINE: ShaderHullAttributeOutputTopology = 1;
pub const TM_SHADER_HULL_ATTRIBUTE_OUTPUT_VALUE_TRIANGLE_CW: ShaderHullAttributeOutputTopology = 2;
pub const TM_SHADER_HULL_ATTRIBUTE_OUTPUT_VALUE_TRIANGLE_CCW: ShaderHullAttributeOutputTopology = 3;
pub const TM_SHADER_HULL_ATTRIBUTE_OUTPUT_VALUE_MAX_VALUES: ShaderHullAttributeOutputTopology = 4;
pub type ShaderHullAttributeOutputTopology = ::std::os::raw::c_int;
pub const TM_SHADER_HULL_ATTRIBUTE_PARTITIONING_VALUE_INTEGER: ShaderHullAttributePartitioning = 0;
pub const TM_SHADER_HULL_ATTRIBUTE_PARTITIONING_VALUE_FRACTIONAL_EVEN:
    ShaderHullAttributePartitioning = 1;
pub const TM_SHADER_HULL_ATTRIBUTE_PARTITIONING_VALUE_FRACTIONAL_ODD:
    ShaderHullAttributePartitioning = 2;
pub const TM_SHADER_HULL_ATTRIBUTE_PARTITIONING_VALUE_POW2: ShaderHullAttributePartitioning = 3;
pub const TM_SHADER_HULL_ATTRIBUTE_PARTITIONING_VALUE_MAX_VALUES: ShaderHullAttributePartitioning =
    4;
pub type ShaderHullAttributePartitioning = ::std::os::raw::c_int;
pub const TM_SHADER_GEOMETRY_ATTRIBUTE_INSTANCE: ShaderAttributeType = 0;
pub const TM_SHADER_TESS_ATTRIBUTE_DOMAIN: ShaderAttributeType = 1;
pub const TM_SHADER_HULL_ATTRIBUTE_MAX_TESS_FACTOR: ShaderAttributeType = 2;
pub const TM_SHADER_HULL_ATTRIBUTE_OUTPUT_CONTROL_POINTS: ShaderAttributeType = 3;
pub const TM_SHADER_HULL_ATTRIBUTE_OUTPUT_TOPOLOGY: ShaderAttributeType = 4;
pub const TM_SHADER_HULL_ATTRIBUTE_PARTITIONING: ShaderAttributeType = 5;
pub const TM_SHADER_HULL_ATTRIBUTE_PATCH_CONSTANT_FUNCTION: ShaderAttributeType = 6;
pub const TM_SHADER_PIXEL_ATTRIBUTE_EARLY_DEPTH_STENCIL: ShaderAttributeType = 7;
pub const TM_SHADER_COMPUTE_ATTRIBUTE_NUM_THREADS: ShaderAttributeType = 8;
pub const TM_SHADER_SURFACE_ATTRIBUTE_PAYLOAD: ShaderAttributeType = 9;
pub const TM_SHADER_ATTRIBUTE_MAX_ATTRIBUTES: ShaderAttributeType = 10;
pub type ShaderAttributeType = ::std::os::raw::c_int;
pub const TM_SHADER_STAGE_ATTRIBUTE_MAX_STRING_LENGTH: ::std::os::raw::c_int = 64;
pub type _bindgen_ty_1 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderStageAttributeT {
    pub type_: ShaderAttributeType,
    pub _padding_190: [::std::os::raw::c_char; 4usize],
    pub __bindgen_anon_1: ShaderStageAttributeTBindgenTy1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ShaderStageAttributeTBindgenTy1 {
    pub boolean: bool,
    pub values: [u32; 4usize],
    pub value: u32,
    pub string: [::std::os::raw::c_char; 64usize],
}
impl Default for ShaderStageAttributeTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for ShaderStageAttributeT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub static tm_shader_constant_type_to_bytes: [u32; 11usize];
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ShaderSystemClTypeT {
    pub uint64: u64,
    pub boolean: bool,
}
impl Default for ShaderSystemClTypeT {
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
pub struct ShaderO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderSystemIoO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderBlobHeaderT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderSystemO {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct CreationGraphUpdateShaderConstantResourceT {
    pub draw_calls: *mut CreationGraphOutputT,
    pub cbuffers: *const ShaderConstantBufferInstanceT,
    pub io_interface: TtIdT,
    pub entity_id: u64,
    pub entity_ctx: *mut EntityContextO,
}
impl Default for CreationGraphUpdateShaderConstantResourceT {
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
pub struct ShaderDeclarationO {
    _unused: [u8; 0],
}
pub const TM_SHADER_STAGE_VERTEX: ::std::os::raw::c_int = 1;
pub const TM_SHADER_STAGE_HULL: ::std::os::raw::c_int = 2;
pub const TM_SHADER_STAGE_DOMAIN: ::std::os::raw::c_int = 4;
pub const TM_SHADER_STAGE_GEOMETRY: ::std::os::raw::c_int = 8;
pub const TM_SHADER_STAGE_PIXEL: ::std::os::raw::c_int = 16;
pub const TM_SHADER_STAGE_COMPUTE: ::std::os::raw::c_int = 32;
pub const TM_SHADER_STAGE_RAYGEN: ::std::os::raw::c_int = 64;
pub const TM_SHADER_STAGE_ANY_HIT: ::std::os::raw::c_int = 128;
pub const TM_SHADER_STAGE_CLOSEST_HIT: ::std::os::raw::c_int = 256;
pub const TM_SHADER_STAGE_MISS: ::std::os::raw::c_int = 512;
pub const TM_SHADER_STAGE_INTERSECTION: ::std::os::raw::c_int = 1024;
pub const TM_SHADER_STAGE_ALL: ::std::os::raw::c_int = 2047;
pub const TM_SHADER_STAGE_MAX_STAGES: ::std::os::raw::c_int = 11;
pub type _bindgen_ty_2 = ::std::os::raw::c_int;
pub const TM_SHADER_SYSTEM_MAX_PASSES: ::std::os::raw::c_int = 16;
pub const TM_SHADER_SYSTEM_MAX_ACTIVE_SYSTEMS: ::std::os::raw::c_int = 16;
pub type _bindgen_ty_3 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ShaderDeclarationApi {
    pub clear: ::std::option::Option<unsafe extern "C" fn(inst: *mut ShaderDeclarationO)>,
    pub append_declaration: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ShaderDeclarationO, src: *const ShaderDeclarationO),
    >,
    pub append_render_states: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderDeclarationO,
            block_type: u32,
            states: *const RendererStateValuePairT,
            num_states: u32,
        ),
    >,
    pub append_serialized_render_states: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ShaderDeclarationO, block_type: u32, data: *const u8),
    >,
    pub append_static_sampler_states: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderDeclarationO,
            state_block_name_hash: StrhashT,
            states: *const RendererStateValuePairT,
            num_states: u32,
        ),
    >,
    pub request_channel: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderDeclarationO,
            stage_flag: u32,
            channel_name: *const ::std::os::raw::c_char,
        ),
    >,
    pub set_constant: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ShaderDeclarationO, constant: ShaderConstantT),
    >,
    pub set_resource: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ShaderDeclarationO, resource: ShaderResourceT),
    >,
    pub set_payload: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ShaderDeclarationO, payload: ShaderPayloadT),
    >,
    pub set_stage_export: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderDeclarationO,
            stage_flag: u32,
            stage_export: ShaderStageExportT,
        ),
    >,
    pub set_stage_system_semantic_import: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderDeclarationO,
            stage_flag: u32,
            type_: ShaderSystemSemanticType,
        ),
    >,
    pub set_stage_system_semantic_export: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderDeclarationO,
            stage_flag: u32,
            type_: ShaderSystemSemanticType,
        ),
    >,
    pub set_stage_attribute: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderDeclarationO,
            stage_flag: u32,
            attr: ShaderStageAttributeT,
        ),
    >,
    pub append_patch_code: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderDeclarationO,
            domain: u32,
            code: *const ::std::os::raw::c_char,
            source_file: *const ::std::os::raw::c_char,
            source_line: u32,
        ),
    >,
    pub append_code: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderDeclarationO,
            stage_flag: u32,
            code: *const ::std::os::raw::c_char,
            source_file: *const ::std::os::raw::c_char,
            source_line: u32,
        ),
    >,
    pub append_common_code: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderDeclarationO,
            code: *const ::std::os::raw::c_char,
            source_file: *const ::std::os::raw::c_char,
            source_line: u32,
        ),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderSystemContextO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ShaderSystemApi {
    pub create_context: ::std::option::Option<
        unsafe extern "C" fn(
            allocator: *mut AllocatorI,
            render_graph: *mut RenderGraphO,
        ) -> *mut ShaderSystemContextO,
    >,
    pub clone_context: ::std::option::Option<
        unsafe extern "C" fn(
            src_context: *const ShaderSystemContextO,
            allocator: *mut AllocatorI,
        ) -> *mut ShaderSystemContextO,
    >,
    pub destroy_context:
        ::std::option::Option<unsafe extern "C" fn(context: *mut ShaderSystemContextO)>,
    pub set_render_graph: ::std::option::Option<
        unsafe extern "C" fn(context: *mut ShaderSystemContextO, render_graph: *mut RenderGraphO),
    >,
    pub activate_system: ::std::option::Option<
        unsafe extern "C" fn(
            context: *mut ShaderSystemContextO,
            system: *mut ShaderSystemO,
            cbuffer_instances: *const ShaderConstantBufferInstanceT,
            num_cbuffer_instances: u32,
            rbinder_instances: *const ShaderResourceBinderInstanceT,
            num_rbinder_instances: u32,
        ),
    >,
    pub deactivate_system: ::std::option::Option<
        unsafe extern "C" fn(context: *mut ShaderSystemContextO, system: *mut ShaderSystemO),
    >,
}
pub const TM_SHADER_SYSTEM_UNINITIALIZED_INSTANCE: ::std::os::raw::c_int = -1;
pub type _bindgen_ty_4 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ShaderConstantBufferInstanceT {
    pub instance_id: u32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ShaderResourceBinderInstanceT {
    pub instance_id: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderConstantUpdateT {
    pub instance_id: u32,
    pub constant_offset: u32,
    pub first_byte: u32,
    pub num_bytes: u32,
    pub data: *const ::std::os::raw::c_void,
}
impl Default for ShaderConstantUpdateT {
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
pub struct ShaderResourceUpdateT {
    pub instance_id: u32,
    pub resource_slot: u32,
    pub first_resource: u32,
    pub num_resources: u32,
    pub resources: *const RendererHandleT,
    pub resources_view_aspect_flags: *const u32,
}
impl Default for ShaderResourceUpdateT {
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
pub struct ShaderIoO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ShaderApi {
    pub create_constant_buffer_instances: ::std::option::Option<
        unsafe extern "C" fn(
            io: *mut ShaderIoO,
            num_instances: u32,
            result: *mut ShaderConstantBufferInstanceT,
        ),
    >,
    pub create_constant_buffer_instances_from_template: ::std::option::Option<
        unsafe extern "C" fn(
            io: *mut ShaderIoO,
            resource_buffer: *mut RendererResourceCommandBufferO,
            num_instances: u32,
            result: *mut ShaderConstantBufferInstanceT,
            cbuf_template: ShaderConstantBufferInstanceT,
        ),
    >,
    pub destroy_constant_buffer_instances: ::std::option::Option<
        unsafe extern "C" fn(
            io: *mut ShaderIoO,
            instances: *mut ShaderConstantBufferInstanceT,
            num_instances: u32,
        ),
    >,
    pub create_resource_binder_instances: ::std::option::Option<
        unsafe extern "C" fn(
            io: *mut ShaderIoO,
            num_instances: u32,
            result: *mut ShaderResourceBinderInstanceT,
        ),
    >,
    pub destroy_resource_binder_instances: ::std::option::Option<
        unsafe extern "C" fn(
            io: *mut ShaderIoO,
            instances: *mut ShaderResourceBinderInstanceT,
            num_instances: u32,
        ),
    >,
    pub reflect_constants: ::std::option::Option<
        unsafe extern "C" fn(
            io: *mut ShaderIoO,
            num_constants: *mut u32,
            constants: *mut ShaderConstantT,
            constant_offsets: *mut u32,
        ),
    >,
    pub lookup_constant: ::std::option::Option<
        unsafe extern "C" fn(
            io: *mut ShaderIoO,
            name: StrhashT,
            constant: *mut ShaderConstantT,
            constant_offset: *mut u32,
        ) -> bool,
    >,
    pub update_constants: ::std::option::Option<
        unsafe extern "C" fn(
            io: *mut ShaderIoO,
            resource_buffer: *mut RendererResourceCommandBufferO,
            constant_updates: *const ShaderConstantUpdateT,
            num_updates: u32,
        ),
    >,
    pub update_constants_raw: ::std::option::Option<
        unsafe extern "C" fn(
            io: *mut ShaderIoO,
            resource_buffer: *mut RendererResourceCommandBufferO,
            instance_ids: *const u32,
            data: *mut *const ::std::os::raw::c_void,
            offset: u32,
            size: u32,
            num_updates: u32,
        ),
    >,
    pub update_constant_buffer_instances_from_template: ::std::option::Option<
        unsafe extern "C" fn(
            io: *mut ShaderIoO,
            resource_buffer: *mut RendererResourceCommandBufferO,
            num_instances: u32,
            instances: *mut ShaderConstantBufferInstanceT,
            cbuf_template: ShaderConstantBufferInstanceT,
        ),
    >,
    pub read_constant_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            io: *mut ShaderIoO,
            instance: ShaderConstantBufferInstanceT,
        ) -> *const ::std::os::raw::c_void,
    >,
    pub reflect_resources: ::std::option::Option<
        unsafe extern "C" fn(
            io: *mut ShaderIoO,
            num_resources: *mut u32,
            resources: *mut ShaderResourceT,
            resource_slots: *mut u32,
        ),
    >,
    pub lookup_resource: ::std::option::Option<
        unsafe extern "C" fn(
            io: *mut ShaderIoO,
            name: StrhashT,
            resource: *mut ShaderResourceT,
            resource_slot: *mut u32,
        ) -> bool,
    >,
    pub update_resources: ::std::option::Option<
        unsafe extern "C" fn(
            io: *mut ShaderIoO,
            resource_buffer: *mut RendererResourceCommandBufferO,
            resource_updates: *const ShaderResourceUpdateT,
            num_updates: u32,
        ),
    >,
    pub read_resource_slot: ::std::option::Option<
        unsafe extern "C" fn(
            io: *mut ShaderIoO,
            instance: ShaderResourceBinderInstanceT,
            slot: u32,
            aspects: *mut *const u32,
        ) -> *const RendererHandleT,
    >,
    pub max_passes: ::std::option::Option<unsafe extern "C" fn(shader: *mut ShaderO) -> u8>,
    pub assemble_shader_infos: ::std::option::Option<
        unsafe extern "C" fn(
            shader: *mut ShaderO,
            state_override_blocks: *const RendererHandleT,
            num_state_override_blocks: u32,
            context: *const ShaderSystemContextO,
            visibility_context: StrhashT,
            resource_buffer: *mut RendererResourceCommandBufferO,
            cbuf_instances: *const ShaderConstantBufferInstanceT,
            rbinder_instances: *const ShaderResourceBinderInstanceT,
            num_shaders: u32,
            results: *mut RendererShaderInfoT,
        ) -> u8,
    >,
    pub shader_io:
        ::std::option::Option<unsafe extern "C" fn(shader: *mut ShaderO) -> *mut ShaderIoO>,
    pub system_io:
        ::std::option::Option<unsafe extern "C" fn(system: *mut ShaderSystemO) -> *mut ShaderIoO>,
    pub name: ::std::option::Option<unsafe extern "C" fn(shader: *const ShaderO) -> StrhashT>,
    pub shader_version: ::std::option::Option<unsafe extern "C" fn(shader: *const ShaderO) -> u32>,
    pub system_version:
        ::std::option::Option<unsafe extern "C" fn(system: *const ShaderSystemO) -> u32>,
}
pub const TM_SHADER_STATE_OVERRIDE_DOUBLE_SIDED: ShaderStateOverrides = 0;
pub const TM_SHADER_STATE_OVERRIDE_FRONT_FACE_CW: ShaderStateOverrides = 1;
pub const TM_SHADER_STATE_OVERRIDE_DEPTH_TEST_GREATER_EQUAL: ShaderStateOverrides = 2;
pub const TM_SHADER_STATE_OVERRIDE_DEPTH_TEST_LESS: ShaderStateOverrides = 3;
pub const TM_SHADER_STATE_OVERRIDE_MAX_SHADER_STATE_OVERRIDES: ShaderStateOverrides = 4;
pub type ShaderStateOverrides = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderRepositoryO {
    _unused: [u8; 0],
}
pub const TM_SHADER_SYSTEM_BACKGROUND_CREATE_STATUS_SUCCESS: ShaderSystemBackgroundStatus = 0;
pub const TM_SHADER_SYSTEM_BACKGROUND_CREATE_STATUS_CANCEL: ShaderSystemBackgroundStatus = 1;
pub const TM_SHADER_SYSTEM_BACKGROUND_CREATE_STATUS_ERROR: ShaderSystemBackgroundStatus = 2;
pub type ShaderSystemBackgroundStatus = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderSystemBackgroundCreateI {
    pub user_data: *mut ::std::os::raw::c_void,
    pub done: ::std::option::Option<
        unsafe extern "C" fn(
            status: ShaderSystemBackgroundStatus,
            blob: *const ShaderBlobHeaderT,
            blob_size: u64,
            user_data: *mut ::std::os::raw::c_void,
        ),
    >,
}
impl Default for ShaderSystemBackgroundCreateI {
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
pub struct ShaderRepositoryApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            system_io: *mut ShaderSystemIoO,
            a: *mut AllocatorI,
            backend: *mut RendererBackendI,
            shader_compiler_api: *mut RendererShaderCompilerApi,
            tt: *mut TheTruthO,
        ) -> *mut ShaderRepositoryO,
    >,
    pub destroy: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderRepositoryO,
            resource_buffer: *mut RendererResourceCommandBufferO,
        ),
    >,
    pub set_the_truth: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ShaderRepositoryO, tt: *mut TheTruthO),
    >,
    pub create_truth_types: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
    pub import_truth_declaration_from_config: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderRepositoryO,
            config: *const ConfigI,
            pi: *const JsonParseInfoT,
            name: StrhashT,
            validity_hash: u64,
            time_stamp: *const FileTimeO,
            filename: *const ::std::os::raw::c_char,
        ) -> TtIdT,
    >,
    pub destroy_truth_declaration:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut ShaderRepositoryO, name: StrhashT)>,
    pub lookup_truth_declaration: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ShaderRepositoryO, name: StrhashT) -> TtIdT,
    >,
    pub lookup_declaration_validity_hash:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut ShaderRepositoryO, id: TtIdT) -> u64>,
    pub load_shader_declaration: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderRepositoryO,
            declaration: *mut ShaderDeclarationO,
            id: TtIdT,
        ),
    >,
    pub last_modified: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ShaderRepositoryO, id: TtIdT, modified: *mut FileTimeO),
    >,
    pub host_save_state: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderRepositoryO,
            carray: *mut *mut ::std::os::raw::c_char,
            a: *mut AllocatorI,
        ),
    >,
    pub client_load_state: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderRepositoryO,
            tt: *mut TheTruthO,
            state: *mut *const ::std::os::raw::c_char,
        ),
    >,
    pub create_shader_declaration: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderRepositoryO,
            a: *mut AllocatorI,
        ) -> *mut ShaderDeclarationO,
    >,
    pub destroy_shader_declaration: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ShaderRepositoryO, declaration: *mut ShaderDeclarationO),
    >,
    pub generate_system_includes: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderRepositoryO,
            output_path: *const ::std::os::raw::c_char,
        ),
    >,
    pub update_shaders_from_directory: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderRepositoryO,
            dir: *const ::std::os::raw::c_char,
            recursive: bool,
            allocator: *mut AllocatorI,
            res_buf: *mut RendererResourceCommandBufferO,
        ),
    >,
    pub allocator: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ShaderRepositoryO) -> *mut AllocatorI,
    >,
    pub refresh_truth_shaders:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut ShaderRepositoryO)>,
    pub lookup_or_create_shader: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ShaderRepositoryO, name: StrhashT) -> *mut ShaderO,
    >,
    pub lookup_or_create_system: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ShaderRepositoryO, name: StrhashT) -> *mut ShaderSystemO,
    >,
    pub create_shader_from_blob: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderRepositoryO,
            res_buf: *mut RendererResourceCommandBufferO,
            shader_blob: *const ShaderBlobHeaderT,
        ) -> *mut ShaderO,
    >,
    pub compile_shader: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderRepositoryO,
            res_buf: *mut RendererResourceCommandBufferO,
            shader: *mut ShaderO,
            active_shader_stages: u32,
            active_systems_bitmask: u64,
            active_system_names: *const StrhashT,
            active_systems: *mut *const ShaderDeclarationO,
            num_active_systems: u32,
            declarations: *mut *const ShaderDeclarationO,
            num_declarations: u32,
            validity_hash: u64,
        ) -> bool,
    >,
    pub create_from_declaration: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderRepositoryO,
            res_buf: *mut RendererResourceCommandBufferO,
            declaration: TtIdT,
            name: StrhashT,
            generated_code: *mut ShaderDeclarationO,
            creation_graph_data: *const ShaderCreationGraphDataO,
        ) -> bool,
    >,
    pub background_create_from_declaration: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderRepositoryO,
            declaration: TtIdT,
            name: StrhashT,
            generated_code: *mut ShaderDeclarationO,
            creation_graph_data: *const ShaderCreationGraphDataO,
            callback: *const ShaderSystemBackgroundCreateI,
        ),
    >,
    pub destroy_shader:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut ShaderRepositoryO, name: StrhashT)>,
    pub destroy_system:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut ShaderRepositoryO, name: StrhashT)>,
    pub lookup_shader: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ShaderRepositoryO, name: StrhashT) -> *mut ShaderO,
    >,
    pub lookup_system: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ShaderRepositoryO, name: StrhashT) -> *mut ShaderSystemO,
    >,
    pub recycle_resources:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut ShaderRepositoryO)>,
    pub shader_state_override: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderRepositoryO,
            state_override: ShaderStateOverrides,
        ) -> RendererHandleT,
    >,
    pub lookup_shader_blob: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ShaderRepositoryO,
            name: StrhashT,
            size: *mut u64,
        ) -> *const ShaderBlobHeaderT,
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreationGraphInterpreterO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreationGraphNodeTypeI {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreationGraphInterpreterWireContentT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConnectorTypeT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreationGraphInstanceT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderCreationGraphDataO {
    pub allocator: *mut AllocatorI,
    pub connected_inputs: *mut bool,
    pub actions: *mut TtIdT,
    pub settings: *mut TtIdT,
}
impl Default for ShaderCreationGraphDataO {
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
pub struct ShaderCreationGraphApi {
    pub create_truth_types: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
    pub compile_data_to_wire: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut CreationGraphInstanceT,
            wire: u32,
            tt: *const TheTruthO,
            data_id: TtIdT,
            to_type_hash: StrhashT,
        ) -> bool,
    >,
    pub node_from_declaration: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            declaration: TtIdT,
            node: *mut CreationGraphNodeTypeI,
            a: *mut AllocatorI,
        ),
    >,
    pub register_static_graph_nodes:
        ::std::option::Option<unsafe extern "C" fn(reg: *mut ApiRegistryApi, load: bool)>,
    pub wire_result_as_string: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            ta: *mut TempAllocatorI,
            cw: *const CreationGraphInterpreterWireContentT,
            requested_type: *const ConnectorTypeT,
        ) -> *const ::std::os::raw::c_char,
    >,
}
pub const TM_TT_PROP__SHADER_REPOSITORY__DECLARATIONS: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__SHADER_REPOSITORY__MATERIALS: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_5 = ::std::os::raw::c_int;
pub const TM_TT_PROP__SHADER_STAGE_DECLARATION__CODE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__SHADER_STAGE_DECLARATION__CODE_LINE_NUMBER: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__SHADER_STAGE_DECLARATION__STAGE_EXPORTS: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__SHADER_STAGE_DECLARATION__SYSTEM_SEMANTICS_IMPORTS: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__SHADER_STAGE_DECLARATION__SYSTEM_SEMANTICS_EXPORTS: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__SHADER_STAGE_DECLARATION__STAGE_ATTRIBUTES: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__SHADER_STAGE_DECLARATION__CONDITION: ::std::os::raw::c_int = 6;
pub type _bindgen_ty_6 = ::std::os::raw::c_int;
pub const TM_TT_PROP__SHADER_DECLARATION__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__SHADER_DECLARATION__COMPILABLE: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__SHADER_DECLARATION__SYSTEM: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__SHADER_DECLARATION__CREATION_GRAPH_NODE: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__SHADER_DECLARATION__INCLUDES: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__SHADER_DECLARATION__VARIATIONS: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__SHADER_DECLARATION__COMPILE_CONFIGURATIONS: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__SHADER_DECLARATION__COMPILE_CONTEXTS: ::std::os::raw::c_int = 7;
pub const TM_TT_PROP__SHADER_DECLARATION__FILENAME: ::std::os::raw::c_int = 8;
pub const TM_TT_PROP__SHADER_DECLARATION__VALIDITY_HASH: ::std::os::raw::c_int = 9;
pub const TM_TT_PROP__SHADER_DECLARATION__LAST_MODIFIED: ::std::os::raw::c_int = 10;
pub const TM_TT_PROP__SHADER_DECLARATION__REQUESTS: ::std::os::raw::c_int = 11;
pub const TM_TT_PROP__SHADER_DECLARATION__CONSTANTS: ::std::os::raw::c_int = 12;
pub const TM_TT_PROP__SHADER_DECLARATION__RESOURCES: ::std::os::raw::c_int = 13;
pub const TM_TT_PROP__SHADER_DECLARATION__PAYLOADS: ::std::os::raw::c_int = 14;
pub const TM_TT_PROP__SHADER_DECLARATION__RENDER_STATE_BLOCKS: ::std::os::raw::c_int = 15;
pub const TM_TT_PROP__SHADER_DECLARATION__SAMPLER_STATE_BLOCKS: ::std::os::raw::c_int = 16;
pub const TM_TT_PROP__SHADER_DECLARATION__COMMON_CODE: ::std::os::raw::c_int = 17;
pub const TM_TT_PROP__SHADER_DECLARATION__COMMON_CODE_LINE_NUMBER: ::std::os::raw::c_int = 18;
pub const TM_TT_PROP__SHADER_DECLARATION__FUNCTION_CODE: ::std::os::raw::c_int = 19;
pub const TM_TT_PROP__SHADER_DECLARATION__FUNCTION_CODE_LINE_NUMBER: ::std::os::raw::c_int = 20;
pub const TM_TT_PROP__SHADER_DECLARATION__VERTEX_STAGE: ::std::os::raw::c_int = 21;
pub const TM_TT_PROP__SHADER_DECLARATION__HULL_STAGE: ::std::os::raw::c_int = 22;
pub const TM_TT_PROP__SHADER_DECLARATION__DOMAIN_STAGE: ::std::os::raw::c_int = 23;
pub const TM_TT_PROP__SHADER_DECLARATION__GEOMETRY_STAGE: ::std::os::raw::c_int = 24;
pub const TM_TT_PROP__SHADER_DECLARATION__PIXEL_STAGE: ::std::os::raw::c_int = 25;
pub const TM_TT_PROP__SHADER_DECLARATION__COMPUTE_STAGE: ::std::os::raw::c_int = 26;
pub const TM_TT_PROP__SHADER_DECLARATION__RAYGEN_STAGE: ::std::os::raw::c_int = 27;
pub const TM_TT_PROP__SHADER_DECLARATION__ANY_HIT_STAGE: ::std::os::raw::c_int = 28;
pub const TM_TT_PROP__SHADER_DECLARATION__CLOSEST_HIT_STAGE: ::std::os::raw::c_int = 29;
pub const TM_TT_PROP__SHADER_DECLARATION__MISS_STAGE: ::std::os::raw::c_int = 30;
pub const TM_TT_PROP__SHADER_DECLARATION__INTERSECTION_STAGE: ::std::os::raw::c_int = 31;
pub const TM_TT_PROP__SHADER_DECLARATION__HULL_PATCH: ::std::os::raw::c_int = 32;
pub const TM_TT_PROP__SHADER_DECLARATION__HULL_PATCH_LINE_NUMBER: ::std::os::raw::c_int = 33;
pub type _bindgen_ty_7 = ::std::os::raw::c_int;
pub const TM_TT_PROP__RENDER_STATE_BLOCK__TYPE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__RENDER_STATE_BLOCK__STATES: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_8 = ::std::os::raw::c_int;
pub const TM_TT_PROP__SAMPLER_STATE_BLOCK__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__SAMPLER_STATE_BLOCK__STATES: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_9 = ::std::os::raw::c_int;
pub const TM_TT_PROP__SHADER_REQUEST__TYPE_NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__SHADER_REQUEST__CHANNEL_NAME: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_10 = ::std::os::raw::c_int;
pub const TM_TT_PROP__SHADER_CONSTANT__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__SHADER_CONSTANT__TYPE: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__SHADER_CONSTANT__ROWS: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__SHADER_CONSTANT__COLUMNS: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__SHADER_CONSTANT__STRUCT_SIZE: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__SHADER_CONSTANT__ELEMENTS: ::std::os::raw::c_int = 5;
pub type _bindgen_ty_11 = ::std::os::raw::c_int;
pub const TM_TT_PROP__SHADER_RESOURCE__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__SHADER_RESOURCE__TYPE: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__SHADER_RESOURCE__ELEMENT_TYPE: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__SHADER_RESOURCE__ELEMENTS: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__SHADER_RESOURCE__STATIC_RESOURCE: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__SHADER_RESOURCE__UAV: ::std::os::raw::c_int = 5;
pub type _bindgen_ty_12 = ::std::os::raw::c_int;
pub const TM_TT_PROP__SHADER_PAYLOAD__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__SHADER_PAYLOAD__FIELDS: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_13 = ::std::os::raw::c_int;
pub const TM_TT_PROP__STAGE_EXPORT__CONSTANT: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__STAGE_EXPORT__INTERPOLATION_MODIFIER: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__STAGE_EXPORT__CHANNEL_REQUESTED: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_14 = ::std::os::raw::c_int;
pub const TM_TT_PROP__INCLUDE__NAME: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_15 = ::std::os::raw::c_int;
pub const TM_TT_PROP__VARIATION__SYSTEMS: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_16 = ::std::os::raw::c_int;
pub const TM_TT_PROP__VARIATION__SYSTEM__NAME: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_17 = ::std::os::raw::c_int;
pub const TM_TT_PROP__COMPILE_CONFIGURATION__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__COMPILE_CONFIGURATION__VARIATIONS: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_18 = ::std::os::raw::c_int;
pub const TM_TT_PROP__COMPILE_BRANCH__CONDITION: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__COMPILE_BRANCH__THEN: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__COMPILE_BRANCH__ELSE: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_19 = ::std::os::raw::c_int;
pub const TM_TT_PROP__PASS_CONDITION__SYSTEMS_ACTIVE: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_20 = ::std::os::raw::c_int;
pub const TM_TT_PROP__PASS__LAYER: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__PASS__CONDITION: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__PASS__ENABLE_SYSTEMS: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__PASS__RENDER_STATE_BLOCKS: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__PASS__COMPILE_CONFIGURATION: ::std::os::raw::c_int = 4;
pub type _bindgen_ty_21 = ::std::os::raw::c_int;
pub const TM_TT_PROP__COMPILE_CONTEXT__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__COMPILE_CONTEXT__PASSES: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_22 = ::std::os::raw::c_int;
pub const TM_TT_PROP__SHADER_SYSTEM_COMPILED_RESULT__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__SHADER_SYSTEM_COMPILED_RESULT__BUFFER: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_23 = ::std::os::raw::c_int;
pub const TM_TT_PROP__SHADER_SYSTEM__CONSTANT_NODE__VALUE_TYPE_HASH: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__SHADER_SYSTEM__CONSTANT_NODE__VALUE: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_24 = ::std::os::raw::c_int;
pub const TM_TT_PROP__CREATION_GRAPH_NODE_RESOURCE_ASPECT__RESOURCE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__CREATION_GRAPH_NODE_RESOURCE_ASPECT__ASPECT: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_25 = ::std::os::raw::c_int;
pub const TM_TT_PROP__CREATION_GRAPH_NODE_DEFINE__NAME: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_26 = ::std::os::raw::c_int;
pub const TM_TT_PROP__CREATION_GRAPH_NODE_ACTION__NAME_HASH: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__CREATION_GRAPH_NODE_ACTION__RESOURCE_ASPECTS: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__CREATION_GRAPH_NODE_ACTION__DEFINES: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__CREATION_GRAPH_NODE_ACTION__REQUESTS: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__CREATION_GRAPH_NODE_ACTION__ENABLE_SYSTEMS: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__CREATION_GRAPH_NODE_ACTION__RENDER_STATE_BLOCKS: ::std::os::raw::c_int = 5;
pub type _bindgen_ty_27 = ::std::os::raw::c_int;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR_SETTING_OPTION__DISPLAY_NAME:
    ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR_SETTING_OPTION__DISPLAY_TOOLTIP:
    ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR_SETTING_OPTION__ACTION: ::std::os::raw::c_int =
    2;
pub type _bindgen_ty_28 = ::std::os::raw::c_int;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR_SETTING__DISPLAY_NAME: ::std::os::raw::c_int =
    0;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR_SETTING__DISPLAY_TOOLTIP:
    ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR_SETTING__SETTING_CONDITION:
    ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR_SETTING__OPTIONS: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR_SETTING__BOOL_ACTION: ::std::os::raw::c_int =
    4;
pub type _bindgen_ty_29 = ::std::os::raw::c_int;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR_SETTINGS__OBJECT_TYPE_HASH:
    ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR_SETTINGS__SETTINGS: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_30 = ::std::os::raw::c_int;
pub const TM_SHADER_SYSTEM__NODE_CONNECTOR__EVALUATION_STAGE__VERTEX: ::std::os::raw::c_int = 0;
pub const TM_SHADER_SYSTEM__NODE_CONNECTOR__EVALUATION_STAGE__HULL: ::std::os::raw::c_int = 1;
pub const TM_SHADER_SYSTEM__NODE_CONNECTOR__EVALUATION_STAGE__DOMAIN: ::std::os::raw::c_int = 2;
pub const TM_SHADER_SYSTEM__NODE_CONNECTOR__EVALUATION_STAGE__GEOMETRY: ::std::os::raw::c_int = 3;
pub const TM_SHADER_SYSTEM__NODE_CONNECTOR__EVALUATION_STAGE__PIXEL: ::std::os::raw::c_int = 4;
pub const TM_SHADER_SYSTEM__NODE_CONNECTOR__EVALUATION_STAGE__COMPUTE: ::std::os::raw::c_int = 5;
pub const TM_SHADER_SYSTEM__NODE_CONNECTOR__EVALUATION_STAGE__RAYGEN: ::std::os::raw::c_int = 6;
pub const TM_SHADER_SYSTEM__NODE_CONNECTOR__EVALUATION_STAGE__ANY_HIT: ::std::os::raw::c_int = 7;
pub const TM_SHADER_SYSTEM__NODE_CONNECTOR__EVALUATION_STAGE__CLOSEST_HIT: ::std::os::raw::c_int =
    8;
pub const TM_SHADER_SYSTEM__NODE_CONNECTOR__EVALUATION_STAGE__MISS: ::std::os::raw::c_int = 9;
pub const TM_SHADER_SYSTEM__NODE_CONNECTOR__EVALUATION_STAGE__INTERSECTION: ::std::os::raw::c_int =
    10;
pub const TM_SHADER_SYSTEM__NODE_CONNECTOR__EVALUATION_STAGE__MAX_STAGES: ::std::os::raw::c_int =
    11;
pub type _bindgen_ty_31 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_shader_system__node__connector__evaluation_stage_names:
        [*const ::std::os::raw::c_char; 11usize];
}
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR__DISPLAY_NAME: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR__DISPLAY_TOOLTIP: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR__TARGET_NAME: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR__TARGET_ELEMENT: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR__CONNECTOR_FLAG: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR__IS_OPTIONAL: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR__TYPE: ::std::os::raw::c_int = 7;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR__STRUCT_NAME: ::std::os::raw::c_int = 8;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR__ROWS: ::std::os::raw::c_int = 9;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR__COLUMNS: ::std::os::raw::c_int = 10;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR__TYPE_OF: ::std::os::raw::c_int = 11;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR__EVALUATION_STAGE: ::std::os::raw::c_int = 12;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR__SETTINGS: ::std::os::raw::c_int = 13;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR__EVALUATION_CONTEXTS: ::std::os::raw::c_int =
    14;
pub type _bindgen_ty_32 = ::std::os::raw::c_int;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR__TYPE_OF__CONNECTOR: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CONNECTOR__TYPE_OF__REQUESTED_CHANNEL:
    ::std::os::raw::c_int = 1;
pub type _bindgen_ty_33 = ::std::os::raw::c_int;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__DECLARATION_NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__NODE_NAME: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__DISPLAY_NAME: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__CATEGORY: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__INCLUDES: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__ACTIONS: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__INPUTS: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__CREATION_GRAPH_NODE__OUTPUTS: ::std::os::raw::c_int = 7;
pub type _bindgen_ty_34 = ::std::os::raw::c_int;
pub const TM_TT_PROP__EVALUATION_CONTEXT_NAME: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_35 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct CreationGraphOutputT {
    pub _address: u8,
}

// Extensions generated by machinery-generator

use const_cstr::{const_cstr, ConstCStr};

use crate::foundation::VersionT;

use crate::foundation::*;
use crate::plugins::entity::*;
use crate::plugins::render_graph::RenderGraphO;
use crate::plugins::renderer::*;

impl ShaderDeclarationApi {
    pub unsafe fn clear(&self, inst: *mut ShaderDeclarationO) {
        self.clear.unwrap()(inst)
    }

    pub unsafe fn append_declaration(
        &self,
        inst: *mut ShaderDeclarationO,
        src: *const ShaderDeclarationO,
    ) {
        self.append_declaration.unwrap()(inst, src)
    }

    pub unsafe fn append_render_states(
        &self,
        inst: *mut ShaderDeclarationO,
        block_type: u32,
        states: *const RendererStateValuePairT,
        num_states: u32,
    ) {
        self.append_render_states.unwrap()(inst, block_type, states, num_states)
    }

    pub unsafe fn append_serialized_render_states(
        &self,
        inst: *mut ShaderDeclarationO,
        block_type: u32,
        data: *const u8,
    ) {
        self.append_serialized_render_states.unwrap()(inst, block_type, data)
    }

    pub unsafe fn append_static_sampler_states(
        &self,
        inst: *mut ShaderDeclarationO,
        state_block_name_hash: StrhashT,
        states: *const RendererStateValuePairT,
        num_states: u32,
    ) {
        self.append_static_sampler_states.unwrap()(inst, state_block_name_hash, states, num_states)
    }

    pub unsafe fn request_channel(
        &self,
        inst: *mut ShaderDeclarationO,
        stage_flag: u32,
        channel_name: *const ::std::os::raw::c_char,
    ) {
        self.request_channel.unwrap()(inst, stage_flag, channel_name)
    }

    pub unsafe fn set_constant(&self, inst: *mut ShaderDeclarationO, constant: ShaderConstantT) {
        self.set_constant.unwrap()(inst, constant)
    }

    pub unsafe fn set_resource(&self, inst: *mut ShaderDeclarationO, resource: ShaderResourceT) {
        self.set_resource.unwrap()(inst, resource)
    }

    pub unsafe fn set_payload(&self, inst: *mut ShaderDeclarationO, payload: ShaderPayloadT) {
        self.set_payload.unwrap()(inst, payload)
    }

    pub unsafe fn set_stage_export(
        &self,
        inst: *mut ShaderDeclarationO,
        stage_flag: u32,
        stage_export: ShaderStageExportT,
    ) {
        self.set_stage_export.unwrap()(inst, stage_flag, stage_export)
    }

    pub unsafe fn set_stage_system_semantic_import(
        &self,
        inst: *mut ShaderDeclarationO,
        stage_flag: u32,
        type_: ShaderSystemSemanticType,
    ) {
        self.set_stage_system_semantic_import.unwrap()(inst, stage_flag, type_)
    }

    pub unsafe fn set_stage_system_semantic_export(
        &self,
        inst: *mut ShaderDeclarationO,
        stage_flag: u32,
        type_: ShaderSystemSemanticType,
    ) {
        self.set_stage_system_semantic_export.unwrap()(inst, stage_flag, type_)
    }

    pub unsafe fn set_stage_attribute(
        &self,
        inst: *mut ShaderDeclarationO,
        stage_flag: u32,
        attr: ShaderStageAttributeT,
    ) {
        self.set_stage_attribute.unwrap()(inst, stage_flag, attr)
    }

    pub unsafe fn append_patch_code(
        &self,
        inst: *mut ShaderDeclarationO,
        domain: u32,
        code: *const ::std::os::raw::c_char,
        source_file: *const ::std::os::raw::c_char,
        source_line: u32,
    ) {
        self.append_patch_code.unwrap()(inst, domain, code, source_file, source_line)
    }

    pub unsafe fn append_code(
        &self,
        inst: *mut ShaderDeclarationO,
        stage_flag: u32,
        code: *const ::std::os::raw::c_char,
        source_file: *const ::std::os::raw::c_char,
        source_line: u32,
    ) {
        self.append_code.unwrap()(inst, stage_flag, code, source_file, source_line)
    }

    pub unsafe fn append_common_code(
        &self,
        inst: *mut ShaderDeclarationO,
        code: *const ::std::os::raw::c_char,
        source_file: *const ::std::os::raw::c_char,
        source_line: u32,
    ) {
        self.append_common_code.unwrap()(inst, code, source_file, source_line)
    }
}

impl crate::Api for ShaderDeclarationApi {
    const NAME: ConstCStr = const_cstr!("tm_shader_declaration_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl ShaderSystemApi {
    pub unsafe fn create_context(
        &self,
        allocator: *mut AllocatorI,
        render_graph: *mut RenderGraphO,
    ) -> *mut ShaderSystemContextO {
        self.create_context.unwrap()(allocator, render_graph)
    }

    pub unsafe fn clone_context(
        &self,
        src_context: *const ShaderSystemContextO,
        allocator: *mut AllocatorI,
    ) -> *mut ShaderSystemContextO {
        self.clone_context.unwrap()(src_context, allocator)
    }

    pub unsafe fn destroy_context(&self, context: *mut ShaderSystemContextO) {
        self.destroy_context.unwrap()(context)
    }

    pub unsafe fn set_render_graph(
        &self,
        context: *mut ShaderSystemContextO,
        render_graph: *mut RenderGraphO,
    ) {
        self.set_render_graph.unwrap()(context, render_graph)
    }

    pub unsafe fn activate_system(
        &self,
        context: *mut ShaderSystemContextO,
        system: *mut ShaderSystemO,
        cbuffer_instances: *const ShaderConstantBufferInstanceT,
        num_cbuffer_instances: u32,
        rbinder_instances: *const ShaderResourceBinderInstanceT,
        num_rbinder_instances: u32,
    ) {
        self.activate_system.unwrap()(
            context,
            system,
            cbuffer_instances,
            num_cbuffer_instances,
            rbinder_instances,
            num_rbinder_instances,
        )
    }

    pub unsafe fn deactivate_system(
        &self,
        context: *mut ShaderSystemContextO,
        system: *mut ShaderSystemO,
    ) {
        self.deactivate_system.unwrap()(context, system)
    }
}

impl crate::Api for ShaderSystemApi {
    const NAME: ConstCStr = const_cstr!("tm_shader_system_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl ShaderApi {
    pub unsafe fn create_constant_buffer_instances(
        &self,
        io: *mut ShaderIoO,
        num_instances: u32,
        result: *mut ShaderConstantBufferInstanceT,
    ) {
        self.create_constant_buffer_instances.unwrap()(io, num_instances, result)
    }

    pub unsafe fn create_constant_buffer_instances_from_template(
        &self,
        io: *mut ShaderIoO,
        resource_buffer: *mut RendererResourceCommandBufferO,
        num_instances: u32,
        result: *mut ShaderConstantBufferInstanceT,
        cbuf_template: ShaderConstantBufferInstanceT,
    ) {
        self.create_constant_buffer_instances_from_template.unwrap()(
            io,
            resource_buffer,
            num_instances,
            result,
            cbuf_template,
        )
    }

    pub unsafe fn destroy_constant_buffer_instances(
        &self,
        io: *mut ShaderIoO,
        instances: *mut ShaderConstantBufferInstanceT,
        num_instances: u32,
    ) {
        self.destroy_constant_buffer_instances.unwrap()(io, instances, num_instances)
    }

    pub unsafe fn create_resource_binder_instances(
        &self,
        io: *mut ShaderIoO,
        num_instances: u32,
        result: *mut ShaderResourceBinderInstanceT,
    ) {
        self.create_resource_binder_instances.unwrap()(io, num_instances, result)
    }

    pub unsafe fn destroy_resource_binder_instances(
        &self,
        io: *mut ShaderIoO,
        instances: *mut ShaderResourceBinderInstanceT,
        num_instances: u32,
    ) {
        self.destroy_resource_binder_instances.unwrap()(io, instances, num_instances)
    }

    pub unsafe fn reflect_constants(
        &self,
        io: *mut ShaderIoO,
        num_constants: *mut u32,
        constants: *mut ShaderConstantT,
        constant_offsets: *mut u32,
    ) {
        self.reflect_constants.unwrap()(io, num_constants, constants, constant_offsets)
    }

    pub unsafe fn lookup_constant(
        &self,
        io: *mut ShaderIoO,
        name: StrhashT,
        constant: *mut ShaderConstantT,
        constant_offset: *mut u32,
    ) -> bool {
        self.lookup_constant.unwrap()(io, name, constant, constant_offset)
    }

    pub unsafe fn update_constants(
        &self,
        io: *mut ShaderIoO,
        resource_buffer: *mut RendererResourceCommandBufferO,
        constant_updates: *const ShaderConstantUpdateT,
        num_updates: u32,
    ) {
        self.update_constants.unwrap()(io, resource_buffer, constant_updates, num_updates)
    }

    pub unsafe fn update_constants_raw(
        &self,
        io: *mut ShaderIoO,
        resource_buffer: *mut RendererResourceCommandBufferO,
        instance_ids: *const u32,
        data: *mut *const ::std::os::raw::c_void,
        offset: u32,
        size: u32,
        num_updates: u32,
    ) {
        self.update_constants_raw.unwrap()(
            io,
            resource_buffer,
            instance_ids,
            data,
            offset,
            size,
            num_updates,
        )
    }

    pub unsafe fn update_constant_buffer_instances_from_template(
        &self,
        io: *mut ShaderIoO,
        resource_buffer: *mut RendererResourceCommandBufferO,
        num_instances: u32,
        instances: *mut ShaderConstantBufferInstanceT,
        cbuf_template: ShaderConstantBufferInstanceT,
    ) {
        self.update_constant_buffer_instances_from_template.unwrap()(
            io,
            resource_buffer,
            num_instances,
            instances,
            cbuf_template,
        )
    }

    pub unsafe fn read_constant_buffer(
        &self,
        io: *mut ShaderIoO,
        instance: ShaderConstantBufferInstanceT,
    ) -> *const ::std::os::raw::c_void {
        self.read_constant_buffer.unwrap()(io, instance)
    }

    pub unsafe fn reflect_resources(
        &self,
        io: *mut ShaderIoO,
        num_resources: *mut u32,
        resources: *mut ShaderResourceT,
        resource_slots: *mut u32,
    ) {
        self.reflect_resources.unwrap()(io, num_resources, resources, resource_slots)
    }

    pub unsafe fn lookup_resource(
        &self,
        io: *mut ShaderIoO,
        name: StrhashT,
        resource: *mut ShaderResourceT,
        resource_slot: *mut u32,
    ) -> bool {
        self.lookup_resource.unwrap()(io, name, resource, resource_slot)
    }

    pub unsafe fn update_resources(
        &self,
        io: *mut ShaderIoO,
        resource_buffer: *mut RendererResourceCommandBufferO,
        resource_updates: *const ShaderResourceUpdateT,
        num_updates: u32,
    ) {
        self.update_resources.unwrap()(io, resource_buffer, resource_updates, num_updates)
    }

    pub unsafe fn read_resource_slot(
        &self,
        io: *mut ShaderIoO,
        instance: ShaderResourceBinderInstanceT,
        slot: u32,
        aspects: *mut *const u32,
    ) -> *const RendererHandleT {
        self.read_resource_slot.unwrap()(io, instance, slot, aspects)
    }

    pub unsafe fn max_passes(&self, shader: *mut ShaderO) -> u8 {
        self.max_passes.unwrap()(shader)
    }

    pub unsafe fn assemble_shader_infos(
        &self,
        shader: *mut ShaderO,
        state_override_blocks: *const RendererHandleT,
        num_state_override_blocks: u32,
        context: *const ShaderSystemContextO,
        visibility_context: StrhashT,
        resource_buffer: *mut RendererResourceCommandBufferO,
        cbuf_instances: *const ShaderConstantBufferInstanceT,
        rbinder_instances: *const ShaderResourceBinderInstanceT,
        num_shaders: u32,
        results: *mut RendererShaderInfoT,
    ) -> u8 {
        self.assemble_shader_infos.unwrap()(
            shader,
            state_override_blocks,
            num_state_override_blocks,
            context,
            visibility_context,
            resource_buffer,
            cbuf_instances,
            rbinder_instances,
            num_shaders,
            results,
        )
    }

    pub unsafe fn shader_io(&self, shader: *mut ShaderO) -> *mut ShaderIoO {
        self.shader_io.unwrap()(shader)
    }

    pub unsafe fn system_io(&self, system: *mut ShaderSystemO) -> *mut ShaderIoO {
        self.system_io.unwrap()(system)
    }

    pub unsafe fn name(&self, shader: *const ShaderO) -> StrhashT {
        self.name.unwrap()(shader)
    }

    pub unsafe fn shader_version(&self, shader: *const ShaderO) -> u32 {
        self.shader_version.unwrap()(shader)
    }

    pub unsafe fn system_version(&self, system: *const ShaderSystemO) -> u32 {
        self.system_version.unwrap()(system)
    }
}

impl crate::Api for ShaderApi {
    const NAME: ConstCStr = const_cstr!("tm_shader_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl ShaderRepositoryApi {
    pub unsafe fn create(
        &self,
        system_io: *mut ShaderSystemIoO,
        a: *mut AllocatorI,
        backend: *mut RendererBackendI,
        shader_compiler_api: *mut RendererShaderCompilerApi,
        tt: *mut TheTruthO,
    ) -> *mut ShaderRepositoryO {
        self.create.unwrap()(system_io, a, backend, shader_compiler_api, tt)
    }

    pub unsafe fn destroy(
        &self,
        inst: *mut ShaderRepositoryO,
        resource_buffer: *mut RendererResourceCommandBufferO,
    ) {
        self.destroy.unwrap()(inst, resource_buffer)
    }

    pub unsafe fn set_the_truth(&self, inst: *mut ShaderRepositoryO, tt: *mut TheTruthO) {
        self.set_the_truth.unwrap()(inst, tt)
    }

    pub unsafe fn create_truth_types(&self, tt: *mut TheTruthO) {
        self.create_truth_types.unwrap()(tt)
    }

    pub unsafe fn import_truth_declaration_from_config(
        &self,
        inst: *mut ShaderRepositoryO,
        config: *const ConfigI,
        pi: *const JsonParseInfoT,
        name: StrhashT,
        validity_hash: u64,
        time_stamp: *const FileTimeO,
        filename: *const ::std::os::raw::c_char,
    ) -> TtIdT {
        self.import_truth_declaration_from_config.unwrap()(
            inst,
            config,
            pi,
            name,
            validity_hash,
            time_stamp,
            filename,
        )
    }

    pub unsafe fn destroy_truth_declaration(&self, inst: *mut ShaderRepositoryO, name: StrhashT) {
        self.destroy_truth_declaration.unwrap()(inst, name)
    }

    pub unsafe fn lookup_truth_declaration(
        &self,
        inst: *mut ShaderRepositoryO,
        name: StrhashT,
    ) -> TtIdT {
        self.lookup_truth_declaration.unwrap()(inst, name)
    }

    pub unsafe fn lookup_declaration_validity_hash(
        &self,
        inst: *mut ShaderRepositoryO,
        id: TtIdT,
    ) -> u64 {
        self.lookup_declaration_validity_hash.unwrap()(inst, id)
    }

    pub unsafe fn load_shader_declaration(
        &self,
        inst: *mut ShaderRepositoryO,
        declaration: *mut ShaderDeclarationO,
        id: TtIdT,
    ) {
        self.load_shader_declaration.unwrap()(inst, declaration, id)
    }

    pub unsafe fn last_modified(
        &self,
        inst: *mut ShaderRepositoryO,
        id: TtIdT,
        modified: *mut FileTimeO,
    ) {
        self.last_modified.unwrap()(inst, id, modified)
    }

    pub unsafe fn host_save_state(
        &self,
        inst: *mut ShaderRepositoryO,
        carray: *mut *mut ::std::os::raw::c_char,
        a: *mut AllocatorI,
    ) {
        self.host_save_state.unwrap()(inst, carray, a)
    }

    pub unsafe fn client_load_state(
        &self,
        inst: *mut ShaderRepositoryO,
        tt: *mut TheTruthO,
        state: *mut *const ::std::os::raw::c_char,
    ) {
        self.client_load_state.unwrap()(inst, tt, state)
    }

    pub unsafe fn create_shader_declaration(
        &self,
        inst: *mut ShaderRepositoryO,
        a: *mut AllocatorI,
    ) -> *mut ShaderDeclarationO {
        self.create_shader_declaration.unwrap()(inst, a)
    }

    pub unsafe fn destroy_shader_declaration(
        &self,
        inst: *mut ShaderRepositoryO,
        declaration: *mut ShaderDeclarationO,
    ) {
        self.destroy_shader_declaration.unwrap()(inst, declaration)
    }

    pub unsafe fn generate_system_includes(
        &self,
        inst: *mut ShaderRepositoryO,
        output_path: *const ::std::os::raw::c_char,
    ) {
        self.generate_system_includes.unwrap()(inst, output_path)
    }

    pub unsafe fn update_shaders_from_directory(
        &self,
        inst: *mut ShaderRepositoryO,
        dir: *const ::std::os::raw::c_char,
        recursive: bool,
        allocator: *mut AllocatorI,
        res_buf: *mut RendererResourceCommandBufferO,
    ) {
        self.update_shaders_from_directory.unwrap()(inst, dir, recursive, allocator, res_buf)
    }

    pub unsafe fn allocator(&self, inst: *mut ShaderRepositoryO) -> *mut AllocatorI {
        self.allocator.unwrap()(inst)
    }

    pub unsafe fn refresh_truth_shaders(&self, inst: *mut ShaderRepositoryO) {
        self.refresh_truth_shaders.unwrap()(inst)
    }

    pub unsafe fn lookup_or_create_shader(
        &self,
        inst: *mut ShaderRepositoryO,
        name: StrhashT,
    ) -> *mut ShaderO {
        self.lookup_or_create_shader.unwrap()(inst, name)
    }

    pub unsafe fn lookup_or_create_system(
        &self,
        inst: *mut ShaderRepositoryO,
        name: StrhashT,
    ) -> *mut ShaderSystemO {
        self.lookup_or_create_system.unwrap()(inst, name)
    }

    pub unsafe fn create_shader_from_blob(
        &self,
        inst: *mut ShaderRepositoryO,
        res_buf: *mut RendererResourceCommandBufferO,
        shader_blob: *const ShaderBlobHeaderT,
    ) -> *mut ShaderO {
        self.create_shader_from_blob.unwrap()(inst, res_buf, shader_blob)
    }

    pub unsafe fn compile_shader(
        &self,
        inst: *mut ShaderRepositoryO,
        res_buf: *mut RendererResourceCommandBufferO,
        shader: *mut ShaderO,
        active_shader_stages: u32,
        active_systems_bitmask: u64,
        active_system_names: *const StrhashT,
        active_systems: *mut *const ShaderDeclarationO,
        num_active_systems: u32,
        declarations: *mut *const ShaderDeclarationO,
        num_declarations: u32,
        validity_hash: u64,
    ) -> bool {
        self.compile_shader.unwrap()(
            inst,
            res_buf,
            shader,
            active_shader_stages,
            active_systems_bitmask,
            active_system_names,
            active_systems,
            num_active_systems,
            declarations,
            num_declarations,
            validity_hash,
        )
    }

    pub unsafe fn create_from_declaration(
        &self,
        inst: *mut ShaderRepositoryO,
        res_buf: *mut RendererResourceCommandBufferO,
        declaration: TtIdT,
        name: StrhashT,
        generated_code: *mut ShaderDeclarationO,
        creation_graph_data: *const ShaderCreationGraphDataO,
    ) -> bool {
        self.create_from_declaration.unwrap()(
            inst,
            res_buf,
            declaration,
            name,
            generated_code,
            creation_graph_data,
        )
    }

    pub unsafe fn background_create_from_declaration(
        &self,
        inst: *mut ShaderRepositoryO,
        declaration: TtIdT,
        name: StrhashT,
        generated_code: *mut ShaderDeclarationO,
        creation_graph_data: *const ShaderCreationGraphDataO,
        callback: *const ShaderSystemBackgroundCreateI,
    ) {
        self.background_create_from_declaration.unwrap()(
            inst,
            declaration,
            name,
            generated_code,
            creation_graph_data,
            callback,
        )
    }

    pub unsafe fn destroy_shader(&self, inst: *mut ShaderRepositoryO, name: StrhashT) {
        self.destroy_shader.unwrap()(inst, name)
    }

    pub unsafe fn destroy_system(&self, inst: *mut ShaderRepositoryO, name: StrhashT) {
        self.destroy_system.unwrap()(inst, name)
    }

    pub unsafe fn lookup_shader(
        &self,
        inst: *mut ShaderRepositoryO,
        name: StrhashT,
    ) -> *mut ShaderO {
        self.lookup_shader.unwrap()(inst, name)
    }

    pub unsafe fn lookup_system(
        &self,
        inst: *mut ShaderRepositoryO,
        name: StrhashT,
    ) -> *mut ShaderSystemO {
        self.lookup_system.unwrap()(inst, name)
    }

    pub unsafe fn recycle_resources(&self, inst: *mut ShaderRepositoryO) {
        self.recycle_resources.unwrap()(inst)
    }

    pub unsafe fn shader_state_override(
        &self,
        inst: *mut ShaderRepositoryO,
        state_override: ShaderStateOverrides,
    ) -> RendererHandleT {
        self.shader_state_override.unwrap()(inst, state_override)
    }

    pub unsafe fn lookup_shader_blob(
        &self,
        inst: *mut ShaderRepositoryO,
        name: StrhashT,
        size: *mut u64,
    ) -> *const ShaderBlobHeaderT {
        self.lookup_shader_blob.unwrap()(inst, name, size)
    }
}

impl crate::Api for ShaderRepositoryApi {
    const NAME: ConstCStr = const_cstr!("tm_shader_repository_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl ShaderCreationGraphApi {
    pub unsafe fn create_truth_types(&self, tt: *mut TheTruthO) {
        self.create_truth_types.unwrap()(tt)
    }

    pub unsafe fn compile_data_to_wire(
        &self,
        inst: *mut CreationGraphInstanceT,
        wire: u32,
        tt: *const TheTruthO,
        data_id: TtIdT,
        to_type_hash: StrhashT,
    ) -> bool {
        self.compile_data_to_wire.unwrap()(inst, wire, tt, data_id, to_type_hash)
    }

    pub unsafe fn node_from_declaration(
        &self,
        tt: *mut TheTruthO,
        declaration: TtIdT,
        node: *mut CreationGraphNodeTypeI,
        a: *mut AllocatorI,
    ) {
        self.node_from_declaration.unwrap()(tt, declaration, node, a)
    }

    pub unsafe fn register_static_graph_nodes(&self, reg: *mut ApiRegistryApi, load: bool) {
        self.register_static_graph_nodes.unwrap()(reg, load)
    }

    pub unsafe fn wire_result_as_string(
        &self,
        tt: *const TheTruthO,
        ta: *mut TempAllocatorI,
        cw: *const CreationGraphInterpreterWireContentT,
        requested_type: *const ConnectorTypeT,
    ) -> *const ::std::os::raw::c_char {
        self.wire_result_as_string.unwrap()(tt, ta, cw, requested_type)
    }
}

impl crate::Api for ShaderCreationGraphApi {
    const NAME: ConstCStr = const_cstr!("tm_shader_creation_graph_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

pub const TM_CREATION_GRAPH_UPDATE_SHADER_CONSTANT_AND_RESOURCES: StrhashT = StrhashT {
    u64_: 13541595712514035875u64,
};
pub const TM_CREATION_GRAPH_UPDATE_SHADER_INSTANCE: StrhashT = StrhashT {
    u64_: 11170773395381692996u64,
};
pub const TM_TT_ASPECT__SHADER_SYSTEM_DATA_DRIVEN_SETTINGS: StrhashT = StrhashT {
    u64_: 15992631312706212254u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_REPOSITORY: StrhashT = StrhashT {
    u64_: 15185502537365699060u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_STAGE_DECLARATION: StrhashT = StrhashT {
    u64_: 6367897737959879055u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_DECLARATION: StrhashT = StrhashT {
    u64_: 10773388807908132640u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_RENDER_STATE_BLOCK: StrhashT = StrhashT {
    u64_: 12194628297937593064u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_SAMPLER_STATE_BLOCK: StrhashT = StrhashT {
    u64_: 18206817971852099530u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_REQUEST: StrhashT = StrhashT {
    u64_: 3031978811566371918u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_CONSTANT: StrhashT = StrhashT {
    u64_: 15497721740042733126u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_RESOURCE: StrhashT = StrhashT {
    u64_: 6063637962741852513u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_PAYLOAD: StrhashT = StrhashT {
    u64_: 3143131508224954804u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_EXPORT: StrhashT = StrhashT {
    u64_: 1638888224759395780u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_INCLUDE: StrhashT = StrhashT {
    u64_: 1810078001569239366u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_VARIATION: StrhashT = StrhashT {
    u64_: 1487435679566144764u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_VARIATION_SYSTEM: StrhashT = StrhashT {
    u64_: 14511874305837397087u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_COMPILE_CONFIGURATION: StrhashT = StrhashT {
    u64_: 6807065315068906446u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_COMPILE_BRANCH: StrhashT = StrhashT {
    u64_: 10255443381759388015u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_PASS_CONDITION: StrhashT = StrhashT {
    u64_: 18069784956397400067u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_PASS: StrhashT = StrhashT {
    u64_: 11701977933788228641u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_COMPILE_CONTEXT: StrhashT = StrhashT {
    u64_: 16696519998799356633u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_COMPILED_RESULT: StrhashT = StrhashT {
    u64_: 10133004307291133025u64,
};
pub const TM_TT_TYPE_HASH___SHADER_SYSTEM__CONSTANT_NODE__SETTINGS: StrhashT = StrhashT {
    u64_: 2923102057256778094u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_CREATION_GRAPH_NODE_RESOURCE_ASPECT: StrhashT = StrhashT {
    u64_: 13030588925502890531u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_CREATION_GRAPH_NODE_DEFINE: StrhashT = StrhashT {
    u64_: 3619968606120579393u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_CREATION_GRAPH_NODE_ACTION: StrhashT = StrhashT {
    u64_: 9259415262580364095u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_CREATION_GRAPH_NODE_CONNECTOR_SETTING_OPTION: StrhashT =
    StrhashT {
        u64_: 4376663816688172497u64,
    };
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_CREATION_GRAPH_NODE_CONNECTOR_SETTING: StrhashT =
    StrhashT {
        u64_: 1414700146970154157u64,
    };
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_CREATION_GRAPH_NODE_CONNECTOR_SETTINGS: StrhashT =
    StrhashT {
        u64_: 7558674430220200475u64,
    };
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_CREATION_GRAPH_NODE_CONNECTOR: StrhashT = StrhashT {
    u64_: 584995607720450482u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_CREATION_GRAPH_NODE_CONNECTOR_TYPE_OF: StrhashT =
    StrhashT {
        u64_: 15480336716737488975u64,
    };
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_CREATION_GRAPH_NODE: StrhashT = StrhashT {
    u64_: 15834386638400285621u64,
};
pub const TM_TT_TYPE_HASH__SHADER_SYSTEM_CREATION_GRAPH_NODE_EVALUATION_CONTEXT_NAME: StrhashT =
    StrhashT {
        u64_: 1601229260840685503u64,
    };
pub const TM_SHADER_SYSTEM_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_SHADER_DECLARATION_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_SHADER_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_SHADER_CREATION_GRAPH_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_SHADER_REPOSITORY_O_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_SHADER_REPOSITORY_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
