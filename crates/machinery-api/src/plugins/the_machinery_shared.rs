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
pub const TYPE__CAMERA_CONTROLLER_COMPONENT: &'static [u8; 31usize] =
    b"tm_camera_controller_component\0";
pub const TM_MAX_RENDER_COMPONENTS: u32 = 64;
pub const TM_MAX_SHADER_DATA_COMPONENTS: u32 = 15;
pub const TM_VIEWPORT_HUD_HEIGHT: f64 = 25.0;
pub const TM_VIEWPORT_HUD_OUTER_MARGIN_X: f64 = 5.0;
pub const TM_VIEWPORT_HUD_OUTER_MARGIN_Y: f64 = 5.0;
pub const TM_VIEWPORT_HUD_INNER_MARGIN: f64 = 5.0;
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
pub struct TheTruthPropertyDefinitionTBindgenTy1 {
    pub enum_editor: __BindgenUnionField<TheTruthEditorEnumT>,
    pub string_open_path_editor: __BindgenUnionField<TheTruthEditorStringOpenPathT>,
    pub string_save_path_editor: __BindgenUnionField<TheTruthEditorStringSavePathT>,
    pub bindgen_union_field: [u64; 3usize],
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
#[repr(C)]
pub struct TtPropValueTBindgenTy1 {
    pub b: __BindgenUnionField<bool>,
    pub u32_: __BindgenUnionField<u32>,
    pub u64_: __BindgenUnionField<u64>,
    pub f32_: __BindgenUnionField<f32>,
    pub f64_: __BindgenUnionField<f64>,
    pub string: __BindgenUnionField<*const ::std::os::raw::c_char>,
    pub buffer: __BindgenUnionField<TtBufferT>,
    pub object: __BindgenUnionField<TtIdT>,
    pub set: __BindgenUnionField<*const TtIdT>,
    pub bindgen_union_field: [u64; 4usize],
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
pub const TM_THE_TRUTH_MAX_PROPERTIES: ::std::os::raw::c_int = 64;
pub type _bindgen_ty_1 = ::std::os::raw::c_int;
#[repr(C)]
pub struct AssetPreviewApiUiArgsT {
    pub tt: *mut TheTruthO,
    pub asset: TtIdT,
    pub entity_ctx: *mut EntityContextO,
    pub entity: *const EntityT,
    pub viewer_render_info: *mut ViewerRenderInfoT,
    pub lighting_environment_settings: *mut LightingEnvironmentSettingsT,
    pub statistics_overlays: *mut StatisticsOverlaysT,
    pub tab: *mut TabI,
    pub ui: *mut UiO,
    pub uistyle: *const UiStyleT,
    pub content_r: RectT,
    pub undo_stack: *mut UndoStackI,
}
impl Default for AssetPreviewApiUiArgsT {
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
pub struct AssetPreviewApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(allocator: *mut AllocatorI) -> *mut AssetPreviewO,
    >,
    pub destroy: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut AssetPreviewO, allocator: *mut AllocatorI),
    >,
    pub create_entity: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetPreviewO,
            tt: *mut TheTruthO,
            asset: TtIdT,
            entity_ctx: *mut EntityContextO,
            result: *mut EntityT,
        ),
    >,
    pub intercept_focus: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetPreviewO,
            tt: *mut TheTruthO,
            root_asset: TtIdT,
            entity_ctx: *mut EntityContextO,
            entity: *const EntityT,
            focus_asset: TtIdT,
        ) -> bool,
    >,
    pub reload: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetPreviewO,
            tt: *mut TheTruthO,
            asset: TtIdT,
            entity_ctx: *mut EntityContextO,
            entity: *mut EntityT,
        ) -> bool,
    >,
    pub dirty: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut AssetPreviewO, tt: *mut TheTruthO, asset: TtIdT) -> bool,
    >,
    pub render: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetPreviewO,
            tt: *mut TheTruthO,
            asset: TtIdT,
            args: *const RenderArgsT,
        ),
    >,
    pub refresh_thumbnail: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetPreviewO,
            tt: *mut TheTruthO,
            asset: TtIdT,
            args: *const RenderArgsT,
        ),
    >,
    pub ui: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut AssetPreviewO, args: *const AssetPreviewApiUiArgsT),
    >,
    pub toolbars: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetPreviewO,
            ta: *mut TempAllocatorI,
            args: *const AssetPreviewApiUiArgsT,
        ) -> *mut ToolbarI,
    >,
    pub update_camera: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetPreviewO,
            tt: *mut TheTruthO,
            cam_tm: *mut TransformT,
            cam_settings: *mut *const CameraSettingsT,
        ),
    >,
    pub show_grid: bool,
    pub _padding_100: [::std::os::raw::c_char; 7usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AssetPreviewI {
    pub api: *mut AssetPreviewApi,
    pub inst: *mut AssetPreviewO,
}
impl Default for AssetPreviewI {
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
pub struct AssetSceneApi {
    pub create:
        ::std::option::Option<unsafe extern "C" fn(allocator: *mut AllocatorI) -> *mut AssetSceneO>,
    pub destroy: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut AssetSceneO, allocator: *mut AllocatorI),
    >,
    pub droppable: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut AssetSceneO, tt: *mut TheTruthO, asset: TtIdT) -> bool,
    >,
    pub create_entity: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetSceneO,
            tt: *mut TheTruthO,
            asset: TtIdT,
            name: *const ::std::os::raw::c_char,
            local_transform: *const TransformT,
            parent_entity: TtIdT,
            asset_root: TtIdT,
            undo_stack: *mut UndoStackI,
        ) -> TtIdT,
    >,
    pub bound_entity_asset: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetSceneO,
            tt: *const TheTruthO,
            asset: TtIdT,
            bounds: *mut Vec3T,
        ),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AssetSceneI {
    pub api: *mut AssetSceneApi,
    pub inst: *mut AssetSceneO,
}
impl Default for AssetSceneI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_ASSET_OPEN_MODE_REUSE_OR_CREATE_TAB: AssetOpenMode = 0;
pub const TM_ASSET_OPEN_MODE_CREATE_TAB: AssetOpenMode = 1;
pub const TM_ASSET_OPEN_MODE_CREATE_TAB_AND_PIN: AssetOpenMode = 2;
pub const TM_ASSET_OPEN_MODE_CREATE_WORKSPACE: AssetOpenMode = 3;
pub type AssetOpenMode = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct AssetOpenAspectI {
    pub open: ::std::option::Option<
        unsafe extern "C" fn(
            app: *mut ApplicationO,
            ui: *mut UiO,
            from_tab: *mut TabI,
            tt: *mut TheTruthO,
            asset: TtIdT,
            open_mode: AssetOpenMode,
        ),
    >,
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
pub struct BakerContextO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct BakerContextApi {
    pub create_context: ::std::option::Option<
        unsafe extern "C" fn(
            a: *mut AllocatorI,
            tt: *mut TheTruthO,
            asset_root: TtIdT,
            render_pipe: *mut RenderPipelineVt,
        ) -> *mut BakerContextO,
    >,
    pub destroy_context: ::std::option::Option<unsafe extern "C" fn(context: *mut BakerContextO)>,
    pub entity_context: ::std::option::Option<
        unsafe extern "C" fn(context: *mut BakerContextO) -> *mut EntityContextO,
    >,
    pub set_entity:
        ::std::option::Option<unsafe extern "C" fn(context: *mut BakerContextO, e: EntityT)>,
    pub render_args: ::std::option::Option<
        unsafe extern "C" fn(context: *mut BakerContextO, args: *mut ViewerRenderArgsT),
    >,
    pub set_camera: ::std::option::Option<
        unsafe extern "C" fn(
            context: *mut BakerContextO,
            camera_tm: *const TransformT,
            camera_settings: *const CameraSettingsT,
        ),
    >,
}
pub const TM_CAMERA_CONTROLLER_MODE__NONE: CameraControllerMode = 0;
pub const TM_CAMERA_CONTROLLER_MODE__FOLLOW: CameraControllerMode = 1;
pub const TM_CAMERA_CONTROLLER_MODE__FREE_FLIGHT: CameraControllerMode = 2;
pub const TM_CAMERA_CONTROLLER_MODE__MAYA_SPIN: CameraControllerMode = 3;
pub const TM_CAMERA_CONTROLLER_MODE__MAYA_ZOOM: CameraControllerMode = 4;
pub const TM_CAMERA_CONTROLLER_MODE__MAYA_PAN: CameraControllerMode = 5;
pub type CameraControllerMode = ::std::os::raw::c_int;
#[repr(C)]
pub struct CameraControllerComponentT {
    pub disable_input: bool,
    pub _padding_33: [::std::os::raw::c_char; 3usize],
    pub mode: CameraControllerMode,
    pub translation_speed: f32,
    pub rotation_speed: f32,
    pub translation_damping: f32,
    pub target_orientation: Vec2T,
    pub follow_speed: f32,
    pub translation: Vec3T,
    pub damped_translation: Vec3T,
    pub rotation: Vec2T,
    pub focus_point: Vec3T,
    pub zoom: f32,
    pub spin: Vec2T,
    pub pan: Vec2T,
}
impl Default for CameraControllerComponentT {
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
pub struct CameraControllerComponentManagerO {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct UiOrientationIndicatorT {
    pub controller: *mut CameraControllerComponentT,
    pub camera: *const CameraT,
    pub viewport: RectT,
    pub rect: RectT,
    pub allow_snapping: bool,
    pub _padding_80: [::std::os::raw::c_char; 7usize],
}
impl Default for UiOrientationIndicatorT {
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
pub struct CameraControllerComponentApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO) -> *mut CameraControllerComponentManagerO,
    >,
    pub feed_ui_input: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut CameraControllerComponentManagerO,
            ui: *mut UiO,
            in_area: bool,
        ),
    >,
    pub register_engines: ::std::option::Option<
        unsafe extern "C" fn(manager: *mut CameraControllerComponentManagerO),
    >,
    pub orientation_indicator: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut CameraControllerComponentManagerO,
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            c: *const UiOrientationIndicatorT,
        ),
    >,
}
pub type CiEditorPropertiesUiF = ::std::option::Option<
    unsafe extern "C" fn(
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        object: TtIdT,
        indent: u32,
    ) -> f32,
>;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct CiEditorUiIconI {
    pub ui_icon: ::std::option::Option<unsafe extern "C" fn() -> u32>,
}
#[repr(C)]
pub struct CiViewportInteract {
    pub tt: *mut TheTruthO,
    pub entity_ctx: *mut EntityContextO,
    pub entity: TtIdT,
    pub component: TtIdT,
    pub ui: *mut UiO,
    pub uistyle: *const UiStyleT,
    pub primitive_buffer: *mut PrimitiveDrawerBufferT,
    pub vertex_buffer: *mut PrimitiveDrawerBufferT,
    pub allocator: *mut AllocatorI,
    pub camera: *const CameraT,
    pub viewport_r: RectT,
    pub viewport_id: u64,
    pub tab_id: u64,
    pub undo_stack: *mut UndoStackI,
    pub active_tool_id: StrhashT,
    pub move_settings: *mut GizmoMoveSettingsT,
    pub rotate_settings: *mut GizmoRotateSettingsT,
    pub scale_settings: *mut GizmoScaleSettingsT,
    pub editor: *mut ::std::os::raw::c_void,
    pub set_selection: ::std::option::Option<
        unsafe extern "C" fn(editor: *mut ::std::os::raw::c_void, item_t: TtIdT),
    >,
}
impl Default for CiViewportInteract {
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
pub struct CiViewportInteractResult {
    pub hide_gizmo: bool,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CiToolbar {
    pub editor: *mut ::std::os::raw::c_void,
    pub active_tool: ::std::option::Option<
        unsafe extern "C" fn(editor: *mut ::std::os::raw::c_void) -> StrhashT,
    >,
    pub set_active_tool: ::std::option::Option<
        unsafe extern "C" fn(editor: *mut ::std::os::raw::c_void, id: StrhashT),
    >,
}
impl Default for CiToolbar {
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
pub struct CiEditorUiI {
    pub disabled: ::std::option::Option<unsafe extern "C" fn() -> bool>,
    pub category: ::std::option::Option<unsafe extern "C" fn() -> *const ::std::os::raw::c_char>,
    pub icon_interface: ::std::option::Option<unsafe extern "C" fn() -> *mut CiEditorUiIconI>,
    pub gizmo_priority: f32,
    pub _padding_95: [::std::os::raw::c_char; 4usize],
    pub gizmo_get_transform: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            ctx: *mut EntityContextO,
            entity: TtIdT,
            component: TtIdT,
            object: TtIdT,
            world: *mut TransformT,
            local: *mut TransformT,
        ) -> bool,
    >,
    pub gizmo_set_transform: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            ctx: *mut EntityContextO,
            entity: TtIdT,
            component: TtIdT,
            object: TtIdT,
            local: *const TransformT,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub gizmo_duplicate: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            ctx: *mut EntityContextO,
            entity: TtIdT,
            component: TtIdT,
            object: TtIdT,
            undo_scope: TtUndoScopeT,
        ) -> TtIdT,
    >,
    pub override_properties: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, other_component: TtIdT) -> CiEditorPropertiesUiF,
    >,
    pub viewport_interact: ::std::option::Option<
        unsafe extern "C" fn(vi: *const CiViewportInteract) -> CiViewportInteractResult,
    >,
    pub editor_ui: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            viewport: RectT,
            manager: *mut ComponentManagerO,
        ),
    >,
    pub create: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, type_: TtTypeT, undo_scope: TtUndoScopeT) -> TtIdT,
    >,
    pub toolbars: ::std::option::Option<
        unsafe extern "C" fn(ci: *mut CiToolbar, ta: *mut TempAllocatorI) -> *mut ToolbarI,
    >,
}
#[repr(C)]
pub struct CiRenderViewerT {
    pub sort_key: u64,
    pub visibility_mask: u64,
    pub visibility_context: StrhashT,
    pub viewer_system: *mut ShaderSystemO,
    pub viewer_cbuffer: *mut ShaderConstantBufferInstanceT,
    pub viewer_rbinder: *mut ShaderResourceBinderInstanceT,
    pub camera: *const CameraT,
    pub gpu_picking: *mut GpuPickingO,
}
impl Default for CiRenderViewerT {
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
pub struct CiRenderI {
    pub init: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ComponentManagerO,
            entities: *const EntityT,
            entity_indices: *const u32,
            render_component_data: *mut *mut ::std::os::raw::c_void,
            num_renderables: u32,
        ),
    >,
    pub bounding_volume_type:
        ::std::option::Option<unsafe extern "C" fn(manager: *mut ComponentManagerO) -> u32>,
    pub fill_bounding_volume_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ComponentManagerO,
            args: *mut RenderArgsT,
            entities: *const EntityT,
            entity_transforms: *const TransformComponentT,
            entity_indices: *const u32,
            render_component_data: *mut *mut ::std::os::raw::c_void,
            num_renderables: u32,
            bv_buffer: *mut u8,
        ),
    >,
    pub render: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ComponentManagerO,
            args: *mut RenderArgsT,
            viewers: *const CiRenderViewerT,
            num_viewers: u32,
            entities: *const EntityT,
            entity_transforms: *const TransformComponentT,
            entity_selection_state: *const bool,
            entity_indices: *const u32,
            render_component_data: *mut *mut ::std::os::raw::c_void,
            num_renderables: u32,
            frustum_visibility: *const u8,
        ),
    >,
}
#[repr(C)]
pub struct CiRenderGatherCallbackArgsT {
    pub allocator: *mut AllocatorI,
    pub selected_entities_lookup: *const SetEntityT,
    pub hidden_entities_lookup: *const SetEntityT,
    pub ignored_entities_lookup: *const SetEntityT,
    pub render_component_names: [StrhashT; 64usize],
    pub render_interfaces: [*mut CiRenderI; 64usize],
    pub num_render_components: u32,
    pub render_component_data_strides: [u32; 64usize],
    pub _padding_74: [::std::os::raw::c_char; 4usize],
    pub component_managers: [*mut ComponentManagerO; 64usize],
    pub entity_selection_state: *mut bool,
    pub entity_ignore_state: *mut bool,
    pub entity_transforms: *mut TransformComponentT,
    pub entities: *mut EntityT,
    pub component_data: [*mut *mut ::std::os::raw::c_void; 64usize],
    pub entity_indices: [*mut u32; 64usize],
    pub num_renderables_per_component: [u32; 64usize],
}
impl Default for CiRenderGatherCallbackArgsT {
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
pub struct CiShaderI {
    pub init: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ComponentManagerO,
            entities: *const EntityT,
            entity_indices: *const u32,
            shader_component_data: *mut *mut ::std::os::raw::c_void,
            num_shader_datas: u32,
        ),
    >,
    pub graph_module_inject: ::std::option::Option<
        unsafe extern "C" fn(manager: *mut ComponentManagerO, module: *mut RenderGraphModuleO),
    >,
    pub graph_requested:
        ::std::option::Option<unsafe extern "C" fn(manager: *mut ComponentManagerO) -> StrhashT>,
    pub activate_systems: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ComponentManagerO,
            shader_context: *mut ShaderSystemContextO,
        ),
    >,
    pub update: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ComponentManagerO,
            args: *mut RenderArgsT,
            entities: *const EntityT,
            entity_transforms: *const TransformComponentT,
            entity_indices: *const u32,
            component_data: *mut *mut ::std::os::raw::c_void,
            num_components: u32,
            frustum_visibility: *const u8,
        ),
    >,
}
#[repr(C)]
pub struct CiShaderDataGatherCallbackArgsT {
    pub allocator: *mut AllocatorI,
    pub hidden_entities_lookup: *const SetEntityT,
    pub shader_component_names: [StrhashT; 15usize],
    pub shader_interfaces: [*mut CiShaderI; 15usize],
    pub num_shader_components: u32,
    pub shader_component_data_strides: [u32; 15usize],
    pub component_managers: [*mut ComponentManagerO; 15usize],
    pub entity_transforms: *mut TransformComponentT,
    pub entities: *mut EntityT,
    pub component_data: [*mut *mut ::std::os::raw::c_void; 15usize],
    pub entity_indices: [*mut u32; 15usize],
    pub num_shader_data_per_component: [u32; 15usize],
    pub _padding_77: [::std::os::raw::c_char; 4usize],
}
impl Default for CiShaderDataGatherCallbackArgsT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct CullingViewerT {
    pub frustum_planes: [Vec4T; 6usize],
    pub visibility_mask: u64,
}
impl Default for CullingViewerT {
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
pub struct GpuCullingHeaderT {
    pub num_viewers: u32,
    pub viewers_offset: u32,
    pub num_transforms: u32,
    pub results_offset: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuCullingArgsT {
    pub shader: *mut ShaderO,
    pub res_buf: *mut RendererResourceCommandBufferO,
    pub cmd_buf: *mut RendererCommandBufferO,
    pub sort_key: u64,
    pub device_affinity_mask: u32,
    pub _padding_57: [::std::os::raw::c_char; 4usize],
    pub viewers: *const CullingViewerT,
    pub viewers_count: u32,
    pub bounding_radius: f32,
    pub transforms: *const RendererHandleT,
    pub transforms_count: u32,
    pub transforms_start: u32,
    pub transform_stride: u32,
    pub culling_distance: f32,
    pub parent_transform: *const Mat44T,
}
impl Default for GpuCullingArgsT {
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
pub struct FrustumCullingApi {
    pub viewer_from_projection_mat: ::std::option::Option<
        unsafe extern "C" fn(
            view_tm: *const Mat44T,
            projection_tm: *const Mat44T,
            visibility_mask: u64,
        ) -> CullingViewerT,
    >,
    pub calc_size_of_objects_buffer:
        ::std::option::Option<unsafe extern "C" fn(bv_type: u32, num_objects: u32) -> u32>,
    pub calc_size_of_results_buffer:
        ::std::option::Option<unsafe extern "C" fn(num_viewers: u32, num_objects: u32) -> u32>,
    pub cull_fast: ::std::option::Option<
        unsafe extern "C" fn(
            viewers: *const CullingViewerT,
            num_viewers: u32,
            bv_type: u32,
            objects: *const u8,
            num_objects: u32,
            results: *mut u8,
            ta: *mut TempAllocatorI,
        ) -> *mut AtomicCounterO,
    >,
    pub cull_precise: ::std::option::Option<
        unsafe extern "C" fn(
            viewers: *const CullingViewerT,
            num_viewers: u32,
            bv_type: u32,
            objects: *const u8,
            num_objects: u32,
            results: *mut u8,
            ta: *mut TempAllocatorI,
        ) -> *mut AtomicCounterO,
    >,
    pub gpu_cull: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut GpuCullingArgsT,
            output: *mut RendererHandleT,
            output_desc: *mut RendererBufferDescT,
        ),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreationGraphDrawCallDataT {
    _unused: [u8; 0],
}
pub const TM_GPU_SCENE_SUBMISSION_DRAW_CALL_LOD_DISABLED: ::std::os::raw::c_int = -1;
pub type _bindgen_ty_2 = ::std::os::raw::c_int;
#[repr(C)]
pub struct GpuSceneSubmissionDrawCallLodSettingsT {
    pub lod_step: u32,
    pub lod_size_range: Vec2T,
}
impl Default for GpuSceneSubmissionDrawCallLodSettingsT {
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
pub struct GpuSceneSubmissionCullAndLodHeaderT {
    pub viewers_offset: u32,
    pub draw_calls_lod_settings_offset: u32,
    pub draw_calls_instance_count_offset: u32,
    pub draw_call_bitmask_offset: u32,
    pub count: u32,
}
pub const TM_GPU_SCENE_SUBMISSION__INDIRECT_DRAW_NON_INDEXED: GpuSceneSubmissionIndirectDrawType =
    0;
pub const TM_GPU_SCENE_SUBMISSION__INDIRECT_DRAW_INDEXED: GpuSceneSubmissionIndirectDrawType = 1;
pub type GpuSceneSubmissionIndirectDrawType = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuSceneSubmissionIndirectDrawCmdT {
    pub draw_type: GpuSceneSubmissionIndirectDrawType,
    pub data: [u32; 5usize],
}
impl Default for GpuSceneSubmissionIndirectDrawCmdT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_GPU_SCENE_SUBMISSION__BYTE_OFFSET__INSTANCE_COUNT: ::std::os::raw::c_int = 8;
pub const TM_GPU_SCENE_SUBMISSION__BYTE_OFFSET__NON_INDEXED__FIRST_INSTANCE: ::std::os::raw::c_int =
    16;
pub const TM_GPU_SCENE_SUBMISSION__BYTE_OFFSET__INDEXED__FIRST_INSTANCE: ::std::os::raw::c_int = 20;
pub type _bindgen_ty_3 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct GpuSceneSubmissionDrawCallsAndTransformIndirectionHeaderT {
    pub draw_calls_offset: u32,
    pub first_instance_index_offset: u32,
    pub stram_compaction_dispatch_command_offset: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuSceneSubmissionWorkloadO {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct GpuSceneSubmissionArgsT {
    pub shader_repo: *mut ShaderRepositoryO,
    pub shader_context: *const ShaderSystemContextO,
    pub workload: *mut GpuSceneSubmissionWorkloadO,
    pub res_buf: *mut RendererResourceCommandBufferO,
    pub cmd_buf: *mut RendererCommandBufferO,
    pub sort_key: u64,
    pub device_affinity_mask: u32,
    pub _padding_169: [::std::os::raw::c_char; 4usize],
    pub viewers: *const CiRenderViewerT,
    pub viewers_count: u32,
    pub bounding_radius: f32,
    pub transforms: RendererHandleT,
    pub _padding_182: [::std::os::raw::c_char; 4usize],
    pub transforms_count: u32,
    pub transforms_start: u32,
    pub transforms_stride: u32,
    pub culling_distance: f32,
    pub parent_transform: *const Mat44T,
}
impl Default for GpuSceneSubmissionArgsT {
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
pub struct GpuSceneSubmissionApi {
    pub create_workload: ::std::option::Option<
        unsafe extern "C" fn(
            draw_calls: *const CreationGraphDrawCallDataT,
            draw_calls_count: u32,
            res_buf: *mut RendererResourceCommandBufferO,
            a: *mut AllocatorI,
        ) -> *mut GpuSceneSubmissionWorkloadO,
    >,
    pub destroy_workload: ::std::option::Option<
        unsafe extern "C" fn(
            workload: *mut GpuSceneSubmissionWorkloadO,
            res_buf: *mut RendererResourceCommandBufferO,
        ),
    >,
    pub cull_and_render:
        ::std::option::Option<unsafe extern "C" fn(args: *mut GpuSceneSubmissionArgsT)>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderContextO {
    _unused: [u8; 0],
}
pub const TM_RENDER_CONTEXT_BUFFER_PHASE_VIEWER: RenderContextBufferPhase = 0;
pub const TM_RENDER_CONTEXT_BUFFER_PHASE_VIEWER_DESTROY: RenderContextBufferPhase = 1;
pub const TM_RENDER_CONTEXT_BUFFER_PHASE_UTILITY: RenderContextBufferPhase = 2;
pub const TM_RENDER_CONTEXT_BUFFER_PHASE_UTILITY_DESTROY: RenderContextBufferPhase = 3;
pub const TM_RENDER_CONTEXT_BUFFER_PHASE_MAX_PHASES: RenderContextBufferPhase = 4;
pub type RenderContextBufferPhase = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct RenderContextApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(allocator: *mut AllocatorI) -> *mut RenderContextO,
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(context: *mut RenderContextO)>,
    pub append_resource_buffers: ::std::option::Option<
        unsafe extern "C" fn(
            context: *mut RenderContextO,
            phase: RenderContextBufferPhase,
            res_buffers: *mut *mut RendererResourceCommandBufferO,
            num_buffers: u32,
        ),
    >,
    pub append_command_buffers: ::std::option::Option<
        unsafe extern "C" fn(
            context: *mut RenderContextO,
            phase: RenderContextBufferPhase,
            cmd_buffers: *mut *mut RendererCommandBufferO,
            num_buffers: u32,
        ),
    >,
    pub resource_buffers: ::std::option::Option<
        unsafe extern "C" fn(
            context: *mut RenderContextO,
            phase: RenderContextBufferPhase,
            res_buffers: *mut *mut RendererResourceCommandBufferO,
        ) -> u32,
    >,
    pub command_buffers: ::std::option::Option<
        unsafe extern "C" fn(
            context: *mut RenderContextO,
            phase: RenderContextBufferPhase,
            cmd_buffers: *mut *mut RendererCommandBufferO,
        ) -> u32,
    >,
}
#[repr(C)]
pub struct RenderArgsT {
    pub camera_tm: TransformT,
    pub camera: *const CameraT,
    pub context: *mut RenderContextO,
    pub render_backend: *mut RendererBackendI,
    pub shader_repository: *mut ShaderRepositoryO,
    pub device_affinity_mask: u32,
    pub _padding_81: [::std::os::raw::c_char; 4usize],
    pub default_resource_buffer: *mut RendererResourceCommandBufferO,
    pub default_command_buffer: *mut RendererCommandBufferO,
    pub render_graph: *mut RenderGraphO,
    pub render_pipeline: *mut RenderPipelineI,
    pub shader_context: *const ShaderSystemContextO,
}
impl Default for RenderArgsT {
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
pub struct RenderPipelineI {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct LightingEnvironmentSettingsT {
    pub enabled: bool,
    pub _padding_32: [::std::os::raw::c_char; 7usize],
    pub asset: TtIdT,
    pub spawned_entity: EntityT,
    pub search_buf: [::std::os::raw::c_char; 1024usize],
}
impl Default for LightingEnvironmentSettingsT {
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
pub struct StatisticsOverlaysT {
    pub tt: *mut TheTruthO,
    pub tab: *mut TabI,
    pub settings_objects: *mut TtIdT,
}
impl Default for StatisticsOverlaysT {
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
pub struct SceneCommonApi {
    pub init_camera:
        ::std::option::Option<unsafe extern "C" fn(camera: *mut TransformT, translation: Vec3T)>,
    pub camera_frame_bounds: ::std::option::Option<
        unsafe extern "C" fn(
            camera: *mut TransformT,
            camera_y_fov: f32,
            bounds: *const Vec3T,
            translation_speed: *mut f32,
            focus_point: *mut Vec3T,
        ),
    >,
    pub find_component_render_interfaces: ::std::option::Option<
        unsafe extern "C" fn(
            entity_ctx: *mut EntityContextO,
            transform_component: ComponentTypeT,
            tt: *const TheTruthO,
            allocator: *mut AllocatorI,
            selection: *const TtIdT,
            selection_n: u64,
            ignore: *const EntityT,
            ignore_n: u64,
            include_entities_without_render_components: bool,
            res: *mut CiRenderGatherCallbackArgsT,
        ),
    >,
    pub bound_assets: ::std::option::Option<
        unsafe extern "C" fn(
            entity_ctx: *mut EntityContextO,
            transform_component: ComponentTypeT,
            tt: *const TheTruthO,
            ignore: *const EntityT,
            ignore_n: u64,
            bounds: *mut Vec3T,
            include_origo: bool,
        ),
    >,
    pub bound_selected_assets: ::std::option::Option<
        unsafe extern "C" fn(
            entity_ctx: *mut EntityContextO,
            transform_component: ComponentTypeT,
            tt: *const TheTruthO,
            selection: *const TtIdT,
            selection_n: u64,
            ignore: *const EntityT,
            ignore_n: u64,
            bounds: *mut Vec3T,
            include_origo: bool,
        ),
    >,
    pub bound_entity_asset: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, entity: TtIdT, bounds: *mut Vec3T),
    >,
    pub find_shader_data_engine_update: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut EngineO,
            data: *mut EngineUpdateSetT,
            commands: *mut EntityCommandsO,
        ),
    >,
    pub gather_shader_data_filter: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut EngineO,
            components: *const ComponentTypeT,
            num_components: u32,
            mask: *const ComponentMaskT,
        ) -> bool,
    >,
    pub find_renderables_engine_update: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut EngineO,
            data: *mut EngineUpdateSetT,
            commands: *mut EntityCommandsO,
        ),
    >,
    pub gather_renderables_filter: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut EngineO,
            components: *const ComponentTypeT,
            num_components: u32,
            mask: *const ComponentMaskT,
        ) -> bool,
    >,
    pub add_default_light_source:
        ::std::option::Option<unsafe extern "C" fn(entity_ctx: *mut EntityContextO) -> EntityT>,
    pub has_any_light_source:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO) -> bool>,
    pub component_visualization_menu: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            entity_ctx: *mut EntityContextO,
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            tab: *mut TabI,
            pos: Vec2T,
        ),
    >,
    pub viewport_visualization_toolbar: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            entity_ctx: *mut EntityContextO,
            le_settings: *mut LightingEnvironmentSettingsT,
            statistics_overlays: *mut StatisticsOverlaysT,
            render_pipeline: *mut RenderPipelineI,
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            tab: *mut TabI,
            toolbar_r: RectT,
            toolbar_draw_mode: u32,
        ) -> RectT,
    >,
    pub statistics_menu: ::std::option::Option<
        unsafe extern "C" fn(
            statistics_overlays: *mut StatisticsOverlaysT,
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            pos: Vec2T,
        ),
    >,
    pub statistics_overlay_toolbars: ::std::option::Option<
        unsafe extern "C" fn(
            statistics_overlays: *mut StatisticsOverlaysT,
            ta: *mut TempAllocatorI,
        ) -> *mut ToolbarI,
    >,
    pub place_entity: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            entity: TtIdT,
            local_transform: *const TransformT,
            parent: TtIdT,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub select_entity: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            tab: *mut TabI,
            entity: TtIdT,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub select_component: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            tab: *mut TabI,
            component: TtIdT,
            undo_scope: TtUndoScopeT,
        ),
    >,
}
#[repr(C)]
pub struct SceneCommandDataT {
    pub tab: *mut TabI,
    pub viewport: RectT,
    pub camera: *mut CameraT,
    pub command_position: Vec2T,
}
impl Default for SceneCommandDataT {
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
pub struct SceneTabCommandI {
    pub can_be_executed:
        ::std::option::Option<unsafe extern "C" fn(data: *mut SceneCommandDataT) -> bool>,
    pub execute: ::std::option::Option<unsafe extern "C" fn(data: *mut SceneCommandDataT)>,
    pub name: *const ::std::os::raw::c_char,
}
impl Default for SceneTabCommandI {
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
pub struct TtAspectNameProperty {
    pub property_idx: u32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct TheTruthReplacerApi {
    pub replace_using_path: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            dest: TtIdT,
            source: TtIdT,
            undo_scope: TtUndoScopeT,
        ),
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct TheTruthStripperI {
    pub strip: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO, asset_root: TtIdT)>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPipelineUpdateFrameParametersT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderdocToolbarArgsT {
    _unused: [u8; 0],
}
pub type ViewerRenderCallbackF = ::std::option::Option<
    unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, args: *const RenderArgsT),
>;
pub type ViewerGatherShaderDataCallbackF = ::std::option::Option<
    unsafe extern "C" fn(
        inst: *mut ::std::os::raw::c_void,
        args: *mut CiShaderDataGatherCallbackArgsT,
    ),
>;
pub type ViewerGatherRenderCallbackF = ::std::option::Option<
    unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, args: *mut CiRenderGatherCallbackArgsT),
>;
#[repr(C)]
pub struct ViewerCameraT {
    pub tm: TransformT,
    pub camera: *const CameraSettingsT,
}
impl Default for ViewerCameraT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type ViewerGatherCameraSettingsCallbackF = ::std::option::Option<
    unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, camera: *mut ViewerCameraT),
>;
pub const TM_VIEWER_CAMERA_SETTINGS_IMMEDIATE: ViewerCameraSettings = 0;
pub const TM_VIEWER_CAMERA_SETTINGS_CALLBACK: ViewerCameraSettings = 1;
pub type ViewerCameraSettings = ::std::os::raw::c_int;
#[repr(C)]
pub struct ViewerRenderArgsTBindgenTy1 {
    pub __bindgen_anon_1: __BindgenUnionField<ViewerRenderArgsTBindgenTy1BindgenTy1>,
    pub camera: __BindgenUnionField<ViewerCameraT>,
    pub bindgen_union_field: [u64; 6usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ViewerRenderArgsTBindgenTy1BindgenTy1 {
    pub gather_camera_settings_callback: ViewerGatherCameraSettingsCallbackF,
    pub gather_camera_settings_callback_inst: *mut ::std::os::raw::c_void,
}
impl Default for ViewerRenderArgsTBindgenTy1BindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for ViewerRenderArgsTBindgenTy1 {
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
pub struct ViewerRenderInfoT {
    pub target_width: u32,
    pub target_height: u32,
    pub vr_context: u32,
    pub _padding_139: [::std::os::raw::c_char; 4usize],
    pub camera: *const CameraT,
    pub render_pipeline: *mut RenderPipelineI,
}
impl Default for ViewerRenderInfoT {
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
pub struct ViewerO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ViewerApi {
    pub request_render: ::std::option::Option<
        unsafe extern "C" fn(
            viewer: *mut ViewerO,
            args: *const ViewerRenderArgsT,
            info: *mut ViewerRenderInfoT,
            res_buf: *mut RendererResourceCommandBufferO,
            cmd_buf: *mut RendererCommandBufferO,
        ) -> RendererHandleT,
    >,
    pub get_color_space: ::std::option::Option<
        unsafe extern "C" fn(viewer: *const ViewerO) -> *const ColorSpaceDescT,
    >,
    pub pipeline:
        ::std::option::Option<unsafe extern "C" fn(viewer: *mut ViewerO) -> *mut RenderPipelineI>,
    pub reset_render_pipeline: ::std::option::Option<unsafe extern "C" fn(viewer: *mut ViewerO)>,
    pub set_render_pipeline_api: ::std::option::Option<
        unsafe extern "C" fn(viewer: *mut ViewerO, pipeline_api: *mut RenderPipelineVt),
    >,
    pub screenshot: ::std::option::Option<unsafe extern "C" fn(viewer: *mut ViewerO)>,
    pub init_vr: ::std::option::Option<unsafe extern "C" fn(viewer: *mut ViewerO, activate: bool)>,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ViewerManagerApi {
    pub create_manager: ::std::option::Option<
        unsafe extern "C" fn(
            allocator: *mut AllocatorI,
            render_backend: *mut RendererBackendI,
            shader_repository: *mut ShaderRepositoryO,
            main_device_affinity: u32,
            default_visibility_context: *mut VisibilityContextO,
            viewport_visibility_flag: u64,
        ) -> *mut ViewerManagerO,
    >,
    pub destroy_manager: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ViewerManagerO,
            res_buf: *mut RendererResourceCommandBufferO,
        ),
    >,
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ViewerManagerO,
            main_module_name: *const ::std::os::raw::c_char,
        ) -> *mut ViewerO,
    >,
    pub destroy: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ViewerManagerO,
            viewer: *mut ViewerO,
            res_buf: *mut RendererResourceCommandBufferO,
        ),
    >,
    pub viewers: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ViewerManagerO,
            active: *mut *mut bool,
        ) -> *mut *mut ViewerO,
    >,
    pub render: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ViewerManagerO,
            shader_context: *const ShaderSystemContextO,
            tt: *const TheTruthO,
            frame_params: *const RenderPipelineUpdateFrameParametersT,
        ),
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct AssetPreviewO {
    pub _address: u8,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct AssetSceneO {
    pub _address: u8,
}

// Extensions generated by machinery-generator

use const_cstr::{const_cstr, ConstCStr};

use crate::foundation::VersionT;

use crate::foundation::*;
use crate::plugins::editor_views::*;
use crate::plugins::entity::*;
use crate::plugins::render_graph::*;
use crate::plugins::renderer::*;
use crate::plugins::shader_system::*;
use crate::plugins::ui::*;

impl AssetPreviewApi {
    pub unsafe fn create(&self, allocator: *mut AllocatorI) -> *mut AssetPreviewO {
        self.create.unwrap()(allocator)
    }

    pub unsafe fn destroy(&self, inst: *mut AssetPreviewO, allocator: *mut AllocatorI) {
        self.destroy.unwrap()(inst, allocator)
    }

    pub unsafe fn create_entity(
        &self,
        inst: *mut AssetPreviewO,
        tt: *mut TheTruthO,
        asset: TtIdT,
        entity_ctx: *mut EntityContextO,
        result: *mut EntityT,
    ) {
        self.create_entity.unwrap()(inst, tt, asset, entity_ctx, result)
    }

    pub unsafe fn intercept_focus(
        &self,
        inst: *mut AssetPreviewO,
        tt: *mut TheTruthO,
        root_asset: TtIdT,
        entity_ctx: *mut EntityContextO,
        entity: *const EntityT,
        focus_asset: TtIdT,
    ) -> bool {
        self.intercept_focus.unwrap()(inst, tt, root_asset, entity_ctx, entity, focus_asset)
    }

    pub unsafe fn reload(
        &self,
        inst: *mut AssetPreviewO,
        tt: *mut TheTruthO,
        asset: TtIdT,
        entity_ctx: *mut EntityContextO,
        entity: *mut EntityT,
    ) -> bool {
        self.reload.unwrap()(inst, tt, asset, entity_ctx, entity)
    }

    pub unsafe fn dirty(&self, inst: *mut AssetPreviewO, tt: *mut TheTruthO, asset: TtIdT) -> bool {
        self.dirty.unwrap()(inst, tt, asset)
    }

    pub unsafe fn render(
        &self,
        inst: *mut AssetPreviewO,
        tt: *mut TheTruthO,
        asset: TtIdT,
        args: *const RenderArgsT,
    ) {
        self.render.unwrap()(inst, tt, asset, args)
    }

    pub unsafe fn refresh_thumbnail(
        &self,
        inst: *mut AssetPreviewO,
        tt: *mut TheTruthO,
        asset: TtIdT,
        args: *const RenderArgsT,
    ) {
        self.refresh_thumbnail.unwrap()(inst, tt, asset, args)
    }

    pub unsafe fn ui(&self, inst: *mut AssetPreviewO, args: *const AssetPreviewApiUiArgsT) {
        self.ui.unwrap()(inst, args)
    }

    pub unsafe fn toolbars(
        &self,
        inst: *mut AssetPreviewO,
        ta: *mut TempAllocatorI,
        args: *const AssetPreviewApiUiArgsT,
    ) -> *mut ToolbarI {
        self.toolbars.unwrap()(inst, ta, args)
    }

    pub unsafe fn update_camera(
        &self,
        inst: *mut AssetPreviewO,
        tt: *mut TheTruthO,
        cam_tm: *mut TransformT,
        cam_settings: *mut *const CameraSettingsT,
    ) {
        self.update_camera.unwrap()(inst, tt, cam_tm, cam_settings)
    }
}

impl crate::Api for AssetPreviewApi {
    const NAME: ConstCStr = const_cstr!("tm_asset_preview_api");
    const VERSION: VersionT = VersionT {
        major: 2u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl AssetSceneApi {
    pub unsafe fn create(&self, allocator: *mut AllocatorI) -> *mut AssetSceneO {
        self.create.unwrap()(allocator)
    }

    pub unsafe fn destroy(&self, inst: *mut AssetSceneO, allocator: *mut AllocatorI) {
        self.destroy.unwrap()(inst, allocator)
    }

    pub unsafe fn droppable(
        &self,
        inst: *mut AssetSceneO,
        tt: *mut TheTruthO,
        asset: TtIdT,
    ) -> bool {
        self.droppable.unwrap()(inst, tt, asset)
    }

    pub unsafe fn create_entity(
        &self,
        inst: *mut AssetSceneO,
        tt: *mut TheTruthO,
        asset: TtIdT,
        name: *const ::std::os::raw::c_char,
        local_transform: *const TransformT,
        parent_entity: TtIdT,
        asset_root: TtIdT,
        undo_stack: *mut UndoStackI,
    ) -> TtIdT {
        self.create_entity.unwrap()(
            inst,
            tt,
            asset,
            name,
            local_transform,
            parent_entity,
            asset_root,
            undo_stack,
        )
    }

    pub unsafe fn bound_entity_asset(
        &self,
        inst: *mut AssetSceneO,
        tt: *const TheTruthO,
        asset: TtIdT,
        bounds: *mut Vec3T,
    ) {
        self.bound_entity_asset.unwrap()(inst, tt, asset, bounds)
    }
}

impl BakerContextApi {
    pub unsafe fn create_context(
        &self,
        a: *mut AllocatorI,
        tt: *mut TheTruthO,
        asset_root: TtIdT,
        render_pipe: *mut RenderPipelineVt,
    ) -> *mut BakerContextO {
        self.create_context.unwrap()(a, tt, asset_root, render_pipe)
    }

    pub unsafe fn destroy_context(&self, context: *mut BakerContextO) {
        self.destroy_context.unwrap()(context)
    }

    pub unsafe fn entity_context(&self, context: *mut BakerContextO) -> *mut EntityContextO {
        self.entity_context.unwrap()(context)
    }

    pub unsafe fn set_entity(&self, context: *mut BakerContextO, e: EntityT) {
        self.set_entity.unwrap()(context, e)
    }

    pub unsafe fn render_args(&self, context: *mut BakerContextO, args: *mut ViewerRenderArgsT) {
        self.render_args.unwrap()(context, args)
    }

    pub unsafe fn set_camera(
        &self,
        context: *mut BakerContextO,
        camera_tm: *const TransformT,
        camera_settings: *const CameraSettingsT,
    ) {
        self.set_camera.unwrap()(context, camera_tm, camera_settings)
    }
}

impl crate::Api for BakerContextApi {
    const NAME: ConstCStr = const_cstr!("tm_baker_context_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl CameraControllerComponentApi {
    pub unsafe fn create(
        &self,
        ctx: *mut EntityContextO,
    ) -> *mut CameraControllerComponentManagerO {
        self.create.unwrap()(ctx)
    }

    pub unsafe fn feed_ui_input(
        &self,
        manager: *mut CameraControllerComponentManagerO,
        ui: *mut UiO,
        in_area: bool,
    ) {
        self.feed_ui_input.unwrap()(manager, ui, in_area)
    }

    pub unsafe fn register_engines(&self, manager: *mut CameraControllerComponentManagerO) {
        self.register_engines.unwrap()(manager)
    }

    pub unsafe fn orientation_indicator(
        &self,
        manager: *mut CameraControllerComponentManagerO,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        c: *const UiOrientationIndicatorT,
    ) {
        self.orientation_indicator.unwrap()(manager, ui, uistyle, c)
    }
}

impl crate::Api for CameraControllerComponentApi {
    const NAME: ConstCStr = const_cstr!("tm_camera_controller_component_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl FrustumCullingApi {
    pub unsafe fn viewer_from_projection_mat(
        &self,
        view_tm: *const Mat44T,
        projection_tm: *const Mat44T,
        visibility_mask: u64,
    ) -> CullingViewerT {
        self.viewer_from_projection_mat.unwrap()(view_tm, projection_tm, visibility_mask)
    }

    pub unsafe fn calc_size_of_objects_buffer(&self, bv_type: u32, num_objects: u32) -> u32 {
        self.calc_size_of_objects_buffer.unwrap()(bv_type, num_objects)
    }

    pub unsafe fn calc_size_of_results_buffer(&self, num_viewers: u32, num_objects: u32) -> u32 {
        self.calc_size_of_results_buffer.unwrap()(num_viewers, num_objects)
    }

    pub unsafe fn cull_fast(
        &self,
        viewers: *const CullingViewerT,
        num_viewers: u32,
        bv_type: u32,
        objects: *const u8,
        num_objects: u32,
        results: *mut u8,
        ta: *mut TempAllocatorI,
    ) -> *mut AtomicCounterO {
        self.cull_fast.unwrap()(
            viewers,
            num_viewers,
            bv_type,
            objects,
            num_objects,
            results,
            ta,
        )
    }

    pub unsafe fn cull_precise(
        &self,
        viewers: *const CullingViewerT,
        num_viewers: u32,
        bv_type: u32,
        objects: *const u8,
        num_objects: u32,
        results: *mut u8,
        ta: *mut TempAllocatorI,
    ) -> *mut AtomicCounterO {
        self.cull_precise.unwrap()(
            viewers,
            num_viewers,
            bv_type,
            objects,
            num_objects,
            results,
            ta,
        )
    }

    pub unsafe fn gpu_cull(
        &self,
        args: *mut GpuCullingArgsT,
        output: *mut RendererHandleT,
        output_desc: *mut RendererBufferDescT,
    ) {
        self.gpu_cull.unwrap()(args, output, output_desc)
    }
}

impl crate::Api for FrustumCullingApi {
    const NAME: ConstCStr = const_cstr!("tm_frustum_culling_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl GpuSceneSubmissionApi {
    pub unsafe fn create_workload(
        &self,
        draw_calls: *const CreationGraphDrawCallDataT,
        draw_calls_count: u32,
        res_buf: *mut RendererResourceCommandBufferO,
        a: *mut AllocatorI,
    ) -> *mut GpuSceneSubmissionWorkloadO {
        self.create_workload.unwrap()(draw_calls, draw_calls_count, res_buf, a)
    }

    pub unsafe fn destroy_workload(
        &self,
        workload: *mut GpuSceneSubmissionWorkloadO,
        res_buf: *mut RendererResourceCommandBufferO,
    ) {
        self.destroy_workload.unwrap()(workload, res_buf)
    }

    pub unsafe fn cull_and_render(&self, args: *mut GpuSceneSubmissionArgsT) {
        self.cull_and_render.unwrap()(args)
    }
}

impl crate::Api for GpuSceneSubmissionApi {
    const NAME: ConstCStr = const_cstr!("tm_gpu_scene_submission_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl RenderContextApi {
    pub unsafe fn create(&self, allocator: *mut AllocatorI) -> *mut RenderContextO {
        self.create.unwrap()(allocator)
    }

    pub unsafe fn destroy(&self, context: *mut RenderContextO) {
        self.destroy.unwrap()(context)
    }

    pub unsafe fn append_resource_buffers(
        &self,
        context: *mut RenderContextO,
        phase: RenderContextBufferPhase,
        res_buffers: *mut *mut RendererResourceCommandBufferO,
        num_buffers: u32,
    ) {
        self.append_resource_buffers.unwrap()(context, phase, res_buffers, num_buffers)
    }

    pub unsafe fn append_command_buffers(
        &self,
        context: *mut RenderContextO,
        phase: RenderContextBufferPhase,
        cmd_buffers: *mut *mut RendererCommandBufferO,
        num_buffers: u32,
    ) {
        self.append_command_buffers.unwrap()(context, phase, cmd_buffers, num_buffers)
    }

    pub unsafe fn resource_buffers(
        &self,
        context: *mut RenderContextO,
        phase: RenderContextBufferPhase,
        res_buffers: *mut *mut RendererResourceCommandBufferO,
    ) -> u32 {
        self.resource_buffers.unwrap()(context, phase, res_buffers)
    }

    pub unsafe fn command_buffers(
        &self,
        context: *mut RenderContextO,
        phase: RenderContextBufferPhase,
        cmd_buffers: *mut *mut RendererCommandBufferO,
    ) -> u32 {
        self.command_buffers.unwrap()(context, phase, cmd_buffers)
    }
}

impl crate::Api for RenderContextApi {
    const NAME: ConstCStr = const_cstr!("tm_render_context_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl SceneCommonApi {
    pub unsafe fn init_camera(&self, camera: *mut TransformT, translation: Vec3T) {
        self.init_camera.unwrap()(camera, translation)
    }

    pub unsafe fn camera_frame_bounds(
        &self,
        camera: *mut TransformT,
        camera_y_fov: f32,
        bounds: *const Vec3T,
        translation_speed: *mut f32,
        focus_point: *mut Vec3T,
    ) {
        self.camera_frame_bounds.unwrap()(
            camera,
            camera_y_fov,
            bounds,
            translation_speed,
            focus_point,
        )
    }

    pub unsafe fn find_component_render_interfaces(
        &self,
        entity_ctx: *mut EntityContextO,
        transform_component: ComponentTypeT,
        tt: *const TheTruthO,
        allocator: *mut AllocatorI,
        selection: *const TtIdT,
        selection_n: u64,
        ignore: *const EntityT,
        ignore_n: u64,
        include_entities_without_render_components: bool,
        res: *mut CiRenderGatherCallbackArgsT,
    ) {
        self.find_component_render_interfaces.unwrap()(
            entity_ctx,
            transform_component,
            tt,
            allocator,
            selection,
            selection_n,
            ignore,
            ignore_n,
            include_entities_without_render_components,
            res,
        )
    }

    pub unsafe fn bound_assets(
        &self,
        entity_ctx: *mut EntityContextO,
        transform_component: ComponentTypeT,
        tt: *const TheTruthO,
        ignore: *const EntityT,
        ignore_n: u64,
        bounds: *mut Vec3T,
        include_origo: bool,
    ) {
        self.bound_assets.unwrap()(
            entity_ctx,
            transform_component,
            tt,
            ignore,
            ignore_n,
            bounds,
            include_origo,
        )
    }

    pub unsafe fn bound_selected_assets(
        &self,
        entity_ctx: *mut EntityContextO,
        transform_component: ComponentTypeT,
        tt: *const TheTruthO,
        selection: *const TtIdT,
        selection_n: u64,
        ignore: *const EntityT,
        ignore_n: u64,
        bounds: *mut Vec3T,
        include_origo: bool,
    ) {
        self.bound_selected_assets.unwrap()(
            entity_ctx,
            transform_component,
            tt,
            selection,
            selection_n,
            ignore,
            ignore_n,
            bounds,
            include_origo,
        )
    }

    pub unsafe fn bound_entity_asset(
        &self,
        tt: *const TheTruthO,
        entity: TtIdT,
        bounds: *mut Vec3T,
    ) {
        self.bound_entity_asset.unwrap()(tt, entity, bounds)
    }

    pub unsafe fn find_shader_data_engine_update(
        &self,
        inst: *mut EngineO,
        data: *mut EngineUpdateSetT,
        commands: *mut EntityCommandsO,
    ) {
        self.find_shader_data_engine_update.unwrap()(inst, data, commands)
    }

    pub unsafe fn gather_shader_data_filter(
        &self,
        inst: *mut EngineO,
        components: *const ComponentTypeT,
        num_components: u32,
        mask: *const ComponentMaskT,
    ) -> bool {
        self.gather_shader_data_filter.unwrap()(inst, components, num_components, mask)
    }

    pub unsafe fn find_renderables_engine_update(
        &self,
        inst: *mut EngineO,
        data: *mut EngineUpdateSetT,
        commands: *mut EntityCommandsO,
    ) {
        self.find_renderables_engine_update.unwrap()(inst, data, commands)
    }

    pub unsafe fn gather_renderables_filter(
        &self,
        inst: *mut EngineO,
        components: *const ComponentTypeT,
        num_components: u32,
        mask: *const ComponentMaskT,
    ) -> bool {
        self.gather_renderables_filter.unwrap()(inst, components, num_components, mask)
    }

    pub unsafe fn add_default_light_source(&self, entity_ctx: *mut EntityContextO) -> EntityT {
        self.add_default_light_source.unwrap()(entity_ctx)
    }

    pub unsafe fn has_any_light_source(&self, ctx: *mut EntityContextO) -> bool {
        self.has_any_light_source.unwrap()(ctx)
    }

    pub unsafe fn component_visualization_menu(
        &self,
        tt: *mut TheTruthO,
        entity_ctx: *mut EntityContextO,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        tab: *mut TabI,
        pos: Vec2T,
    ) {
        self.component_visualization_menu.unwrap()(tt, entity_ctx, ui, uistyle, tab, pos)
    }

    pub unsafe fn viewport_visualization_toolbar(
        &self,
        tt: *mut TheTruthO,
        entity_ctx: *mut EntityContextO,
        le_settings: *mut LightingEnvironmentSettingsT,
        statistics_overlays: *mut StatisticsOverlaysT,
        render_pipeline: *mut RenderPipelineI,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        tab: *mut TabI,
        toolbar_r: RectT,
        toolbar_draw_mode: u32,
    ) -> RectT {
        self.viewport_visualization_toolbar.unwrap()(
            tt,
            entity_ctx,
            le_settings,
            statistics_overlays,
            render_pipeline,
            ui,
            uistyle,
            tab,
            toolbar_r,
            toolbar_draw_mode,
        )
    }

    pub unsafe fn statistics_menu(
        &self,
        statistics_overlays: *mut StatisticsOverlaysT,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        pos: Vec2T,
    ) {
        self.statistics_menu.unwrap()(statistics_overlays, ui, uistyle, pos)
    }

    pub unsafe fn statistics_overlay_toolbars(
        &self,
        statistics_overlays: *mut StatisticsOverlaysT,
        ta: *mut TempAllocatorI,
    ) -> *mut ToolbarI {
        self.statistics_overlay_toolbars.unwrap()(statistics_overlays, ta)
    }

    pub unsafe fn place_entity(
        &self,
        tt: *mut TheTruthO,
        entity: TtIdT,
        local_transform: *const TransformT,
        parent: TtIdT,
        undo_scope: TtUndoScopeT,
    ) {
        self.place_entity.unwrap()(tt, entity, local_transform, parent, undo_scope)
    }

    pub unsafe fn select_entity(
        &self,
        tt: *mut TheTruthO,
        tab: *mut TabI,
        entity: TtIdT,
        undo_scope: TtUndoScopeT,
    ) {
        self.select_entity.unwrap()(tt, tab, entity, undo_scope)
    }

    pub unsafe fn select_component(
        &self,
        tt: *mut TheTruthO,
        tab: *mut TabI,
        component: TtIdT,
        undo_scope: TtUndoScopeT,
    ) {
        self.select_component.unwrap()(tt, tab, component, undo_scope)
    }
}

impl crate::Api for SceneCommonApi {
    const NAME: ConstCStr = const_cstr!("tm_scene_common_api");
    const VERSION: VersionT = VersionT {
        major: 2u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl TheTruthReplacerApi {
    pub unsafe fn replace_using_path(
        &self,
        tt: *mut TheTruthO,
        dest: TtIdT,
        source: TtIdT,
        undo_scope: TtUndoScopeT,
    ) {
        self.replace_using_path.unwrap()(tt, dest, source, undo_scope)
    }
}

impl crate::Api for TheTruthReplacerApi {
    const NAME: ConstCStr = const_cstr!("tm_the_truth_replacer_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl ViewerApi {
    pub unsafe fn request_render(
        &self,
        viewer: *mut ViewerO,
        args: *const ViewerRenderArgsT,
        info: *mut ViewerRenderInfoT,
        res_buf: *mut RendererResourceCommandBufferO,
        cmd_buf: *mut RendererCommandBufferO,
    ) -> RendererHandleT {
        self.request_render.unwrap()(viewer, args, info, res_buf, cmd_buf)
    }

    pub unsafe fn get_color_space(&self, viewer: *const ViewerO) -> *const ColorSpaceDescT {
        self.get_color_space.unwrap()(viewer)
    }

    pub unsafe fn pipeline(&self, viewer: *mut ViewerO) -> *mut RenderPipelineI {
        self.pipeline.unwrap()(viewer)
    }

    pub unsafe fn reset_render_pipeline(&self, viewer: *mut ViewerO) {
        self.reset_render_pipeline.unwrap()(viewer)
    }

    pub unsafe fn set_render_pipeline_api(
        &self,
        viewer: *mut ViewerO,
        pipeline_api: *mut RenderPipelineVt,
    ) {
        self.set_render_pipeline_api.unwrap()(viewer, pipeline_api)
    }

    pub unsafe fn screenshot(&self, viewer: *mut ViewerO) {
        self.screenshot.unwrap()(viewer)
    }

    pub unsafe fn init_vr(&self, viewer: *mut ViewerO, activate: bool) {
        self.init_vr.unwrap()(viewer, activate)
    }
}

impl crate::Api for ViewerApi {
    const NAME: ConstCStr = const_cstr!("tm_viewer_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl ViewerManagerApi {
    pub unsafe fn create_manager(
        &self,
        allocator: *mut AllocatorI,
        render_backend: *mut RendererBackendI,
        shader_repository: *mut ShaderRepositoryO,
        main_device_affinity: u32,
        default_visibility_context: *mut VisibilityContextO,
        viewport_visibility_flag: u64,
    ) -> *mut ViewerManagerO {
        self.create_manager.unwrap()(
            allocator,
            render_backend,
            shader_repository,
            main_device_affinity,
            default_visibility_context,
            viewport_visibility_flag,
        )
    }

    pub unsafe fn destroy_manager(
        &self,
        manager: *mut ViewerManagerO,
        res_buf: *mut RendererResourceCommandBufferO,
    ) {
        self.destroy_manager.unwrap()(manager, res_buf)
    }

    pub unsafe fn create(
        &self,
        manager: *mut ViewerManagerO,
        main_module_name: *const ::std::os::raw::c_char,
    ) -> *mut ViewerO {
        self.create.unwrap()(manager, main_module_name)
    }

    pub unsafe fn destroy(
        &self,
        manager: *mut ViewerManagerO,
        viewer: *mut ViewerO,
        res_buf: *mut RendererResourceCommandBufferO,
    ) {
        self.destroy.unwrap()(manager, viewer, res_buf)
    }

    pub unsafe fn viewers(
        &self,
        manager: *mut ViewerManagerO,
        active: *mut *mut bool,
    ) -> *mut *mut ViewerO {
        self.viewers.unwrap()(manager, active)
    }

    pub unsafe fn render(
        &self,
        manager: *mut ViewerManagerO,
        shader_context: *const ShaderSystemContextO,
        tt: *const TheTruthO,
        frame_params: *const RenderPipelineUpdateFrameParametersT,
    ) {
        self.render.unwrap()(manager, shader_context, tt, frame_params)
    }
}

impl crate::Api for ViewerManagerApi {
    const NAME: ConstCStr = const_cstr!("tm_viewer_manager_api");
    const VERSION: VersionT = VersionT {
        major: 2u32,
        minor: 0u32,
        patch: 0u32,
    };
}

pub const TM_TT_ASPECT__ASSET_PREVIEW: StrhashT = StrhashT {
    u64_: 14212721863639798132u64,
};
pub const TM_TT_ASPECT__ASSET_SCENE: StrhashT = StrhashT {
    u64_: 14329318064558651605u64,
};
pub const TM_TT_ASPECT__ASSET_OPEN: StrhashT = StrhashT {
    u64_: 5594051701220254319u64,
};
pub const TYPE_HASH__CAMERA_CONTROLLER_COMPONENT: StrhashT = StrhashT {
    u64_: 9760961870676976776u64,
};
pub const TM_ENGINE__FREEFLIGHT_CAMERA_CONTROLLER: StrhashT = StrhashT {
    u64_: 16405980432310866292u64,
};
pub const TM_ENGINE__FREEFLIGHT_CAMERA_TRANSFORM: StrhashT = StrhashT {
    u64_: 3643124764479369251u64,
};
pub const TM_CI_EDITOR_UI: StrhashT = StrhashT {
    u64_: 15967003850867459386u64,
};
pub const TM_EDITOR_TOOL_ID__SELECT: StrhashT = StrhashT {
    u64_: 15419100652914668230u64,
};
pub const TM_EDITOR_TOOL_ID__MOVE: StrhashT = StrhashT {
    u64_: 10765360271784010468u64,
};
pub const TM_EDITOR_TOOL_ID__ROTATE: StrhashT = StrhashT {
    u64_: 4957850385211195158u64,
};
pub const TM_EDITOR_TOOL_ID__SCALE: StrhashT = StrhashT {
    u64_: 10577229183153927243u64,
};
pub const TM_EDITOR_TOOL_ID__ORIENTATION: StrhashT = StrhashT {
    u64_: 6220572659606744657u64,
};
pub const TM_CI_RENDER: StrhashT = StrhashT {
    u64_: 6430888070237176841u64,
};
pub const TM_CI_SHADER: StrhashT = StrhashT {
    u64_: 14389400674037110261u64,
};
pub const TM_GPU_CULLING__TRANSFORMS_INPUT: StrhashT = StrhashT {
    u64_: 2665470916605338210u64,
};
pub const TM_GPU_CULLING__OUTPUT: StrhashT = StrhashT {
    u64_: 13816956930322693720u64,
};
pub const TM_GPU_CULLING__DRAW_CMDS_OUTPUT: StrhashT = StrhashT {
    u64_: 12832042406061263999u64,
};
pub const TM_GPU_CULLING__BOUNDING_RADIUS: StrhashT = StrhashT {
    u64_: 13926938770589846664u64,
};
pub const TM_GPU_CULLING__TRANSFORMS_COUNT: StrhashT = StrhashT {
    u64_: 7762110212670973143u64,
};
pub const TM_GPU_CULLING__TRANSFORMS_START: StrhashT = StrhashT {
    u64_: 17570208879660267539u64,
};
pub const TM_GPU_CULLING__TRANSFORMS_STRIDE: StrhashT = StrhashT {
    u64_: 194691516363235041u64,
};
pub const TM_GPU_CULLING__CULLING_DISTANCE: StrhashT = StrhashT {
    u64_: 5958167400858114767u64,
};
pub const TM_GPU_CULLING__DRAW_CMDS_COUNT: StrhashT = StrhashT {
    u64_: 17165353573604852259u64,
};
pub const TM_GPU_CULLING__PARENT_TRANSFORM: StrhashT = StrhashT {
    u64_: 10460656992834597080u64,
};
pub const TM_GPU_SCENE_SUBMISSION__TRANSFORMS_INPUT: StrhashT = StrhashT {
    u64_: 2735135666531474372u64,
};
pub const TM_GPU_SCENE_SUBMISSION__OUTPUT: StrhashT = StrhashT {
    u64_: 18345216586284322428u64,
};
pub const TM_GPU_SCENE_SUBMISSION__DRAW_CMDS_OUTPUT: StrhashT = StrhashT {
    u64_: 3450145739906582207u64,
};
pub const TM_GPU_SCENE_SUBMISSION__DISPATCH_INDIRECT_OUTPUT: StrhashT = StrhashT {
    u64_: 8013803713849225952u64,
};
pub const TM_GPU_SCENE_SUBMISSION__BOUNDING_RADIUS: StrhashT = StrhashT {
    u64_: 1112308192711211455u64,
};
pub const TM_GPU_SCENE_SUBMISSION__TRANSFORMS_COUNT: StrhashT = StrhashT {
    u64_: 16164211266129383767u64,
};
pub const TM_GPU_SCENE_SUBMISSION__TRANSFORMS_START: StrhashT = StrhashT {
    u64_: 5762445022806614575u64,
};
pub const TM_GPU_SCENE_SUBMISSION__TRANSFORMS_STRIDE: StrhashT = StrhashT {
    u64_: 12307040261741271801u64,
};
pub const TM_GPU_SCENE_SUBMISSION__PARENT_TRANSFORM: StrhashT = StrhashT {
    u64_: 12159040891817738833u64,
};
pub const TM_GPU_SCENE_SUBMISSION__CULLING_DISTANCE: StrhashT = StrhashT {
    u64_: 5100655888310955492u64,
};
pub const TM_GPU_SCENE_SUBMISSION__DRAW_CMDS_COUNT: StrhashT = StrhashT {
    u64_: 5210628088815794490u64,
};
pub const TM_GPU_SCENE_SUBMISSION__VIEWERS_COUNT: StrhashT = StrhashT {
    u64_: 8158439013611647872u64,
};
pub const TM_GPU_SCENE_SUBMISSION__INDIRECT_DRAW_CMDS_START: StrhashT = StrhashT {
    u64_: 7929537108828966574u64,
};
pub const TM_GPU_SCENE_SUBMISSION__INDIRECT_COMPUTE_CMD_OFFSET: StrhashT = StrhashT {
    u64_: 6871324773626176928u64,
};
pub const TM_GPU_SCENE_SUBMISSION__INSTANCE_COUNTERS_START: StrhashT = StrhashT {
    u64_: 8003617856616968400u64,
};
pub const TM_GPU_SCENE_SUBMISSION__INSTANCE_INDIRECTION_START: StrhashT = StrhashT {
    u64_: 8389349747083231397u64,
};
pub const TM_TT_ASPECT__NAME_PROPERTY: StrhashT = StrhashT {
    u64_: 16328471694850579054u64,
};
pub const TM_CAMERA_CONTROLLER_COMPONENT_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_VIEWER_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_BAKER_CONTEXT_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_SCENE_COMMON_API_VERSION: VersionT = VersionT {
    major: 2u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_VIEWER_MANAGER_API_VERSION: VersionT = VersionT {
    major: 2u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_SCENE_TAB_COMMAND_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_RENDER_CONTEXT_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_THE_TRUTH_REPLACER_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_ASSET_PREVIEW_API_VERSION: VersionT = VersionT {
    major: 2u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_THE_TRUTH_STRIPPER_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_FRUSTUM_CULLING_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_GPU_SCENE_SUBMISSION_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
