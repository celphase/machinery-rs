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
pub const TM_TYPE__CREATION_GRAPH__INSTANCE: &'static [u8; 27usize] =
    b"tm_creation_graph_instance\0";
pub const TM_CREATION_GRAPH_NODE_INTERFACE_NAME: &'static [u8; 33usize] =
    b"tm_creation_graph_node_interface\0";
pub const TM_CREATION_GRAPH_IO_TYPE_INTERFACE_NAME: &'static [u8; 36usize] =
    b"tm_creation_graph_io_type_interface\0";
pub const TM_CREATION_GRAPH_OUTPUT_NODE_TYPE_INTERFACE_NAME: &'static [u8; 45usize] =
    b"tm_creation_graph_output_node_type_interface\0";
pub const TM_CREATION_GRAPH_COMPILE_DATA_TO_WIRE_INTERFACE_NAME: &'static [u8; 49usize] =
    b"tm_creation_graph_compile_data_to_wire_interface\0";
pub const TM_TT_TYPE__CREATION_GRAPH: &'static [u8; 18usize] = b"tm_creation_graph\0";
pub const TM_TT_TYPE__CREATION_GRAPH__RESOURCE_REFERENCE: &'static [u8; 37usize] =
    b"tm_creation_graph_resource_reference\0";
pub const TM_CREATION_GRAPH_PREVIEW_INTERFACE_NAME: &'static [u8; 28usize] =
    b"tm_creation_graph_preview_i\0";
pub const TM_CREATION_GRAPH_API_NAME: &'static [u8; 22usize] = b"tm_creation_graph_api\0";
pub const TM_CREATION_GRAPH_INTERPRETER_API_NAME: &'static [u8; 34usize] =
    b"tm_creation_graph_interpreter_api\0";
pub const TM_TT_TYPE__IMAGE_MIPMAP_SETTINGS: &'static [u8; 19usize] = b"tm_mipmap_settings\0";
pub const TM_TT_TYPE__IMAGE_DESCRIPTION: &'static [u8; 21usize] = b"tm_image_description\0";
pub const TM_TT_TYPE__IMAGE_ARCHIVE: &'static [u8; 17usize] = b"tm_image_archive\0";
pub const TM_CREATION_GRAPH__IMAGE__OUTPUT_NODE: &'static [u8; 13usize] = b"image_output\0";
pub const TM_CREATION_GRAPH__SHADER_INSTANCE_OUTPUT: &'static [u8; 26usize] =
    b"tm_shader_instance_output\0";
pub const TM_CREATION_GRAPH__BOUNDING_VOLUME: &'static [u8; 16usize] = b"bounding_volume\0";
pub const TM_CREATION_GRAPH__DRAW_CALL: &'static [u8; 9usize] = b"drawcall\0";
pub const TM_CREATION_GRAPH__PHYSICS_SHAPE__OUTPUT_NODE: &'static [u8; 14usize] =
    b"physics_shape\0";
pub const TM_CREATION_GRAPH__RAY_TRACE_INSTANCE: &'static [u8; 19usize] = b"ray_trace_instance\0";
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
pub struct CreationGraphOutputNodeTypeT {
    pub node_name_hash: StrhashT,
    pub display_name: *const ::std::os::raw::c_char,
    pub asset_label: *const AssetLabelT,
    pub size: u64,
    pub write_wire: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut CreationGraphInterpreterContextT,
            wire: u32,
            data: *const ::std::os::raw::c_void,
        ),
    >,
    pub create_thumbnail: ::std::option::Option<
        unsafe extern "C" fn(
            data: *const ::std::os::raw::c_void,
            thumbnail_desc: *const RendererImageDescT,
            rb: *mut RendererBackendI,
            res_buf: *mut RendererResourceCommandBufferO,
            cmd_buf: *mut RendererCommandBufferO,
            res: *mut RendererHandleT,
        ),
    >,
}
impl Default for CreationGraphOutputNodeTypeT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct CreationGraphOutputWireT {
    pub id: StrhashT,
    pub name: StrhashT,
    pub type_hash: StrhashT,
    pub wire: u32,
    pub _padding_79: [::std::os::raw::c_char; 4usize],
}
impl Default for CreationGraphOutputWireT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct CreationGraphInputT {
    pub name: StrhashT,
    pub id: StrhashT,
    pub type_: StrhashT,
    pub value: *mut ::std::os::raw::c_void,
    pub value_size: u32,
    pub wire: u32,
}
impl Default for CreationGraphInputT {
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
pub struct CreationGraphIoDataT {
    pub allocator: *mut AllocatorI,
    pub output_node_types: *mut CreationGraphOutputNodeTypeT,
    pub output_data: *mut *mut u8,
    pub output_wires: *mut CreationGraphOutputWireT,
    pub consumers: *mut CreationGraphInstanceT,
    pub inputs: *mut CreationGraphInputT,
    pub dirty: bool,
    pub _padding_113: [::std::os::raw::c_char; 7usize],
}
impl Default for CreationGraphIoDataT {
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
pub struct CreationGraphNodeCacheT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreationGraphBoundingVolumeDataT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreationGraphImageDataT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetIdT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreationGraphBlackboardValueT {
    _unused: [u8; 0],
}
pub type CompileDataToWireF = ::std::option::Option<
    unsafe extern "C" fn(
        rc: *mut CreationGraphInstanceT,
        wire: u32,
        tt: *const TheTruthO,
        data_id: TtIdT,
        to_type_hash: StrhashT,
    ) -> bool,
>;
pub const TM_TT_PROP__CREATION_GRAPH__GRAPH: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_1 = ::std::os::raw::c_int;
pub const TM_TT_PROP__CREATION_GRAPH__RESOURCE_REFERENCE__CREATION_GRAPH: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__CREATION_GRAPH__RESOURCE_REFERENCE__OUTPUT_NODE_TYPE_HASH:
    ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__CREATION_GRAPH__RESOURCE_REFERENCE__OUTPUT_NODE_NAME: ::std::os::raw::c_int =
    2;
pub type _bindgen_ty_2 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct CreationGraphDestroyInstanceContextT {
    pub manager_shutdown: bool,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreationGraphTickContextT {
    pub dt: f32,
    pub _padding_119: [::std::os::raw::c_char; 4usize],
    pub shader_context: *mut ShaderSystemContextO,
}
impl Default for CreationGraphTickContextT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct CreationGraphViewportInteractContextT {
    pub viewport: RectT,
    pub transform: *const TransformT,
    pub input: *mut UiInputStateT,
    pub camera: *mut CameraT,
    pub render_graph: *mut RenderGraphO,
    pub shader_context: *const ShaderSystemContextO,
    pub undo_stack: *mut UndoStackI,
}
impl Default for CreationGraphViewportInteractContextT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_CREATION_GRAPH_MAX_OUTPUT_NODE_TYPES: ::std::os::raw::c_int = 32;
pub type _bindgen_ty_3 = ::std::os::raw::c_int;
#[repr(C)]
pub struct CreationGraphAllOutputsT {
    pub version: u64,
    pub num_output_node_types: u32,
    pub _padding_191: [::std::os::raw::c_char; 4usize],
    pub output_node_type_hashes: [StrhashT; 32usize],
    pub output_node_type_display_names: [*const ::std::os::raw::c_char; 32usize],
}
impl Default for CreationGraphAllOutputsT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct CreationGraphNamedOutputT {
    pub data: *mut ::std::os::raw::c_void,
    pub size: u32,
    pub _padding_205: [::std::os::raw::c_char; 4usize],
    pub type_: StrhashT,
}
impl Default for CreationGraphNamedOutputT {
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
pub struct CiCreationGraphInstanceProviderI {
    pub get_instances: ::std::option::Option<
        unsafe extern "C" fn(
            component_data: *mut ::std::os::raw::c_void,
            ta: *mut TempAllocatorI,
        ) -> *mut *mut CreationGraphInstanceT,
    >,
}
pub const TM_CREATION_GRAPH_RESOURCE_BUFFERS__PRE_CMD: ::std::os::raw::c_int = 0;
pub const TM_CREATION_GRAPH_RESOURCE_BUFFERS__POST_CMD: ::std::os::raw::c_int = 1;
pub const TM_CREATION_GRAPH_RESOURCE_BUFFERS__MAX: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_4 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreationGraphTaskT {
    pub f: ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void, id: u64)>,
    pub data: *mut ::std::os::raw::c_void,
    pub debug_name: *const ::std::os::raw::c_char,
}
impl Default for CreationGraphTaskT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct CreationGraphContextT {
    pub rb: *mut RendererBackendI,
    pub device_affinity_mask: u32,
    pub default_instance: bool,
    pub _padding_258: [::std::os::raw::c_char; 3usize],
    pub ta: *mut TempAllocatorI,
    pub requested_tasks: *mut *mut CreationGraphTaskT,
    pub res_buf: [*mut RendererResourceCommandBufferO; 2usize],
    pub cmd_buf: *mut RendererCommandBufferO,
    pub event_context: *mut ::std::os::raw::c_void,
    pub entity_id: u64,
    pub entity_ctx: *mut EntityContextO,
    pub force_build_node: TtIdT,
    pub force_build_cache_tag: StrhashT,
    pub shader_repository: *mut ShaderRepositoryO,
    pub event_id: StrhashT,
    pub tt: *mut TheTruthO,
    pub shader_context: *mut ShaderSystemContextO,
}
impl Default for CreationGraphContextT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct CachedNodeResultT {
    pub object: TtIdT,
    pub validity_hash: u64,
}
impl Default for CachedNodeResultT {
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
pub struct CreationGraphPreviewO {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct CreationGraphPreviewI {
    pub output_name: StrhashT,
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            allocator: *mut AllocatorI,
            data: *const ::std::os::raw::c_void,
        ) -> *mut CreationGraphPreviewO,
    >,
    pub destroy: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut CreationGraphPreviewO, allocator: *mut AllocatorI),
    >,
    pub render: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut CreationGraphPreviewO,
            tt: *mut TheTruthO,
            asset: TtIdT,
            args: *const RenderArgsT,
        ),
    >,
    pub ui: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut CreationGraphPreviewO, args: *const AssetPreviewApiUiArgsT),
    >,
    pub create_entity: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut CreationGraphPreviewO,
            tt: *mut TheTruthO,
            asset: TtIdT,
            entity_ctx: *mut EntityContextO,
            result: *mut EntityT,
        ),
    >,
    pub toolbars: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut CreationGraphPreviewO,
            ta: *mut TempAllocatorI,
            args: *const AssetPreviewApiUiArgsT,
        ) -> *mut ToolbarI,
    >,
}
impl Default for CreationGraphPreviewI {
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
pub struct CreationGraphO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct CreationGraphApi {
    pub create_truth_types: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
    pub asset_browser_create_interface:
        ::std::option::Option<unsafe extern "C" fn() -> *mut AssetBrowserCreateAssetI>,
    pub create_instance: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            asset: TtIdT,
            context: *mut CreationGraphContextT,
        ) -> CreationGraphInstanceT,
    >,
    pub destroy_instance: ::std::option::Option<
        unsafe extern "C" fn(
            instance: *mut CreationGraphInstanceT,
            context: *mut CreationGraphContextT,
        ),
    >,
    pub invalidate: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO, asset: TtIdT)>,
    pub write_blackboard: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            asset: TtIdT,
            key_name: StrhashT,
            value: *const CreationGraphBlackboardValueT,
        ),
    >,
    pub read_blackboard: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            asset: TtIdT,
            key_name: StrhashT,
            value: *mut CreationGraphBlackboardValueT,
        ) -> bool,
    >,
    pub remove_blackboard: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, asset: TtIdT, key_name: StrhashT),
    >,
    pub lock_resource_cache: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            graph_id: TtIdT,
            node_id: TtIdT,
        ) -> *mut CreationGraphNodeCacheT,
    >,
    pub unlock_resource_cache:
        ::std::option::Option<unsafe extern "C" fn(cache: *mut CreationGraphNodeCacheT)>,
    pub add_listener: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, asset: TtIdT, object: TtIdT),
    >,
    pub tick_queue: ::std::option::Option<
        unsafe extern "C" fn(
            instance: *mut CreationGraphInstanceT,
            dt: f32,
            context: *mut CreationGraphContextT,
        ),
    >,
    pub has_event: ::std::option::Option<
        unsafe extern "C" fn(instance: *mut CreationGraphInstanceT, event_id: StrhashT) -> bool,
    >,
    pub trigger_event: ::std::option::Option<
        unsafe extern "C" fn(
            instance: *mut CreationGraphInstanceT,
            event_id: StrhashT,
            context: *mut CreationGraphContextT,
        ),
    >,
    pub update_interface: ::std::option::Option<
        unsafe extern "C" fn(
            instance: *mut CreationGraphInstanceT,
            interface_id: StrhashT,
            context: *mut CreationGraphContextT,
        ),
    >,
    pub lookup_cached_node_result: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            graph_id: TtIdT,
            node_id: TtIdT,
        ) -> CachedNodeResultT,
    >,
    pub set_cached_node_result: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            graph_id: TtIdT,
            node_id: TtIdT,
            validity_hash: u64,
            object: TtIdT,
        ),
    >,
    pub output: ::std::option::Option<
        unsafe extern "C" fn(
            instance: *mut CreationGraphInstanceT,
            node_name_hash: StrhashT,
            context: *mut CreationGraphContextT,
            output_node_type: *mut *const CreationGraphOutputNodeTypeT,
        ) -> CreationGraphOutputT,
    >,
    pub all_outputs: ::std::option::Option<
        unsafe extern "C" fn(
            instance: *mut CreationGraphInstanceT,
            context: *mut CreationGraphContextT,
        ) -> CreationGraphAllOutputsT,
    >,
    pub refresh_outputs: ::std::option::Option<
        unsafe extern "C" fn(
            instance: *mut CreationGraphInstanceT,
            context: *mut CreationGraphContextT,
        ),
    >,
    pub named_output: ::std::option::Option<
        unsafe extern "C" fn(
            instance: *mut CreationGraphInstanceT,
            type_: StrhashT,
            name: StrhashT,
        ) -> CreationGraphNamedOutputT,
    >,
    pub gather_stripped_graph: ::std::option::Option<
        unsafe extern "C" fn(
            instance: *mut CreationGraphInstanceT,
            context: *mut CreationGraphContextT,
            nodes: *mut SetIdT,
        ),
    >,
    pub get_instances_from_component: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            entity_ctx: *mut EntityContextO,
            entity: EntityT,
            component_name_hash: StrhashT,
            ta: *mut TempAllocatorI,
        ) -> *mut *mut CreationGraphInstanceT,
    >,
    pub set_input_value: ::std::option::Option<
        unsafe extern "C" fn(
            instance: *mut CreationGraphInstanceT,
            context: *mut CreationGraphContextT,
            name: StrhashT,
            data: *mut ::std::os::raw::c_void,
            data_size: u32,
        ),
    >,
}
#[repr(C)]
pub struct CreationGraphInterpreterContextT {
    pub instance: *mut CreationGraphInstanceT,
    pub node: u32,
    pub _padding_53: [::std::os::raw::c_char; 4usize],
    pub graph_id: TtIdT,
    pub node_id: TtIdT,
    pub node_type: *const GraphNodeTypeI,
    pub wires: *const u32,
}
impl Default for CreationGraphInterpreterContextT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type CreationGraphInterpreterRunNodeF =
    ::std::option::Option<unsafe extern "C" fn(ctx: *mut CreationGraphInterpreterContextT)>;
pub type CreationGraphInterpreterRunDependenciesF =
    ::std::option::Option<unsafe extern "C" fn(ctx: *mut CreationGraphInterpreterContextT) -> bool>;
#[repr(C)]
pub struct CreationGraphInterpreterNodeT {
    pub run: CreationGraphInterpreterRunNodeF,
    pub run_dependencies: CreationGraphInterpreterRunDependenciesF,
    pub wires_index: u32,
    pub _padding_96: [::std::os::raw::c_char; 4usize],
    pub node_type: *const GraphNodeTypeI,
    pub graph_id: TtIdT,
    pub node_id: TtIdT,
    pub dependencies_index: u32,
    pub _padding_113: [::std::os::raw::c_char; 4usize],
}
impl Default for CreationGraphInterpreterNodeT {
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
pub struct CreationGraphInterpreterApi {
    pub create_graph_interpreter: ::std::option::Option<
        unsafe extern "C" fn(allocator: *mut AllocatorI) -> *mut CreationGraphInterpreterO,
    >,
    pub destroy_graph_interpreter:
        ::std::option::Option<unsafe extern "C" fn(gr: *mut CreationGraphInterpreterO)>,
    pub set_graph: ::std::option::Option<
        unsafe extern "C" fn(
            gr: *mut CreationGraphInterpreterO,
            nodes: *const CreationGraphInterpreterNodeT,
            num_nodes: u32,
            num_wires: u32,
            node_wires: *mut u32,
            num_node_wires: u32,
            dependencies: *mut u32,
            num_dependencies: u32,
        ),
    >,
    pub default_instance: ::std::option::Option<
        unsafe extern "C" fn(gr: *mut CreationGraphInterpreterO) -> CreationGraphInstanceT,
    >,
    pub active_instances: ::std::option::Option<
        unsafe extern "C" fn(
            gr: *mut CreationGraphInterpreterO,
            count: *mut u32,
        ) -> *mut *mut CreationGraphInstanceDataO,
    >,
    pub create_instance: ::std::option::Option<
        unsafe extern "C" fn(gr: *mut CreationGraphInterpreterO) -> CreationGraphInstanceT,
    >,
    pub destroy_instance:
        ::std::option::Option<unsafe extern "C" fn(rc: *mut CreationGraphInstanceT)>,
    pub run_node_at_index:
        ::std::option::Option<unsafe extern "C" fn(rc: *mut CreationGraphInstanceT, node: u32)>,
    pub add_event_node: ::std::option::Option<
        unsafe extern "C" fn(gr: *mut CreationGraphInterpreterO, e: StrhashT, node: u32),
    >,
    pub has_event: ::std::option::Option<
        unsafe extern "C" fn(gr: *mut CreationGraphInterpreterO, e: StrhashT) -> bool,
    >,
    pub trigger_event:
        ::std::option::Option<unsafe extern "C" fn(rc: *mut CreationGraphInstanceT, e: StrhashT)>,
    pub trigger_wire:
        ::std::option::Option<unsafe extern "C" fn(rc: *mut CreationGraphInstanceT, wire: u32)>,
    pub gather_stripped_graph: ::std::option::Option<
        unsafe extern "C" fn(rc: *mut CreationGraphInstanceT, e: StrhashT, nodes: *mut SetIdT),
    >,
    pub queue_wire: ::std::option::Option<
        unsafe extern "C" fn(rc: *mut CreationGraphInstanceT, wire: u32, delay: f32),
    >,
    pub tick_queue:
        ::std::option::Option<unsafe extern "C" fn(rc: *mut CreationGraphInstanceT, dt: f32)>,
    pub get_node_run_context: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut CreationGraphInstanceT,
            node: u32,
        ) -> CreationGraphInterpreterContextT,
    >,
    pub read_wire: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut CreationGraphInstanceT,
            wire: u32,
        ) -> CreationGraphInterpreterWireContentT,
    >,
    pub read_wires: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut CreationGraphInstanceT,
            content: *mut CreationGraphInterpreterWireContentT,
            wires: *const u32,
            n: u32,
        ),
    >,
    pub read_wires_indirect: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut CreationGraphInstanceT,
            content: *mut *mut CreationGraphInterpreterWireContentT,
            wires: *const u32,
            n: u32,
        ),
    >,
    pub write_wire: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut CreationGraphInstanceT,
            wire: u32,
            type_info: StrhashT,
            n: u32,
            size: u32,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub read_variable: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut CreationGraphInstanceT,
            variable: u64,
        ) -> CreationGraphInterpreterWireContentT,
    >,
    pub write_variable: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut CreationGraphInstanceT,
            variable: u64,
            n: u32,
            size: u32,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub random_float:
        ::std::option::Option<unsafe extern "C" fn(rc: *mut CreationGraphInstanceT) -> f32>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreationGraphRuntimeDataT {
    _unused: [u8; 0],
}
pub const TM_TT_PROP__GRAPH__NODES: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH__CONNECTIONS: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH__DATA: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH__COMMENTS: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH__INTERFACE: ::std::os::raw::c_int = 4;
pub type _bindgen_ty_5 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_NODE__TYPE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_NODE__LABEL: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_NODE__POSITION_X: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_NODE__POSITION_Y: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH_NODE__WIDTH: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__GRAPH_NODE__SETTINGS: ::std::os::raw::c_int = 5;
pub type _bindgen_ty_6 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_CONNECTION__FROM_NODE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_CONNECTION__TO_NODE: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_CONNECTION__FROM_CONNECTOR_HASH: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_CONNECTION__TO_CONNECTOR_HASH: ::std::os::raw::c_int = 3;
pub type _bindgen_ty_7 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_DATA__TO_NODE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_DATA__TO_CONNECTOR_HASH: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_DATA__DATA: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_8 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_COMMENT__TEXT: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_COMMENT__POSITION_X: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_COMMENT__POSITION_Y: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_COMMENT__FONT_SCALE: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH_COMMENT__COLOR: ::std::os::raw::c_int = 4;
pub type _bindgen_ty_9 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_INTERFACE__INPUTS: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_INTERFACE__OUTPUTS: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_INTERFACE__LAST_ID: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_INTERFACE__CACHED_NODE_RESULT: ::std::os::raw::c_int = 3;
pub type _bindgen_ty_10 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_INPUT__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_INPUT__DISPLAY_NAME: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_INPUT__ID: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_INPUT__TYPE_HASH: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH_INPUT__VALUE_SET_BY_USER: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__GRAPH_INPUT__VALUE: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__GRAPH_INPUT__TOOLTIP: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__GRAPH_INPUT__PUBLIC: ::std::os::raw::c_int = 7;
pub const TM_TT_PROP__GRAPH_INPUT__ORDER: ::std::os::raw::c_int = 8;
pub type _bindgen_ty_11 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_OUTPUT__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_OUTPUT__DISPLAY_NAME: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_OUTPUT__ID: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_OUTPUT__TYPE_HASH: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH_OUTPUT__ORDER: ::std::os::raw::c_int = 4;
pub type _bindgen_ty_12 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_CACHED_NODE_RESULT__NODE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_CACHED_NODE_RESULT__VALIDITY_HASH: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_CACHED_NODE_RESULT__OBJECT: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_13 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_SETTINGS__GRAPH_UUID_A: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_SETTINGS__VIEW_POSITION: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_SETTINGS__VIEW_SCALE: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_SETTINGS__GRID_SNAPPING: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH_SETTINGS__GRID_SIZE: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__GRAPH_SETTINGS__DEBUG_WATCH_WIRE_OVERLAY_TABLE_MODE: ::std::os::raw::c_int =
    5;
pub const TM_TT_PROP__GRAPH_SETTINGS__SHOW_REMOVED_INHERITED: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__GRAPH_SETTINGS__DEBUG_WATCHED_WIRES: ::std::os::raw::c_int = 7;
pub const TM_TT_PROP__GRAPH_SETTINGS__DEBUG_PERSISTENT_SETTINGS: ::std::os::raw::c_int = 8;
pub type _bindgen_ty_14 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_SETTINGS__BREAKPOINTS: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_SETTINGS__OVERLAY_OPENED: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_15 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__NODE_UNIQUE_ID: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__NODE_UUID_A: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__NODE_UUID_B: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__SUBGRAPH_NODE_UNIQUE_ID: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__SUBGRAPH_NODE_UUID_A: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__SUBGRAPH_NODE_UUID_B: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__DISABLED: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__GRAPH_UUID_A: ::std::os::raw::c_int = 7;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__GRAPH_UUID_B: ::std::os::raw::c_int = 8;
pub type _bindgen_ty_16 = ::std::os::raw::c_int;
pub const TM_GRAPH_MAX_CONNECTORS: ::std::os::raw::c_int = 24;
pub type _bindgen_ty_17 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union GraphGenericValueTBindgenTy1 {
    pub data: *const ::std::os::raw::c_void,
    pub str_: *const ::std::os::raw::c_char,
    pub f: *const f32,
    pub uint64: *const u64,
    pub uint32: *const u32,
    pub vec2: *const Vec2T,
    pub vec3: *const Vec3T,
    pub vec4: *const Vec4T,
    pub rect: *const RectT,
    pub srgb: *const ColorSrgbT,
    pub boolean: *const bool,
}
impl Default for GraphGenericValueTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_CREATION_GRAPH__STATIC_WIRE__SINK: ::std::os::raw::c_int = 0;
pub const TM_CREATION_GRAPH__STATIC_WIRE__CONTEXT: ::std::os::raw::c_int = 1;
pub const TM_CREATION_GRAPH__STATIC_WIRE__COUNT: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_18 = ::std::os::raw::c_int;
#[repr(C)]
pub struct CreationGraphCompileContextT {
    pub tt: *mut TheTruthO,
    pub graph_id: TtIdT,
    pub settings_id: TtIdT,
    pub instanced_settings_id: TtIdT,
    pub asset_label_uuid: u64,
}
impl Default for CreationGraphCompileContextT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_VERTEX_SEMANTIC_POSITION: ::std::os::raw::c_int = 0;
pub const TM_VERTEX_SEMANTIC_NORMAL: ::std::os::raw::c_int = 1;
pub const TM_VERTEX_SEMANTIC_TANGENT0: ::std::os::raw::c_int = 2;
pub const TM_VERTEX_SEMANTIC_TANGENT1: ::std::os::raw::c_int = 3;
pub const TM_VERTEX_SEMANTIC_SKIN_DATA: ::std::os::raw::c_int = 4;
pub const TM_VERTEX_SEMANTIC_TEXCOORD0: ::std::os::raw::c_int = 5;
pub const TM_VERTEX_SEMANTIC_TEXCOORD1: ::std::os::raw::c_int = 6;
pub const TM_VERTEX_SEMANTIC_TEXCOORD2: ::std::os::raw::c_int = 7;
pub const TM_VERTEX_SEMANTIC_TEXCOORD3: ::std::os::raw::c_int = 8;
pub const TM_VERTEX_SEMANTIC_COLOR0: ::std::os::raw::c_int = 9;
pub const TM_VERTEX_SEMANTIC_COLOR1: ::std::os::raw::c_int = 10;
pub const TM_VERTEX_SEMANTIC_MAX_SEMANTICS: ::std::os::raw::c_int = 11;
pub const TM_INDEX_SEMANTIC: ::std::os::raw::c_int = 16;
pub type _bindgen_ty_19 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuGeometryT {
    pub vfetch_system: *mut ShaderSystemO,
    pub vfetch_system_cbuffer: u32,
    pub vfetch_system_rbinder: u32,
    pub ray_tracing_geometry_flags: u32,
    pub _padding_48: [::std::os::raw::c_char; 4usize],
}
impl Default for GpuGeometryT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_TT_PROP__IMAGE_FILTER_SETTINGS__COLOR_SPACE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__IMAGE_FILTER_SETTINGS__FILTER: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__IMAGE_FILTER_SETTINGS__OVERRIDE_EXISTING_MIPS: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__IMAGE_FILTER_SETTINGS__STORAGE_POLICY: ::std::os::raw::c_int = 3;
pub type _bindgen_ty_20 = ::std::os::raw::c_int;
pub const TM_TT_PROP__IMAGE_DESCRIPTION__TYPE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__IMAGE_DESCRIPTION__PIXEL_FORMAT: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__IMAGE_DESCRIPTION__WIDTH: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__IMAGE_DESCRIPTION__HEIGHT: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__IMAGE_DESCRIPTION__DEPTH: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__IMAGE_DESCRIPTION__MIP_LEVELS: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__IMAGE_DESCRIPTION__LAYER_COUNT: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__IMAGE_DESCRIPTION__SAMPLE_COUNT: ::std::os::raw::c_int = 7;
pub const TM_TT_PROP__IMAGE_DESCRIPTION__DEBUG_TAG: ::std::os::raw::c_int = 8;
pub type _bindgen_ty_21 = ::std::os::raw::c_int;
pub const TM_TT_PROP__IMAGE_ARCHIVE__FILENAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__IMAGE_ARCHIVE__VALIDITY_HASH: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__IMAGE_ARCHIVE__DESCRIPTION: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__IMAGE_ARCHIVE__BUFFER: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__IMAGE_ARCHIVE__RAW: ::std::os::raw::c_int = 4;
pub type _bindgen_ty_22 = ::std::os::raw::c_int;
#[repr(C)]
pub struct GpuBufferT {
    pub validity_hash: u64,
    pub size: u32,
    pub usage_flags: u32,
    pub handle: RendererHandleT,
    pub resource_state: u16,
    pub _padding_21: [::std::os::raw::c_char; 2usize],
}
impl Default for GpuBufferT {
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
pub struct CreationGraphShaderInstanceT {
    pub shader: *mut ShaderO,
    pub cbuf: u32,
    pub rbinder: u32,
}
impl Default for CreationGraphShaderInstanceT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct CreationGraphPhysicsShapeT {
    pub dcc_asset: TtIdT,
    pub dcc_mesh: TtIdT,
}
impl Default for CreationGraphPhysicsShapeT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct GpuSimulationChannelBufferDataT {
    pub buffer: RendererHandleT,
    pub buffer_resource_state: u16,
    pub _padding_14: [::std::os::raw::c_char; 6usize],
    pub buffer_header_size: u32,
    pub buffer_header_count_offset: u32,
    pub buffer_header_stride_offset: u32,
}
impl Default for GpuSimulationChannelBufferDataT {
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
pub struct CreationGraphInstanceDataO {
    pub _address: u8,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct CreationGraphManagerO {
    pub _address: u8,
}

// Extensions generated by machinery-generator

use const_cstr::{const_cstr, ConstCStr};

use crate::foundation::*;
use crate::plugins::editor_views::*;
use crate::plugins::entity::*;
use crate::plugins::render_graph::*;
use crate::plugins::renderer::*;
use crate::plugins::shader_system::*;
use crate::plugins::the_machinery_shared::*;
use crate::plugins::ui::*;

impl CreationGraphApi {
    pub unsafe fn create_truth_types(&self, tt: *mut TheTruthO) {
        self.create_truth_types.unwrap()(tt)
    }

    pub unsafe fn asset_browser_create_interface(&self) -> *mut AssetBrowserCreateAssetI {
        self.asset_browser_create_interface.unwrap()()
    }

    pub unsafe fn create_instance(
        &self,
        tt: *mut TheTruthO,
        asset: TtIdT,
        context: *mut CreationGraphContextT,
    ) -> CreationGraphInstanceT {
        self.create_instance.unwrap()(tt, asset, context)
    }

    pub unsafe fn destroy_instance(
        &self,
        instance: *mut CreationGraphInstanceT,
        context: *mut CreationGraphContextT,
    ) {
        self.destroy_instance.unwrap()(instance, context)
    }

    pub unsafe fn invalidate(&self, tt: *mut TheTruthO, asset: TtIdT) {
        self.invalidate.unwrap()(tt, asset)
    }

    pub unsafe fn write_blackboard(
        &self,
        tt: *mut TheTruthO,
        asset: TtIdT,
        key_name: StrhashT,
        value: *const CreationGraphBlackboardValueT,
    ) {
        self.write_blackboard.unwrap()(tt, asset, key_name, value)
    }

    pub unsafe fn read_blackboard(
        &self,
        tt: *mut TheTruthO,
        asset: TtIdT,
        key_name: StrhashT,
        value: *mut CreationGraphBlackboardValueT,
    ) -> bool {
        self.read_blackboard.unwrap()(tt, asset, key_name, value)
    }

    pub unsafe fn remove_blackboard(&self, tt: *mut TheTruthO, asset: TtIdT, key_name: StrhashT) {
        self.remove_blackboard.unwrap()(tt, asset, key_name)
    }

    pub unsafe fn lock_resource_cache(
        &self,
        tt: *mut TheTruthO,
        graph_id: TtIdT,
        node_id: TtIdT,
    ) -> *mut CreationGraphNodeCacheT {
        self.lock_resource_cache.unwrap()(tt, graph_id, node_id)
    }

    pub unsafe fn unlock_resource_cache(&self, cache: *mut CreationGraphNodeCacheT) {
        self.unlock_resource_cache.unwrap()(cache)
    }

    pub unsafe fn add_listener(&self, tt: *mut TheTruthO, asset: TtIdT, object: TtIdT) {
        self.add_listener.unwrap()(tt, asset, object)
    }

    pub unsafe fn tick_queue(
        &self,
        instance: *mut CreationGraphInstanceT,
        dt: f32,
        context: *mut CreationGraphContextT,
    ) {
        self.tick_queue.unwrap()(instance, dt, context)
    }

    pub unsafe fn has_event(
        &self,
        instance: *mut CreationGraphInstanceT,
        event_id: StrhashT,
    ) -> bool {
        self.has_event.unwrap()(instance, event_id)
    }

    pub unsafe fn trigger_event(
        &self,
        instance: *mut CreationGraphInstanceT,
        event_id: StrhashT,
        context: *mut CreationGraphContextT,
    ) {
        self.trigger_event.unwrap()(instance, event_id, context)
    }

    pub unsafe fn update_interface(
        &self,
        instance: *mut CreationGraphInstanceT,
        interface_id: StrhashT,
        context: *mut CreationGraphContextT,
    ) {
        self.update_interface.unwrap()(instance, interface_id, context)
    }

    pub unsafe fn lookup_cached_node_result(
        &self,
        tt: *mut TheTruthO,
        graph_id: TtIdT,
        node_id: TtIdT,
    ) -> CachedNodeResultT {
        self.lookup_cached_node_result.unwrap()(tt, graph_id, node_id)
    }

    pub unsafe fn set_cached_node_result(
        &self,
        tt: *mut TheTruthO,
        graph_id: TtIdT,
        node_id: TtIdT,
        validity_hash: u64,
        object: TtIdT,
    ) {
        self.set_cached_node_result.unwrap()(tt, graph_id, node_id, validity_hash, object)
    }

    pub unsafe fn output(
        &self,
        instance: *mut CreationGraphInstanceT,
        node_name_hash: StrhashT,
        context: *mut CreationGraphContextT,
        output_node_type: *mut *const CreationGraphOutputNodeTypeT,
    ) -> CreationGraphOutputT {
        self.output.unwrap()(instance, node_name_hash, context, output_node_type)
    }

    pub unsafe fn all_outputs(
        &self,
        instance: *mut CreationGraphInstanceT,
        context: *mut CreationGraphContextT,
    ) -> CreationGraphAllOutputsT {
        self.all_outputs.unwrap()(instance, context)
    }

    pub unsafe fn refresh_outputs(
        &self,
        instance: *mut CreationGraphInstanceT,
        context: *mut CreationGraphContextT,
    ) {
        self.refresh_outputs.unwrap()(instance, context)
    }

    pub unsafe fn named_output(
        &self,
        instance: *mut CreationGraphInstanceT,
        type_: StrhashT,
        name: StrhashT,
    ) -> CreationGraphNamedOutputT {
        self.named_output.unwrap()(instance, type_, name)
    }

    pub unsafe fn gather_stripped_graph(
        &self,
        instance: *mut CreationGraphInstanceT,
        context: *mut CreationGraphContextT,
        nodes: *mut SetIdT,
    ) {
        self.gather_stripped_graph.unwrap()(instance, context, nodes)
    }

    pub unsafe fn get_instances_from_component(
        &self,
        tt: *mut TheTruthO,
        entity_ctx: *mut EntityContextO,
        entity: EntityT,
        component_name_hash: StrhashT,
        ta: *mut TempAllocatorI,
    ) -> *mut *mut CreationGraphInstanceT {
        self.get_instances_from_component.unwrap()(tt, entity_ctx, entity, component_name_hash, ta)
    }

    pub unsafe fn set_input_value(
        &self,
        instance: *mut CreationGraphInstanceT,
        context: *mut CreationGraphContextT,
        name: StrhashT,
        data: *mut ::std::os::raw::c_void,
        data_size: u32,
    ) {
        self.set_input_value.unwrap()(instance, context, name, data, data_size)
    }
}

impl crate::Api for CreationGraphApi {
    const NAME: ConstCStr = const_cstr!("tm_creation_graph_api");
}

impl CreationGraphInterpreterApi {
    pub unsafe fn create_graph_interpreter(
        &self,
        allocator: *mut AllocatorI,
    ) -> *mut CreationGraphInterpreterO {
        self.create_graph_interpreter.unwrap()(allocator)
    }

    pub unsafe fn destroy_graph_interpreter(&self, gr: *mut CreationGraphInterpreterO) {
        self.destroy_graph_interpreter.unwrap()(gr)
    }

    pub unsafe fn set_graph(
        &self,
        gr: *mut CreationGraphInterpreterO,
        nodes: *const CreationGraphInterpreterNodeT,
        num_nodes: u32,
        num_wires: u32,
        node_wires: *mut u32,
        num_node_wires: u32,
        dependencies: *mut u32,
        num_dependencies: u32,
    ) {
        self.set_graph.unwrap()(
            gr,
            nodes,
            num_nodes,
            num_wires,
            node_wires,
            num_node_wires,
            dependencies,
            num_dependencies,
        )
    }

    pub unsafe fn default_instance(
        &self,
        gr: *mut CreationGraphInterpreterO,
    ) -> CreationGraphInstanceT {
        self.default_instance.unwrap()(gr)
    }

    pub unsafe fn active_instances(
        &self,
        gr: *mut CreationGraphInterpreterO,
        count: *mut u32,
    ) -> *mut *mut CreationGraphInstanceDataO {
        self.active_instances.unwrap()(gr, count)
    }

    pub unsafe fn create_instance(
        &self,
        gr: *mut CreationGraphInterpreterO,
    ) -> CreationGraphInstanceT {
        self.create_instance.unwrap()(gr)
    }

    pub unsafe fn destroy_instance(&self, rc: *mut CreationGraphInstanceT) {
        self.destroy_instance.unwrap()(rc)
    }

    pub unsafe fn run_node_at_index(&self, rc: *mut CreationGraphInstanceT, node: u32) {
        self.run_node_at_index.unwrap()(rc, node)
    }

    pub unsafe fn add_event_node(
        &self,
        gr: *mut CreationGraphInterpreterO,
        e: StrhashT,
        node: u32,
    ) {
        self.add_event_node.unwrap()(gr, e, node)
    }

    pub unsafe fn has_event(&self, gr: *mut CreationGraphInterpreterO, e: StrhashT) -> bool {
        self.has_event.unwrap()(gr, e)
    }

    pub unsafe fn trigger_event(&self, rc: *mut CreationGraphInstanceT, e: StrhashT) {
        self.trigger_event.unwrap()(rc, e)
    }

    pub unsafe fn trigger_wire(&self, rc: *mut CreationGraphInstanceT, wire: u32) {
        self.trigger_wire.unwrap()(rc, wire)
    }

    pub unsafe fn gather_stripped_graph(
        &self,
        rc: *mut CreationGraphInstanceT,
        e: StrhashT,
        nodes: *mut SetIdT,
    ) {
        self.gather_stripped_graph.unwrap()(rc, e, nodes)
    }

    pub unsafe fn queue_wire(&self, rc: *mut CreationGraphInstanceT, wire: u32, delay: f32) {
        self.queue_wire.unwrap()(rc, wire, delay)
    }

    pub unsafe fn tick_queue(&self, rc: *mut CreationGraphInstanceT, dt: f32) {
        self.tick_queue.unwrap()(rc, dt)
    }

    pub unsafe fn get_node_run_context(
        &self,
        inst: *mut CreationGraphInstanceT,
        node: u32,
    ) -> CreationGraphInterpreterContextT {
        self.get_node_run_context.unwrap()(inst, node)
    }

    pub unsafe fn read_wire(
        &self,
        inst: *mut CreationGraphInstanceT,
        wire: u32,
    ) -> CreationGraphInterpreterWireContentT {
        self.read_wire.unwrap()(inst, wire)
    }

    pub unsafe fn read_wires(
        &self,
        inst: *mut CreationGraphInstanceT,
        content: *mut CreationGraphInterpreterWireContentT,
        wires: *const u32,
        n: u32,
    ) {
        self.read_wires.unwrap()(inst, content, wires, n)
    }

    pub unsafe fn read_wires_indirect(
        &self,
        inst: *mut CreationGraphInstanceT,
        content: *mut *mut CreationGraphInterpreterWireContentT,
        wires: *const u32,
        n: u32,
    ) {
        self.read_wires_indirect.unwrap()(inst, content, wires, n)
    }

    pub unsafe fn write_wire(
        &self,
        inst: *mut CreationGraphInstanceT,
        wire: u32,
        type_info: StrhashT,
        n: u32,
        size: u32,
    ) -> *mut ::std::os::raw::c_void {
        self.write_wire.unwrap()(inst, wire, type_info, n, size)
    }

    pub unsafe fn read_variable(
        &self,
        inst: *mut CreationGraphInstanceT,
        variable: u64,
    ) -> CreationGraphInterpreterWireContentT {
        self.read_variable.unwrap()(inst, variable)
    }

    pub unsafe fn write_variable(
        &self,
        inst: *mut CreationGraphInstanceT,
        variable: u64,
        n: u32,
        size: u32,
    ) -> *mut ::std::os::raw::c_void {
        self.write_variable.unwrap()(inst, variable, n, size)
    }

    pub unsafe fn random_float(&self, rc: *mut CreationGraphInstanceT) -> f32 {
        self.random_float.unwrap()(rc)
    }
}

impl crate::Api for CreationGraphInterpreterApi {
    const NAME: ConstCStr = const_cstr!("tm_creation_graph_interpreter_api");
}

pub const TM_TT_TYPE_HASH__CREATION_GRAPH: StrhashT = StrhashT {
    u64_: 14263233325791661430u64,
};
pub const TM_CREATION_GRAPH_EVENT_COMPILE: StrhashT = StrhashT {
    u64_: 4357650677474677863u64,
};
pub const TM_CREATION_GRAPH_EVENT_INIT: StrhashT = StrhashT {
    u64_: 4906703793241200758u64,
};
pub const TM_CREATION_GRAPH_EVENT_TICK: StrhashT = StrhashT {
    u64_: 15852286598673457824u64,
};
pub const TM_CREATION_GRAPH_EVENT_NODE_OUTPUT: StrhashT = StrhashT {
    u64_: 8475023083591677920u64,
};
pub const TM_CI_CREATION_GRAPH_INSTANCE_PROVIDER: StrhashT = StrhashT {
    u64_: 17478698051045250385u64,
};
pub const TM_TYPE_HASH__CREATION_GRAPH__INSTANCE: StrhashT = StrhashT {
    u64_: 13519466112014054944u64,
};
pub const TM_TT_TYPE_HASH__STRING_HASH: StrhashT = StrhashT {
    u64_: 17092832035897777594u64,
};
pub const TM_TYPE_HASH__GPU_GEOMETRY: StrhashT = StrhashT {
    u64_: 111032956890368325u64,
};
pub const TM_TT_TYPE_HASH__IMAGE_MIPMAP_SETTINGS: StrhashT = StrhashT {
    u64_: 11382068134251936923u64,
};
pub const TM_TT_TYPE_HASH__IMAGE_DESCRIPTION: StrhashT = StrhashT {
    u64_: 8033654588499689407u64,
};
pub const TM_TT_TYPE_HASH__IMAGE_ARCHIVE: StrhashT = StrhashT {
    u64_: 13900148506330692421u64,
};
pub const TM_CREATION_GRAPH__IMAGE__OUTPUT_NODE_HASH: StrhashT = StrhashT {
    u64_: 14434400046901944053u64,
};
pub const TM_TYPE_HASH__GPU_BUFFER: StrhashT = StrhashT {
    u64_: 8638320117310676114u64,
};
pub const TM_CREATION_GRAPH__SHADER_INSTANCE_OUTPUT_HASH: StrhashT = StrhashT {
    u64_: 17335244708126637406u64,
};
pub const TM_TYPE_HASH__SHADER_INSTANCE: StrhashT = StrhashT {
    u64_: 7875495152435762712u64,
};
pub const TM_TYPE_HASH__CREATION_GRAPH_SHADER_INSTANCE: StrhashT = StrhashT {
    u64_: 6679199256464242056u64,
};
pub const TM_CREATION_GRAPH__BOUNDING_VOLUME_HASH: StrhashT = StrhashT {
    u64_: 7116090811976251669u64,
};
pub const TM_CREATION_GRAPH__DRAW_CALL_HASH: StrhashT = StrhashT {
    u64_: 3141252993814894569u64,
};
pub const TM_CREATION_GRAPH__PHYSICS_SHAPE__OUTPUT_NODE_HASH: StrhashT = StrhashT {
    u64_: 11679706363691094784u64,
};
pub const TM_CREATION_GRAPH__RAY_TRACE_INSTANCE_HASH: StrhashT = StrhashT {
    u64_: 18169300602035247580u64,
};
pub const TM_TYPE_HASH__GPU_SIMULATION__CHANNEL_BUFFER_DATA: StrhashT = StrhashT {
    u64_: 17496717424579581404u64,
};
