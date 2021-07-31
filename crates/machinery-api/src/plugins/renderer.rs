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
pub const TM_DEVICE_MEMORY_VIEW_API_NAME: &'static [u8; 22usize] = b"tm_device_memory_view\0";
pub const TM_NIL_RENDER_BACKEND_API_NAME: &'static [u8; 26usize] = b"tm_nil_render_backend_api\0";
pub const TM_RENDERER_API_NAME: &'static [u8; 16usize] = b"tm_renderer_api\0";
pub const TM_RENDERER_INIT_API_NAME: &'static [u8; 21usize] = b"tm_renderer_init_api\0";
pub const TM_RENDERER_MEMORY_STATISTICS_MAX_ALLOCATOR_BLOCKS: u32 = 256;
pub const TM_RENDERER_DEVICE_AFFINITY_MASK_ALL: u32 = 255;
pub const TM_RENDER_BACKEND_INTERFACE_NAME: &'static [u8; 20usize] = b"tm_render_backend_i\0";
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
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererHandleT {
    pub resource: u32,
    pub bindless_srv: u32,
    pub bindless_uav: u32,
}
pub const TM_RENDERER_SHADER_STAGE_VERTEX: RendererShaderStage = 0;
pub const TM_RENDERER_SHADER_STAGE_HULL: RendererShaderStage = 1;
pub const TM_RENDERER_SHADER_STAGE_DOMAIN: RendererShaderStage = 2;
pub const TM_RENDERER_SHADER_STAGE_GEOMETRY: RendererShaderStage = 3;
pub const TM_RENDERER_SHADER_STAGE_PIXEL: RendererShaderStage = 4;
pub const TM_RENDERER_SHADER_STAGE_COMPUTE: RendererShaderStage = 5;
pub const TM_RENDERER_SHADER_STAGE_RAYGEN: RendererShaderStage = 6;
pub const TM_RENDERER_SHADER_STAGE_ANY_HIT: RendererShaderStage = 7;
pub const TM_RENDERER_SHADER_STAGE_CLOSEST_HIT: RendererShaderStage = 8;
pub const TM_RENDERER_SHADER_STAGE_MISS: RendererShaderStage = 9;
pub const TM_RENDERER_SHADER_STAGE_INTERSECTION: RendererShaderStage = 10;
pub const TM_RENDERER_SHADER_STAGE_MAX: RendererShaderStage = 11;
pub type RendererShaderStage = ::std::os::raw::c_int;
pub const TM_RENDERER_STATE_BLOCK_TYPE_TESSELLATION: RendererStateBlockType = 0;
pub const TM_RENDERER_STATE_BLOCK_TYPE_RASTER: RendererStateBlockType = 1;
pub const TM_RENDERER_STATE_BLOCK_TYPE_DEPTH_STENCIL: RendererStateBlockType = 2;
pub const TM_RENDERER_STATE_BLOCK_TYPE_TEXTURE_SAMPLER: RendererStateBlockType = 3;
pub const TM_RENDERER_STATE_BLOCK_TYPE_RENDER_TARGET_BLEND: RendererStateBlockType = 4;
pub const TM_RENDERER_STATE_BLOCK_TYPE_BLEND: RendererStateBlockType = 5;
pub const TM_RENDERER_STATE_BLOCK_TYPE_MULTI_SAMPLE: RendererStateBlockType = 6;
pub const TM_RENDERER_STATE_BLOCK_TYPE_MAX_STATE_BLOCK_TYPES: RendererStateBlockType = 7;
pub type RendererStateBlockType = ::std::os::raw::c_int;
pub const TM_RENDERER_STATE_VALUE_TYPE_BOOL: RendererStateValueType = 0;
pub const TM_RENDERER_STATE_VALUE_TYPE_UINT32: RendererStateValueType = 1;
pub const TM_RENDERER_STATE_VALUE_TYPE_FLOAT32: RendererStateValueType = 2;
pub const TM_RENDERER_STATE_VALUE_TYPE_COMPARE_OP: RendererStateValueType = 3;
pub const TM_RENDERER_STATE_VALUE_TYPE_CULL: RendererStateValueType = 4;
pub const TM_RENDERER_STATE_VALUE_TYPE_FRONT_FACE: RendererStateValueType = 5;
pub const TM_RENDERER_STATE_VALUE_TYPE_POLYGON_MODE: RendererStateValueType = 6;
pub const TM_RENDERER_STATE_VALUE_TYPE_STENCIL_OP: RendererStateValueType = 7;
pub const TM_RENDERER_STATE_VALUE_TYPE_FILTER: RendererStateValueType = 8;
pub const TM_RENDERER_STATE_VALUE_TYPE_MIP_MODE: RendererStateValueType = 9;
pub const TM_RENDERER_STATE_VALUE_TYPE_ADDRESS_MODE: RendererStateValueType = 10;
pub const TM_RENDERER_STATE_VALUE_TYPE_BORDER_COLOR: RendererStateValueType = 11;
pub const TM_RENDERER_STATE_VALUE_TYPE_BLEND_FACTOR: RendererStateValueType = 12;
pub const TM_RENDERER_STATE_VALUE_TYPE_BLEND_OPERATION: RendererStateValueType = 13;
pub const TM_RENDERER_STATE_VALUE_TYPE_BLEND_WRITE_MASK: RendererStateValueType = 14;
pub const TM_RENDERER_STATE_VALUE_TYPE_LOGICAL_OPERATION: RendererStateValueType = 15;
pub const TM_RENDERER_STATE_VALUE_TYPE_STATE_BLOCK: RendererStateValueType = -16777216;
pub const TM_RENDERER_STATE_VALUE_TYPE_MAX_VALUE_TYPES: RendererStateValueType = -16777215;
pub type RendererStateValueType = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererShaderBlobT {
    pub size: u64,
    pub data: *mut u8,
}
impl Default for RendererShaderBlobT {
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
pub struct RendererImageSubresourceLayersT {
    pub first_layer: u16,
    pub layer_count: u16,
    pub first_mip: u16,
    pub mip_levels: u16,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererImageOffsetT {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererImageExtentT {
    pub width: u16,
    pub height: u16,
    pub depth: u16,
}
pub const TM_RENDERER_QUEUE_GRAPHICS: RendererQueue = 0;
pub const TM_RENDERER_QUEUE_COMPUTE: RendererQueue = 1;
pub const TM_RENDERER_QUEUE_TRANSFER: RendererQueue = 2;
pub type RendererQueue = ::std::os::raw::c_int;
pub const TM_RENDERER_RESOURCE_LOAD_OP_LOAD: RendererResourceLoadOp = 0;
pub const TM_RENDERER_RESOURCE_LOAD_OP_CLEAR: RendererResourceLoadOp = 1;
pub const TM_RENDERER_RESOURCE_LOAD_OP_DISCARD: RendererResourceLoadOp = 2;
pub type RendererResourceLoadOp = ::std::os::raw::c_int;
pub const TM_RENDERER_RESOURCE_STORE_OP_STORE: RendererResourceStoreOp = 0;
pub const TM_RENDERER_RESOURCE_STORE_OP_DISCARD: RendererResourceStoreOp = 1;
pub type RendererResourceStoreOp = ::std::os::raw::c_int;
pub const TM_RENDERER_RESOURCE_STATE_VERTEX_INPUT: RendererResourceState = 1;
pub const TM_RENDERER_RESOURCE_STATE_RENDER_TARGET: RendererResourceState = 2;
pub const TM_RENDERER_RESOURCE_STATE_UAV: RendererResourceState = 4;
pub const TM_RENDERER_RESOURCE_STATE_RESOURCE: RendererResourceState = 8;
pub const TM_RENDERER_RESOURCE_STATE_VERTEX_SHADER: RendererResourceState = 16;
pub const TM_RENDERER_RESOURCE_STATE_HULL_SHADER: RendererResourceState = 32;
pub const TM_RENDERER_RESOURCE_STATE_DOMAIN_SHADER: RendererResourceState = 64;
pub const TM_RENDERER_RESOURCE_STATE_GEOMETRY_SHADER: RendererResourceState = 128;
pub const TM_RENDERER_RESOURCE_STATE_PIXEL_SHADER: RendererResourceState = 256;
pub const TM_RENDERER_RESOURCE_STATE_COMPUTE_SHADER: RendererResourceState = 512;
pub const TM_RENDERER_RESOURCE_STATE_RAY_TRACING_SHADER: RendererResourceState = 1024;
pub const TM_RENDERER_RESOURCE_STATE_COPY_SOURCE: RendererResourceState = 2048;
pub const TM_RENDERER_RESOURCE_STATE_COPY_DESTINATION: RendererResourceState = 4096;
pub const TM_RENDERER_RESOURCE_STATE_INDIRECT_ARGUMENT: RendererResourceState = 8192;
pub const TM_RENDERER_RESOURCE_STATE_PRESENT: RendererResourceState = 16384;
pub type RendererResourceState = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererCommandBuffer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererRenderPassBindRenderTargetT {
    pub resource: RendererHandleT,
    pub aspect: u32,
    pub initial_state: u16,
    pub final_state: u16,
    pub load_op: u8,
    pub store_op: u8,
    pub stencil_load_op: u8,
    pub stencil_store_op: u8,
}
pub const TM_RENDERER_MAX_RENDER_TARGETS: ::std::os::raw::c_int = 8;
pub type _bindgen_ty_1 = ::std::os::raw::c_int;
pub const TM_RENDERER_MAX_QUEUE_FENCES: ::std::os::raw::c_int = 4;
pub type _bindgen_ty_2 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererSchedulingT {
    pub wait_queue_fences: [RendererHandleT; 4usize],
    pub wait_device_affinity_masks: [u32; 4usize],
    pub num_wait_fences: u32,
    pub _padding_126: [::std::os::raw::c_char; 4usize],
    pub signal_queue_fence: RendererHandleT,
    pub signal_device_affinity_mask: u32,
    pub _padding_133: [::std::os::raw::c_char; 4usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererRenderPassBindT {
    pub device_affinity_mask: u32,
    pub _padding_141: [::std::os::raw::c_char; 4usize],
    pub render_targets: [RendererRenderPassBindRenderTargetT; 8usize],
    pub depth_stencil_target: RendererRenderPassBindRenderTargetT,
    pub scheduling: RendererSchedulingT,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererQueueBindT {
    pub device_affinity_mask: u32,
    pub queue_family: u8,
    pub queue_index: u8,
    pub _padding_159: [::std::os::raw::c_char; 2usize],
    pub scheduling: RendererSchedulingT,
}
pub const TM_RENDERER_MAX_VIEWPORTS_SCISSOR_RECTS: ::std::os::raw::c_int = 8;
pub type _bindgen_ty_3 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererSetViewportsT {
    pub viewports: [RendererSetViewportsTBindgenTy1; 8usize],
    pub num_viewports: u32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererSetViewportsTBindgenTy1 {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererSetScissorRectsT {
    pub rects: [RendererSetScissorRectsTBindgenTy1; 8usize],
    pub num_rects: u32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererSetScissorRectsTBindgenTy1 {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererBeginStatisticsT {
    pub category: *const ::std::os::raw::c_char,
    pub name: *const ::std::os::raw::c_char,
    pub flags: u32,
    pub _padding_202: [::std::os::raw::c_char; 4usize],
    pub uid: u64,
    pub backend_data: [u32; 8usize],
}
impl Default for RendererBeginStatisticsT {
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
pub struct RendererEndStatisticsT {
    pub uid: u64,
    pub backend_data: [u32; 8usize],
}
pub const TM_RENDERER_DISPATCH_TYPE_NORMAL: RendererDispatchType = 0;
pub const TM_RENDERER_DISPATCH_TYPE_INDIRECT: RendererDispatchType = 1;
pub type RendererDispatchType = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererDispatchCommandT {
    pub group_count: [u32; 3usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererDispatchIndirectCommandT {
    pub indirect_buffer: RendererHandleT,
    pub _padding_231: [::std::os::raw::c_char; 4usize],
    pub offset: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererComputeInfoT {
    pub dispatch_type: u8,
    pub _padding_240: [::std::os::raw::c_char; 7usize],
    pub __bindgen_anon_1: RendererComputeInfoTBindgenTy1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union RendererComputeInfoTBindgenTy1 {
    pub dispatch: RendererDispatchCommandT,
    pub indirect: RendererDispatchIndirectCommandT,
}
impl Default for RendererComputeInfoTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for RendererComputeInfoT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_RENDERER_PRIMITIVE_TYPE_TRIANGLE_LIST: RendererPrimitiveType = 0;
pub const TM_RENDERER_PRIMITIVE_TYPE_LINE_LIST: RendererPrimitiveType = 1;
pub const TM_RENDERER_PRIMITIVE_TYPE_POINT_LIST: RendererPrimitiveType = 2;
pub const TM_RENDERER_PRIMITIVE_TYPE_MAX_TYPES: RendererPrimitiveType = 3;
pub type RendererPrimitiveType = ::std::os::raw::c_int;
pub const TM_RENDERER_DRAW_TYPE_NON_INDEXED: RendererDrawType = 0;
pub const TM_RENDERER_DRAW_TYPE_INDEXED: RendererDrawType = 1;
pub const TM_RENDERER_DRAW_TYPE_NON_INDEXED_INDIRECT: RendererDrawType = 2;
pub const TM_RENDERER_DRAW_TYPE_INDEXED_INDIRECT: RendererDrawType = 3;
pub type RendererDrawType = ::std::os::raw::c_int;
pub const TM_RENDERER_INDEX_TYPE_UINT16: RendererIndexType = 0;
pub const TM_RENDERER_INDEX_TYPE_UINT32: RendererIndexType = 1;
pub type RendererIndexType = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererDrawCommandT {
    pub num_vertices: u32,
    pub num_instances: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererDrawIndexedCommandT {
    pub num_indices: u32,
    pub num_instances: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererDrawIndirectCommandT {
    pub indirect_buffer: RendererHandleT,
    pub _padding_314: [::std::os::raw::c_char; 4usize],
    pub offset: u64,
    pub stride: u32,
    pub num_draws: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererDrawCallInfoT {
    pub __bindgen_anon_1: RendererDrawCallInfoTBindgenTy1,
    pub index_buffer: RendererHandleT,
    pub primitive_type: u8,
    pub draw_type: u8,
    pub index_type: u8,
    pub _padding_345: [::std::os::raw::c_char; 1usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union RendererDrawCallInfoTBindgenTy1 {
    pub non_indexed: RendererDrawCommandT,
    pub indexed: RendererDrawIndexedCommandT,
    pub indirect: RendererDrawIndirectCommandT,
}
impl Default for RendererDrawCallInfoTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for RendererDrawCallInfoT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_RENDERER_MAX_RESOURCE_BINDERS: ::std::os::raw::c_int = 8;
pub const TM_RENDERER_MAX_PUSH_CONSTANTS: ::std::os::raw::c_int = 16;
pub const TM_RENDERER_MAX_SHADER_STATE_OVERRIDE_BLOCKS: ::std::os::raw::c_int = 4;
pub type _bindgen_ty_4 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererShaderStateOverrideBlocksT {
    pub num_override_blocks: u8,
    pub _padding_361: [::std::os::raw::c_char; 7usize],
    pub shader_state_override_blocks: [RendererHandleT; 4usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererShaderResourceBindersT {
    pub num_resource_binders: u8,
    pub _padding_370: [::std::os::raw::c_char; 7usize],
    pub resource_binders: [RendererHandleT; 8usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererShaderPushConstantsT {
    pub num_constants: u8,
    pub _padding_379: [::std::os::raw::c_char; 3usize],
    pub data: [u32; 16usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererShaderInfoT {
    pub shader: RendererHandleT,
    pub state_override_blocks: RendererShaderStateOverrideBlocksT,
    pub binders: RendererShaderResourceBindersT,
    pub constants: RendererShaderPushConstantsT,
    pub sort_key: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererShaderHeaderT {
    pub shader: RendererHandleT,
    pub num_state_override_blocks: u16,
    pub num_resource_binders: u16,
    pub num_bytes_push_constants: u16,
    pub _padding_406: [::std::os::raw::c_char; 2usize],
}
pub const TM_RENDERER_IMAGE_ASPECT_DEFAULT: RendererImageAspect = 0;
pub const TM_RENDERER_IMAGE_ASPECT_SRGB: RendererImageAspect = 1;
pub const TM_RENDERER_IMAGE_ASPECT_DEPTH: RendererImageAspect = 2;
pub const TM_RENDERER_IMAGE_ASPECT_STENCIL: RendererImageAspect = 3;
pub const TM_RENDERER_IMAGE_ASPECT_ADDITONAL_VIEW: RendererImageAspect = 134217728;
pub type RendererImageAspect = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererSetResourceT {
    pub bind_point: u32,
    pub _padding_434: [::std::os::raw::c_char; 4usize],
    pub resource_handle: RendererHandleT,
    pub resource_aspect_flag: u32,
    pub _padding_439: [::std::os::raw::c_char; 4usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererHandleAspectPairT {
    pub handle: RendererHandleT,
    pub aspect: u32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererSetArrayResourcesT {
    pub bind_point: u32,
    pub first_element: u32,
    pub n_handles: u32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererUpdateBufferCommandT {
    pub offset: u64,
    pub size: u64,
    pub device_affinity_mask: u32,
    pub _padding_468: [::std::os::raw::c_char; 4usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererFillBufferCommandT {
    pub offset: u64,
    pub size: u64,
    pub data: u32,
    pub device_affinity_mask: u32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererUpdateImageCommandT {
    pub resource_state: u16,
    pub subresource: RendererImageSubresourceLayersT,
    pub offset: RendererImageOffsetT,
    pub extent: RendererImageExtentT,
    pub _padding_490: [::std::os::raw::c_char; 2usize],
    pub size: u64,
    pub device_affinity_mask: u32,
    pub _padding_495: [::std::os::raw::c_char; 4usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererResourceBarrierHeaderT {
    pub num_barriers: u32,
}
pub const TM_RENDERER_RESOURCE_BARRIER_FLAG_IMAGE_ASPECT_STENCIL: RendererResourceBarrierFlag = 1;
pub const TM_RENDERER_RESOURCE_BARRIER_FLAG_SUBRESOURCE: RendererResourceBarrierFlag = 2;
pub type RendererResourceBarrierFlag = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererResourceBarrierT {
    pub resource_handle: RendererHandleT,
    pub source_state: u16,
    pub destination_state: u16,
    pub source_queue: u16,
    pub destination_queue: u16,
    pub barrier_flags: u16,
    pub _padding_527: [::std::os::raw::c_char; 2usize],
    pub __bindgen_anon_1: RendererResourceBarrierTBindgenTy1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union RendererResourceBarrierTBindgenTy1 {
    pub subresource_image: RendererResourceBarrierTBindgenTy1BindgenTy1,
    pub subresource_buffer: RendererResourceBarrierTBindgenTy1BindgenTy2,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererResourceBarrierTBindgenTy1BindgenTy1 {
    pub first_layer: u16,
    pub layer_count: u16,
    pub first_mip_level: u16,
    pub mip_count: u16,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererResourceBarrierTBindgenTy1BindgenTy2 {
    pub offset: u64,
    pub size: u64,
}
impl Default for RendererResourceBarrierTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for RendererResourceBarrierT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_RENDERER_COPY_IMAGE_FLAG_IMAGE_ASPECT_STENCIL: RendererCopyImageFlags = 1;
pub const TM_RENDERER_COPY_IMAGE_FLAG_SUBRESOURCE: RendererCopyImageFlags = 2;
pub type RendererCopyImageFlags = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererCopyImageT {
    pub source_resource: RendererHandleT,
    pub destination_resource: RendererHandleT,
    pub copy_flags: u16,
    pub src_subresource_layers: RendererImageSubresourceLayersT,
    pub src_offset: RendererImageOffsetT,
    pub dst_subresource_layers: RendererImageSubresourceLayersT,
    pub dst_offset: RendererImageOffsetT,
    pub extent: RendererImageExtentT,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererCopyBufferT {
    pub source_resource: RendererHandleT,
    pub destination_resource: RendererHandleT,
    pub source_offset: u64,
    pub destination_offset: u64,
    pub size: u64,
}
pub const TM_RENDERER_READ_IMAGE_FLAG_IMAGE_ASPECT_STENCIL: RendererReadImageFlags = 1;
pub const TM_RENDERER_READ_IMAGE_FLAG_SUBRESOURCE: RendererReadImageFlags = 2;
pub type RendererReadImageFlags = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererReadImageT {
    pub device_affinity_mask: u32,
    pub _padding_606: [::std::os::raw::c_char; 4usize],
    pub resource_handle: RendererHandleT,
    pub resource_state: u16,
    pub resource_queue: u16,
    pub read_flags: u16,
    pub subresource_layers: RendererImageSubresourceLayersT,
    pub offset: RendererImageOffsetT,
    pub extent: RendererImageExtentT,
    pub _padding_619: [::std::os::raw::c_char; 2usize],
    pub bits: *mut ::std::os::raw::c_void,
    pub size_bits: u64,
}
impl Default for RendererReadImageT {
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
pub struct RendererReadBufferT {
    pub device_affinity_mask: u32,
    pub _padding_631: [::std::os::raw::c_char; 4usize],
    pub resource_handle: RendererHandleT,
    pub resource_state: u16,
    pub resource_queue: u16,
    pub offset: u64,
    pub size: u64,
    pub bits: *mut ::std::os::raw::c_void,
}
impl Default for RendererReadBufferT {
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
pub struct RendererResizeBufferT {
    pub bindless_srv: u32,
    pub bindless_uav: u32,
    pub size: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererTraceCallT {
    pub pipeline: RendererHandleT,
    pub raygen_sbt: RendererHandleT,
    pub miss_sbt: RendererHandleT,
    pub hit_sbt: RendererHandleT,
    pub group_count: [u32; 3usize],
    pub raygen_index: u32,
    pub binders: RendererShaderResourceBindersT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceMemoryViewO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct DeviceMemoryViewApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(allocator: *mut AllocatorI) -> *mut DeviceMemoryViewO,
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(view: *mut DeviceMemoryViewO)>,
    pub ui: ::std::option::Option<
        unsafe extern "C" fn(
            view: *mut DeviceMemoryViewO,
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            content_r: RectT,
            tab_id: u64,
        ),
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct NilRendererBackendApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(allocator: *mut AllocatorI) -> *mut RendererBackendI,
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(backend: *mut RendererBackendI)>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererApi {
    pub tm_renderer_memory_block_pool_o: *mut RendererMemoryBlockPoolO,
    pub tm_renderer_command_buffer_pool_o: *mut RendererCommandBufferPoolO,
    pub tm_renderer_resource_command_buffer_pool_o: *mut RendererResourceCommandBufferPoolO,
    pub tm_renderer_command_buffer_pool_api: *mut RendererCommandBufferPoolApi,
    pub tm_renderer_command_buffer_api: *mut RendererCommandBufferApi,
    pub tm_renderer_command_buffer_sort_api: *mut RendererCommandBufferSortApi,
    pub tm_renderer_resource_command_buffer_pool_api: *mut RendererResourceCommandBufferPoolApi,
    pub tm_renderer_resource_command_buffer_api: *mut RendererResourceCommandBufferApi,
}
impl Default for RendererApi {
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
pub struct RendererInitApi {
    pub init: ::std::option::Option<
        unsafe extern "C" fn(allocator: *mut AllocatorI, user_data_size: u64) -> *mut RendererApi,
    >,
    pub shutdown: ::std::option::Option<unsafe extern "C" fn()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WindowPlatformDataO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererCommandBufferO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererResourceCommandBufferO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererMemoryStatisticsAllocationT {
    pub tag: *const ::std::os::raw::c_char,
    pub offset: u64,
    pub requested_size: u64,
    pub aligned_size: u64,
    pub allocated_size: u64,
}
impl Default for RendererMemoryStatisticsAllocationT {
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
pub struct RendererMemoryStatisticsAllocatorT {
    pub name: *const ::std::os::raw::c_char,
    pub num_blocks: u32,
    pub _padding_39: [::std::os::raw::c_char; 4usize],
    pub block_size: [u64; 256usize],
}
impl Default for RendererMemoryStatisticsAllocatorT {
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
pub struct RendererSwapChainO {
    pub opaque: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererDeviceMemoryO {
    pub opaque: [u64; 8usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererBackendResourceAllocatorI {
    pub inst: *mut ::std::os::raw::c_void,
    pub command_buffer_size_buffer: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, device_affinity_mask: u32) -> u32,
    >,
    pub command_buffer_size_image: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, device_affinity_mask: u32) -> u32,
    >,
    pub command_buffer_size_image_views: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, device_affinity_mask: u32) -> u32,
    >,
    pub command_buffer_size_shader: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, device_affinity_mask: u32) -> u32,
    >,
    pub command_buffer_size_shader_state_override: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, device_affinity_mask: u32) -> u32,
    >,
    pub command_buffer_size_sampler: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, device_affinity_mask: u32) -> u32,
    >,
    pub command_buffer_size_queue_fence: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, device_affinity_mask: u32) -> u32,
    >,
    pub command_buffer_size_resource_binder: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            num_bind_points: u32,
            device_affinity_mask: u32,
        ) -> u32,
    >,
    pub command_buffer_size_ray_tracing_pipeline: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, device_affinity_mask: u32) -> u32,
    >,
    pub command_buffer_size_acceleration_structure: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, device_affinity_mask: u32) -> u32,
    >,
    pub create_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            dest: *mut ::std::os::raw::c_void,
            buffer: *const RendererBufferDescT,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_image: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            dest: *mut ::std::os::raw::c_void,
            image: *const RendererImageDescT,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_image_views: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            dest: *mut ::std::os::raw::c_void,
            image: RendererHandleT,
            desc: *const RendererImageDescT,
            views: *const RendererImageViewT,
            num_views: u32,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_shader: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            dest: *mut ::std::os::raw::c_void,
            shader: *const RendererShaderT,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_shader_state_override: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            dest: *mut ::std::os::raw::c_void,
            state_override: *const RendererShaderStateOverrideT,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_sampler: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            dest: *mut ::std::os::raw::c_void,
            sampler: *const RendererSamplerT,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_queue_fence: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            dest: *mut ::std::os::raw::c_void,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_resource_binder: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            dest: *mut ::std::os::raw::c_void,
            bind_points: *const RendererResourceBindPointT,
            num_bind_points: u32,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_ray_tracing_pipeline: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            dest: *mut ::std::os::raw::c_void,
            pipeline: *const RendererRayTracingPipelineDescT,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_shader_binding_table: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            dest: *mut ::std::os::raw::c_void,
            shader_binding_table: *const RendererShaderBindingTableDescT,
            device_affinity_mask: u32,
            staging_mem: *mut RendererDeviceMemoryO,
        ) -> RendererHandleT,
    >,
    pub create_top_level_acceleration_structure: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            dest: *mut ::std::os::raw::c_void,
            acceleration_structure: *const RendererTopLevelAccelerationStructureDescT,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_bottom_level_acceleration_structure: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            dest: *mut ::std::os::raw::c_void,
            acceleration_structure: *const RendererBottomLevelAccelerationStructureDescT,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub reallocate_bindless_handles: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, handle: *mut RendererHandleT),
    >,
}
impl Default for RendererBackendResourceAllocatorI {
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
pub struct RendererDeviceMemoryAllocatorI {
    pub inst: *mut ::std::os::raw::c_void,
    pub allocate_staging_memory_command: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            cmd_type: u32,
            cmd_data: *mut ::std::os::raw::c_void,
            device_affinity_mask: u32,
            data: *mut *mut ::std::os::raw::c_void,
        ) -> RendererDeviceMemoryO,
    >,
    pub allocate_staging_memory: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            size: u64,
            device_affinity_mask: u32,
            data: *mut *mut ::std::os::raw::c_void,
        ) -> RendererDeviceMemoryO,
    >,
    pub allocate_resource_read_fence: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, device_affinity_mask: u32) -> u32,
    >,
    pub append_user_data: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            dest: *mut ::std::os::raw::c_void,
            src: *mut ::std::os::raw::c_void,
        ),
    >,
}
impl Default for RendererDeviceMemoryAllocatorI {
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
pub struct RendererBackendO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererBackendI {
    pub inst: *mut RendererBackendO,
    pub create_swap_chain: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererBackendO,
            window: *const WindowPlatformDataO,
            color_space: *const ColorSpaceDescT,
            device_affinity: u32,
        ) -> RendererSwapChainO,
    >,
    pub destroy_swap_chain: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut RendererBackendO, swap_chain: RendererSwapChainO),
    >,
    pub resize_swap_chain: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererBackendO,
            swap_chain: RendererSwapChainO,
            width: u32,
            height: u32,
        ),
    >,
    pub present_swap_chain: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut RendererBackendO, swap_chain: RendererSwapChainO),
    >,
    pub swap_chain_resource: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererBackendO,
            swap_chain: RendererSwapChainO,
        ) -> RendererHandleT,
    >,
    pub create_command_buffers: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererBackendO,
            command_buffers: *mut *mut RendererCommandBufferO,
            num_buffers: u32,
        ),
    >,
    pub submit_command_buffers: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererBackendO,
            command_buffers: *mut *mut RendererCommandBufferO,
            num_buffers: u32,
        ),
    >,
    pub destroy_command_buffers: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererBackendO,
            command_buffers: *mut *mut RendererCommandBufferO,
            num_buffers: u32,
        ),
    >,
    pub num_command_queues: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererBackendO,
            queue_family: u8,
            device_affinity_mask: u32,
        ) -> u8,
    >,
    pub supports_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererBackendO,
            buffer: *const RendererBufferDescT,
            device_affinity_mask: u32,
        ) -> bool,
    >,
    pub supports_image: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererBackendO,
            image: *const RendererImageDescT,
            device_affinity_mask: u32,
        ) -> bool,
    >,
    pub supported_color_spaces: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererBackendO,
            window: *const WindowPlatformDataO,
            num_color_spaces: *mut u32,
            descs: *mut ColorSpaceDescT,
            formats: *mut u32,
            device_affinity_mask: u32,
        ) -> bool,
    >,
    pub supports_ray_tracing: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut RendererBackendO, device_affinity_mask: u32) -> bool,
    >,
    pub create_resource_command_buffers: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererBackendO,
            resource_buffers: *mut *mut RendererResourceCommandBufferO,
            num_buffers: u32,
        ),
    >,
    pub submit_resource_command_buffers: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererBackendO,
            resource_buffers: *mut *mut RendererResourceCommandBufferO,
            num_buffers: u32,
        ),
    >,
    pub destroy_resource_command_buffers: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererBackendO,
            resource_buffers: *mut *mut RendererResourceCommandBufferO,
            num_buffers: u32,
        ),
    >,
    pub frame_begin: ::std::option::Option<unsafe extern "C" fn(inst: *mut RendererBackendO)>,
    pub recycle_buffers: ::std::option::Option<unsafe extern "C" fn(inst: *mut RendererBackendO)>,
    pub read_complete: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererBackendO,
            read_fence: u32,
            device_affinity_mask: u32,
        ) -> bool,
    >,
    pub hashed_backend_name:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut RendererBackendO) -> StrhashT>,
    pub statistics_memory_allocators: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererBackendO,
            device_affinity: u32,
            allocators: *mut RendererMemoryStatisticsAllocatorT,
        ) -> u32,
    >,
    pub statistics_memory_allocations: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererBackendO,
            device_affinity: u32,
            allocator: u32,
            block: u32,
            allocators: *mut RendererMemoryStatisticsAllocationT,
        ) -> u32,
    >,
    pub statistics: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererBackendO,
            device_affinity: u32,
            stats: *mut RendererStatisticsT,
        ),
    >,
}
impl Default for RendererBackendI {
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
pub struct RendererCommandBufferPoolO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererIndexBufferBindT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererMemoryBlockPoolO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererResourceCommandBufferPoolO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererBuildAccelerationStructureT {
    _unused: [u8; 0],
}
pub const TM_RENDERER_COMMAND_BIND_RENDER_PASS: RendererCommand = 0;
pub const TM_RENDERER_COMMAND_SET_VIEWPORTS: RendererCommand = 1;
pub const TM_RENDERER_COMMAND_SET_SCISSOR_RECTS: RendererCommand = 2;
pub const TM_RENDERER_COMMAND_DRAW_CALL: RendererCommand = 3;
pub const TM_RENDERER_COMMAND_BEGIN_STATISTICS: RendererCommand = 4;
pub const TM_RENDERER_COMMAND_END_STATISTICS: RendererCommand = 5;
pub const TM_RENDERER_COMMAND_BIND_QUEUE: RendererCommand = 6;
pub const TM_RENDERER_COMMAND_COMPUTE: RendererCommand = 7;
pub const TM_RENDERER_COMMAND_TRANSITION_RESOURCES: RendererCommand = 8;
pub const TM_RENDERER_COMMAND_COPY_IMAGE: RendererCommand = 9;
pub const TM_RENDERER_COMMAND_COPY_BUFFER: RendererCommand = 10;
pub const TM_RENDERER_COMMAND_READ_IMAGE: RendererCommand = 11;
pub const TM_RENDERER_COMMAND_READ_BUFFER: RendererCommand = 12;
pub const TM_RENDERER_COMMAND_TRACE: RendererCommand = 13;
pub type RendererCommand = ::std::os::raw::c_int;
pub const TM_RENDERER_MAP_FLAGS_CPU_CACHED: RendererMapFlags = 1;
pub type RendererMapFlags = ::std::os::raw::c_int;
pub const TM_RENDERER_GPU_TIMINGS: RendererStatisticsFlags = 1;
pub type RendererStatisticsFlags = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererCommandsT {
    pub sort_keys: *mut u64,
    pub types: *mut u32,
    pub data: *mut *mut ::std::os::raw::c_void,
}
impl Default for RendererCommandsT {
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
pub struct RendererCommandBufferApi {
    pub bind_render_pass: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererCommandBufferO,
            sort_key: u64,
            render_pass: *const RendererRenderPassBindT,
        ),
    >,
    pub set_viewports: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererCommandBufferO,
            sort_key: u64,
            viewports: *const RendererSetViewportsT,
        ),
    >,
    pub set_scissor_rects: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererCommandBufferO,
            sort_key: u64,
            rects: *const RendererSetScissorRectsT,
        ),
    >,
    pub draw_calls: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererCommandBufferO,
            sort_keys: *const u64,
            draw_calls: *const RendererDrawCallInfoT,
            shaders: *const RendererShaderInfoT,
            num_draw_calls: u32,
        ),
    >,
    pub bind_queue: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererCommandBufferO,
            sort_key: u64,
            queue_bind: *const RendererQueueBindT,
        ),
    >,
    pub begin_statistics_scope: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererCommandBufferO,
            sort_key: u64,
            category: *const ::std::os::raw::c_char,
            name: *const ::std::os::raw::c_char,
            flags: u32,
        ) -> u64,
    >,
    pub end_statistics_scope: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererCommandBufferO,
            sort_key: u64,
            statistics_scope: u64,
        ),
    >,
    pub compute_dispatches: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererCommandBufferO,
            sort_keys: *const u64,
            compute_dispatches: *const RendererComputeInfoT,
            shaders: *const RendererShaderInfoT,
            num_compute_dispatches: u32,
        ),
    >,
    pub transition_resources: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererCommandBufferO,
            sort_key: u64,
            barriers: *const RendererResourceBarrierT,
            num_barriers: u32,
        ),
    >,
    pub copy_image: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererCommandBufferO,
            sort_key: u64,
            copy_image: *const RendererCopyImageT,
        ),
    >,
    pub copy_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererCommandBufferO,
            sort_key: u64,
            copy_buffer: *const RendererCopyBufferT,
        ),
    >,
    pub trace_dispatches: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererCommandBufferO,
            sort_keys: *const u64,
            trace_calls: *const RendererTraceCallT,
            num_trace_calls: u32,
        ),
    >,
    pub read_image: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererCommandBufferO,
            sort_key: u64,
            read_image: *const RendererReadImageT,
        ) -> u32,
    >,
    pub read_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererCommandBufferO,
            sort_key: u64,
            read_buffer: *const RendererReadBufferT,
        ) -> u32,
    >,
    pub num_commands:
        ::std::option::Option<unsafe extern "C" fn(inst: *const RendererCommandBufferO) -> u32>,
    pub commands: ::std::option::Option<
        unsafe extern "C" fn(inst: *const RendererCommandBufferO) -> RendererCommandsT,
    >,
    pub user_data: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut RendererCommandBufferO) -> *mut ::std::os::raw::c_void,
    >,
    pub set_backend_allocator: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererCommandBufferO,
            memory_allocator: *mut RendererDeviceMemoryAllocatorI,
        ),
    >,
    pub append_buffers: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererCommandBufferO,
            buffers: *mut *mut RendererCommandBufferO,
            num_buffers: u32,
        ),
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererCommandBufferPoolApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut RendererCommandBufferPoolO) -> *mut RendererCommandBufferO,
    >,
    pub destroy: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererCommandBufferPoolO,
            buffer: *mut RendererCommandBufferO,
        ),
    >,
    pub user_data_size:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut RendererCommandBufferPoolO) -> u64>,
}
extern "C" {
    pub fn tm_renderer_create_command_buffer_pool(
        backing_memory_block_pool: *mut RendererMemoryBlockPoolO,
        user_data_size: u64,
        allocator: *mut AllocatorI,
        memory_tracker_scope: u32,
    ) -> *mut RendererCommandBufferPoolO;
}
extern "C" {
    pub fn tm_renderer_destroy_command_buffer_pool(
        pool: *mut RendererCommandBufferPoolO,
        allocator: *mut AllocatorI,
        memory_tracker_scope: u32,
    );
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererCommandBufferSortApi {
    pub sort_memory_needed: ::std::option::Option<
        unsafe extern "C" fn(buffers: *mut *const RendererCommandBufferO, num_buffers: u32) -> u64,
    >,
    pub sort_commands: ::std::option::Option<
        unsafe extern "C" fn(
            buffers: *mut *const RendererCommandBufferO,
            num_buffers: u32,
            sort_memory: *mut ::std::os::raw::c_void,
            sorted_output: *mut *mut RendererCommandsT,
            num_commands: *mut u32,
        ),
    >,
}
pub const TM_RENDERER_RESOURCE_CREATE_BUFFER: RendererResource = 0;
pub const TM_RENDERER_RESOURCE_MAP_CREATE_BUFFER: RendererResource = 1;
pub const TM_RENDERER_RESOURCE_RESIZE_BUFFER: RendererResource = 2;
pub const TM_RENDERER_RESOURCE_UPDATE_BUFFER: RendererResource = 3;
pub const TM_RENDERER_RESOURCE_FILL_BUFFER: RendererResource = 4;
pub const TM_RENDERER_RESOURCE_CREATE_IMAGE: RendererResource = 5;
pub const TM_RENDERER_RESOURCE_MAP_CREATE_IMAGE: RendererResource = 6;
pub const TM_RENDERER_RESOURCE_UPDATE_IMAGE: RendererResource = 7;
pub const TM_RENDERER_RESOURCE_CREATE_IMAGE_VIEWS: RendererResource = 8;
pub const TM_RENDERER_RESOURCE_CREATE_SHADER: RendererResource = 9;
pub const TM_RENDERER_RESOURCE_CREATE_SHADER_STATE_OVERRIDE: RendererResource = 10;
pub const TM_RENDERER_RESOURCE_CREATE_SAMPLER: RendererResource = 11;
pub const TM_RENDERER_RESOURCE_CREATE_QUEUE_FENCE: RendererResource = 12;
pub const TM_RENDERER_RESOURCE_CREATE_RESOURCE_BINDER: RendererResource = 13;
pub const TM_RENDERER_RESOURCE_CREATE_RAY_TRACING_PIPELINE: RendererResource = 14;
pub const TM_RENDERER_RESOURCE_CREATE_SHADER_BINDING_TABLE: RendererResource = 15;
pub const TM_RENDERER_RESOURCE_CREATE_TOP_LEVEL_ACCELERATION_STRUCTURE: RendererResource = 16;
pub const TM_RENDERER_RESOURCE_CREATE_BOTTOM_LEVEL_ACCELERATION_STRUCTURE: RendererResource = 17;
pub const TM_RENDERER_RESOURCE_SET_RESOURCE: RendererResource = 18;
pub const TM_RENDERER_RESOURCE_SET_ARRAY_RESOURCES: RendererResource = 19;
pub const TM_RENDERER_RESOURCE_DESTROY: RendererResource = 20;
pub const TM_RENDERER_RESOURCE_NUM_COMMAND_TYPES: RendererResource = 21;
pub type RendererResource = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererResourceCommandsT {
    pub types: *mut u32,
    pub handles: *mut RendererHandleT,
    pub data: *mut *mut ::std::os::raw::c_void,
}
impl Default for RendererResourceCommandsT {
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
pub struct RendererResourceCommandBufferApi {
    pub create_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            buffer: *const RendererBufferDescT,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub map_create_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            buffer: *const RendererBufferDescT,
            device_affinity_mask: u32,
            map_flags: RendererMapFlags,
            data: *mut *mut ::std::os::raw::c_void,
        ) -> RendererHandleT,
    >,
    pub resize_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            handle: *mut RendererHandleT,
            size: u64,
        ),
    >,
    pub update_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            handle: RendererHandleT,
            offset: u64,
            size: u64,
            device_affinity_mask: u32,
            map_flags: RendererMapFlags,
            data: *mut *mut ::std::os::raw::c_void,
        ),
    >,
    pub fill_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            handle: RendererHandleT,
            offset: u64,
            size: u64,
            data: u32,
            device_affinity_mask: u32,
        ),
    >,
    pub create_image: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            image: *const RendererImageDescT,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub map_create_image: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            image: *const RendererImageDescT,
            device_affinity_mask: u32,
            map_flags: RendererMapFlags,
            data: *mut *mut ::std::os::raw::c_void,
        ) -> RendererHandleT,
    >,
    pub update_image: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            handle: RendererHandleT,
            resource_state: u16,
            format: u32,
            subresource: *const RendererImageSubresourceLayersT,
            offset: *const RendererImageOffsetT,
            extent: *const RendererImageExtentT,
            device_affinity_mask: u32,
            map_flags: RendererMapFlags,
            data: *mut *mut ::std::os::raw::c_void,
        ),
    >,
    pub create_image_views: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            image: RendererHandleT,
            desc: *const RendererImageDescT,
            device_affinity_mask: u32,
            views: *const RendererImageViewT,
            num_views: u32,
        ) -> RendererHandleT,
    >,
    pub create_shader: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            shader: *const RendererShaderT,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_sampler: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            sampler: *const RendererSamplerT,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_shader_state_override: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            state_override: *const RendererShaderStateOverrideT,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_queue_fence: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_resource_binder: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            bind_points: *const RendererResourceBindPointT,
            num_bind_points: u32,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_ray_tracing_pipeline: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            pipeline: *const RendererRayTracingPipelineDescT,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_shader_binding_table: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            shader_binding_table: *const RendererShaderBindingTableDescT,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_top_level_acceleration_structure: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            acceleration_structure: *const RendererTopLevelAccelerationStructureDescT,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub create_bottom_level_acceleration_structure: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            acceleration_structure: *const RendererBottomLevelAccelerationStructureDescT,
            device_affinity_mask: u32,
        ) -> RendererHandleT,
    >,
    pub set_resource: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            resource_binder: RendererHandleT,
            bind_point: u32,
            resource_handles: RendererHandleT,
            reosource_aspect_flags: u32,
        ),
    >,
    pub set_array_resources: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            resource_binder: RendererHandleT,
            bind_point: u32,
            first_element: u32,
            resource_handles: *const RendererHandleT,
            resource_aspect_flags: *const u32,
            n_handles: u32,
        ),
    >,
    pub destroy_resource: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut RendererResourceCommandBufferO, handle: RendererHandleT),
    >,
    pub num_commands: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            count_per_command_type: *mut u32,
        ) -> u32,
    >,
    pub commands: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
        ) -> RendererResourceCommandsT,
    >,
    pub user_data: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub set_backend_allocators: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            resource_allocator: *mut RendererBackendResourceAllocatorI,
            memory_allocator: *mut RendererDeviceMemoryAllocatorI,
        ),
    >,
    pub append_buffers: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferO,
            buffers: *mut *mut RendererResourceCommandBufferO,
            num_buffers: u32,
        ),
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererResourceCommandBufferPoolApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferPoolO,
        ) -> *mut RendererResourceCommandBufferO,
    >,
    pub destroy: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererResourceCommandBufferPoolO,
            buffer: *mut RendererResourceCommandBufferO,
        ),
    >,
    pub user_data_size: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut RendererResourceCommandBufferPoolO) -> u64,
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererStatisticsT {
    pub non_indexed_draws: u64,
    pub indexed_draws: u64,
    pub non_indexed_indirect_draws: u64,
    pub indexed_indirect_draws: u64,
    pub compute_dispatches: u64,
    pub compute_indirect_dispatches: u64,
    pub trace_dispatches: u64,
    pub primitive_count: u64,
    pub pipeline_binds: u64,
    pub resource_barriers: u64,
    pub render_pass_binds: u64,
}
pub const TM_RENDERER_RESOURCE_TYPE_NOT_INITIALIZED: RendererResourceType = 0;
pub const TM_RENDERER_RESOURCE_TYPE_BUFFER: RendererResourceType = 1;
pub const TM_RENDERER_RESOURCE_TYPE_IMAGE: RendererResourceType = 2;
pub const TM_RENDERER_RESOURCE_TYPE_IMAGE_VIEWS: RendererResourceType = 3;
pub const TM_RENDERER_RESOURCE_TYPE_SAMPLER: RendererResourceType = 4;
pub const TM_RENDERER_RESOURCE_TYPE_QUEUE_FENCE: RendererResourceType = 5;
pub const TM_RENDERER_RESOURCE_TYPE_SHADER: RendererResourceType = 6;
pub const TM_RENDERER_RESOURCE_TYPE_SHADER_STATE_OVERRIDE: RendererResourceType = 7;
pub const TM_RENDERER_RESOURCE_TYPE_RESOURCE_BINDER: RendererResourceType = 8;
pub const TM_RENDERER_RESOURCE_TYPE_BACK_BUFFER: RendererResourceType = 9;
pub const TM_RENDERER_RESOURCE_TYPE_ACCELERATION_STRUCTURE: RendererResourceType = 10;
pub const TM_RENDERER_RESOURCE_TYPE_RAY_TRACING_PIPELINE: RendererResourceType = 11;
pub type RendererResourceType = ::std::os::raw::c_int;
pub const TM_RENDERER_BUFFER_USAGE_UNIFORM: RendererBufferUsageFlags = 1;
pub const TM_RENDERER_BUFFER_USAGE_STORAGE: RendererBufferUsageFlags = 2;
pub const TM_RENDERER_BUFFER_USAGE_UPDATABLE: RendererBufferUsageFlags = 4;
pub const TM_RENDERER_BUFFER_USAGE_UAV: RendererBufferUsageFlags = 8;
pub const TM_RENDERER_BUFFER_USAGE_INDEX: RendererBufferUsageFlags = 16;
pub const TM_RENDERER_BUFFER_USAGE_VERTEX: RendererBufferUsageFlags = 32;
pub const TM_RENDERER_BUFFER_USAGE_INDIRECT: RendererBufferUsageFlags = 64;
pub const TM_RENDERER_BUFFER_USAGE_ACCELERATION_STRUCTURE: RendererBufferUsageFlags = 128;
pub type RendererBufferUsageFlags = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererBufferDescT {
    pub size: u32,
    pub usage_flags: RendererBufferUsageFlags,
    pub debug_tag: *const ::std::os::raw::c_char,
}
impl Default for RendererBufferDescT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_RENDERER_IMAGE_USAGE_RENDER_TARGET: RendererImageUsageFlags = 1;
pub const TM_RENDERER_IMAGE_USAGE_UAV: RendererImageUsageFlags = 2;
pub type RendererImageUsageFlags = ::std::os::raw::c_int;
pub const TM_RENDERER_IMAGE_TYPE_1D: RendererImageType = 0;
pub const TM_RENDERER_IMAGE_TYPE_2D: RendererImageType = 1;
pub const TM_RENDERER_IMAGE_TYPE_3D: RendererImageType = 2;
pub const TM_RENDERER_IMAGE_TYPE_CUBE: RendererImageType = 3;
pub const TM_RENDERER_IMAGE_TYPE_MAX_VIEWS: RendererImageType = 4;
pub type RendererImageType = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union RendererClearValueT {
    pub color: RendererClearValueTColor,
    pub depth_stencil: RendererClearValueTDepthStencil,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererClearValueTColor {
    pub __bindgen_anon_1: RendererClearValueTColorBindgenTy1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union RendererClearValueTColorBindgenTy1 {
    pub float32: [f32; 4usize],
    pub int32: [i32; 4usize],
    pub uint32: [u32; 4usize],
}
impl Default for RendererClearValueTColorBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for RendererClearValueTColor {
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
pub struct RendererClearValueTDepthStencil {
    pub depth: f32,
    pub stencil: u32,
}
impl Default for RendererClearValueT {
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
pub struct RendererImageDescT {
    pub type_: RendererImageType,
    pub usage_flags: RendererImageUsageFlags,
    pub format: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mip_levels: u32,
    pub layer_count: u32,
    pub sample_count: u32,
    pub clear_value: RendererClearValueT,
    pub _padding_132: [::std::os::raw::c_char; 4usize],
    pub debug_tag: *const ::std::os::raw::c_char,
}
impl Default for RendererImageDescT {
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
pub struct RendererImageViewT {
    pub first_layer: u16,
    pub layer_count: u16,
    pub first_mip: u8,
    pub mip_count: u8,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub _padding_168: [::std::os::raw::c_char; 1usize],
}
impl RendererImageViewT {
    #[inline]
    pub fn aspect(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_aspect(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn padding(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_padding(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(aspect: u8, padding: u8) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let aspect: u8 = unsafe { ::std::mem::transmute(aspect) };
            aspect as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let padding: u8 = unsafe { ::std::mem::transmute(padding) };
            padding as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererShaderT {
    pub tessellation_states: RendererShaderBlobT,
    pub raster_states: RendererShaderBlobT,
    pub depth_stencil_states: RendererShaderBlobT,
    pub blend_states: RendererShaderBlobT,
    pub multi_sample_states: RendererShaderBlobT,
    pub stages: [RendererShaderBlobT; 11usize],
    pub debug_tag: *const ::std::os::raw::c_char,
}
impl Default for RendererShaderT {
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
pub struct RendererSamplerT {
    pub sampler_states: RendererShaderBlobT,
}
impl Default for RendererSamplerT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_RENDERER_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL: RendererAccelerationStructureType = 0;
pub const TM_RENDERER_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL: RendererAccelerationStructureType =
    1;
pub type RendererAccelerationStructureType = ::std::os::raw::c_int;
pub const TM_RENDERER_ACCELERATION_STRUCTURE_BUILD_PREFER_FAST_TRACE:
    RendererAccelerationStructureBuildFlags = 1;
pub const TM_RENDERER_ACCELERATION_STRUCTURE_BUILD_PREFER_FAST_BUILD:
    RendererAccelerationStructureBuildFlags = 2;
pub const TM_RENDERER_ACCELERATION_STRUCTURE_BUILD_PREFER_LOW_MEMORY:
    RendererAccelerationStructureBuildFlags = 4;
pub const TM_RENDERER_ACCELERATION_STRUCTURE_BUILD_ALLOW_UPDATES:
    RendererAccelerationStructureBuildFlags = 8;
pub const TM_RENDERER_ACCELERATION_STRUCTURE_BUILD_ALLOW_COMPACTION:
    RendererAccelerationStructureBuildFlags = 16;
pub type RendererAccelerationStructureBuildFlags = ::std::os::raw::c_int;
pub const TM_RENDERER_GEOMETRY_TYPE_TRIANGLES: RendererGeometryType = 0;
pub const TM_RENDERER_GEOMETRY_TYPE_AABBS: RendererGeometryType = 1;
pub type RendererGeometryType = ::std::os::raw::c_int;
pub const TM_RENDERER_GEOMETRY_OPAQUE: RendererGeometryFlags = 1;
pub const TM_RENDERER_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION: RendererGeometryFlags = 2;
pub type RendererGeometryFlags = ::std::os::raw::c_int;
pub const TM_RENDERER_GEOMETRY_INDEX_TYPE_NONE: RendererGeometryIndexType = 0;
pub const TM_RENDERER_GEOMETRY_INDEX_TYPE_UINT16: RendererGeometryIndexType = 1;
pub const TM_RENDERER_GEOMETRY_INDEX_TYPE_UINT32: RendererGeometryIndexType = 2;
pub type RendererGeometryIndexType = ::std::os::raw::c_int;
pub const TM_RENDERER_GEOMETRY_INSTANCE_DISABLE_TRIANGLE_CULL: RendererGeometryInstanceFlags = 1;
pub const TM_RENDERER_GEOMETRY_INSTANCE_COUNTERCLOCKWISE: RendererGeometryInstanceFlags = 2;
pub const TM_RENDERER_GEOMETRY_INSTANCE_FORCE_OPAQUE: RendererGeometryInstanceFlags = 4;
pub const TM_RENDERER_GEOMETRY_INSTANCE_FORCE_NO_OPQUE: RendererGeometryInstanceFlags = 8;
pub type RendererGeometryInstanceFlags = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererGeometryTriangleDescT {
    pub format: u32,
    pub vertex_data: RendererHandleT,
    pub vertex_offset: u32,
    pub vertex_stride: u32,
    pub vertex_count: u32,
    pub primitive_count: u32,
    pub index_type: RendererGeometryIndexType,
    pub index_data: RendererHandleT,
    pub index_offset: u32,
    pub transform_data: RendererHandleT,
    pub transform_offset: u32,
}
impl Default for RendererGeometryTriangleDescT {
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
pub struct RendererGeometryAabbDescT {
    pub data: RendererHandleT,
    pub data_offset: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererGeometryDescT {
    pub type_: RendererGeometryType,
    pub flags: RendererGeometryFlags,
    pub __bindgen_anon_1: RendererGeometryDescTBindgenTy1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union RendererGeometryDescTBindgenTy1 {
    pub triangle_desc: RendererGeometryTriangleDescT,
    pub aabb_desc: RendererGeometryAabbDescT,
}
impl Default for RendererGeometryDescTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for RendererGeometryDescT {
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
pub struct RendererBottomLevelAccelerationStructureDescT {
    pub build_flags: RendererAccelerationStructureBuildFlags,
    pub geometry_desc_count: u32,
    pub geometry_desc: *const RendererGeometryDescT,
    pub debug_tag: *const ::std::os::raw::c_char,
}
impl Default for RendererBottomLevelAccelerationStructureDescT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct RendererTopLevelAccelerationStructureInstanceT {
    pub transform: Mat44T,
    pub shader_info_idx: u32,
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 3usize]>,
    pub __bindgen_padding_0: u8,
    pub mask: u8,
    pub _padding_333: [::std::os::raw::c_char; 3usize],
    pub _bitfield_align_2: [u8; 0],
    pub _bitfield_2: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub blas_handle: RendererHandleT,
}
impl Default for RendererTopLevelAccelerationStructureInstanceT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl RendererTopLevelAccelerationStructureInstanceT {
    #[inline]
    pub fn instance_id(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 24u8) as u32) }
    }
    #[inline]
    pub fn set_instance_id(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 24u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(instance_id: u32) -> __BindgenBitfieldUnit<[u8; 3usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 3usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 24u8, {
            let instance_id: u32 = unsafe { ::std::mem::transmute(instance_id) };
            instance_id as u64
        });
        __bindgen_bitfield_unit
    }
    #[inline]
    pub fn flags(&self) -> RendererGeometryInstanceFlags {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(0usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_flags(&mut self, val: RendererGeometryInstanceFlags) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_2.set(0usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_2(
        flags: RendererGeometryInstanceFlags,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 8u8, {
            let flags: u32 = unsafe { ::std::mem::transmute(flags) };
            flags as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererTopLevelAccelerationStructureDescT {
    pub build_flags: RendererAccelerationStructureBuildFlags,
    pub geometry_flags: RendererGeometryFlags,
    pub num_instances: u32,
    pub _padding_348: [::std::os::raw::c_char; 4usize],
    pub instaces: *const RendererTopLevelAccelerationStructureInstanceT,
    pub debug_tag: *const ::std::os::raw::c_char,
}
impl Default for RendererTopLevelAccelerationStructureDescT {
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
pub struct RendererShaderBindingTableDescT {
    pub pipeline: RendererHandleT,
    pub num_shader_infos: u32,
    pub shader_infos: *const RendererShaderInfoT,
    pub debug_tag: *const ::std::os::raw::c_char,
}
impl Default for RendererShaderBindingTableDescT {
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
pub struct RendererRayTracingPipelineDescT {
    pub max_recursion_depth: u32,
    pub num_shaders: u32,
    pub shader_infos: *const RendererShaderInfoT,
    pub debug_tag: *const ::std::os::raw::c_char,
}
impl Default for RendererRayTracingPipelineDescT {
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
pub struct RendererShaderStateOverrideT {
    pub tessellation_states: RendererShaderBlobT,
    pub raster_states: RendererShaderBlobT,
    pub depth_stencil_states: RendererShaderBlobT,
    pub blend_states: RendererShaderBlobT,
    pub multi_sample_states: RendererShaderBlobT,
}
impl Default for RendererShaderStateOverrideT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_RENDERER_SHADER_STAGE_FLAG_VERTEX: RendererShaderStageFlag = 1;
pub const TM_RENDERER_SHADER_STAGE_FLAG_HULL: RendererShaderStageFlag = 2;
pub const TM_RENDERER_SHADER_STAGE_FLAG_DOMAIN: RendererShaderStageFlag = 4;
pub const TM_RENDERER_SHADER_STAGE_FLAG_GEOMETRY: RendererShaderStageFlag = 8;
pub const TM_RENDERER_SHADER_STAGE_FLAG_PIXEL: RendererShaderStageFlag = 16;
pub const TM_RENDERER_SHADER_STAGE_FLAG_COMPUTE: RendererShaderStageFlag = 32;
pub const TM_RENDERER_SHADER_STAGE_FLAG_ALL: RendererShaderStageFlag = 63;
pub type RendererShaderStageFlag = ::std::os::raw::c_int;
pub const TM_RENDERER_RESOURCE_BIND_USAGE_FLAG_UAV: RendererResourceBindUsageFlag = 1;
pub type RendererResourceBindUsageFlag = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererResourceBindPointT {
    pub bind_point: u32,
    pub stage_flags: u32,
    pub type_: u32,
    pub view: u32,
    pub usage: u32,
    pub count: u32,
}
pub const TM_RENDERER_SHADER_SOURCE_LANGUAGE_HLSL: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_5 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererShaderCompilerO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererStateValuePairT {
    pub state: u32,
    pub _padding_17: [::std::os::raw::c_char; 4usize],
    pub __bindgen_anon_1: RendererStateValuePairTBindgenTy1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union RendererStateValuePairTBindgenTy1 {
    pub enum_value: u32,
    pub uint32_value: u32,
    pub float_value: f32,
    pub nested_states: *mut RendererStateValuePairT,
}
impl Default for RendererStateValuePairTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for RendererStateValuePairT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_RENDERER_STATE_BLOCK_COMPLETE: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_STATE_BLOCK_OVERRIDE: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_6 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererBindlessAccessorT {
    pub set: u16,
    pub slot: u16,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RendererShaderCompilerApi {
    pub init: ::std::option::Option<
        unsafe extern "C" fn(allocator: *mut AllocatorI) -> *mut RendererShaderCompilerO,
    >,
    pub shutdown: ::std::option::Option<unsafe extern "C" fn(inst: *mut RendererShaderCompilerO)>,
    pub num_state_block_types:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut RendererShaderCompilerO) -> u32>,
    pub state_block_type: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut RendererShaderCompilerO, state_block_type_idx: u32) -> u32,
    >,
    pub state_block_name: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererShaderCompilerO,
            state_block_type: u32,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub num_states: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut RendererShaderCompilerO, state_block_type: u32) -> u32,
    >,
    pub state_name: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererShaderCompilerO,
            state_block_type: u32,
            state: u32,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub value_type: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererShaderCompilerO,
            state_block_type: u32,
            state: u32,
        ) -> u32,
    >,
    pub num_values: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut RendererShaderCompilerO, value_type: u32) -> u32,
    >,
    pub value_name: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererShaderCompilerO,
            value_type: u32,
            value: u32,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub enum_value: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererShaderCompilerO,
            value_type: u32,
            value: u32,
        ) -> u32,
    >,
    pub compile_state_block: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererShaderCompilerO,
            bind_type: u32,
            block_type: u32,
            states: *const RendererStateValuePairT,
            num_raster_states: u32,
        ) -> RendererShaderBlobT,
    >,
    pub compile_shader: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererShaderCompilerO,
            source: *const ::std::os::raw::c_char,
            entry_point: *const ::std::os::raw::c_char,
            source_language: u32,
            stage: u32,
        ) -> RendererShaderBlobT,
    >,
    pub bindless:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut RendererShaderCompilerO) -> bool>,
    pub bindless_access_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererShaderCompilerO,
            usage_flags: u32,
        ) -> RendererBindlessAccessorT,
    >,
    pub bindless_access_image: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut RendererShaderCompilerO,
            type_: u32,
            usage_flags: u32,
        ) -> RendererBindlessAccessorT,
    >,
    pub bindless_access_sampler: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut RendererShaderCompilerO) -> RendererBindlessAccessorT,
    >,
    pub bindless_access_acceleration_structure: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut RendererShaderCompilerO) -> RendererBindlessAccessorT,
    >,
    pub release_blob: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut RendererShaderCompilerO, blob: RendererShaderBlobT),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RendererShaderCompilerI {
    pub api: *mut RendererShaderCompilerApi,
    pub inst: *mut RendererShaderCompilerO,
}
impl Default for RendererShaderCompilerI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub static mut tm_renderer_state_block_names: [*const ::std::os::raw::c_char; 7usize];
}
pub const TM_RENDERER_VALUE_BOOL_FALSE: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_VALUE_BOOL_TRUE: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_VALUE_BOOL_MAX_VALUES: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_7 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_value_bool_names: [*const ::std::os::raw::c_char; 2usize];
}
pub const TM_RENDERER_COMPARE_OP_NEVER: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_COMPARE_OP_LESS: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_COMPARE_OP_EQUAL: ::std::os::raw::c_int = 2;
pub const TM_RENDERER_COMPARE_OP_LESS_OR_EQUAL: ::std::os::raw::c_int = 3;
pub const TM_RENDERER_COMPARE_OP_GREATER: ::std::os::raw::c_int = 4;
pub const TM_RENDERER_COMPARE_OP_NOT_EQUAL: ::std::os::raw::c_int = 5;
pub const TM_RENDERER_COMPARE_OP_GREATER_OR_EQUAL: ::std::os::raw::c_int = 6;
pub const TM_RENDERER_COMPARE_OP_MAX_VALUES: ::std::os::raw::c_int = 7;
pub type _bindgen_ty_8 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_value_compare_op_names: [*const ::std::os::raw::c_char; 7usize];
}
pub const TM_RENDERER_TESSELLATION_STATE_CONTROL_POINTS: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_TESSELLATION_STATE_MAX_STATES: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_9 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_tessellation_state_names: [*const ::std::os::raw::c_char; 1usize];
}
extern "C" {
    pub static mut tm_renderer_tessellation_state_value_types: [u32; 1usize];
}
pub const TM_RENDERER_RASTER_STATE_CULL_MODE: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_RASTER_STATE_FRONT_FACE: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_RASTER_STATE_POLYGON_MODE: ::std::os::raw::c_int = 2;
pub const TM_RENDERER_RASTER_STATE_DEPTH_BIAS_ENABLE: ::std::os::raw::c_int = 3;
pub const TM_RENDERER_RASTER_STATE_DEPTH_BIAS_CONSTANT_FACTOR: ::std::os::raw::c_int = 4;
pub const TM_RENDERER_RASTER_STATE_DEPTH_BIAS_CLAMP: ::std::os::raw::c_int = 5;
pub const TM_RENDERER_RASTER_STATE_DEPTH_BIAS_SLOPE_FACTOR: ::std::os::raw::c_int = 6;
pub const TM_RENDERER_RASTER_STATE_MAX_STATES: ::std::os::raw::c_int = 7;
pub type _bindgen_ty_10 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_raster_state_names: [*const ::std::os::raw::c_char; 7usize];
}
extern "C" {
    pub static mut tm_renderer_raster_state_value_types: [u32; 7usize];
}
pub const TM_RENDERER_RASTER_VALUE_CULL_NONE: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_RASTER_VALUE_CULL_FRONT: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_RASTER_VALUE_CULL_BACK: ::std::os::raw::c_int = 2;
pub const TM_RENDERER_RASTER_VALUE_CULL_MAX_VALUES: ::std::os::raw::c_int = 3;
pub type _bindgen_ty_11 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_raster_value_cull_names: [*const ::std::os::raw::c_char; 3usize];
}
pub const TM_RENDERER_RASTER_VALUE_FRONT_FACE_CW: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_RASTER_VALUE_FRONT_FACE_CCW: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_RASTER_VALUE_FRONT_FACE_MAX_VALUES: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_12 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_raster_value_front_face_names:
        [*const ::std::os::raw::c_char; 2usize];
}
pub const TM_RENDERER_RASTER_VALUE_POLYGON_MODE_FILL: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_RASTER_VALUE_POLYGON_MODE_LINE: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_RASTER_VALUE_POLYGON_MODE_POINT: ::std::os::raw::c_int = 2;
pub const TM_RENDERER_RASTER_VALUE_POLYGON_MODE_MAX_VALUES: ::std::os::raw::c_int = 3;
pub type _bindgen_ty_13 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_raster_value_polygon_names: [*const ::std::os::raw::c_char; 3usize];
}
pub const TM_RENDERER_DEPTH_STENCIL_STATE_DEPTH_TEST_ENABLE: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_DEPTH_WRITE_ENABLE: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_DEPTH_COMPARE_OP: ::std::os::raw::c_int = 2;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_STENCIL_TEST_ENABLE: ::std::os::raw::c_int = 3;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_STENCIL_FRONT_FAIL_OP: ::std::os::raw::c_int = 4;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_STENCIL_FRONT_PASS_OP: ::std::os::raw::c_int = 5;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_STENCIL_FRONT_DEPTH_FAIL_OP: ::std::os::raw::c_int = 6;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_STENCIL_FRONT_COMPARE_OP: ::std::os::raw::c_int = 7;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_STENCIL_FRONT_COMPARE_MASK: ::std::os::raw::c_int = 8;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_STENCIL_FRONT_COMPARE_WRITE_MASK: ::std::os::raw::c_int =
    9;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_STENCIL_FRONT_REFERENCE: ::std::os::raw::c_int = 10;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_STENCIL_BACK_FAIL_OP: ::std::os::raw::c_int = 11;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_STENCIL_BACK_PASS_OP: ::std::os::raw::c_int = 12;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_STENCIL_BACK_DEPTH_FAIL_OP: ::std::os::raw::c_int = 13;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_STENCIL_BACK_COMPARE_OP: ::std::os::raw::c_int = 14;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_STENCIL_BACK_COMPARE_MASK: ::std::os::raw::c_int = 15;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_STENCIL_BACK_COMPARE_WRITE_MASK: ::std::os::raw::c_int =
    16;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_STENCIL_BACK_REFERENCE: ::std::os::raw::c_int = 17;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_DEPTH_BOUND_TEST_ENABLE: ::std::os::raw::c_int = 18;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_DEPTH_BOUND_MIN: ::std::os::raw::c_int = 19;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_DEPTH_BOUND_MAX: ::std::os::raw::c_int = 20;
pub const TM_RENDERER_DEPTH_STENCIL_STATE_MAX_STATES: ::std::os::raw::c_int = 21;
pub type _bindgen_ty_14 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_depth_stencil_state_names: [*const ::std::os::raw::c_char; 21usize];
}
extern "C" {
    pub static mut tm_renderer_depth_stencil_state_value_types: [u32; 21usize];
}
pub const TM_RENDERER_STENCIL_OP_KEEP: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_STENCIL_OP_ZERO: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_STENCIL_OP_REPLACE: ::std::os::raw::c_int = 2;
pub const TM_RENDERER_STENCIL_OP_INC_CLAMP: ::std::os::raw::c_int = 3;
pub const TM_RENDERER_STENCIL_OP_DEC_CLAMP: ::std::os::raw::c_int = 4;
pub const TM_RENDERER_STENCIL_OP_INVERT: ::std::os::raw::c_int = 5;
pub const TM_RENDERER_STENCIL_OP_INC_WRAP: ::std::os::raw::c_int = 6;
pub const TM_RENDERER_STENCIL_OP_DEC_WRAP: ::std::os::raw::c_int = 7;
pub const TM_RENDERER_STENCIL_OP_MAX_VALUES: ::std::os::raw::c_int = 8;
pub type _bindgen_ty_15 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_value_stencil_op_names: [*const ::std::os::raw::c_char; 8usize];
}
pub const TM_RENDERER_SAMPLER_STATE_MIN_FILTER: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_SAMPLER_STATE_MAG_FILTER: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_SAMPLER_STATE_MIP_MODE: ::std::os::raw::c_int = 2;
pub const TM_RENDERER_SAMPLER_STATE_ADDRESS_U: ::std::os::raw::c_int = 3;
pub const TM_RENDERER_SAMPLER_STATE_ADDRESS_V: ::std::os::raw::c_int = 4;
pub const TM_RENDERER_SAMPLER_STATE_ADDRESS_W: ::std::os::raw::c_int = 5;
pub const TM_RENDERER_SAMPLER_STATE_MIP_LOD_BIAS: ::std::os::raw::c_int = 6;
pub const TM_RENDERER_SAMPLER_STATE_ANISOTROPY_ENABLE: ::std::os::raw::c_int = 7;
pub const TM_RENDERER_SAMPLER_STATE_MAX_ANISOTROPY: ::std::os::raw::c_int = 8;
pub const TM_RENDERER_SAMPLER_STATE_COMPARE_ENABLE: ::std::os::raw::c_int = 9;
pub const TM_RENDERER_SAMPLER_STATE_COMPARE_OP: ::std::os::raw::c_int = 10;
pub const TM_RENDERER_SAMPLER_STATE_MIN_LOD: ::std::os::raw::c_int = 11;
pub const TM_RENDERER_SAMPLER_STATE_MAX_LOD: ::std::os::raw::c_int = 12;
pub const TM_RENDERER_SAMPLER_STATE_BORDER_COLOR: ::std::os::raw::c_int = 13;
pub const TM_RENDERER_SAMPLER_STATE_MAX_STATES: ::std::os::raw::c_int = 14;
pub type _bindgen_ty_16 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_sampler_state_names: [*const ::std::os::raw::c_char; 14usize];
}
extern "C" {
    pub static mut tm_renderer_sampler_state_types: [u32; 14usize];
}
pub const TM_RENDERER_FILTER_POINT: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_FILTER_LINEAR: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_FILTER_MAX_VALUES: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_17 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_value_filter_names: [*const ::std::os::raw::c_char; 2usize];
}
pub const TM_RENDERER_MIP_MODE_POINT: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_MIP_MODE_LINEAR: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_MIP_MODE_MAX_VALUES: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_18 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_value_mip_mode_names: [*const ::std::os::raw::c_char; 2usize];
}
pub const TM_RENDERER_ADDRESS_MODE_WRAP: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_ADDRESS_MODE_MIRROR_WRAP: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_ADDRESS_MODE_CLAMP: ::std::os::raw::c_int = 2;
pub const TM_RENDERER_ADDRESS_MODE_CLAMP_BORDER_COLOR: ::std::os::raw::c_int = 3;
pub const TM_RENDERER_ADDRESS_MODE_MIRROR_CLAMP: ::std::os::raw::c_int = 4;
pub const TM_RENDERER_ADDRESS_MODE_MAX_VALUES: ::std::os::raw::c_int = 5;
pub type _bindgen_ty_19 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_value_address_mode_names: [*const ::std::os::raw::c_char; 5usize];
}
pub const TM_RENDERER_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_BORDER_COLOR_INT_TRANSPARENT_BLACK: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_BORDER_COLOR_FLOAT_OPAQUE_BLACK: ::std::os::raw::c_int = 2;
pub const TM_RENDERER_BORDER_COLOR_INT_OPAQUE_BLACK: ::std::os::raw::c_int = 3;
pub const TM_RENDERER_BORDER_COLOR_FLOAT_OPAQUE_WHITE: ::std::os::raw::c_int = 4;
pub const TM_RENDERER_BORDER_COLOR_INT_OPAQUE_WHITE: ::std::os::raw::c_int = 5;
pub const TM_RENDERER_BORDER_COLOR_MAX_VALUES: ::std::os::raw::c_int = 6;
pub type _bindgen_ty_20 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_value_border_color_names: [*const ::std::os::raw::c_char; 6usize];
}
pub const TM_RENDERER_RENDER_TARGET_BLEND_STATE_BLEND_ENABLE: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_RENDER_TARGET_BLEND_STATE_SOURCE_BLEND_FACTOR_COLOR: ::std::os::raw::c_int =
    1;
pub const TM_RENDERER_RENDER_TARGET_BLEND_STATE_DESTINATION_BLEND_FACTOR_COLOR:
    ::std::os::raw::c_int = 2;
pub const TM_RENDERER_RENDER_TARGET_BLEND_STATE_BLEND_OP_COLOR: ::std::os::raw::c_int = 3;
pub const TM_RENDERER_RENDER_TARGET_BLEND_STATE_SOURCE_BLEND_FACTOR_ALPHA: ::std::os::raw::c_int =
    4;
pub const TM_RENDERER_RENDER_TARGET_BLEND_STATE_DESTINATION_BLEND_FACTOR_ALPHA:
    ::std::os::raw::c_int = 5;
pub const TM_RENDERER_RENDER_TARGET_BLEND_STATE_BLEND_OP_ALPHA: ::std::os::raw::c_int = 6;
pub const TM_RENDERER_RENDER_TARGET_BLEND_STATE_WRITE_MASK: ::std::os::raw::c_int = 7;
pub const TM_RENDERER_RENDER_TARGET_BLEND_STATE_MAX_STATES: ::std::os::raw::c_int = 8;
pub type _bindgen_ty_21 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_render_target_blend_state_names:
        [*const ::std::os::raw::c_char; 8usize];
}
extern "C" {
    pub static mut tm_renderer_render_target_blend_state_types: [u32; 8usize];
}
pub const TM_RENDERER_BLEND_FACTOR_ZERO: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_BLEND_FACTOR_ONE: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_BLEND_FACTOR_SOURCE_COLOR: ::std::os::raw::c_int = 2;
pub const TM_RENDERER_BLEND_FACTOR_ONE_MINUS_SOURCE_COLOR: ::std::os::raw::c_int = 3;
pub const TM_RENDERER_BLEND_FACTOR_DESTINATION_COLOR: ::std::os::raw::c_int = 4;
pub const TM_RENDERER_BLEND_FACTOR_ONE_MINUS_DESTINATION_COLOR: ::std::os::raw::c_int = 5;
pub const TM_RENDERER_BLEND_FACTOR_SOURCE_ALPHA: ::std::os::raw::c_int = 6;
pub const TM_RENDERER_BLEND_FACTOR_ONE_MINUS_SOURCE_ALPHA: ::std::os::raw::c_int = 7;
pub const TM_RENDERER_BLEND_FACTOR_DESTINATION_ALPHA: ::std::os::raw::c_int = 8;
pub const TM_RENDERER_BLEND_FACTOR_ONE_MINUS_DESTINATION_ALPHA: ::std::os::raw::c_int = 9;
pub const TM_RENDERER_BLEND_FACTOR_CONSTANT_COLOR: ::std::os::raw::c_int = 10;
pub const TM_RENDERER_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR: ::std::os::raw::c_int = 11;
pub const TM_RENDERER_BLEND_FACTOR_CONSTANT_ALPHA: ::std::os::raw::c_int = 12;
pub const TM_RENDERER_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: ::std::os::raw::c_int = 13;
pub const TM_RENDERER_BLEND_FACTOR_SOURCE_ALPHA_SATURATE: ::std::os::raw::c_int = 14;
pub const TM_RENDERER_BLEND_FACTOR_SOURCE1_COLOR: ::std::os::raw::c_int = 15;
pub const TM_RENDERER_BLEND_FACTOR_ONE_MINUS_SOURCE1_COLOR: ::std::os::raw::c_int = 16;
pub const TM_RENDERER_BLEND_FACTOR_SOURCE1_ALPHA: ::std::os::raw::c_int = 17;
pub const TM_RENDERER_BLEND_FACTOR_ONE_MINUS_SOURCE1_ALPHA: ::std::os::raw::c_int = 18;
pub const TM_RENDERER_BLEND_FACTOR_MAX_VALUES: ::std::os::raw::c_int = 19;
pub type _bindgen_ty_22 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_value_blend_factor_names: [*const ::std::os::raw::c_char; 19usize];
}
pub const TM_RENDERER_LOGICAL_OP_CLEAR: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_LOGICAL_OP_AND: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_LOGICAL_OP_AND_REVERSE: ::std::os::raw::c_int = 2;
pub const TM_RENDERER_LOGICAL_OP_COPY: ::std::os::raw::c_int = 3;
pub const TM_RENDERER_LOGICAL_OP_AND_INVERTED: ::std::os::raw::c_int = 4;
pub const TM_RENDERER_LOGICAL_OP_NO_OP: ::std::os::raw::c_int = 5;
pub const TM_RENDERER_LOGICAL_OP_XOR: ::std::os::raw::c_int = 6;
pub const TM_RENDERER_LOGICAL_OP_OR: ::std::os::raw::c_int = 7;
pub const TM_RENDERER_LOGICAL_OP_NOR: ::std::os::raw::c_int = 8;
pub const TM_RENDERER_LOGICAL_OP_EQUIVALENT: ::std::os::raw::c_int = 9;
pub const TM_RENDERER_LOGICAL_OP_INVERT: ::std::os::raw::c_int = 10;
pub const TM_RENDERER_LOGICAL_OP_OR_REVERSE: ::std::os::raw::c_int = 11;
pub const TM_RENDERER_LOGICAL_OP_COPY_INVERTED: ::std::os::raw::c_int = 12;
pub const TM_RENDERER_LOGICAL_OP_OR_INVERTED: ::std::os::raw::c_int = 13;
pub const TM_RENDERER_LOGICAL_OP_NAND: ::std::os::raw::c_int = 14;
pub const TM_RENDERER_LOGICAL_OP_SET: ::std::os::raw::c_int = 15;
pub const TM_RENDERER_LOGICAL_OP_MAX_VALUES: ::std::os::raw::c_int = 16;
pub type _bindgen_ty_23 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_value_logical_op_names: [*const ::std::os::raw::c_char; 16usize];
}
pub const TM_RENDERER_BLEND_OP_ADD: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_BLEND_OP_SUBTRACT: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_BLEND_OP_REVERSE_SUBTRACT: ::std::os::raw::c_int = 2;
pub const TM_RENDERER_BLEND_OP_MIN: ::std::os::raw::c_int = 3;
pub const TM_RENDERER_BLEND_OP_MAX: ::std::os::raw::c_int = 4;
pub const TM_RENDERER_BLEND_OP_MAX_VALUES: ::std::os::raw::c_int = 5;
pub type _bindgen_ty_24 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_value_blend_op_names: [*const ::std::os::raw::c_char; 5usize];
}
pub const TM_RENDERER_BLEND_WRITE_MASK_DISABLED: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_BLEND_WRITE_MASK_RED: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_BLEND_WRITE_MASK_GREEN: ::std::os::raw::c_int = 2;
pub const TM_RENDERER_BLEND_WRITE_MASK_BLUE: ::std::os::raw::c_int = 4;
pub const TM_RENDERER_BLEND_WRITE_MASK_ALPHA: ::std::os::raw::c_int = 8;
pub const TM_RENDERER_BLEND_WRITE_MASK_RED_GREEN: ::std::os::raw::c_int = 3;
pub const TM_RENDERER_BLEND_WRITE_MASK_RED_BLUE: ::std::os::raw::c_int = 5;
pub const TM_RENDERER_BLEND_WRITE_MASK_GREEN_BLUE: ::std::os::raw::c_int = 6;
pub const TM_RENDERER_BLEND_WRITE_MASK_RED_GREEN_BLUE: ::std::os::raw::c_int = 7;
pub const TM_RENDERER_BLEND_WRITE_MASK_RED_ALPHA: ::std::os::raw::c_int = 9;
pub const TM_RENDERER_BLEND_WRITE_MASK_GREEN_ALPHA: ::std::os::raw::c_int = 10;
pub const TM_RENDERER_BLEND_WRITE_MASK_RED_GREEN_ALPHA: ::std::os::raw::c_int = 9;
pub const TM_RENDERER_BLEND_WRITE_MASK_BLUE_ALPHA: ::std::os::raw::c_int = 12;
pub const TM_RENDERER_BLEND_WRITE_MASK_RED_BLUE_ALPHA: ::std::os::raw::c_int = 13;
pub const TM_RENDERER_BLEND_WRITE_MASK_GREEN_BLUE_ALPHA: ::std::os::raw::c_int = 14;
pub const TM_RENDERER_BLEND_WRITE_MASK_RED_GREEN_BLUE_ALPHA: ::std::os::raw::c_int = 15;
pub const TM_RENDERER_BLEND_WRITE_MASK_MAX_VALUES: ::std::os::raw::c_int = 16;
pub type _bindgen_ty_25 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_enum_value_write_mask: [u32; 16usize];
}
extern "C" {
    pub static mut tm_renderer_value_write_mask_names: [*const ::std::os::raw::c_char; 16usize];
}
pub const TM_RENDERER_BLEND_STATE_LOGICAL_OPERATION_ENABLE: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_BLEND_STATE_LOGICAL_OPERATION: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_BLEND_STATE_RENDER_TARGET_0: ::std::os::raw::c_int = 2;
pub const TM_RENDERER_BLEND_STATE_RENDER_TARGET_1: ::std::os::raw::c_int = 3;
pub const TM_RENDERER_BLEND_STATE_RENDER_TARGET_2: ::std::os::raw::c_int = 4;
pub const TM_RENDERER_BLEND_STATE_RENDER_TARGET_3: ::std::os::raw::c_int = 5;
pub const TM_RENDERER_BLEND_STATE_RENDER_TARGET_4: ::std::os::raw::c_int = 6;
pub const TM_RENDERER_BLEND_STATE_RENDER_TARGET_5: ::std::os::raw::c_int = 7;
pub const TM_RENDERER_BLEND_STATE_RENDER_TARGET_6: ::std::os::raw::c_int = 8;
pub const TM_RENDERER_BLEND_STATE_RENDER_TARGET_7: ::std::os::raw::c_int = 9;
pub const TM_RENDERER_BLEND_STATE_MAX_STATES: ::std::os::raw::c_int = 10;
pub type _bindgen_ty_26 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_blend_state_names: [*const ::std::os::raw::c_char; 10usize];
}
extern "C" {
    pub static mut tm_renderer_blend_state_types: [u32; 10usize];
}
pub const TM_RENDERER_MULTI_SAMPLE_SHADING_ENABLE: ::std::os::raw::c_int = 0;
pub const TM_RENDERER_MULTI_MIN_SAMPLE_SHADING: ::std::os::raw::c_int = 1;
pub const TM_RENDERER_MULTI_SAMPLE_ALPHA_TO_COVERAGE_ENABLE: ::std::os::raw::c_int = 2;
pub const TM_RENDERER_MULTI_SAMPLE_ALPHA_TO_ONE_ENABLE: ::std::os::raw::c_int = 3;
pub const TM_RENDERER_MULTI_SAMPLE_MAX_STATES: ::std::os::raw::c_int = 4;
pub type _bindgen_ty_27 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_renderer_multi_sample_state_names: [*const ::std::os::raw::c_char; 4usize];
}
extern "C" {
    pub static mut tm_renderer_multi_sample_state_types: [u32; 4usize];
}

// Extensions generated by machinery-generator

use const_cstr::{const_cstr, ConstCStr};

use crate::foundation::*;
use crate::plugins::ui::UiStyleT;

impl DeviceMemoryViewApi {
    pub unsafe fn create(&self, allocator: *mut AllocatorI) -> *mut DeviceMemoryViewO {
        self.create.unwrap()(allocator)
    }

    pub unsafe fn destroy(&self, view: *mut DeviceMemoryViewO) {
        self.destroy.unwrap()(view)
    }

    pub unsafe fn ui(
        &self,
        view: *mut DeviceMemoryViewO,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        content_r: RectT,
        tab_id: u64,
    ) {
        self.ui.unwrap()(view, ui, uistyle, content_r, tab_id)
    }
}

impl crate::Api for DeviceMemoryViewApi {
    const NAME: ConstCStr = const_cstr!("tm_device_memory_view_api");
}

impl NilRendererBackendApi {
    pub unsafe fn create(&self, allocator: *mut AllocatorI) -> *mut RendererBackendI {
        self.create.unwrap()(allocator)
    }

    pub unsafe fn destroy(&self, backend: *mut RendererBackendI) {
        self.destroy.unwrap()(backend)
    }
}

impl RendererApi {}

impl crate::Api for RendererApi {
    const NAME: ConstCStr = const_cstr!("tm_renderer_api");
}

impl RendererInitApi {
    pub unsafe fn init(&self, allocator: *mut AllocatorI, user_data_size: u64) -> *mut RendererApi {
        self.init.unwrap()(allocator, user_data_size)
    }

    pub unsafe fn shutdown(&self) {
        self.shutdown.unwrap()()
    }
}

impl crate::Api for RendererInitApi {
    const NAME: ConstCStr = const_cstr!("tm_renderer_init_api");
}

impl RendererCommandBufferApi {
    pub unsafe fn bind_render_pass(
        &self,
        inst: *mut RendererCommandBufferO,
        sort_key: u64,
        render_pass: *const RendererRenderPassBindT,
    ) {
        self.bind_render_pass.unwrap()(inst, sort_key, render_pass)
    }

    pub unsafe fn set_viewports(
        &self,
        inst: *mut RendererCommandBufferO,
        sort_key: u64,
        viewports: *const RendererSetViewportsT,
    ) {
        self.set_viewports.unwrap()(inst, sort_key, viewports)
    }

    pub unsafe fn set_scissor_rects(
        &self,
        inst: *mut RendererCommandBufferO,
        sort_key: u64,
        rects: *const RendererSetScissorRectsT,
    ) {
        self.set_scissor_rects.unwrap()(inst, sort_key, rects)
    }

    pub unsafe fn draw_calls(
        &self,
        inst: *mut RendererCommandBufferO,
        sort_keys: *const u64,
        draw_calls: *const RendererDrawCallInfoT,
        shaders: *const RendererShaderInfoT,
        num_draw_calls: u32,
    ) {
        self.draw_calls.unwrap()(inst, sort_keys, draw_calls, shaders, num_draw_calls)
    }

    pub unsafe fn bind_queue(
        &self,
        inst: *mut RendererCommandBufferO,
        sort_key: u64,
        queue_bind: *const RendererQueueBindT,
    ) {
        self.bind_queue.unwrap()(inst, sort_key, queue_bind)
    }

    pub unsafe fn begin_statistics_scope(
        &self,
        inst: *mut RendererCommandBufferO,
        sort_key: u64,
        category: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
        flags: u32,
    ) -> u64 {
        self.begin_statistics_scope.unwrap()(inst, sort_key, category, name, flags)
    }

    pub unsafe fn end_statistics_scope(
        &self,
        inst: *mut RendererCommandBufferO,
        sort_key: u64,
        statistics_scope: u64,
    ) {
        self.end_statistics_scope.unwrap()(inst, sort_key, statistics_scope)
    }

    pub unsafe fn compute_dispatches(
        &self,
        inst: *mut RendererCommandBufferO,
        sort_keys: *const u64,
        compute_dispatches: *const RendererComputeInfoT,
        shaders: *const RendererShaderInfoT,
        num_compute_dispatches: u32,
    ) {
        self.compute_dispatches.unwrap()(
            inst,
            sort_keys,
            compute_dispatches,
            shaders,
            num_compute_dispatches,
        )
    }

    pub unsafe fn transition_resources(
        &self,
        inst: *mut RendererCommandBufferO,
        sort_key: u64,
        barriers: *const RendererResourceBarrierT,
        num_barriers: u32,
    ) {
        self.transition_resources.unwrap()(inst, sort_key, barriers, num_barriers)
    }

    pub unsafe fn copy_image(
        &self,
        inst: *mut RendererCommandBufferO,
        sort_key: u64,
        copy_image: *const RendererCopyImageT,
    ) {
        self.copy_image.unwrap()(inst, sort_key, copy_image)
    }

    pub unsafe fn copy_buffer(
        &self,
        inst: *mut RendererCommandBufferO,
        sort_key: u64,
        copy_buffer: *const RendererCopyBufferT,
    ) {
        self.copy_buffer.unwrap()(inst, sort_key, copy_buffer)
    }

    pub unsafe fn trace_dispatches(
        &self,
        inst: *mut RendererCommandBufferO,
        sort_keys: *const u64,
        trace_calls: *const RendererTraceCallT,
        num_trace_calls: u32,
    ) {
        self.trace_dispatches.unwrap()(inst, sort_keys, trace_calls, num_trace_calls)
    }

    pub unsafe fn read_image(
        &self,
        inst: *mut RendererCommandBufferO,
        sort_key: u64,
        read_image: *const RendererReadImageT,
    ) -> u32 {
        self.read_image.unwrap()(inst, sort_key, read_image)
    }

    pub unsafe fn read_buffer(
        &self,
        inst: *mut RendererCommandBufferO,
        sort_key: u64,
        read_buffer: *const RendererReadBufferT,
    ) -> u32 {
        self.read_buffer.unwrap()(inst, sort_key, read_buffer)
    }

    pub unsafe fn num_commands(&self, inst: *const RendererCommandBufferO) -> u32 {
        self.num_commands.unwrap()(inst)
    }

    pub unsafe fn commands(&self, inst: *const RendererCommandBufferO) -> RendererCommandsT {
        self.commands.unwrap()(inst)
    }

    pub unsafe fn user_data(
        &self,
        inst: *mut RendererCommandBufferO,
    ) -> *mut ::std::os::raw::c_void {
        self.user_data.unwrap()(inst)
    }

    pub unsafe fn set_backend_allocator(
        &self,
        inst: *mut RendererCommandBufferO,
        memory_allocator: *mut RendererDeviceMemoryAllocatorI,
    ) {
        self.set_backend_allocator.unwrap()(inst, memory_allocator)
    }

    pub unsafe fn append_buffers(
        &self,
        inst: *mut RendererCommandBufferO,
        buffers: *mut *mut RendererCommandBufferO,
        num_buffers: u32,
    ) {
        self.append_buffers.unwrap()(inst, buffers, num_buffers)
    }
}

impl RendererCommandBufferPoolApi {
    pub unsafe fn create(
        &self,
        inst: *mut RendererCommandBufferPoolO,
    ) -> *mut RendererCommandBufferO {
        self.create.unwrap()(inst)
    }

    pub unsafe fn destroy(
        &self,
        inst: *mut RendererCommandBufferPoolO,
        buffer: *mut RendererCommandBufferO,
    ) {
        self.destroy.unwrap()(inst, buffer)
    }

    pub unsafe fn user_data_size(&self, inst: *mut RendererCommandBufferPoolO) -> u64 {
        self.user_data_size.unwrap()(inst)
    }
}

impl RendererCommandBufferSortApi {
    pub unsafe fn sort_memory_needed(
        &self,
        buffers: *mut *const RendererCommandBufferO,
        num_buffers: u32,
    ) -> u64 {
        self.sort_memory_needed.unwrap()(buffers, num_buffers)
    }

    pub unsafe fn sort_commands(
        &self,
        buffers: *mut *const RendererCommandBufferO,
        num_buffers: u32,
        sort_memory: *mut ::std::os::raw::c_void,
        sorted_output: *mut *mut RendererCommandsT,
        num_commands: *mut u32,
    ) {
        self.sort_commands.unwrap()(
            buffers,
            num_buffers,
            sort_memory,
            sorted_output,
            num_commands,
        )
    }
}

impl RendererResourceCommandBufferApi {
    pub unsafe fn create_buffer(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        buffer: *const RendererBufferDescT,
        device_affinity_mask: u32,
    ) -> RendererHandleT {
        self.create_buffer.unwrap()(inst, buffer, device_affinity_mask)
    }

    pub unsafe fn map_create_buffer(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        buffer: *const RendererBufferDescT,
        device_affinity_mask: u32,
        map_flags: RendererMapFlags,
        data: *mut *mut ::std::os::raw::c_void,
    ) -> RendererHandleT {
        self.map_create_buffer.unwrap()(inst, buffer, device_affinity_mask, map_flags, data)
    }

    pub unsafe fn resize_buffer(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        handle: *mut RendererHandleT,
        size: u64,
    ) {
        self.resize_buffer.unwrap()(inst, handle, size)
    }

    pub unsafe fn update_buffer(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        handle: RendererHandleT,
        offset: u64,
        size: u64,
        device_affinity_mask: u32,
        map_flags: RendererMapFlags,
        data: *mut *mut ::std::os::raw::c_void,
    ) {
        self.update_buffer.unwrap()(
            inst,
            handle,
            offset,
            size,
            device_affinity_mask,
            map_flags,
            data,
        )
    }

    pub unsafe fn fill_buffer(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        handle: RendererHandleT,
        offset: u64,
        size: u64,
        data: u32,
        device_affinity_mask: u32,
    ) {
        self.fill_buffer.unwrap()(inst, handle, offset, size, data, device_affinity_mask)
    }

    pub unsafe fn create_image(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        image: *const RendererImageDescT,
        device_affinity_mask: u32,
    ) -> RendererHandleT {
        self.create_image.unwrap()(inst, image, device_affinity_mask)
    }

    pub unsafe fn map_create_image(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        image: *const RendererImageDescT,
        device_affinity_mask: u32,
        map_flags: RendererMapFlags,
        data: *mut *mut ::std::os::raw::c_void,
    ) -> RendererHandleT {
        self.map_create_image.unwrap()(inst, image, device_affinity_mask, map_flags, data)
    }

    pub unsafe fn update_image(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        handle: RendererHandleT,
        resource_state: u16,
        format: u32,
        subresource: *const RendererImageSubresourceLayersT,
        offset: *const RendererImageOffsetT,
        extent: *const RendererImageExtentT,
        device_affinity_mask: u32,
        map_flags: RendererMapFlags,
        data: *mut *mut ::std::os::raw::c_void,
    ) {
        self.update_image.unwrap()(
            inst,
            handle,
            resource_state,
            format,
            subresource,
            offset,
            extent,
            device_affinity_mask,
            map_flags,
            data,
        )
    }

    pub unsafe fn create_image_views(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        image: RendererHandleT,
        desc: *const RendererImageDescT,
        device_affinity_mask: u32,
        views: *const RendererImageViewT,
        num_views: u32,
    ) -> RendererHandleT {
        self.create_image_views.unwrap()(inst, image, desc, device_affinity_mask, views, num_views)
    }

    pub unsafe fn create_shader(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        shader: *const RendererShaderT,
        device_affinity_mask: u32,
    ) -> RendererHandleT {
        self.create_shader.unwrap()(inst, shader, device_affinity_mask)
    }

    pub unsafe fn create_sampler(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        sampler: *const RendererSamplerT,
        device_affinity_mask: u32,
    ) -> RendererHandleT {
        self.create_sampler.unwrap()(inst, sampler, device_affinity_mask)
    }

    pub unsafe fn create_shader_state_override(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        state_override: *const RendererShaderStateOverrideT,
        device_affinity_mask: u32,
    ) -> RendererHandleT {
        self.create_shader_state_override.unwrap()(inst, state_override, device_affinity_mask)
    }

    pub unsafe fn create_queue_fence(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        device_affinity_mask: u32,
    ) -> RendererHandleT {
        self.create_queue_fence.unwrap()(inst, device_affinity_mask)
    }

    pub unsafe fn create_resource_binder(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        bind_points: *const RendererResourceBindPointT,
        num_bind_points: u32,
        device_affinity_mask: u32,
    ) -> RendererHandleT {
        self.create_resource_binder.unwrap()(
            inst,
            bind_points,
            num_bind_points,
            device_affinity_mask,
        )
    }

    pub unsafe fn create_ray_tracing_pipeline(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        pipeline: *const RendererRayTracingPipelineDescT,
        device_affinity_mask: u32,
    ) -> RendererHandleT {
        self.create_ray_tracing_pipeline.unwrap()(inst, pipeline, device_affinity_mask)
    }

    pub unsafe fn create_shader_binding_table(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        shader_binding_table: *const RendererShaderBindingTableDescT,
        device_affinity_mask: u32,
    ) -> RendererHandleT {
        self.create_shader_binding_table.unwrap()(inst, shader_binding_table, device_affinity_mask)
    }

    pub unsafe fn create_top_level_acceleration_structure(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        acceleration_structure: *const RendererTopLevelAccelerationStructureDescT,
        device_affinity_mask: u32,
    ) -> RendererHandleT {
        self.create_top_level_acceleration_structure.unwrap()(
            inst,
            acceleration_structure,
            device_affinity_mask,
        )
    }

    pub unsafe fn create_bottom_level_acceleration_structure(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        acceleration_structure: *const RendererBottomLevelAccelerationStructureDescT,
        device_affinity_mask: u32,
    ) -> RendererHandleT {
        self.create_bottom_level_acceleration_structure.unwrap()(
            inst,
            acceleration_structure,
            device_affinity_mask,
        )
    }

    pub unsafe fn set_resource(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        resource_binder: RendererHandleT,
        bind_point: u32,
        resource_handles: RendererHandleT,
        reosource_aspect_flags: u32,
    ) {
        self.set_resource.unwrap()(
            inst,
            resource_binder,
            bind_point,
            resource_handles,
            reosource_aspect_flags,
        )
    }

    pub unsafe fn set_array_resources(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        resource_binder: RendererHandleT,
        bind_point: u32,
        first_element: u32,
        resource_handles: *const RendererHandleT,
        resource_aspect_flags: *const u32,
        n_handles: u32,
    ) {
        self.set_array_resources.unwrap()(
            inst,
            resource_binder,
            bind_point,
            first_element,
            resource_handles,
            resource_aspect_flags,
            n_handles,
        )
    }

    pub unsafe fn destroy_resource(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        handle: RendererHandleT,
    ) {
        self.destroy_resource.unwrap()(inst, handle)
    }

    pub unsafe fn num_commands(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        count_per_command_type: *mut u32,
    ) -> u32 {
        self.num_commands.unwrap()(inst, count_per_command_type)
    }

    pub unsafe fn commands(
        &self,
        inst: *mut RendererResourceCommandBufferO,
    ) -> RendererResourceCommandsT {
        self.commands.unwrap()(inst)
    }

    pub unsafe fn user_data(
        &self,
        inst: *mut RendererResourceCommandBufferO,
    ) -> *mut ::std::os::raw::c_void {
        self.user_data.unwrap()(inst)
    }

    pub unsafe fn set_backend_allocators(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        resource_allocator: *mut RendererBackendResourceAllocatorI,
        memory_allocator: *mut RendererDeviceMemoryAllocatorI,
    ) {
        self.set_backend_allocators.unwrap()(inst, resource_allocator, memory_allocator)
    }

    pub unsafe fn append_buffers(
        &self,
        inst: *mut RendererResourceCommandBufferO,
        buffers: *mut *mut RendererResourceCommandBufferO,
        num_buffers: u32,
    ) {
        self.append_buffers.unwrap()(inst, buffers, num_buffers)
    }
}

impl RendererResourceCommandBufferPoolApi {
    pub unsafe fn create(
        &self,
        inst: *mut RendererResourceCommandBufferPoolO,
    ) -> *mut RendererResourceCommandBufferO {
        self.create.unwrap()(inst)
    }

    pub unsafe fn destroy(
        &self,
        inst: *mut RendererResourceCommandBufferPoolO,
        buffer: *mut RendererResourceCommandBufferO,
    ) {
        self.destroy.unwrap()(inst, buffer)
    }

    pub unsafe fn user_data_size(&self, inst: *mut RendererResourceCommandBufferPoolO) -> u64 {
        self.user_data_size.unwrap()(inst)
    }
}

impl RendererShaderCompilerApi {
    pub unsafe fn init(&self, allocator: *mut AllocatorI) -> *mut RendererShaderCompilerO {
        self.init.unwrap()(allocator)
    }

    pub unsafe fn shutdown(&self, inst: *mut RendererShaderCompilerO) {
        self.shutdown.unwrap()(inst)
    }

    pub unsafe fn num_state_block_types(&self, inst: *mut RendererShaderCompilerO) -> u32 {
        self.num_state_block_types.unwrap()(inst)
    }

    pub unsafe fn state_block_type(
        &self,
        inst: *mut RendererShaderCompilerO,
        state_block_type_idx: u32,
    ) -> u32 {
        self.state_block_type.unwrap()(inst, state_block_type_idx)
    }

    pub unsafe fn state_block_name(
        &self,
        inst: *mut RendererShaderCompilerO,
        state_block_type: u32,
    ) -> *const ::std::os::raw::c_char {
        self.state_block_name.unwrap()(inst, state_block_type)
    }

    pub unsafe fn num_states(
        &self,
        inst: *mut RendererShaderCompilerO,
        state_block_type: u32,
    ) -> u32 {
        self.num_states.unwrap()(inst, state_block_type)
    }

    pub unsafe fn state_name(
        &self,
        inst: *mut RendererShaderCompilerO,
        state_block_type: u32,
        state: u32,
    ) -> *const ::std::os::raw::c_char {
        self.state_name.unwrap()(inst, state_block_type, state)
    }

    pub unsafe fn value_type(
        &self,
        inst: *mut RendererShaderCompilerO,
        state_block_type: u32,
        state: u32,
    ) -> u32 {
        self.value_type.unwrap()(inst, state_block_type, state)
    }

    pub unsafe fn num_values(&self, inst: *mut RendererShaderCompilerO, value_type: u32) -> u32 {
        self.num_values.unwrap()(inst, value_type)
    }

    pub unsafe fn value_name(
        &self,
        inst: *mut RendererShaderCompilerO,
        value_type: u32,
        value: u32,
    ) -> *const ::std::os::raw::c_char {
        self.value_name.unwrap()(inst, value_type, value)
    }

    pub unsafe fn enum_value(
        &self,
        inst: *mut RendererShaderCompilerO,
        value_type: u32,
        value: u32,
    ) -> u32 {
        self.enum_value.unwrap()(inst, value_type, value)
    }

    pub unsafe fn compile_state_block(
        &self,
        inst: *mut RendererShaderCompilerO,
        bind_type: u32,
        block_type: u32,
        states: *const RendererStateValuePairT,
        num_raster_states: u32,
    ) -> RendererShaderBlobT {
        self.compile_state_block.unwrap()(inst, bind_type, block_type, states, num_raster_states)
    }

    pub unsafe fn compile_shader(
        &self,
        inst: *mut RendererShaderCompilerO,
        source: *const ::std::os::raw::c_char,
        entry_point: *const ::std::os::raw::c_char,
        source_language: u32,
        stage: u32,
    ) -> RendererShaderBlobT {
        self.compile_shader.unwrap()(inst, source, entry_point, source_language, stage)
    }

    pub unsafe fn bindless(&self, inst: *mut RendererShaderCompilerO) -> bool {
        self.bindless.unwrap()(inst)
    }

    pub unsafe fn bindless_access_buffer(
        &self,
        inst: *mut RendererShaderCompilerO,
        usage_flags: u32,
    ) -> RendererBindlessAccessorT {
        self.bindless_access_buffer.unwrap()(inst, usage_flags)
    }

    pub unsafe fn bindless_access_image(
        &self,
        inst: *mut RendererShaderCompilerO,
        type_: u32,
        usage_flags: u32,
    ) -> RendererBindlessAccessorT {
        self.bindless_access_image.unwrap()(inst, type_, usage_flags)
    }

    pub unsafe fn bindless_access_sampler(
        &self,
        inst: *mut RendererShaderCompilerO,
    ) -> RendererBindlessAccessorT {
        self.bindless_access_sampler.unwrap()(inst)
    }

    pub unsafe fn bindless_access_acceleration_structure(
        &self,
        inst: *mut RendererShaderCompilerO,
    ) -> RendererBindlessAccessorT {
        self.bindless_access_acceleration_structure.unwrap()(inst)
    }

    pub unsafe fn release_blob(
        &self,
        inst: *mut RendererShaderCompilerO,
        blob: RendererShaderBlobT,
    ) {
        self.release_blob.unwrap()(inst, blob)
    }
}

pub const TM_TYPE_HASH__RENDERER_COMPUTE_INFO_T: StrhashT = StrhashT {
    u64_: 14466277375278292668u64,
};
pub const TM_TYPE_HASH__RENDERER_DRAW_CALL_INFO_T: StrhashT = StrhashT {
    u64_: 10978780929728728095u64,
};
pub const TM_NIL_RENDER_BACKEND_INTERFACE_HASH: StrhashT = StrhashT {
    u64_: 11404379728544521701u64,
};
pub const TM_TYPE_HASH__RENDERER_BUFFER_DESC: StrhashT = StrhashT {
    u64_: 6564013608050018986u64,
};
pub const TM_TYPE_HASH__RENDERER_IMAGE_DESC: StrhashT = StrhashT {
    u64_: 11834704360853175208u64,
};
