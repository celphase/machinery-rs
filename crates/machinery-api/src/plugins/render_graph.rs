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
pub const TM_RENDER_GRAPH_API_NAME: &'static [u8; 20usize] = b"tm_render_graph_api\0";
pub const TM_RENDER_GRAPH_MODULE_API_NAME: &'static [u8; 27usize] = b"tm_render_graph_module_api\0";
pub const TM_RENDER_GRAPH_SETUP_API_NAME: &'static [u8; 26usize] = b"tm_render_graph_setup_api\0";
pub const TM_RENDER_GRAPH_EXECUTE_API_NAME: &'static [u8; 28usize] =
    b"tm_render_graph_execute_api\0";
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
#[derive(Copy, Clone)]
pub struct RenderGraphO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderGraphSetupO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderGraphModuleO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderGraphExecuteO {
    _unused: [u8; 0],
}
pub const TM_RENDER_GRAPH_SORT_SUB_MODULE_BITS_START: RenderGraphSortKey = 0;
pub const TM_RENDER_GRAPH_SORT_SUB_MODULE_BITS: RenderGraphSortKey = 8;
pub const TM_RENDER_GRAPH_SORT_DEPTH_BITS_START: RenderGraphSortKey = 8;
pub const TM_RENDER_GRAPH_SORT_DEPTH_BITS: RenderGraphSortKey = 16;
pub const TM_RENDER_GRAPH_SORT_INTERNAL_PASS_BITS_START: RenderGraphSortKey = 24;
pub const TM_RENDER_GRAPH_SORT_INTERNAL_PASS_BITS: RenderGraphSortKey = 27;
pub const TM_RENDER_GRAPH_SORT_PASS_BITS_START: RenderGraphSortKey = 51;
pub const TM_RENDER_GRAPH_SORT_PASS_BITS: RenderGraphSortKey = 11;
pub const TM_RENDER_GRAPH_SORT_GRAPH_LOOP_BITS_START: RenderGraphSortKey = 62;
pub const TM_RENDER_GRAPH_SORT_GRAPH_LOOP_BITS: RenderGraphSortKey = 2;
pub type RenderGraphSortKey = ::std::os::raw::c_int;
pub const TM_RENDER_GRAPH_SORT_ORDER_FRONT_BACK: RenderGraphSortOrder = 0;
pub const TM_RENDER_GRAPH_SORT_ORDER_BACK_FRONT: RenderGraphSortOrder = 1;
pub type RenderGraphSortOrder = ::std::os::raw::c_int;
pub const TM_RENDER_GRAPH_WRITE_BIND_FLAG_UAV: RenderGraphWriteBindFlag = 1;
pub const TM_RENDER_GRAPH_WRITE_BIND_FLAG_COLOR_TARGET: RenderGraphWriteBindFlag = 2;
pub const TM_RENDER_GRAPH_WRITE_BIND_FLAG_DEPTH_STENCIL_TARGET: RenderGraphWriteBindFlag = 4;
pub type RenderGraphWriteBindFlag = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RenderGraphHandleT {
    pub value: u32,
}
#[repr(C)]
pub struct RenderGraphBlackboardValue {
    pub __bindgen_anon_1: __BindgenUnionField<RenderGraphBlackboardValueBindgenTy1>,
    pub data: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    pub float32: __BindgenUnionField<f32>,
    pub uint64: __BindgenUnionField<u64>,
    pub uint32: __BindgenUnionField<u32>,
    pub int64: __BindgenUnionField<i64>,
    pub int32: __BindgenUnionField<i32>,
    pub boolean: __BindgenUnionField<bool>,
    pub bindgen_union_field: [u64; 2usize],
}
#[repr(C)]
pub struct RenderGraphBlackboardValueBindgenTy1 {
    pub handle: RendererHandleT,
    pub _padding_91: [::std::os::raw::c_char; 4usize],
}
impl Default for RenderGraphBlackboardValueBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for RenderGraphBlackboardValue {
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
pub struct RenderGraphViewerT {
    pub sort_key: u64,
    pub visibility_mask: u64,
    pub viewer_system: *mut ShaderSystemO,
    pub viewer_cbuffer: *mut ShaderConstantBufferInstanceT,
    pub viewer_rbinder: *mut ShaderResourceBinderInstanceT,
    pub camera: *const CameraT,
}
impl Default for RenderGraphViewerT {
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
pub struct RenderGraphApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            sort_key: u64,
            static_sort_key: bool,
            allocator: *mut AllocatorI,
            shader_repository: *mut ShaderRepositoryO,
        ) -> *mut RenderGraphO,
    >,
    pub destroy: ::std::option::Option<
        unsafe extern "C" fn(
            graph: *mut RenderGraphO,
            res_buf: *mut RendererResourceCommandBufferO,
        ),
    >,
    pub sort_key: ::std::option::Option<
        unsafe extern "C" fn(graph: *const RenderGraphO, material_layer_name: StrhashT) -> u64,
    >,
    pub register_gpu_image: ::std::option::Option<
        unsafe extern "C" fn(
            graph: *mut RenderGraphO,
            name: StrhashT,
            resource: RendererHandleT,
            resource_state: u32,
            image_desc: *const RendererImageDescT,
        ),
    >,
    pub register_gpu_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            graph: *mut RenderGraphO,
            name: StrhashT,
            resource: RendererHandleT,
            resource_state: u32,
            buffer_desc: *const RendererBufferDescT,
        ),
    >,
    pub write_blackboard: ::std::option::Option<
        unsafe extern "C" fn(
            graph: *mut RenderGraphO,
            key: StrhashT,
            value: RenderGraphBlackboardValue,
        ),
    >,
    pub read_blackboard: ::std::option::Option<
        unsafe extern "C" fn(
            graph: *const RenderGraphO,
            key: StrhashT,
            value: *mut RenderGraphBlackboardValue,
        ) -> bool,
    >,
    pub setup_passes: ::std::option::Option<
        unsafe extern "C" fn(graph: *mut RenderGraphO, render_module: *mut RenderGraphModuleO),
    >,
    pub validate_and_build: ::std::option::Option<
        unsafe extern "C" fn(
            graph: *mut RenderGraphO,
            backend: *mut RendererBackendI,
            device_affinity_mask: u32,
        ) -> bool,
    >,
    pub visualize: ::std::option::Option<
        unsafe extern "C" fn(
            graph: *const RenderGraphO,
            ta: *mut TempAllocatorI,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub execute: ::std::option::Option<
        unsafe extern "C" fn(
            graph: *mut RenderGraphO,
            backend: *mut RendererBackendI,
            shader_context: *const ShaderSystemContextO,
            device_affinity_mask: u32,
        ) -> *mut AtomicCounterO,
    >,
    pub resource_buffers: ::std::option::Option<
        unsafe extern "C" fn(
            graph: *const RenderGraphO,
            resource_buffers: *mut *mut RendererResourceCommandBufferO,
            num_resource_buffers: *mut u32,
        ),
    >,
    pub command_buffers: ::std::option::Option<
        unsafe extern "C" fn(
            graph: *const RenderGraphO,
            command_buffers: *mut *mut RendererCommandBufferO,
            num_command_buffers: *mut u32,
        ),
    >,
    pub viewers: ::std::option::Option<
        unsafe extern "C" fn(
            graph: *mut RenderGraphO,
            viewers: *mut RenderGraphViewerT,
            num_viewers: *mut u32,
        ),
    >,
    pub backend_handle: ::std::option::Option<
        unsafe extern "C" fn(
            graph: *const RenderGraphO,
            graph_handle: RenderGraphHandleT,
        ) -> RendererHandleT,
    >,
    pub data_requested: ::std::option::Option<
        unsafe extern "C" fn(graph: *const RenderGraphO, name: StrhashT) -> bool,
    >,
    pub external_resource: ::std::option::Option<
        unsafe extern "C" fn(graph: *mut RenderGraphO, name: StrhashT) -> RenderGraphHandleT,
    >,
    pub image_desc: ::std::option::Option<
        unsafe extern "C" fn(
            graph: *mut RenderGraphO,
            handle: RenderGraphHandleT,
        ) -> *const RendererImageDescT,
    >,
    pub buffer_desc: ::std::option::Option<
        unsafe extern "C" fn(
            graph: *mut RenderGraphO,
            handle: RenderGraphHandleT,
        ) -> *const RendererBufferDescT,
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderGraphModuleEditorSubmenuI {
    pub submenu_name: *const ::std::os::raw::c_char,
    pub inst: *mut ::std::os::raw::c_void,
    pub active:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void) -> bool>,
    pub submenu: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            submenu_pos: Vec2T,
        ),
    >,
    pub draw_as_toolbar: ::std::option::Option<unsafe extern "C" fn() -> bool>,
}
impl Default for RenderGraphModuleEditorSubmenuI {
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
pub struct RenderGraphModuleApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            allocator: *mut AllocatorI,
            name: *const ::std::os::raw::c_char,
        ) -> *mut RenderGraphModuleO,
    >,
    pub destroy: ::std::option::Option<
        unsafe extern "C" fn(
            render_module: *mut RenderGraphModuleO,
            res_buf: *mut RendererResourceCommandBufferO,
        ),
    >,
    pub name: ::std::option::Option<
        unsafe extern "C" fn(render_module: *const RenderGraphModuleO) -> StrhashT,
    >,
    pub human_readable_name: ::std::option::Option<
        unsafe extern "C" fn(
            render_module: *const RenderGraphModuleO,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub add_pass: ::std::option::Option<
        unsafe extern "C" fn(render_module: *mut RenderGraphModuleO, pass: *const RenderGraphPassI),
    >,
    pub add_extension_point: ::std::option::Option<
        unsafe extern "C" fn(
            render_module: *mut RenderGraphModuleO,
            extension_point_name: StrhashT,
        ),
    >,
    pub insert_extension: ::std::option::Option<
        unsafe extern "C" fn(
            render_module: *mut RenderGraphModuleO,
            extension_point_name: StrhashT,
            extension: *mut RenderGraphModuleO,
            ordering_weight: f32,
        ) -> bool,
    >,
    pub remove_extension: ::std::option::Option<
        unsafe extern "C" fn(
            render_module: *mut RenderGraphModuleO,
            res_buf: *mut RendererResourceCommandBufferO,
            extension_point_name: StrhashT,
            extension: *mut RenderGraphModuleO,
        ) -> bool,
    >,
    pub add_sub_module: ::std::option::Option<
        unsafe extern "C" fn(
            render_module: *mut RenderGraphModuleO,
            shader_layer_name: StrhashT,
            sub_module: *mut RenderGraphModuleO,
        ),
    >,
    pub init_passes: ::std::option::Option<
        unsafe extern "C" fn(
            render_module: *mut RenderGraphModuleO,
            res_buf: *mut RendererResourceCommandBufferO,
        ),
    >,
    pub set_editor_submenu_interface: ::std::option::Option<
        unsafe extern "C" fn(
            render_module: *mut RenderGraphModuleO,
            submenu_i: *const RenderGraphModuleEditorSubmenuI,
        ),
    >,
    pub editor_submenu: ::std::option::Option<
        unsafe extern "C" fn(
            render_module: *mut RenderGraphModuleO,
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            submenu_pos: Vec2T,
        ),
    >,
    pub set_editor_toolbar: ::std::option::Option<
        unsafe extern "C" fn(render_module: *mut RenderGraphModuleO, toolbar: *mut ToolbarI),
    >,
    pub editor_toolbars: ::std::option::Option<
        unsafe extern "C" fn(
            render_module: *mut RenderGraphModuleO,
            ta: *mut TempAllocatorI,
        ) -> *mut ToolbarI,
    >,
    pub create_persistent_gpu_image: ::std::option::Option<
        unsafe extern "C" fn(
            render_module: *mut RenderGraphModuleO,
            name: StrhashT,
            image_desc: *const RendererImageDescT,
            inherits: StrhashT,
            scale: *const f32,
        ),
    >,
    pub create_persistent_gpu_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            render_module: *mut RenderGraphModuleO,
            name: StrhashT,
            buffer_desc: *const RendererBufferDescT,
            inherits: StrhashT,
        ),
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RenderGraphSetupApi {
    pub external_resource: ::std::option::Option<
        unsafe extern "C" fn(
            graph_setup: *mut RenderGraphSetupO,
            name: StrhashT,
        ) -> RenderGraphHandleT,
    >,
    pub image_desc: ::std::option::Option<
        unsafe extern "C" fn(
            graph_setup: *mut RenderGraphSetupO,
            handle: RenderGraphHandleT,
        ) -> *const RendererImageDescT,
    >,
    pub buffer_desc: ::std::option::Option<
        unsafe extern "C" fn(
            graph_setup: *mut RenderGraphSetupO,
            handle: RenderGraphHandleT,
        ) -> *const RendererBufferDescT,
    >,
    pub shader_repository: ::std::option::Option<
        unsafe extern "C" fn(graph_setup: *mut RenderGraphSetupO) -> *mut ShaderRepositoryO,
    >,
    pub create_gpu_images: ::std::option::Option<
        unsafe extern "C" fn(
            graph_setup: *mut RenderGraphSetupO,
            image_descs: *const RendererImageDescT,
            num_images: u32,
            handles: *mut RenderGraphHandleT,
        ),
    >,
    pub create_gpu_buffers: ::std::option::Option<
        unsafe extern "C" fn(
            graph_setup: *mut RenderGraphSetupO,
            buffer_descs: *const RendererBufferDescT,
            num_buffers: u32,
            handles: *mut RenderGraphHandleT,
        ),
    >,
    pub read_gpu_resource: ::std::option::Option<
        unsafe extern "C" fn(
            graph_setup: *mut RenderGraphSetupO,
            handle: RenderGraphHandleT,
            wanted_resource_state: u32,
            subresource_view: *const RendererImageViewT,
        ),
    >,
    pub write_gpu_resource: ::std::option::Option<
        unsafe extern "C" fn(
            graph_setup: *mut RenderGraphSetupO,
            handle: RenderGraphHandleT,
            write_bind_flags: RenderGraphWriteBindFlag,
            wanted_resource_state: u32,
            load_op: u32,
            bind_slot: u32,
            blackboard_key: StrhashT,
            subresource_view: *const RendererImageViewT,
        ),
    >,
    pub set_active: ::std::option::Option<
        unsafe extern "C" fn(graph_setup: *mut RenderGraphSetupO, state: bool),
    >,
    pub set_early_out: ::std::option::Option<
        unsafe extern "C" fn(graph_setup: *mut RenderGraphSetupO, state: bool),
    >,
    pub set_output: ::std::option::Option<
        unsafe extern "C" fn(graph_setup: *mut RenderGraphSetupO, state: bool),
    >,
    pub set_request_async_compute: ::std::option::Option<
        unsafe extern "C" fn(graph_setup: *mut RenderGraphSetupO, state: bool),
    >,
    pub set_request_multi_gpu: ::std::option::Option<
        unsafe extern "C" fn(graph_setup: *mut RenderGraphSetupO, device_affinity_mask: u32),
    >,
    pub expose_as_material_layer: ::std::option::Option<
        unsafe extern "C" fn(graph_setup: *mut RenderGraphSetupO, layer_name: StrhashT),
    >,
    pub write_blackboard: ::std::option::Option<
        unsafe extern "C" fn(
            graph_setup: *mut RenderGraphSetupO,
            key: StrhashT,
            value: RenderGraphBlackboardValue,
        ),
    >,
    pub read_blackboard: ::std::option::Option<
        unsafe extern "C" fn(
            graph_setup: *mut RenderGraphSetupO,
            key: StrhashT,
            value: *mut RenderGraphBlackboardValue,
        ) -> bool,
    >,
    pub request_data: ::std::option::Option<
        unsafe extern "C" fn(graph_setup: *mut RenderGraphSetupO, name: StrhashT),
    >,
}
#[repr(C)]
pub struct RenderGraphLayerSortKeyT {
    pub layer_name: StrhashT,
    pub sort_key: u64,
}
impl Default for RenderGraphLayerSortKeyT {
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
pub struct RenderGraphExecuteApi {
    pub default_command_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            graph_execute: *mut RenderGraphExecuteO,
        ) -> *mut RendererCommandBufferO,
    >,
    pub default_resource_command_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            graph_execute: *mut RenderGraphExecuteO,
        ) -> *mut RendererResourceCommandBufferO,
    >,
    pub shader_context: ::std::option::Option<
        unsafe extern "C" fn(
            graph_execute: *mut RenderGraphExecuteO,
        ) -> *const ShaderSystemContextO,
    >,
    pub device_affinity_mask:
        ::std::option::Option<unsafe extern "C" fn(graph_execute: *mut RenderGraphExecuteO) -> u32>,
    pub backend_handle: ::std::option::Option<
        unsafe extern "C" fn(
            graph_execute: *mut RenderGraphExecuteO,
            graph_handle: RenderGraphHandleT,
            subresource_view: *const RendererImageViewT,
        ) -> RendererHandleT,
    >,
    pub run_sub_module: ::std::option::Option<
        unsafe extern "C" fn(
            graph_execute: *mut RenderGraphExecuteO,
            sub_module_name: StrhashT,
            sort_key: u64,
            commands: *mut RendererCommandBufferO,
        ),
    >,
    pub read_blackboard: ::std::option::Option<
        unsafe extern "C" fn(
            graph_execute: *mut RenderGraphExecuteO,
            key: StrhashT,
            value: *mut RenderGraphBlackboardValue,
        ) -> bool,
    >,
    pub append_viewers: ::std::option::Option<
        unsafe extern "C" fn(
            graph_execute: *mut RenderGraphExecuteO,
            viewers: *const RenderGraphViewerT,
            num_viewers: u32,
        ) -> u32,
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RenderGraphPassApi {
    pub init_pass: ::std::option::Option<
        unsafe extern "C" fn(
            const_data: *mut ::std::os::raw::c_void,
            allocator: *mut AllocatorI,
            res_buf: *mut RendererResourceCommandBufferO,
        ),
    >,
    pub shutdown_pass: ::std::option::Option<
        unsafe extern "C" fn(
            const_data: *mut ::std::os::raw::c_void,
            allocator: *mut AllocatorI,
            res_buf: *mut RendererResourceCommandBufferO,
        ),
    >,
    pub setup_pass: ::std::option::Option<
        unsafe extern "C" fn(
            const_data: *const ::std::os::raw::c_void,
            runtime_data: *mut ::std::os::raw::c_void,
            graph_setup: *mut RenderGraphSetupO,
        ),
    >,
    pub execute_pass: ::std::option::Option<
        unsafe extern "C" fn(
            const_data: *const ::std::os::raw::c_void,
            runtime_data: *mut ::std::os::raw::c_void,
            sort_key: u64,
            graph_execute: *mut RenderGraphExecuteO,
        ),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderGraphPassI {
    pub const_data: *mut ::std::os::raw::c_void,
    pub const_data_size: u64,
    pub runtime_data_size: u64,
    pub profiling_scope: *const ::std::os::raw::c_char,
    pub api: RenderGraphPassApi,
}
impl Default for RenderGraphPassI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

// Extensions generated by machinery-generator

use const_cstr::{const_cstr, ConstCStr};

use crate::foundation::*;
use crate::plugins::renderer::*;
use crate::plugins::shader_system::*;
use crate::plugins::ui::{ToolbarI, UiStyleT};

impl RenderGraphApi {
    pub unsafe fn create(
        &self,
        sort_key: u64,
        static_sort_key: bool,
        allocator: *mut AllocatorI,
        shader_repository: *mut ShaderRepositoryO,
    ) -> *mut RenderGraphO {
        self.create.unwrap()(sort_key, static_sort_key, allocator, shader_repository)
    }

    pub unsafe fn destroy(
        &self,
        graph: *mut RenderGraphO,
        res_buf: *mut RendererResourceCommandBufferO,
    ) {
        self.destroy.unwrap()(graph, res_buf)
    }

    pub unsafe fn sort_key(
        &self,
        graph: *const RenderGraphO,
        material_layer_name: StrhashT,
    ) -> u64 {
        self.sort_key.unwrap()(graph, material_layer_name)
    }

    pub unsafe fn register_gpu_image(
        &self,
        graph: *mut RenderGraphO,
        name: StrhashT,
        resource: RendererHandleT,
        resource_state: u32,
        image_desc: *const RendererImageDescT,
    ) {
        self.register_gpu_image.unwrap()(graph, name, resource, resource_state, image_desc)
    }

    pub unsafe fn register_gpu_buffer(
        &self,
        graph: *mut RenderGraphO,
        name: StrhashT,
        resource: RendererHandleT,
        resource_state: u32,
        buffer_desc: *const RendererBufferDescT,
    ) {
        self.register_gpu_buffer.unwrap()(graph, name, resource, resource_state, buffer_desc)
    }

    pub unsafe fn write_blackboard(
        &self,
        graph: *mut RenderGraphO,
        key: StrhashT,
        value: RenderGraphBlackboardValue,
    ) {
        self.write_blackboard.unwrap()(graph, key, value)
    }

    pub unsafe fn read_blackboard(
        &self,
        graph: *const RenderGraphO,
        key: StrhashT,
        value: *mut RenderGraphBlackboardValue,
    ) -> bool {
        self.read_blackboard.unwrap()(graph, key, value)
    }

    pub unsafe fn setup_passes(
        &self,
        graph: *mut RenderGraphO,
        render_module: *mut RenderGraphModuleO,
    ) {
        self.setup_passes.unwrap()(graph, render_module)
    }

    pub unsafe fn validate_and_build(
        &self,
        graph: *mut RenderGraphO,
        backend: *mut RendererBackendI,
        device_affinity_mask: u32,
    ) -> bool {
        self.validate_and_build.unwrap()(graph, backend, device_affinity_mask)
    }

    pub unsafe fn visualize(
        &self,
        graph: *const RenderGraphO,
        ta: *mut TempAllocatorI,
    ) -> *mut ::std::os::raw::c_char {
        self.visualize.unwrap()(graph, ta)
    }

    pub unsafe fn execute(
        &self,
        graph: *mut RenderGraphO,
        backend: *mut RendererBackendI,
        shader_context: *const ShaderSystemContextO,
        device_affinity_mask: u32,
    ) -> *mut AtomicCounterO {
        self.execute.unwrap()(graph, backend, shader_context, device_affinity_mask)
    }

    pub unsafe fn resource_buffers(
        &self,
        graph: *const RenderGraphO,
        resource_buffers: *mut *mut RendererResourceCommandBufferO,
        num_resource_buffers: *mut u32,
    ) {
        self.resource_buffers.unwrap()(graph, resource_buffers, num_resource_buffers)
    }

    pub unsafe fn command_buffers(
        &self,
        graph: *const RenderGraphO,
        command_buffers: *mut *mut RendererCommandBufferO,
        num_command_buffers: *mut u32,
    ) {
        self.command_buffers.unwrap()(graph, command_buffers, num_command_buffers)
    }

    pub unsafe fn viewers(
        &self,
        graph: *mut RenderGraphO,
        viewers: *mut RenderGraphViewerT,
        num_viewers: *mut u32,
    ) {
        self.viewers.unwrap()(graph, viewers, num_viewers)
    }

    pub unsafe fn backend_handle(
        &self,
        graph: *const RenderGraphO,
        graph_handle: RenderGraphHandleT,
    ) -> RendererHandleT {
        self.backend_handle.unwrap()(graph, graph_handle)
    }

    pub unsafe fn data_requested(&self, graph: *const RenderGraphO, name: StrhashT) -> bool {
        self.data_requested.unwrap()(graph, name)
    }

    pub unsafe fn external_resource(
        &self,
        graph: *mut RenderGraphO,
        name: StrhashT,
    ) -> RenderGraphHandleT {
        self.external_resource.unwrap()(graph, name)
    }

    pub unsafe fn image_desc(
        &self,
        graph: *mut RenderGraphO,
        handle: RenderGraphHandleT,
    ) -> *const RendererImageDescT {
        self.image_desc.unwrap()(graph, handle)
    }

    pub unsafe fn buffer_desc(
        &self,
        graph: *mut RenderGraphO,
        handle: RenderGraphHandleT,
    ) -> *const RendererBufferDescT {
        self.buffer_desc.unwrap()(graph, handle)
    }
}

impl crate::Api for RenderGraphApi {
    const NAME: ConstCStr = const_cstr!("tm_render_graph_api");
}

impl RenderGraphModuleApi {
    pub unsafe fn create(
        &self,
        allocator: *mut AllocatorI,
        name: *const ::std::os::raw::c_char,
    ) -> *mut RenderGraphModuleO {
        self.create.unwrap()(allocator, name)
    }

    pub unsafe fn destroy(
        &self,
        render_module: *mut RenderGraphModuleO,
        res_buf: *mut RendererResourceCommandBufferO,
    ) {
        self.destroy.unwrap()(render_module, res_buf)
    }

    pub unsafe fn name(&self, render_module: *const RenderGraphModuleO) -> StrhashT {
        self.name.unwrap()(render_module)
    }

    pub unsafe fn human_readable_name(
        &self,
        render_module: *const RenderGraphModuleO,
    ) -> *const ::std::os::raw::c_char {
        self.human_readable_name.unwrap()(render_module)
    }

    pub unsafe fn add_pass(
        &self,
        render_module: *mut RenderGraphModuleO,
        pass: *const RenderGraphPassI,
    ) {
        self.add_pass.unwrap()(render_module, pass)
    }

    pub unsafe fn add_extension_point(
        &self,
        render_module: *mut RenderGraphModuleO,
        extension_point_name: StrhashT,
    ) {
        self.add_extension_point.unwrap()(render_module, extension_point_name)
    }

    pub unsafe fn insert_extension(
        &self,
        render_module: *mut RenderGraphModuleO,
        extension_point_name: StrhashT,
        extension: *mut RenderGraphModuleO,
        ordering_weight: f32,
    ) -> bool {
        self.insert_extension.unwrap()(
            render_module,
            extension_point_name,
            extension,
            ordering_weight,
        )
    }

    pub unsafe fn remove_extension(
        &self,
        render_module: *mut RenderGraphModuleO,
        res_buf: *mut RendererResourceCommandBufferO,
        extension_point_name: StrhashT,
        extension: *mut RenderGraphModuleO,
    ) -> bool {
        self.remove_extension.unwrap()(render_module, res_buf, extension_point_name, extension)
    }

    pub unsafe fn add_sub_module(
        &self,
        render_module: *mut RenderGraphModuleO,
        shader_layer_name: StrhashT,
        sub_module: *mut RenderGraphModuleO,
    ) {
        self.add_sub_module.unwrap()(render_module, shader_layer_name, sub_module)
    }

    pub unsafe fn init_passes(
        &self,
        render_module: *mut RenderGraphModuleO,
        res_buf: *mut RendererResourceCommandBufferO,
    ) {
        self.init_passes.unwrap()(render_module, res_buf)
    }

    pub unsafe fn set_editor_submenu_interface(
        &self,
        render_module: *mut RenderGraphModuleO,
        submenu_i: *const RenderGraphModuleEditorSubmenuI,
    ) {
        self.set_editor_submenu_interface.unwrap()(render_module, submenu_i)
    }

    pub unsafe fn editor_submenu(
        &self,
        render_module: *mut RenderGraphModuleO,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        submenu_pos: Vec2T,
    ) {
        self.editor_submenu.unwrap()(render_module, ui, uistyle, submenu_pos)
    }

    pub unsafe fn set_editor_toolbar(
        &self,
        render_module: *mut RenderGraphModuleO,
        toolbar: *mut ToolbarI,
    ) {
        self.set_editor_toolbar.unwrap()(render_module, toolbar)
    }

    pub unsafe fn editor_toolbars(
        &self,
        render_module: *mut RenderGraphModuleO,
        ta: *mut TempAllocatorI,
    ) -> *mut ToolbarI {
        self.editor_toolbars.unwrap()(render_module, ta)
    }

    pub unsafe fn create_persistent_gpu_image(
        &self,
        render_module: *mut RenderGraphModuleO,
        name: StrhashT,
        image_desc: *const RendererImageDescT,
        inherits: StrhashT,
        scale: *const f32,
    ) {
        self.create_persistent_gpu_image.unwrap()(render_module, name, image_desc, inherits, scale)
    }

    pub unsafe fn create_persistent_gpu_buffer(
        &self,
        render_module: *mut RenderGraphModuleO,
        name: StrhashT,
        buffer_desc: *const RendererBufferDescT,
        inherits: StrhashT,
    ) {
        self.create_persistent_gpu_buffer.unwrap()(render_module, name, buffer_desc, inherits)
    }
}

impl crate::Api for RenderGraphModuleApi {
    const NAME: ConstCStr = const_cstr!("tm_render_graph_module_api");
}

impl RenderGraphSetupApi {
    pub unsafe fn external_resource(
        &self,
        graph_setup: *mut RenderGraphSetupO,
        name: StrhashT,
    ) -> RenderGraphHandleT {
        self.external_resource.unwrap()(graph_setup, name)
    }

    pub unsafe fn image_desc(
        &self,
        graph_setup: *mut RenderGraphSetupO,
        handle: RenderGraphHandleT,
    ) -> *const RendererImageDescT {
        self.image_desc.unwrap()(graph_setup, handle)
    }

    pub unsafe fn buffer_desc(
        &self,
        graph_setup: *mut RenderGraphSetupO,
        handle: RenderGraphHandleT,
    ) -> *const RendererBufferDescT {
        self.buffer_desc.unwrap()(graph_setup, handle)
    }

    pub unsafe fn shader_repository(
        &self,
        graph_setup: *mut RenderGraphSetupO,
    ) -> *mut ShaderRepositoryO {
        self.shader_repository.unwrap()(graph_setup)
    }

    pub unsafe fn create_gpu_images(
        &self,
        graph_setup: *mut RenderGraphSetupO,
        image_descs: *const RendererImageDescT,
        num_images: u32,
        handles: *mut RenderGraphHandleT,
    ) {
        self.create_gpu_images.unwrap()(graph_setup, image_descs, num_images, handles)
    }

    pub unsafe fn create_gpu_buffers(
        &self,
        graph_setup: *mut RenderGraphSetupO,
        buffer_descs: *const RendererBufferDescT,
        num_buffers: u32,
        handles: *mut RenderGraphHandleT,
    ) {
        self.create_gpu_buffers.unwrap()(graph_setup, buffer_descs, num_buffers, handles)
    }

    pub unsafe fn read_gpu_resource(
        &self,
        graph_setup: *mut RenderGraphSetupO,
        handle: RenderGraphHandleT,
        wanted_resource_state: u32,
        subresource_view: *const RendererImageViewT,
    ) {
        self.read_gpu_resource.unwrap()(
            graph_setup,
            handle,
            wanted_resource_state,
            subresource_view,
        )
    }

    pub unsafe fn write_gpu_resource(
        &self,
        graph_setup: *mut RenderGraphSetupO,
        handle: RenderGraphHandleT,
        write_bind_flags: RenderGraphWriteBindFlag,
        wanted_resource_state: u32,
        load_op: u32,
        bind_slot: u32,
        blackboard_key: StrhashT,
        subresource_view: *const RendererImageViewT,
    ) {
        self.write_gpu_resource.unwrap()(
            graph_setup,
            handle,
            write_bind_flags,
            wanted_resource_state,
            load_op,
            bind_slot,
            blackboard_key,
            subresource_view,
        )
    }

    pub unsafe fn set_active(&self, graph_setup: *mut RenderGraphSetupO, state: bool) {
        self.set_active.unwrap()(graph_setup, state)
    }

    pub unsafe fn set_early_out(&self, graph_setup: *mut RenderGraphSetupO, state: bool) {
        self.set_early_out.unwrap()(graph_setup, state)
    }

    pub unsafe fn set_output(&self, graph_setup: *mut RenderGraphSetupO, state: bool) {
        self.set_output.unwrap()(graph_setup, state)
    }

    pub unsafe fn set_request_async_compute(
        &self,
        graph_setup: *mut RenderGraphSetupO,
        state: bool,
    ) {
        self.set_request_async_compute.unwrap()(graph_setup, state)
    }

    pub unsafe fn set_request_multi_gpu(
        &self,
        graph_setup: *mut RenderGraphSetupO,
        device_affinity_mask: u32,
    ) {
        self.set_request_multi_gpu.unwrap()(graph_setup, device_affinity_mask)
    }

    pub unsafe fn expose_as_material_layer(
        &self,
        graph_setup: *mut RenderGraphSetupO,
        layer_name: StrhashT,
    ) {
        self.expose_as_material_layer.unwrap()(graph_setup, layer_name)
    }

    pub unsafe fn write_blackboard(
        &self,
        graph_setup: *mut RenderGraphSetupO,
        key: StrhashT,
        value: RenderGraphBlackboardValue,
    ) {
        self.write_blackboard.unwrap()(graph_setup, key, value)
    }

    pub unsafe fn read_blackboard(
        &self,
        graph_setup: *mut RenderGraphSetupO,
        key: StrhashT,
        value: *mut RenderGraphBlackboardValue,
    ) -> bool {
        self.read_blackboard.unwrap()(graph_setup, key, value)
    }

    pub unsafe fn request_data(&self, graph_setup: *mut RenderGraphSetupO, name: StrhashT) {
        self.request_data.unwrap()(graph_setup, name)
    }
}

impl crate::Api for RenderGraphSetupApi {
    const NAME: ConstCStr = const_cstr!("tm_render_graph_setup_api");
}

impl RenderGraphExecuteApi {
    pub unsafe fn default_command_buffer(
        &self,
        graph_execute: *mut RenderGraphExecuteO,
    ) -> *mut RendererCommandBufferO {
        self.default_command_buffer.unwrap()(graph_execute)
    }

    pub unsafe fn default_resource_command_buffer(
        &self,
        graph_execute: *mut RenderGraphExecuteO,
    ) -> *mut RendererResourceCommandBufferO {
        self.default_resource_command_buffer.unwrap()(graph_execute)
    }

    pub unsafe fn shader_context(
        &self,
        graph_execute: *mut RenderGraphExecuteO,
    ) -> *const ShaderSystemContextO {
        self.shader_context.unwrap()(graph_execute)
    }

    pub unsafe fn device_affinity_mask(&self, graph_execute: *mut RenderGraphExecuteO) -> u32 {
        self.device_affinity_mask.unwrap()(graph_execute)
    }

    pub unsafe fn backend_handle(
        &self,
        graph_execute: *mut RenderGraphExecuteO,
        graph_handle: RenderGraphHandleT,
        subresource_view: *const RendererImageViewT,
    ) -> RendererHandleT {
        self.backend_handle.unwrap()(graph_execute, graph_handle, subresource_view)
    }

    pub unsafe fn run_sub_module(
        &self,
        graph_execute: *mut RenderGraphExecuteO,
        sub_module_name: StrhashT,
        sort_key: u64,
        commands: *mut RendererCommandBufferO,
    ) {
        self.run_sub_module.unwrap()(graph_execute, sub_module_name, sort_key, commands)
    }

    pub unsafe fn read_blackboard(
        &self,
        graph_execute: *mut RenderGraphExecuteO,
        key: StrhashT,
        value: *mut RenderGraphBlackboardValue,
    ) -> bool {
        self.read_blackboard.unwrap()(graph_execute, key, value)
    }

    pub unsafe fn append_viewers(
        &self,
        graph_execute: *mut RenderGraphExecuteO,
        viewers: *const RenderGraphViewerT,
        num_viewers: u32,
    ) -> u32 {
        self.append_viewers.unwrap()(graph_execute, viewers, num_viewers)
    }
}

impl crate::Api for RenderGraphExecuteApi {
    const NAME: ConstCStr = const_cstr!("tm_render_graph_execute_api");
}

impl RenderGraphPassApi {
    pub unsafe fn init_pass(
        &self,
        const_data: *mut ::std::os::raw::c_void,
        allocator: *mut AllocatorI,
        res_buf: *mut RendererResourceCommandBufferO,
    ) {
        self.init_pass.unwrap()(const_data, allocator, res_buf)
    }

    pub unsafe fn shutdown_pass(
        &self,
        const_data: *mut ::std::os::raw::c_void,
        allocator: *mut AllocatorI,
        res_buf: *mut RendererResourceCommandBufferO,
    ) {
        self.shutdown_pass.unwrap()(const_data, allocator, res_buf)
    }

    pub unsafe fn setup_pass(
        &self,
        const_data: *const ::std::os::raw::c_void,
        runtime_data: *mut ::std::os::raw::c_void,
        graph_setup: *mut RenderGraphSetupO,
    ) {
        self.setup_pass.unwrap()(const_data, runtime_data, graph_setup)
    }

    pub unsafe fn execute_pass(
        &self,
        const_data: *const ::std::os::raw::c_void,
        runtime_data: *mut ::std::os::raw::c_void,
        sort_key: u64,
        graph_execute: *mut RenderGraphExecuteO,
    ) {
        self.execute_pass.unwrap()(const_data, runtime_data, sort_key, graph_execute)
    }
}
