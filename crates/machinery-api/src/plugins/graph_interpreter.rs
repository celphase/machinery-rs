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
pub const TM_TT_TYPE__GRAPH_COMPONENT: &'static [u8; 19usize] = b"tm_graph_component\0";
pub const TM_TT_TYPE__ENTITY_GRAPH: &'static [u8; 16usize] = b"tm_entity_graph\0";
pub const TM_GRAPH_COMPONENT_COMPILE_DATA_INTERFACE_NAME: &'static [u8; 32usize] =
    b"tm_graph_component_compile_data\0";
pub const TM_GRAPH_COMPONENT_API_NAME: &'static [u8; 23usize] = b"tm_graph_component_api\0";
pub const TM_GRAPH_COMPONENT_NODE_INTERFACE_NAME: &'static [u8; 34usize] =
    b"tm_graph_component_node_interface\0";
pub const TM_GRAPH_COMPONENT_IO_TYPE_INTERFACE_NAME: &'static [u8; 37usize] =
    b"tm_graph_component_io_type_interface\0";
pub const TM_TT_TYPE__KEYBOARD_ITEM: &'static [u8; 17usize] = b"tm_keyboard_item\0";
pub const TM_TT_TYPE__MOUSE_BUTTON: &'static [u8; 16usize] = b"tm_mouse_button\0";
pub const TM_TT_TYPE__ENTITY_ASSET_REFERENCE: &'static [u8; 26usize] =
    b"tm_entity_asset_reference\0";
pub const TM_TT_TYPE__LOCAL_ENTITY_ASSET_REFERENCE: &'static [u8; 32usize] =
    b"tm_local_entity_asset_reference\0";
pub const TM_TT_TYPE__TAG_REFERENCE: &'static [u8; 17usize] = b"tm_tag_reference\0";
pub const TM_TT_TYPE__CREATION_GRAPH_ASSET_REFERENCE: &'static [u8; 34usize] =
    b"tm_creation_graph_asset_reference\0";
pub const TM_GRAPH_INTERPRETER_API_NAME: &'static [u8; 25usize] = b"tm_graph_interpreter_api\0";
pub const TM_GRAPH_INTERPRETER_DEBUGGER_API_NAME: &'static [u8; 34usize] =
    b"tm_graph_interpreter_debugger_api\0";
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
pub const TM_TT_PROP__GRAPH__NODES: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH__CONNECTIONS: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH__DATA: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH__COMMENTS: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH__INTERFACE: ::std::os::raw::c_int = 4;
pub type _bindgen_ty_1 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_NODE__TYPE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_NODE__LABEL: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_NODE__POSITION_X: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_NODE__POSITION_Y: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH_NODE__WIDTH: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__GRAPH_NODE__SETTINGS: ::std::os::raw::c_int = 5;
pub type _bindgen_ty_2 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_CONNECTION__FROM_NODE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_CONNECTION__TO_NODE: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_CONNECTION__FROM_CONNECTOR_HASH: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_CONNECTION__TO_CONNECTOR_HASH: ::std::os::raw::c_int = 3;
pub type _bindgen_ty_3 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_DATA__TO_NODE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_DATA__TO_CONNECTOR_HASH: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_DATA__DATA: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_4 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_COMMENT__TEXT: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_COMMENT__POSITION_X: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_COMMENT__POSITION_Y: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_COMMENT__FONT_SCALE: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH_COMMENT__COLOR: ::std::os::raw::c_int = 4;
pub type _bindgen_ty_5 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_INTERFACE__INPUTS: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_INTERFACE__OUTPUTS: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_INTERFACE__LAST_ID: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_INTERFACE__CACHED_NODE_RESULT: ::std::os::raw::c_int = 3;
pub type _bindgen_ty_6 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_INPUT__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_INPUT__DISPLAY_NAME: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_INPUT__ID: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_INPUT__TYPE_HASH: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH_INPUT__VALUE_SET_BY_USER: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__GRAPH_INPUT__VALUE: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__GRAPH_INPUT__TOOLTIP: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__GRAPH_INPUT__PUBLIC: ::std::os::raw::c_int = 7;
pub const TM_TT_PROP__GRAPH_INPUT__ORDER: ::std::os::raw::c_int = 8;
pub type _bindgen_ty_7 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_OUTPUT__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_OUTPUT__DISPLAY_NAME: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_OUTPUT__ID: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_OUTPUT__TYPE_HASH: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH_OUTPUT__ORDER: ::std::os::raw::c_int = 4;
pub type _bindgen_ty_8 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_CACHED_NODE_RESULT__NODE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_CACHED_NODE_RESULT__VALIDITY_HASH: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_CACHED_NODE_RESULT__OBJECT: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_9 = ::std::os::raw::c_int;
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
pub type _bindgen_ty_10 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_SETTINGS__BREAKPOINTS: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_SETTINGS__OVERLAY_OPENED: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_11 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__NODE_UNIQUE_ID: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__NODE_UUID_A: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__NODE_UUID_B: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__SUBGRAPH_NODE_UNIQUE_ID: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__SUBGRAPH_NODE_UUID_A: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__SUBGRAPH_NODE_UUID_B: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__DISABLED: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__GRAPH_UUID_A: ::std::os::raw::c_int = 7;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__GRAPH_UUID_B: ::std::os::raw::c_int = 8;
pub type _bindgen_ty_12 = ::std::os::raw::c_int;
pub const TM_GRAPH_MAX_CONNECTORS: ::std::os::raw::c_int = 24;
pub type _bindgen_ty_13 = ::std::os::raw::c_int;
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GraphInterpreterO {
    _unused: [u8; 0],
}
pub const TM_TT_PROP__GRAPH_COMPONENT__GRAPH: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_14 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GraphComponentT {
    pub gr: *mut GraphInterpreterO,
}
impl Default for GraphComponentT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_TT_PROP__ENTITY_GRAPH__GRAPH: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_15 = ::std::os::raw::c_int;
pub type GraphComponentCompileDataI = ::std::option::Option<
    unsafe extern "C" fn(
        gr: *mut GraphInterpreterO,
        wire: u32,
        tt: *const TheTruthO,
        data_id: TtIdT,
        to_type_hash: StrhashT,
    ) -> bool,
>;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct GraphComponentApi {
    pub create_truth_types: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
    pub create: ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO)>,
    pub run_event:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO, event: StrhashT)>,
    pub queue_event:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO, event: StrhashT)>,
    pub tick_queues: ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO)>,
}
#[repr(C)]
pub struct GraphComponentCompileContextT {
    pub tt: *const TheTruthO,
    pub graph_id: TtIdT,
    pub settings_id: TtIdT,
    pub node_id: TtIdT,
    pub node_unique_id: u64,
    pub connectors: *mut GraphNodeConnectorsT,
}
impl Default for GraphComponentCompileContextT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct GraphComponentNodeTypeI {
    pub super_: GraphNodeTypeI,
    pub run: ::std::option::Option<unsafe extern "C" fn(ctx: *mut GraphInterpreterContextT)>,
    pub compile: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut GraphInterpreterContextT,
            compile_ctx: *mut GraphComponentCompileContextT,
        ),
    >,
}
impl Default for GraphComponentNodeTypeI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_GRAPH_COMPONENT__STATIC_WIRE__SINK: ::std::os::raw::c_int = 0;
pub const TM_GRAPH_COMPONENT__STATIC_WIRE__ENTITY_CONTEXT: ::std::os::raw::c_int = 1;
pub const TM_GRAPH_COMPONENT__STATIC_WIRE__ENTITY: ::std::os::raw::c_int = 2;
pub const TM_GRAPH_COMPONENT__STATIC_WIRE__EVENT_DATA: ::std::os::raw::c_int = 3;
pub const TM_GRAPH_COMPONENT__STATIC_WIRE__USER_DATA: ::std::os::raw::c_int = 4;
pub const TM_GRAPH_COMPONENT__STATIC_WIRE__COUNT: ::std::os::raw::c_int = 5;
pub type _bindgen_ty_16 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GraphInterpreterWireContentT {
    pub n: u32,
    pub size: u32,
    pub data: *mut ::std::os::raw::c_void,
}
impl Default for GraphInterpreterWireContentT {
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
pub struct GraphInterpreterContextT {
    pub interpreter: *mut GraphInterpreterO,
    pub node: u32,
    pub _padding_55: [::std::os::raw::c_char; 4usize],
    pub node_type: *const GraphNodeTypeI,
    pub wires: *const u32,
}
impl Default for GraphInterpreterContextT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type GraphInterpreterRunNodeF =
    ::std::option::Option<unsafe extern "C" fn(ctx: *mut GraphInterpreterContextT)>;
#[repr(C)]
pub struct GraphInterpreterNodeT {
    pub source: TtIdT,
    pub unique_id: u64,
    pub f: GraphInterpreterRunNodeF,
    pub node_type: *const GraphNodeTypeI,
    pub dependencies_index: u32,
    pub wires_index: u32,
    pub is_pure: bool,
    pub _padding_106: [::std::os::raw::c_char; 7usize],
}
impl Default for GraphInterpreterNodeT {
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
pub struct GraphInterpreterApi {
    pub create_graph_interpreter: ::std::option::Option<
        unsafe extern "C" fn(allocator: *mut AllocatorI) -> *mut GraphInterpreterO,
    >,
    pub destroy_graph_interpreter:
        ::std::option::Option<unsafe extern "C" fn(gr: *mut GraphInterpreterO)>,
    pub set_graph: ::std::option::Option<
        unsafe extern "C" fn(
            gr: *mut GraphInterpreterO,
            graph_id: TtIdT,
            nodes: *const GraphInterpreterNodeT,
            num_nodes: u32,
            num_wires: u32,
            node_wires: *mut u32,
            num_node_wires: u32,
            dependencies: *mut u32,
            num_dependencies: u32,
        ),
    >,
    pub get_graph: ::std::option::Option<unsafe extern "C" fn(gr: *mut GraphInterpreterO) -> TtIdT>,
    pub run_node_at_index:
        ::std::option::Option<unsafe extern "C" fn(gr: *mut GraphInterpreterO, node: u32)>,
    pub add_event_node: ::std::option::Option<
        unsafe extern "C" fn(gr: *mut GraphInterpreterO, unique_id: u64, e: StrhashT, node: u32),
    >,
    pub trigger_event:
        ::std::option::Option<unsafe extern "C" fn(gr: *mut GraphInterpreterO, e: StrhashT)>,
    pub trigger_wire:
        ::std::option::Option<unsafe extern "C" fn(gr: *mut GraphInterpreterO, wire: u32)>,
    pub queue_event:
        ::std::option::Option<unsafe extern "C" fn(gr: *mut GraphInterpreterO, e: StrhashT)>,
    pub queue_wire: ::std::option::Option<
        unsafe extern "C" fn(gr: *mut GraphInterpreterO, wire: u32, delay: f32),
    >,
    pub tick_queue:
        ::std::option::Option<unsafe extern "C" fn(gr: *mut GraphInterpreterO, dt: f32)>,
    pub get_node_run_context: ::std::option::Option<
        unsafe extern "C" fn(gr: *mut GraphInterpreterO, node: u32) -> GraphInterpreterContextT,
    >,
    pub read_wire: ::std::option::Option<
        unsafe extern "C" fn(gr: *mut GraphInterpreterO, wire: u32) -> GraphInterpreterWireContentT,
    >,
    pub read_wires: ::std::option::Option<
        unsafe extern "C" fn(
            gr: *mut GraphInterpreterO,
            content: *mut GraphInterpreterWireContentT,
            wires: *const u32,
            n: u32,
        ),
    >,
    pub read_wires_indirect: ::std::option::Option<
        unsafe extern "C" fn(
            gr: *mut GraphInterpreterO,
            content: *mut *mut GraphInterpreterWireContentT,
            wires: *const u32,
            n: u32,
        ),
    >,
    pub write_wire: ::std::option::Option<
        unsafe extern "C" fn(
            gr: *mut GraphInterpreterO,
            wire: u32,
            n: u32,
            size: u32,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub read_variable: ::std::option::Option<
        unsafe extern "C" fn(
            gr: *mut GraphInterpreterO,
            variable: u64,
        ) -> GraphInterpreterWireContentT,
    >,
    pub write_variable: ::std::option::Option<
        unsafe extern "C" fn(
            gr: *mut GraphInterpreterO,
            variable: u64,
            n: u32,
            size: u32,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub all_variables: ::std::option::Option<
        unsafe extern "C" fn(gr: *mut GraphInterpreterO, allocator: *mut AllocatorI) -> *mut u64,
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GraphImportedRewiredConnectionT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GraphImportedRewiredNodeT {
    _unused: [u8; 0],
}
pub const TM_GRAPH_INTERPRETER_DEBUGGER_MAX_DATA_BYTES: ::std::os::raw::c_int = 64;
pub type _bindgen_ty_17 = ::std::os::raw::c_int;
#[repr(C)]
pub struct GraphInterpreterDebuggerWireT {
    pub activity: ClockO,
    pub n: u32,
    pub size: u32,
    pub data: [::std::os::raw::c_char; 64usize],
}
impl Default for GraphInterpreterDebuggerWireT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct GraphInterpreterDebuggerWireToIdxT {
    pub wire_id: TtIdT,
    pub wire_index: u32,
    pub _padding_38: [::std::os::raw::c_char; 4usize],
}
impl Default for GraphInterpreterDebuggerWireToIdxT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct GraphInterpreterDebuggerBreakpointT {
    pub breakpoint: TtIdT,
    pub subgraph_node: TtIdT,
    pub graph_id: TtIdT,
    pub subgraph_id: TtIdT,
    pub step_node_id: TtIdT,
    pub breakpoint_unique_id: u64,
    pub subgraph_node_unique_id: u64,
}
impl Default for GraphInterpreterDebuggerBreakpointT {
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
pub struct GraphInterpreterDebuggerStatePointersT {
    pub enabled: *mut bool,
    pub breakpoints_enabled: *mut bool,
    pub connected: *mut bool,
    pub at_breakpoint: *mut bool,
    pub stepping: *mut bool,
}
impl Default for GraphInterpreterDebuggerStatePointersT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct GraphInterpreterDebuggerBrokenNodeQueueItemT {
    pub interpreter: *mut GraphInterpreterO,
    pub graph_id: TtIdT,
    pub node_id: TtIdT,
    pub idx: u32,
    pub _padding_131: [::std::os::raw::c_char; 4usize],
}
impl Default for GraphInterpreterDebuggerBrokenNodeQueueItemT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_GRAPH_INTERPRETER_DEBUGGER_KNOWN_GRAPH__INSTANCE: GraphInterpreterDebuggerKnownType =
    0;
pub const TM_GRAPH_INTERPRETER_DEBUGGER_KNOWN_GRAPH__SUBGRAPH: GraphInterpreterDebuggerKnownType =
    1;
pub const TM_GRAPH_INTERPRETER_DEBUGGER_KNOWN_GRAPH__PROTOTYPE: GraphInterpreterDebuggerKnownType =
    2;
pub type GraphInterpreterDebuggerKnownType = ::std::os::raw::c_int;
#[repr(C)]
pub struct GraphInterpreterDebuggerKnownGraphT {
    pub graph_id: TtIdT,
    pub instance_id: u64,
    pub instance_graph: TtIdT,
    pub associated_type: GraphInterpreterDebuggerKnownType,
    pub _padding_156: [::std::os::raw::c_char; 4usize],
}
impl Default for GraphInterpreterDebuggerKnownGraphT {
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
pub struct GraphInterpreterDebuggerApi {
    pub enable_debugger: ::std::option::Option<unsafe extern "C" fn()>,
    pub enable_breakpoints: ::std::option::Option<unsafe extern "C" fn()>,
    pub disable_debugger: ::std::option::Option<unsafe extern "C" fn()>,
    pub disable_breakpoints: ::std::option::Option<unsafe extern "C" fn()>,
    pub is_debugger_enabled: ::std::option::Option<unsafe extern "C" fn() -> bool>,
    pub are_breakpoints_enabled: ::std::option::Option<unsafe extern "C" fn() -> bool>,
    pub is_connected: ::std::option::Option<unsafe extern "C" fn() -> bool>,
    pub is_graph_instances_known:
        ::std::option::Option<unsafe extern "C" fn(instance_id: u64) -> bool>,
    pub known_graphs: ::std::option::Option<
        unsafe extern "C" fn(ta: *mut TempAllocatorI) -> *const GraphInterpreterDebuggerKnownGraphT,
    >,
    pub instances: ::std::option::Option<
        unsafe extern "C" fn(
            graph_id: TtIdT,
            ta: *mut TempAllocatorI,
        ) -> *const GraphInterpreterDebuggerKnownGraphT,
    >,
    pub state_pointers:
        ::std::option::Option<unsafe extern "C" fn() -> GraphInterpreterDebuggerStatePointersT>,
    pub add_graph_information: ::std::option::Option<
        unsafe extern "C" fn(
            instance_id: u64,
            graph: TtIdT,
            prototype: TtIdT,
            graph_version: u32,
            nodes: *const GraphInterpreterNodeT,
            subgraphs: *mut TtIdT,
            graph_prototypes: *mut TtIdT,
            wires_mapping: *mut GraphInterpreterDebuggerWireToIdxT,
            rewired_connection: *const GraphImportedRewiredConnectionT,
            rewired_nodes: *const GraphImportedRewiredNodeT,
            num_nodes: u32,
            num_graph_prototypes: u32,
            num_rewired_connection: u32,
            num_rewired_nodes: u32,
            num_wires_mapping: u32,
            num_wires: u32,
        ),
    >,
    pub node_activity: ::std::option::Option<
        unsafe extern "C" fn(instance_id: u64, node_unique_id: u64) -> ClockO,
    >,
    pub set_node_active:
        ::std::option::Option<unsafe extern "C" fn(instance_id: u64, node_unique_id: u64)>,
    pub write_wire: ::std::option::Option<
        unsafe extern "C" fn(
            instance_id: u64,
            wire_index: u32,
            n: u32,
            data: *const ::std::os::raw::c_void,
            size: u32,
        ),
    >,
    pub read_wire: ::std::option::Option<
        unsafe extern "C" fn(instance_id: u64, wire_id: TtIdT) -> GraphInterpreterDebuggerWireT,
    >,
    pub add_breakpoint:
        ::std::option::Option<unsafe extern "C" fn(graph: TtIdT, node_unique_id: u64)>,
    pub remove_breakpoint:
        ::std::option::Option<unsafe extern "C" fn(graph: TtIdT, node_unique_id: u64)>,
    pub has_breakpoint:
        ::std::option::Option<unsafe extern "C" fn(graph: TtIdT, node_unique_id: u64) -> bool>,
    pub break_here: ::std::option::Option<unsafe extern "C" fn(graph: TtIdT, node_unique_id: u64)>,
    pub breakpoint:
        ::std::option::Option<unsafe extern "C" fn() -> GraphInterpreterDebuggerBreakpointT>,
    pub at_breakpoint: ::std::option::Option<unsafe extern "C" fn() -> bool>,
    pub continue_execution: ::std::option::Option<unsafe extern "C" fn()>,
    pub breakpoints: ::std::option::Option<
        unsafe extern "C" fn(
            graph: TtIdT,
            ta: *mut TempAllocatorI,
        ) -> *const GraphInterpreterDebuggerBreakpointT,
    >,
    pub add_node_to_broken_queue: ::std::option::Option<
        unsafe extern "C" fn(gr: *mut GraphInterpreterO, graph: TtIdT, unique_id: u64, idx: u32),
    >,
    pub broken_node_queue: ::std::option::Option<
        unsafe extern "C" fn(
            ta: *mut TempAllocatorI,
        ) -> *const GraphInterpreterDebuggerBrokenNodeQueueItemT,
    >,
    pub broken_node_queue_size: ::std::option::Option<unsafe extern "C" fn() -> u64>,
    pub reset_broken_node_queue: ::std::option::Option<unsafe extern "C" fn()>,
    pub step: ::std::option::Option<unsafe extern "C" fn()>,
    pub internal__remove_instance: ::std::option::Option<unsafe extern "C" fn(instance_id: u64)>,
}

// Extensions generated by machinery-generator

use const_cstr::{const_cstr, ConstCStr};

use crate::foundation::*;
use crate::plugins::editor_views::*;
use crate::plugins::entity::*;

impl GraphComponentApi {
    pub unsafe fn create_truth_types(&self, tt: *mut TheTruthO) {
        self.create_truth_types.unwrap()(tt)
    }

    pub unsafe fn create(&self, ctx: *mut EntityContextO) {
        self.create.unwrap()(ctx)
    }

    pub unsafe fn run_event(&self, ctx: *mut EntityContextO, event: StrhashT) {
        self.run_event.unwrap()(ctx, event)
    }

    pub unsafe fn queue_event(&self, ctx: *mut EntityContextO, event: StrhashT) {
        self.queue_event.unwrap()(ctx, event)
    }

    pub unsafe fn tick_queues(&self, ctx: *mut EntityContextO) {
        self.tick_queues.unwrap()(ctx)
    }
}

impl crate::Api for GraphComponentApi {
    const NAME: ConstCStr = const_cstr!("tm_graph_component_api");
}

impl GraphInterpreterApi {
    pub unsafe fn create_graph_interpreter(
        &self,
        allocator: *mut AllocatorI,
    ) -> *mut GraphInterpreterO {
        self.create_graph_interpreter.unwrap()(allocator)
    }

    pub unsafe fn destroy_graph_interpreter(&self, gr: *mut GraphInterpreterO) {
        self.destroy_graph_interpreter.unwrap()(gr)
    }

    pub unsafe fn set_graph(
        &self,
        gr: *mut GraphInterpreterO,
        graph_id: TtIdT,
        nodes: *const GraphInterpreterNodeT,
        num_nodes: u32,
        num_wires: u32,
        node_wires: *mut u32,
        num_node_wires: u32,
        dependencies: *mut u32,
        num_dependencies: u32,
    ) {
        self.set_graph.unwrap()(
            gr,
            graph_id,
            nodes,
            num_nodes,
            num_wires,
            node_wires,
            num_node_wires,
            dependencies,
            num_dependencies,
        )
    }

    pub unsafe fn get_graph(&self, gr: *mut GraphInterpreterO) -> TtIdT {
        self.get_graph.unwrap()(gr)
    }

    pub unsafe fn run_node_at_index(&self, gr: *mut GraphInterpreterO, node: u32) {
        self.run_node_at_index.unwrap()(gr, node)
    }

    pub unsafe fn add_event_node(
        &self,
        gr: *mut GraphInterpreterO,
        unique_id: u64,
        e: StrhashT,
        node: u32,
    ) {
        self.add_event_node.unwrap()(gr, unique_id, e, node)
    }

    pub unsafe fn trigger_event(&self, gr: *mut GraphInterpreterO, e: StrhashT) {
        self.trigger_event.unwrap()(gr, e)
    }

    pub unsafe fn trigger_wire(&self, gr: *mut GraphInterpreterO, wire: u32) {
        self.trigger_wire.unwrap()(gr, wire)
    }

    pub unsafe fn queue_event(&self, gr: *mut GraphInterpreterO, e: StrhashT) {
        self.queue_event.unwrap()(gr, e)
    }

    pub unsafe fn queue_wire(&self, gr: *mut GraphInterpreterO, wire: u32, delay: f32) {
        self.queue_wire.unwrap()(gr, wire, delay)
    }

    pub unsafe fn tick_queue(&self, gr: *mut GraphInterpreterO, dt: f32) {
        self.tick_queue.unwrap()(gr, dt)
    }

    pub unsafe fn get_node_run_context(
        &self,
        gr: *mut GraphInterpreterO,
        node: u32,
    ) -> GraphInterpreterContextT {
        self.get_node_run_context.unwrap()(gr, node)
    }

    pub unsafe fn read_wire(
        &self,
        gr: *mut GraphInterpreterO,
        wire: u32,
    ) -> GraphInterpreterWireContentT {
        self.read_wire.unwrap()(gr, wire)
    }

    pub unsafe fn read_wires(
        &self,
        gr: *mut GraphInterpreterO,
        content: *mut GraphInterpreterWireContentT,
        wires: *const u32,
        n: u32,
    ) {
        self.read_wires.unwrap()(gr, content, wires, n)
    }

    pub unsafe fn read_wires_indirect(
        &self,
        gr: *mut GraphInterpreterO,
        content: *mut *mut GraphInterpreterWireContentT,
        wires: *const u32,
        n: u32,
    ) {
        self.read_wires_indirect.unwrap()(gr, content, wires, n)
    }

    pub unsafe fn write_wire(
        &self,
        gr: *mut GraphInterpreterO,
        wire: u32,
        n: u32,
        size: u32,
    ) -> *mut ::std::os::raw::c_void {
        self.write_wire.unwrap()(gr, wire, n, size)
    }

    pub unsafe fn read_variable(
        &self,
        gr: *mut GraphInterpreterO,
        variable: u64,
    ) -> GraphInterpreterWireContentT {
        self.read_variable.unwrap()(gr, variable)
    }

    pub unsafe fn write_variable(
        &self,
        gr: *mut GraphInterpreterO,
        variable: u64,
        n: u32,
        size: u32,
    ) -> *mut ::std::os::raw::c_void {
        self.write_variable.unwrap()(gr, variable, n, size)
    }

    pub unsafe fn all_variables(
        &self,
        gr: *mut GraphInterpreterO,
        allocator: *mut AllocatorI,
    ) -> *mut u64 {
        self.all_variables.unwrap()(gr, allocator)
    }
}

impl crate::Api for GraphInterpreterApi {
    const NAME: ConstCStr = const_cstr!("tm_graph_interpreter_api");
}

impl GraphInterpreterDebuggerApi {
    pub unsafe fn enable_debugger(&self) {
        self.enable_debugger.unwrap()()
    }

    pub unsafe fn enable_breakpoints(&self) {
        self.enable_breakpoints.unwrap()()
    }

    pub unsafe fn disable_debugger(&self) {
        self.disable_debugger.unwrap()()
    }

    pub unsafe fn disable_breakpoints(&self) {
        self.disable_breakpoints.unwrap()()
    }

    pub unsafe fn is_debugger_enabled(&self) -> bool {
        self.is_debugger_enabled.unwrap()()
    }

    pub unsafe fn are_breakpoints_enabled(&self) -> bool {
        self.are_breakpoints_enabled.unwrap()()
    }

    pub unsafe fn is_connected(&self) -> bool {
        self.is_connected.unwrap()()
    }

    pub unsafe fn is_graph_instances_known(&self, instance_id: u64) -> bool {
        self.is_graph_instances_known.unwrap()(instance_id)
    }

    pub unsafe fn known_graphs(
        &self,
        ta: *mut TempAllocatorI,
    ) -> *const GraphInterpreterDebuggerKnownGraphT {
        self.known_graphs.unwrap()(ta)
    }

    pub unsafe fn instances(
        &self,
        graph_id: TtIdT,
        ta: *mut TempAllocatorI,
    ) -> *const GraphInterpreterDebuggerKnownGraphT {
        self.instances.unwrap()(graph_id, ta)
    }

    pub unsafe fn state_pointers(&self) -> GraphInterpreterDebuggerStatePointersT {
        self.state_pointers.unwrap()()
    }

    pub unsafe fn add_graph_information(
        &self,
        instance_id: u64,
        graph: TtIdT,
        prototype: TtIdT,
        graph_version: u32,
        nodes: *const GraphInterpreterNodeT,
        subgraphs: *mut TtIdT,
        graph_prototypes: *mut TtIdT,
        wires_mapping: *mut GraphInterpreterDebuggerWireToIdxT,
        rewired_connection: *const GraphImportedRewiredConnectionT,
        rewired_nodes: *const GraphImportedRewiredNodeT,
        num_nodes: u32,
        num_graph_prototypes: u32,
        num_rewired_connection: u32,
        num_rewired_nodes: u32,
        num_wires_mapping: u32,
        num_wires: u32,
    ) {
        self.add_graph_information.unwrap()(
            instance_id,
            graph,
            prototype,
            graph_version,
            nodes,
            subgraphs,
            graph_prototypes,
            wires_mapping,
            rewired_connection,
            rewired_nodes,
            num_nodes,
            num_graph_prototypes,
            num_rewired_connection,
            num_rewired_nodes,
            num_wires_mapping,
            num_wires,
        )
    }

    pub unsafe fn node_activity(&self, instance_id: u64, node_unique_id: u64) -> ClockO {
        self.node_activity.unwrap()(instance_id, node_unique_id)
    }

    pub unsafe fn set_node_active(&self, instance_id: u64, node_unique_id: u64) {
        self.set_node_active.unwrap()(instance_id, node_unique_id)
    }

    pub unsafe fn write_wire(
        &self,
        instance_id: u64,
        wire_index: u32,
        n: u32,
        data: *const ::std::os::raw::c_void,
        size: u32,
    ) {
        self.write_wire.unwrap()(instance_id, wire_index, n, data, size)
    }

    pub unsafe fn read_wire(
        &self,
        instance_id: u64,
        wire_id: TtIdT,
    ) -> GraphInterpreterDebuggerWireT {
        self.read_wire.unwrap()(instance_id, wire_id)
    }

    pub unsafe fn add_breakpoint(&self, graph: TtIdT, node_unique_id: u64) {
        self.add_breakpoint.unwrap()(graph, node_unique_id)
    }

    pub unsafe fn remove_breakpoint(&self, graph: TtIdT, node_unique_id: u64) {
        self.remove_breakpoint.unwrap()(graph, node_unique_id)
    }

    pub unsafe fn has_breakpoint(&self, graph: TtIdT, node_unique_id: u64) -> bool {
        self.has_breakpoint.unwrap()(graph, node_unique_id)
    }

    pub unsafe fn break_here(&self, graph: TtIdT, node_unique_id: u64) {
        self.break_here.unwrap()(graph, node_unique_id)
    }

    pub unsafe fn breakpoint(&self) -> GraphInterpreterDebuggerBreakpointT {
        self.breakpoint.unwrap()()
    }

    pub unsafe fn at_breakpoint(&self) -> bool {
        self.at_breakpoint.unwrap()()
    }

    pub unsafe fn continue_execution(&self) {
        self.continue_execution.unwrap()()
    }

    pub unsafe fn breakpoints(
        &self,
        graph: TtIdT,
        ta: *mut TempAllocatorI,
    ) -> *const GraphInterpreterDebuggerBreakpointT {
        self.breakpoints.unwrap()(graph, ta)
    }

    pub unsafe fn add_node_to_broken_queue(
        &self,
        gr: *mut GraphInterpreterO,
        graph: TtIdT,
        unique_id: u64,
        idx: u32,
    ) {
        self.add_node_to_broken_queue.unwrap()(gr, graph, unique_id, idx)
    }

    pub unsafe fn broken_node_queue(
        &self,
        ta: *mut TempAllocatorI,
    ) -> *const GraphInterpreterDebuggerBrokenNodeQueueItemT {
        self.broken_node_queue.unwrap()(ta)
    }

    pub unsafe fn broken_node_queue_size(&self) -> u64 {
        self.broken_node_queue_size.unwrap()()
    }

    pub unsafe fn reset_broken_node_queue(&self) {
        self.reset_broken_node_queue.unwrap()()
    }

    pub unsafe fn step(&self) {
        self.step.unwrap()()
    }
}

impl crate::Api for GraphInterpreterDebuggerApi {
    const NAME: ConstCStr = const_cstr!("tm_graph_interpreter_debugger_api");
}

pub const TM_TT_TYPE_HASH__GRAPH_COMPONENT: StrhashT = StrhashT {
    u64_: 11077266511707467055u64,
};
pub const TM_ENTITY_SYSTEM__GRAPH: StrhashT = StrhashT {
    u64_: 4740580164780661651u64,
};
pub const TM_TT_TYPE_HASH__ENTITY_GRAPH: StrhashT = StrhashT {
    u64_: 7834953514833573731u64,
};
pub const TM_ENTITY_GRAPH_EVENT_INIT: StrhashT = StrhashT {
    u64_: 4906703793241200758u64,
};
pub const TM_ENTITY_GRAPH_EVENT_TICK: StrhashT = StrhashT {
    u64_: 15852286598673457824u64,
};
pub const TM_ENTITY_GRAPH_EVENT_RELOAD: StrhashT = StrhashT {
    u64_: 17410914052309711187u64,
};
pub const TM_ENTITY_GRAPH_EVENT_TERMINATE: StrhashT = StrhashT {
    u64_: 8446025307795536291u64,
};
pub const TM_TT_TYPE_HASH__STRING_HASH: StrhashT = StrhashT {
    u64_: 17092832035897777594u64,
};
pub const TM_TT_TYPE_HASH__KEYBOARD_ITEM: StrhashT = StrhashT {
    u64_: 15822893916688273071u64,
};
pub const TM_TT_TYPE_HASH__MOUSE_BUTTON: StrhashT = StrhashT {
    u64_: 17997745604684857831u64,
};
pub const TM_TT_TYPE_HASH__ENTITY_T: StrhashT = StrhashT {
    u64_: 8449892293430067154u64,
};
pub const TM_TT_TYPE_HASH__ENTITY_ASSET_REFERENCE: StrhashT = StrhashT {
    u64_: 3972629628147668976u64,
};
pub const TM_TT_TYPE_HASH__LOCAL_ENTITY_ASSET_REFERENCE: StrhashT = StrhashT {
    u64_: 17314817880627982338u64,
};
pub const TM_TT_TYPE_HASH__TAG_REFERENCE: StrhashT = StrhashT {
    u64_: 917265911594846130u64,
};
pub const TM_TT_TYPE_HASH__CREATION_GRAPH_ASSET_REFERENCE: StrhashT = StrhashT {
    u64_: 16770856107365122312u64,
};
