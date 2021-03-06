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
pub const TM_TT_TYPE__CLIPBOARD: &'static [u8; 13usize] = b"tm_clipboard\0";
pub const TM_TT_TYPE__DOCKING_SETTINGS: &'static [u8; 20usize] = b"tm_docking_settings\0";
pub const TM_TT_TYPE__TABWELL: &'static [u8; 11usize] = b"tm_tabwell\0";
pub const TM_TT_TYPE__TAB_VIEW: &'static [u8; 12usize] = b"tm_tab_view\0";
pub const TM_MODAL_PROGRESS_DONE: u32 = 4294967295;
pub const TM_TT_TYPE__SHORTCUTS_ENTRY: &'static [u8; 19usize] = b"tm_shortcuts_entry\0";
pub const TM_TT_TYPE__SHORTCUTS_INDEX: &'static [u8; 19usize] = b"tm_shortcuts_index\0";
pub const TM_TT_TYPE__TOOLBAR_SETTINGS: &'static [u8; 20usize] = b"tm_toolbar_settings\0";
pub const TM_TT_TYPE__UI_THEME: &'static [u8; 12usize] = b"tm_ui_theme\0";
pub const TM_TT_TYPE__UI_THEME_COLOR: &'static [u8; 18usize] = b"tm_ui_theme_color\0";
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
pub const TM_TT_PROP__CLIPBOARD__OBJECTS: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__CLIPBOARD__IS_CUT: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_1 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct UiClipboardApi {
    pub cut: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            objects: *const TtIdT,
            count: u32,
            undo_stack: *mut UndoStackI,
        ),
    >,
    pub copy: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            objects: *const TtIdT,
            count: u32,
            undo_stack: *mut UndoStackI,
        ),
    >,
    pub empty:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO, undo_scope: TtUndoScopeT)>,
    pub cut_objects: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, ta: *mut TempAllocatorI) -> *const TtIdT,
    >,
    pub copied_objects: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, ta: *mut TempAllocatorI) -> *const TtIdT,
    >,
    pub truth: ::std::option::Option<unsafe extern "C" fn() -> *const TheTruthO>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DockingTabwellO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UiRendererO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsDroppedFileT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ToolbarsStateO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ViewerRenderArgsT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TabO {
    _unused: [u8; 0],
}
pub const TM_TAB_FOCUS_EVENT__FOCUS: TabFocusEvent = 0;
pub const TM_TAB_FOCUS_EVENT__OPEN: TabFocusEvent = 1;
pub type TabFocusEvent = ::std::os::raw::c_int;
#[repr(C)]
pub struct TabVtRootT {
    pub tt: *mut TheTruthO,
    pub root: TtIdT,
    pub internal_root: TtIdT,
    pub counter: u64,
}
impl Default for TabVtRootT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct DockingFeedEventsArgsT {
    pub window_ui: *mut UiO,
    pub window_rect: RectT,
    pub window_dpi_scale_factor: f32,
    pub window_has_focus: bool,
    pub window_under_cursor: bool,
    pub _padding_94: [::std::os::raw::c_char; 2usize],
}
impl Default for DockingFeedEventsArgsT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_TT_PROP__DOCKING_SETTINGS__TOOLBARS_SETTINGS: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_2 = ::std::os::raw::c_int;
#[repr(C)]
pub struct TabVt {
    pub name: *const ::std::os::raw::c_char,
    pub name_hash: StrhashT,
    pub create_menu_name:
        ::std::option::Option<unsafe extern "C" fn() -> *const ::std::os::raw::c_char>,
    pub create_menu_category:
        ::std::option::Option<unsafe extern "C" fn() -> *const ::std::os::raw::c_char>,
    pub create: ::std::option::Option<
        unsafe extern "C" fn(context: *mut TabCreateContextT, ui: *mut UiO) -> *mut TabI,
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(inst: *mut TabO)>,
    pub ui: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut TabO, ui: *mut UiO, uistyle: *const UiStyleT, rect: RectT),
    >,
    pub ui_serial: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut TabO, ui: *mut UiO, uistyle: *const UiStyleT, rect: RectT),
    >,
    pub hidden_update: ::std::option::Option<unsafe extern "C" fn(inst: *mut TabO)>,
    pub title: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut TabO, ui: *mut UiO) -> *const ::std::os::raw::c_char,
    >,
    pub has_custom_tabbar_border_color: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut TabO, color: *mut ColorSrgbT) -> bool,
    >,
    pub set_root: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut TabO, tt: *mut TheTruthO, root: TtIdT),
    >,
    pub root: ::std::option::Option<unsafe extern "C" fn(inst: *mut TabO) -> TabVtRootT>,
    pub restore_settings: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut TabO, tt: *mut TheTruthO, settings_id: TtIdT),
    >,
    pub save_settings:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut TabO, tt: *mut TheTruthO) -> TtIdT>,
    pub save_settings_from_object: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, settings: *const ::std::os::raw::c_void) -> TtIdT,
    >,
    pub can_close: ::std::option::Option<unsafe extern "C" fn(inst: *mut TabO) -> bool>,
    pub focus_event: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut TabO,
            from: *mut TabI,
            event: TabFocusEvent,
            tt: *mut TheTruthO,
            object: TtIdT,
            selection: *const TtIdT,
            selection_n: u32,
        ),
    >,
    pub feed_events: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut TabO,
            events: *const InputEventT,
            count: u32,
            args: *mut DockingFeedEventsArgsT,
        ),
    >,
    pub process_dropped_os_files: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut TabO,
            ui: *mut UiO,
            files: *mut *mut ::std::os::raw::c_char,
            num_files: u32,
        ),
    >,
    pub toolbars: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut TabO, ta: *mut TempAllocatorI) -> *mut ToolbarI,
    >,
    pub custom_padding: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut TabO,
            ui: *mut UiO,
            content_r: *mut RectT,
            safety_padding: f32,
        ),
    >,
    pub need_update: ::std::option::Option<unsafe extern "C" fn(inst: *mut TabO) -> bool>,
    pub hot_reload: ::std::option::Option<unsafe extern "C" fn(inst: *mut TabO)>,
    pub entity_context:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut TabO) -> *mut EntityContextO>,
    pub viewer_render_args:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut TabO, args: *mut ViewerRenderArgsT)>,
    pub cant_be_pinned: bool,
    pub run_as_job: bool,
    pub dont_restore_at_startup: bool,
    pub always_restore_settings: bool,
    pub _padding_272: [::std::os::raw::c_char; 4usize],
    pub menu_title: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut TabO, ui: *mut UiO) -> *const ::std::os::raw::c_char,
    >,
    pub menu: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut TabO,
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            submenu_pos: Vec2T,
        ),
    >,
}
impl Default for TabVt {
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
pub struct TabI {
    pub vt: *mut TabVt,
    pub inst: *mut TabO,
    pub root_id: u64,
}
impl Default for TabI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_DOCKING_TABWELL_SPLIT_NONE: DockingTabwellSplit = 0;
pub const TM_DOCKING_TABWELL_SPLIT_RIGHT: DockingTabwellSplit = 1;
pub const TM_DOCKING_TABWELL_SPLIT_LEFT: DockingTabwellSplit = 2;
pub const TM_DOCKING_TABWELL_SPLIT_TOP: DockingTabwellSplit = 3;
pub const TM_DOCKING_TABWELL_SPLIT_BOTTOM: DockingTabwellSplit = 4;
pub type DockingTabwellSplit = ::std::os::raw::c_int;
#[repr(C)]
pub struct DockingTabInfoT {
    pub tab: *mut TabI,
    pub ui: *mut UiO,
    pub visible: bool,
    pub _padding_322: [::std::os::raw::c_char; 7usize],
    pub last_focused: ClockO,
    pub pin_type: u32,
    pub _padding_329: [::std::os::raw::c_char; 4usize],
}
impl Default for DockingTabInfoT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct DockingTabwellInfoT {
    pub left: *mut DockingTabwellO,
    pub right: *mut DockingTabwellO,
    pub top: *mut DockingTabwellO,
    pub bottom: *mut DockingTabwellO,
    pub bias: f32,
    pub _padding_341: [::std::os::raw::c_char; 4usize],
    pub tabs: *mut *mut TabI,
    pub last_rect: RectT,
}
impl Default for DockingTabwellInfoT {
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
pub struct DockingFindTabT {
    pub tab: *mut TabI,
    pub ui: *mut UiO,
}
impl Default for DockingFindTabT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct DockingFindTabOptT {
    pub from_tab: *mut TabI,
    pub to_tab: *mut TabI,
    pub in_ui: *mut UiO,
    pub find_asset_tt: *mut TheTruthO,
    pub find_asset: TtIdT,
    pub exclude_pinned: bool,
    pub _padding_379: [::std::os::raw::c_char; 7usize],
}
impl Default for DockingFindTabOptT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_DOCKING_WORKSPACES_BAR_MODE__FULL: DockingWorkspacesBarMode = 0;
pub const TM_DOCKING_WORKSPACES_BAR_MODE__COMPACT: DockingWorkspacesBarMode = 1;
pub const TM_DOCKING_WORKSPACES_BAR_MODE__HIDDEN: DockingWorkspacesBarMode = 2;
pub type DockingWorkspacesBarMode = ::std::os::raw::c_int;
pub type DockingWorkspaceContextMenuF = ::std::option::Option<
    unsafe extern "C" fn(
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const UiMenuT,
        workspace_idx: u32,
        result: *mut UiMenuResultT,
    ),
>;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct DockingApi {
    pub create_truth_types: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
    pub add_ui: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, r: RectT)>,
    pub remove_ui: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO)>,
    pub add_workspace: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, name: *const ::std::os::raw::c_char),
    >,
    pub close_focused_workspace: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> bool>,
    pub num_workspaces: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> u32>,
    pub current_workspace: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> u32>,
    pub workspace_name: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, idx: u32) -> *const ::std::os::raw::c_char,
    >,
    pub set_workspace_name: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, idx: u32, name: *const ::std::os::raw::c_char),
    >,
    pub workspace_icon: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, idx: u32) -> u32>,
    pub set_workspace_icon:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, idx: u32, icon: u32)>,
    pub workspace_root:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, idx: u32) -> *mut DockingTabwellO>,
    pub hot_reload: ::std::option::Option<unsafe extern "C" fn()>,
    pub root: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> *mut DockingTabwellO>,
    pub split_tabwell: ::std::option::Option<
        unsafe extern "C" fn(
            parent: *mut DockingTabwellO,
            split: DockingTabwellSplit,
            bias: f32,
            sibling: *mut *mut DockingTabwellO,
        ) -> *mut DockingTabwellO,
    >,
    pub add_tab: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, tabwell: *mut DockingTabwellO, tab: *mut TabI),
    >,
    pub remove_tab: ::std::option::Option<unsafe extern "C" fn(tab: *mut TabI)>,
    pub move_tab: ::std::option::Option<
        unsafe extern "C" fn(tab: *mut TabI, ui: *mut UiO, tabwell: *mut DockingTabwellO),
    >,
    pub clear: ::std::option::Option<unsafe extern "C" fn()>,
    pub feed_events: ::std::option::Option<
        unsafe extern "C" fn(
            events: *const InputEventT,
            count: u32,
            args: *mut DockingFeedEventsArgsT,
        ),
    >,
    pub ui: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            ui_renderer: *mut UiRendererO,
            rect: RectT,
            ui_has_focus: bool,
            dropped_files: *const OsDroppedFileT,
            num_dropped_files: u32,
            context: *mut TabCreateContextT,
            settings_tt: *mut TheTruthO,
            settings_obj: TtIdT,
            default_padding: f32,
            workspaces_mode: DockingWorkspacesBarMode,
        ),
    >,
    pub is_dragging_tab: ::std::option::Option<unsafe extern "C" fn(pos: *mut Vec2T) -> *mut TabI>,
    pub should_create_new_ui_for_tab: ::std::option::Option<
        unsafe extern "C" fn(create_context: *mut TabCreateContextT, rect: *mut RectT) -> *mut TabI,
    >,
    pub focused_tabwell:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> *mut DockingTabwellO>,
    pub focused_tab: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> *mut TabI>,
    pub tab_has_focus: ::std::option::Option<unsafe extern "C" fn(tab: *mut TabI) -> bool>,
    pub tabwell_info: ::std::option::Option<
        unsafe extern "C" fn(
            tw: *mut DockingTabwellO,
            ta: *mut TempAllocatorI,
        ) -> DockingTabwellInfoT,
    >,
    pub tab_info: ::std::option::Option<
        unsafe extern "C" fn(
            tabs: *mut DockingTabInfoT,
            n: u32,
            filter_ui: *mut UiO,
            filter_visible: bool,
        ) -> u32,
    >,
    pub root_history:
        ::std::option::Option<unsafe extern "C" fn(tab: *mut TabI, n: *mut u32) -> *mut TabVtRootT>,
    pub clear_root_history_with_greater_counter:
        ::std::option::Option<unsafe extern "C" fn(tab: *mut TabI, counter: u64)>,
    pub tab_ui: ::std::option::Option<unsafe extern "C" fn(tab: *mut TabI) -> *mut UiO>,
    pub can_remove_ui: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> bool>,
    pub set_focus_tab: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, tab: *mut TabI)>,
    pub close_focused_tab: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> bool>,
    pub close_all_tabs_and_workspaces: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO)>,
    pub close_all_tabs_in_workspace: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO)>,
    pub send_focus_event: ::std::option::Option<
        unsafe extern "C" fn(
            from: *mut TabI,
            event: TabFocusEvent,
            tt: *mut TheTruthO,
            object: TtIdT,
            selection: *const TtIdT,
            selection_n: u32,
        ),
    >,
    pub destroy_truth: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
    pub find_tab: ::std::option::Option<
        unsafe extern "C" fn(
            type_name_hash: StrhashT,
            opt: *const DockingFindTabOptT,
        ) -> DockingFindTabT,
    >,
    pub find_tabs: ::std::option::Option<
        unsafe extern "C" fn(
            type_name_hash: StrhashT,
            ta: *mut TempAllocatorI,
        ) -> *mut DockingFindTabT,
    >,
    pub pin_type: ::std::option::Option<unsafe extern "C" fn(tab: *mut TabI) -> u32>,
    pub pin_object: ::std::option::Option<
        unsafe extern "C" fn(tab: *mut TabI, tt: *mut TheTruthO, root: TtIdT),
    >,
    pub toolbars_state:
        ::std::option::Option<unsafe extern "C" fn(tab: *mut TabI) -> *mut ToolbarsStateO>,
    pub clear_cached_ui: ::std::option::Option<unsafe extern "C" fn(tab: *mut TabI)>,
    pub tab_vt: ::std::option::Option<unsafe extern "C" fn(name_hash: StrhashT) -> *mut TabVt>,
    pub set_workspace_context_menu:
        ::std::option::Option<unsafe extern "C" fn(menu: DockingWorkspaceContextMenuF)>,
    pub set_current_workspace:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, workspace_idx: u32)>,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct UiDragApi {
    pub start_dragging:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO, object: TtIdT)>,
    pub start_dragging_multiple_objects: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, objects: *const TtIdT, object_count: u64),
    >,
    pub single_dragged_object:
        ::std::option::Option<unsafe extern "C" fn(tt: *const TheTruthO) -> TtIdT>,
    pub first_dragged_object:
        ::std::option::Option<unsafe extern "C" fn(tt: *const TheTruthO) -> TtIdT>,
    pub all_dragged_objects: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, ta: *mut TempAllocatorI) -> *mut TtIdT,
    >,
    pub dragged_truth: ::std::option::Option<unsafe extern "C" fn() -> *const TheTruthO>,
    pub stop_dragging: ::std::option::Option<unsafe extern "C" fn()>,
}
#[repr(C)]
pub struct FontGlyphT {
    pub texture: u32,
    pub uv: RectT,
    pub offset: Vec2T,
    pub xadvance: f32,
    pub glyph_segments_offset: u32,
}
impl Default for FontGlyphT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct FontGlyphLineSegmentT {
    pub p0: Vec2T,
    pub p1: Vec2T,
}
impl Default for FontGlyphLineSegmentT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct FontGlyphBezierSegmentT {
    pub p0: Vec2T,
    pub p1: Vec2T,
    pub c: Vec2T,
}
impl Default for FontGlyphBezierSegmentT {
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
pub struct FontRangeT {
    pub start: u32,
    pub n: u32,
}
pub const TM_FONT_MAX_BAKED_SCALES: ::std::os::raw::c_int = 8;
pub type _bindgen_ty_3 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FontT {
    pub num_glyphs: u32,
    pub _padding_86: [::std::os::raw::c_char; 4usize],
    pub glyphs: *mut FontGlyphT,
    pub glyphs_segments_size: u32,
    pub _padding_93: [::std::os::raw::c_char; 4usize],
    pub glyphs_segments: *mut u8,
    pub num_ranges: u32,
    pub em: f32,
    pub ranges: *mut FontRangeT,
    pub num_font_scales: u32,
    pub font_scales: [f32; 8usize],
    pub ascent: [f32; 8usize],
    pub descent: [f32; 8usize],
    pub line_gap: [f32; 8usize],
    pub ex: [f32; 8usize],
    pub cap_height: [f32; 8usize],
    pub _padding_134: [::std::os::raw::c_char; 4usize],
}
impl Default for FontT {
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
pub struct Draw2dFontT {
    pub vbuffer_offset: u32,
    pub _padding_142: [::std::os::raw::c_char; 4usize],
    pub info: *const FontT,
}
impl Default for Draw2dFontT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_DRAW2D__FEATHER__DEFAULT: ::std::os::raw::c_int = 0;
pub const TM_DRAW2D__FEATHER__NONE: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_4 = ::std::os::raw::c_int;
#[repr(C)]
pub struct Draw2dStyleT {
    pub color: ColorSrgbT,
    pub line_width: f32,
    pub feather_width: f32,
    pub feather: u32,
    pub clip: u32,
    pub _padding_174: [::std::os::raw::c_char; 4usize],
    pub font: *const Draw2dFontT,
    pub font_scale: f32,
    pub include_alpha: bool,
    pub _padding_184: [::std::os::raw::c_char; 3usize],
}
impl Default for Draw2dStyleT {
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
pub struct Draw2dVbufferT {
    pub vbuffer: *mut u8,
    pub vbytes: u32,
    pub vbytes_allocated: u32,
}
impl Default for Draw2dVbufferT {
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
pub struct Draw2dIbufferT {
    pub ibuffer: *mut u32,
    pub in_: u32,
    pub in_allocated: u32,
}
impl Default for Draw2dIbufferT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_DRAW2D_AUX_DATA_TYPE_GRID: ::std::os::raw::c_int = 3;
pub type _bindgen_ty_5 = ::std::os::raw::c_int;
#[repr(C)]
pub struct Draw2dAuxDataGridT {
    pub offset: Vec2T,
    pub cell_size: f32,
    pub thin_lines_color: ColorSrgbT,
    pub thick_lines_color: ColorSrgbT,
}
impl Default for Draw2dAuxDataGridT {
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
pub struct Draw2dApi {
    pub add_clip_rect: ::std::option::Option<
        unsafe extern "C" fn(vbuffer: *mut Draw2dVbufferT, clip: RectT) -> u32,
    >,
    pub add_sub_clip_rect: ::std::option::Option<
        unsafe extern "C" fn(vbuffer: *mut Draw2dVbufferT, parent: u32, clip: RectT) -> u32,
    >,
    pub clip_rect: ::std::option::Option<
        unsafe extern "C" fn(vbuffer: *mut Draw2dVbufferT, clip: u32) -> RectT,
    >,
    pub font_memory: ::std::option::Option<unsafe extern "C" fn(font: *const FontT) -> u32>,
    pub add_font: ::std::option::Option<
        unsafe extern "C" fn(vbuffer: *mut Draw2dVbufferT, font: *const FontT) -> Draw2dFontT,
    >,
    pub fill_rect: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            r: RectT,
        ),
    >,
    pub fill_rect_feathered: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            r: RectT,
        ),
    >,
    pub stroke_rect: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            r: RectT,
        ),
    >,
    pub textured_rect: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            r: RectT,
            texture: u32,
            uv: RectT,
            native_color_space: bool,
        ),
    >,
    pub aux_rect: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            r: RectT,
            aux_data_type: u32,
            aux_data: *const ::std::os::raw::c_void,
            aux_data_size: u32,
        ),
    >,
    pub gradient_rect: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            r: RectT,
            colors: *mut ColorSrgbT,
        ),
    >,
    pub fill_rounded_rect: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            r: RectT,
            radius: f32,
        ),
    >,
    pub fill_rounded_rect_per_corner: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            r: RectT,
            radiuses: Vec4T,
        ),
    >,
    pub stroke_rounded_rect: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            r: RectT,
            radius: f32,
        ),
    >,
    pub stroke_rounded_rect_per_corner: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            r: RectT,
            radiuses: Vec4T,
        ),
    >,
    pub fill_circle: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            pos: Vec2T,
            radius: f32,
        ),
    >,
    pub stroke_circle: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            pos: Vec2T,
            radius: f32,
        ),
    >,
    pub fill_triangles: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            points: *const Vec2T,
            num_points: u32,
            indices: *const u32,
            num_indices: u32,
        ),
    >,
    pub fill_convex_polyline: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            points: *const Vec2T,
            num_points: u32,
        ),
    >,
    pub stroke_polyline: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            points: *const Vec2T,
            num_points: u32,
            closed: bool,
        ),
    >,
    pub stroke_polyline_widths: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            points: *const Vec2T,
            widths: *const f32,
            num_points: u32,
            closed: bool,
        ),
    >,
    pub bezier_path: ::std::option::Option<
        unsafe extern "C" fn(
            curve: *const Vec2T,
            num_curve_points: u32,
            tolerance: f32,
            ta: *mut TempAllocatorI,
            num_points: *mut u32,
            allocated_points: *mut u32,
        ) -> *mut Vec2T,
    >,
    pub fill_convex_bezier: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            curve: *const Vec2T,
            num_curve_points: u32,
            tolerance: f32,
            ta: *mut TempAllocatorI,
        ),
    >,
    pub stroke_bezier: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            curve: *const Vec2T,
            num_curve_points: u32,
            closed: bool,
            tolerance: f32,
            ta: *mut TempAllocatorI,
        ),
    >,
    pub draw_glyphs: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            pos: Vec2T,
            glyphs: *const u16,
            num_glyphs: u32,
        ) -> RectT,
    >,
    pub draw_glyphs_rotated: ::std::option::Option<
        unsafe extern "C" fn(
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
            style: *const Draw2dStyleT,
            pos: Vec2T,
            glyphs: *const u16,
            num_glyphs: u32,
            x: Vec2T,
            y: Vec2T,
        ) -> f32,
    >,
    pub merge_buffers: ::std::option::Option<
        unsafe extern "C" fn(
            to_vbuffer: *mut Draw2dVbufferT,
            to_ibuffers: *mut *mut Draw2dIbufferT,
            from_vbuffer: *const Draw2dVbufferT,
            from_ibuffers: *const *mut Draw2dIbufferT,
            num_ibuffers: u32,
        ),
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct FontApi {
    pub glyphs: ::std::option::Option<
        unsafe extern "C" fn(
            font: *const FontT,
            glyphs: *mut u16,
            codepoints: *const u32,
            size: u32,
        ),
    >,
    pub metrics: ::std::option::Option<
        unsafe extern "C" fn(
            font: *const FontT,
            font_scale: f32,
            glyphs: *const u16,
            num_glyphs: u32,
        ) -> RectT,
    >,
    pub metrics_array: ::std::option::Option<
        unsafe extern "C" fn(
            font: *const FontT,
            font_scale: f32,
            pos_x: *mut f32,
            rects: *mut RectT,
            glyphs: *const u16,
            num_glyphs: u32,
        ) -> RectT,
    >,
    pub line: ::std::option::Option<
        unsafe extern "C" fn(
            font: *const FontT,
            font_scale: f32,
            line_width: *mut f32,
            glyphs: *const u16,
            num_glyphs: u32,
        ) -> u32,
    >,
    pub rescale_font:
        ::std::option::Option<unsafe extern "C" fn(font: *mut FontT, scale_factor: f32)>,
    pub glyph_set_from_scale: ::std::option::Option<
        unsafe extern "C" fn(font: *const FontT, font_scale: f32, new_font_scale: *mut f32) -> u32,
    >,
    pub first_glyph_of_set: ::std::option::Option<
        unsafe extern "C" fn(font: *const FontT, set: u32) -> *const FontGlyphT,
    >,
}
pub const TM_DRAW2D_PRIMITIVE_TRIANGLE: Draw2dPrimitive = 67108864;
pub const TM_DRAW2D_PRIMITIVE_RECT: Draw2dPrimitive = 134217728;
pub const TM_DRAW2D_PRIMITIVE_RECT_TEXTURED: Draw2dPrimitive = 201326592;
pub const TM_DRAW2D_PRIMITIVE_RECT_AUX_DATA: Draw2dPrimitive = 268435456;
pub const TM_DRAW2D_PRIMITIVE_GLYPH: Draw2dPrimitive = -2147483648;
pub type Draw2dPrimitive = ::std::os::raw::c_int;
pub const TM_DRAW2D_CORNER_TL: Draw2dCorner = 0;
pub const TM_DRAW2D_CORNER_TR: Draw2dCorner = 16777216;
pub const TM_DRAW2D_CORNER_BL: Draw2dCorner = 33554432;
pub const TM_DRAW2D_CORNER_BR: Draw2dCorner = 50331648;
pub type Draw2dCorner = ::std::os::raw::c_int;
#[repr(C)]
pub struct Draw2dTriangleVertexT {
    pub pos: Vec2T,
    pub color: ColorSrgbT,
    pub clip: u32,
}
impl Default for Draw2dTriangleVertexT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct Draw2dRectVertexT {
    pub rect: RectT,
    pub color: ColorSrgbT,
    pub clip: u32,
}
impl Default for Draw2dRectVertexT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct Draw2dRectTexturedVertexT {
    pub rect: RectT,
    pub tint: ColorSrgbT,
    pub clip: u32,
    pub texture: u32,
    pub uv: RectT,
}
impl Default for Draw2dRectTexturedVertexT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct Draw2dGlyphRangeVertexT {
    pub origin: Vec2T,
    pub color: ColorSrgbT,
    pub scale: f32,
    pub clip: u32,
    pub font: u32,
    pub x: Vec2T,
    pub y: Vec2T,
}
impl Default for Draw2dGlyphRangeVertexT {
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
pub struct Draw2dGlyphVertexT {
    pub x_offset: f32,
    pub glyph: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FontLibraryT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct TtfHandleT {
    pub id: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FontDescriptorT {
    pub path: *const ::std::os::raw::c_char,
    pub ttf_data: *const ::std::os::raw::c_char,
    pub ttf_data_bytes: u64,
    pub num_ranges: u32,
    pub font_index: u32,
    pub ranges: *const TtfRangeT,
}
impl Default for FontDescriptorT {
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
pub struct FontLibraryApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(allocator: *mut AllocatorI) -> *mut FontLibraryT,
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(lib: *mut FontLibraryT)>,
    pub add_ttf: ::std::option::Option<
        unsafe extern "C" fn(lib: *mut FontLibraryT, ttf: *const FontDescriptorT) -> TtfHandleT,
    >,
    pub lookup: ::std::option::Option<
        unsafe extern "C" fn(
            lib: *const FontLibraryT,
            ttf: TtfHandleT,
            size: u32,
            display_dpi: f32,
        ) -> *const FontT,
    >,
    pub generate: ::std::option::Option<
        unsafe extern "C" fn(
            lib: *mut FontLibraryT,
            tm_res_buf_api: *mut RendererResourceCommandBufferApi,
            res_buf: *mut RendererResourceCommandBufferO,
            device_affinity: u32,
            font_texture_id: u32,
            ttf: TtfHandleT,
            size: u32,
            display_dpi: f32,
            scales: *const f32,
            n_scales: u32,
            texture_handle: *mut RendererHandleT,
        ) -> *const FontT,
    >,
}
pub const TM_GIZMO__MOVE_RESULT__NO_MOVE: GizmoMoveResult = 0;
pub const TM_GIZMO__MOVE_RESULT__PREVIEW: GizmoMoveResult = 1;
pub const TM_GIZMO__MOVE_RESULT__COMMIT: GizmoMoveResult = 2;
pub const TM_GIZMO__MOVE_RESULT__DUPLICATE: GizmoMoveResult = 3;
pub type GizmoMoveResult = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct GizmoMoveSettingsT {
    pub use_world_axes: bool,
    pub grid_snap: bool,
    pub disable_selection: bool,
    pub _padding_39: [::std::os::raw::c_char; 1usize],
    pub grid_size: f32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct GizmoRotateSettingsT {
    pub use_world_axes: bool,
    pub snap: bool,
    pub disable_selection: bool,
    pub _padding_56: [::std::os::raw::c_char; 1usize],
    pub snap_angle: f32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct GizmoScaleSettingsT {
    pub absolute_handle: bool,
    pub disable_selection: bool,
    pub snap: bool,
    pub _padding_74: [::std::os::raw::c_char; 1usize],
    pub snap_increment: f32,
}
#[repr(C)]
pub struct GizmoGridSettingsT {
    pub visible: bool,
    pub _padding_85: [::std::os::raw::c_char; 3usize],
    pub transform: TransformT,
}
impl Default for GizmoGridSettingsT {
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
pub struct GizmoApi {
    pub move_: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            pbuf: *mut PrimitiveDrawerBufferT,
            vbuf: *mut PrimitiveDrawerBufferT,
            camera: *const CameraT,
            viewport: RectT,
            tm: *const TransformT,
            local: *mut TransformT,
            settings: *const GizmoMoveSettingsT,
            grid_settings: *mut GizmoGridSettingsT,
        ) -> GizmoMoveResult,
    >,
    pub rotate: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            pbuf: *mut PrimitiveDrawerBufferT,
            vbuf: *mut PrimitiveDrawerBufferT,
            camera: *const CameraT,
            viewport: RectT,
            tm: *const TransformT,
            local: *mut TransformT,
            settings: *const GizmoRotateSettingsT,
        ) -> GizmoMoveResult,
    >,
    pub scale: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            pbuf: *mut PrimitiveDrawerBufferT,
            vbuf: *mut PrimitiveDrawerBufferT,
            camera: *const CameraT,
            viewport: RectT,
            tm: *const TransformT,
            local: *mut TransformT,
            settings: *const GizmoScaleSettingsT,
        ) -> GizmoMoveResult,
    >,
}
pub const IONICON__ADD: ::std::os::raw::c_int = 61696;
pub const IONICON__ADD_CIRCLE: ::std::os::raw::c_int = 61697;
pub const IONICON__ADD_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61698;
pub const IONICON__ADD_CIRCLE_SHARP: ::std::os::raw::c_int = 61699;
pub const IONICON__ADD_OUTLINE: ::std::os::raw::c_int = 61700;
pub const IONICON__ADD_SHARP: ::std::os::raw::c_int = 61701;
pub const IONICON__AIRPLANE: ::std::os::raw::c_int = 61702;
pub const IONICON__AIRPLANE_OUTLINE: ::std::os::raw::c_int = 61703;
pub const IONICON__AIRPLANE_SHARP: ::std::os::raw::c_int = 61704;
pub const IONICON__ALARM: ::std::os::raw::c_int = 61705;
pub const IONICON__ALARM_OUTLINE: ::std::os::raw::c_int = 61706;
pub const IONICON__ALARM_SHARP: ::std::os::raw::c_int = 61707;
pub const IONICON__ALBUMS: ::std::os::raw::c_int = 61708;
pub const IONICON__ALBUMS_OUTLINE: ::std::os::raw::c_int = 61709;
pub const IONICON__ALBUMS_SHARP: ::std::os::raw::c_int = 61710;
pub const IONICON__ALERT: ::std::os::raw::c_int = 61711;
pub const IONICON__ALERT_CIRCLE: ::std::os::raw::c_int = 61712;
pub const IONICON__ALERT_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61713;
pub const IONICON__ALERT_CIRCLE_SHARP: ::std::os::raw::c_int = 61714;
pub const IONICON__ALERT_OUTLINE: ::std::os::raw::c_int = 61715;
pub const IONICON__ALERT_SHARP: ::std::os::raw::c_int = 61716;
pub const IONICON__AMERICAN_FOOTBALL: ::std::os::raw::c_int = 61717;
pub const IONICON__AMERICAN_FOOTBALL_OUTLINE: ::std::os::raw::c_int = 61718;
pub const IONICON__AMERICAN_FOOTBALL_SHARP: ::std::os::raw::c_int = 61719;
pub const IONICON__ANALYTICS: ::std::os::raw::c_int = 61720;
pub const IONICON__ANALYTICS_OUTLINE: ::std::os::raw::c_int = 61721;
pub const IONICON__ANALYTICS_SHARP: ::std::os::raw::c_int = 61722;
pub const IONICON__APERTURE: ::std::os::raw::c_int = 61723;
pub const IONICON__APERTURE_OUTLINE: ::std::os::raw::c_int = 61724;
pub const IONICON__APERTURE_SHARP: ::std::os::raw::c_int = 61725;
pub const IONICON__APPS: ::std::os::raw::c_int = 61726;
pub const IONICON__APPS_OUTLINE: ::std::os::raw::c_int = 61727;
pub const IONICON__APPS_SHARP: ::std::os::raw::c_int = 61728;
pub const IONICON__ARCHIVE: ::std::os::raw::c_int = 61729;
pub const IONICON__ARCHIVE_OUTLINE: ::std::os::raw::c_int = 61730;
pub const IONICON__ARCHIVE_SHARP: ::std::os::raw::c_int = 61731;
pub const IONICON__ARROW_BACK: ::std::os::raw::c_int = 61732;
pub const IONICON__ARROW_BACK_CIRCLE: ::std::os::raw::c_int = 61733;
pub const IONICON__ARROW_BACK_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61734;
pub const IONICON__ARROW_BACK_CIRCLE_SHARP: ::std::os::raw::c_int = 61735;
pub const IONICON__ARROW_BACK_OUTLINE: ::std::os::raw::c_int = 61736;
pub const IONICON__ARROW_BACK_SHARP: ::std::os::raw::c_int = 61737;
pub const IONICON__ARROW_DOWN: ::std::os::raw::c_int = 61738;
pub const IONICON__ARROW_DOWN_CIRCLE: ::std::os::raw::c_int = 61739;
pub const IONICON__ARROW_DOWN_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61740;
pub const IONICON__ARROW_DOWN_CIRCLE_SHARP: ::std::os::raw::c_int = 61741;
pub const IONICON__ARROW_DOWN_OUTLINE: ::std::os::raw::c_int = 61742;
pub const IONICON__ARROW_DOWN_SHARP: ::std::os::raw::c_int = 61743;
pub const IONICON__ARROW_FORWARD: ::std::os::raw::c_int = 61744;
pub const IONICON__ARROW_FORWARD_CIRCLE: ::std::os::raw::c_int = 61745;
pub const IONICON__ARROW_FORWARD_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61746;
pub const IONICON__ARROW_FORWARD_CIRCLE_SHARP: ::std::os::raw::c_int = 61747;
pub const IONICON__ARROW_FORWARD_OUTLINE: ::std::os::raw::c_int = 61748;
pub const IONICON__ARROW_FORWARD_SHARP: ::std::os::raw::c_int = 61749;
pub const IONICON__ARROW_REDO: ::std::os::raw::c_int = 61750;
pub const IONICON__ARROW_REDO_CIRCLE: ::std::os::raw::c_int = 61751;
pub const IONICON__ARROW_REDO_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61752;
pub const IONICON__ARROW_REDO_CIRCLE_SHARP: ::std::os::raw::c_int = 61753;
pub const IONICON__ARROW_REDO_OUTLINE: ::std::os::raw::c_int = 61754;
pub const IONICON__ARROW_REDO_SHARP: ::std::os::raw::c_int = 61755;
pub const IONICON__ARROW_UNDO: ::std::os::raw::c_int = 61756;
pub const IONICON__ARROW_UNDO_CIRCLE: ::std::os::raw::c_int = 61757;
pub const IONICON__ARROW_UNDO_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61758;
pub const IONICON__ARROW_UNDO_CIRCLE_SHARP: ::std::os::raw::c_int = 61759;
pub const IONICON__ARROW_UNDO_OUTLINE: ::std::os::raw::c_int = 61760;
pub const IONICON__ARROW_UNDO_SHARP: ::std::os::raw::c_int = 61761;
pub const IONICON__ARROW_UP: ::std::os::raw::c_int = 61762;
pub const IONICON__ARROW_UP_CIRCLE: ::std::os::raw::c_int = 61763;
pub const IONICON__ARROW_UP_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61764;
pub const IONICON__ARROW_UP_CIRCLE_SHARP: ::std::os::raw::c_int = 61765;
pub const IONICON__ARROW_UP_OUTLINE: ::std::os::raw::c_int = 61766;
pub const IONICON__ARROW_UP_SHARP: ::std::os::raw::c_int = 61767;
pub const IONICON__AT: ::std::os::raw::c_int = 61768;
pub const IONICON__AT_CIRCLE: ::std::os::raw::c_int = 61769;
pub const IONICON__AT_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61770;
pub const IONICON__AT_CIRCLE_SHARP: ::std::os::raw::c_int = 61771;
pub const IONICON__AT_OUTLINE: ::std::os::raw::c_int = 61772;
pub const IONICON__AT_SHARP: ::std::os::raw::c_int = 61773;
pub const IONICON__ATTACH: ::std::os::raw::c_int = 61774;
pub const IONICON__ATTACH_OUTLINE: ::std::os::raw::c_int = 61775;
pub const IONICON__ATTACH_SHARP: ::std::os::raw::c_int = 61776;
pub const IONICON__BACKSPACE: ::std::os::raw::c_int = 61777;
pub const IONICON__BACKSPACE_OUTLINE: ::std::os::raw::c_int = 61778;
pub const IONICON__BACKSPACE_SHARP: ::std::os::raw::c_int = 61779;
pub const IONICON__BANDAGE: ::std::os::raw::c_int = 61780;
pub const IONICON__BANDAGE_OUTLINE: ::std::os::raw::c_int = 61781;
pub const IONICON__BANDAGE_SHARP: ::std::os::raw::c_int = 61782;
pub const IONICON__BAR_CHART: ::std::os::raw::c_int = 61783;
pub const IONICON__BAR_CHART_OUTLINE: ::std::os::raw::c_int = 61784;
pub const IONICON__BAR_CHART_SHARP: ::std::os::raw::c_int = 61785;
pub const IONICON__BARBELL: ::std::os::raw::c_int = 61786;
pub const IONICON__BARBELL_OUTLINE: ::std::os::raw::c_int = 61787;
pub const IONICON__BARBELL_SHARP: ::std::os::raw::c_int = 61788;
pub const IONICON__BARCODE: ::std::os::raw::c_int = 61789;
pub const IONICON__BARCODE_OUTLINE: ::std::os::raw::c_int = 61790;
pub const IONICON__BARCODE_SHARP: ::std::os::raw::c_int = 61791;
pub const IONICON__BASEBALL: ::std::os::raw::c_int = 61792;
pub const IONICON__BASEBALL_OUTLINE: ::std::os::raw::c_int = 61793;
pub const IONICON__BASEBALL_SHARP: ::std::os::raw::c_int = 61794;
pub const IONICON__BASKET: ::std::os::raw::c_int = 61795;
pub const IONICON__BASKET_OUTLINE: ::std::os::raw::c_int = 61796;
pub const IONICON__BASKET_SHARP: ::std::os::raw::c_int = 61797;
pub const IONICON__BASKETBALL: ::std::os::raw::c_int = 61798;
pub const IONICON__BASKETBALL_OUTLINE: ::std::os::raw::c_int = 61799;
pub const IONICON__BASKETBALL_SHARP: ::std::os::raw::c_int = 61800;
pub const IONICON__BATTERY_CHARGING: ::std::os::raw::c_int = 61801;
pub const IONICON__BATTERY_CHARGING_OUTLINE: ::std::os::raw::c_int = 61802;
pub const IONICON__BATTERY_CHARGING_SHARP: ::std::os::raw::c_int = 61803;
pub const IONICON__BATTERY_DEAD: ::std::os::raw::c_int = 61804;
pub const IONICON__BATTERY_DEAD_OUTLINE: ::std::os::raw::c_int = 61805;
pub const IONICON__BATTERY_DEAD_SHARP: ::std::os::raw::c_int = 61806;
pub const IONICON__BATTERY_FULL: ::std::os::raw::c_int = 61807;
pub const IONICON__BATTERY_FULL_OUTLINE: ::std::os::raw::c_int = 61808;
pub const IONICON__BATTERY_FULL_SHARP: ::std::os::raw::c_int = 61809;
pub const IONICON__BATTERY_HALF: ::std::os::raw::c_int = 61810;
pub const IONICON__BATTERY_HALF_OUTLINE: ::std::os::raw::c_int = 61811;
pub const IONICON__BATTERY_HALF_SHARP: ::std::os::raw::c_int = 61812;
pub const IONICON__BEAKER: ::std::os::raw::c_int = 61813;
pub const IONICON__BEAKER_OUTLINE: ::std::os::raw::c_int = 61814;
pub const IONICON__BEAKER_SHARP: ::std::os::raw::c_int = 61815;
pub const IONICON__BED: ::std::os::raw::c_int = 61816;
pub const IONICON__BED_OUTLINE: ::std::os::raw::c_int = 61817;
pub const IONICON__BED_SHARP: ::std::os::raw::c_int = 61818;
pub const IONICON__BEER: ::std::os::raw::c_int = 61819;
pub const IONICON__BEER_OUTLINE: ::std::os::raw::c_int = 61820;
pub const IONICON__BEER_SHARP: ::std::os::raw::c_int = 61821;
pub const IONICON__BICYCLE: ::std::os::raw::c_int = 61822;
pub const IONICON__BICYCLE_OUTLINE: ::std::os::raw::c_int = 61823;
pub const IONICON__BICYCLE_SHARP: ::std::os::raw::c_int = 61824;
pub const IONICON__BLUETOOTH: ::std::os::raw::c_int = 61825;
pub const IONICON__BLUETOOTH_OUTLINE: ::std::os::raw::c_int = 61826;
pub const IONICON__BLUETOOTH_SHARP: ::std::os::raw::c_int = 61827;
pub const IONICON__BOAT: ::std::os::raw::c_int = 61828;
pub const IONICON__BOAT_OUTLINE: ::std::os::raw::c_int = 61829;
pub const IONICON__BOAT_SHARP: ::std::os::raw::c_int = 61830;
pub const IONICON__BODY: ::std::os::raw::c_int = 61831;
pub const IONICON__BODY_OUTLINE: ::std::os::raw::c_int = 61832;
pub const IONICON__BODY_SHARP: ::std::os::raw::c_int = 61833;
pub const IONICON__BONFIRE: ::std::os::raw::c_int = 61834;
pub const IONICON__BONFIRE_OUTLINE: ::std::os::raw::c_int = 61835;
pub const IONICON__BONFIRE_SHARP: ::std::os::raw::c_int = 61836;
pub const IONICON__BOOK: ::std::os::raw::c_int = 61837;
pub const IONICON__BOOK_OUTLINE: ::std::os::raw::c_int = 61838;
pub const IONICON__BOOK_SHARP: ::std::os::raw::c_int = 61839;
pub const IONICON__BOOKMARK: ::std::os::raw::c_int = 61840;
pub const IONICON__BOOKMARK_OUTLINE: ::std::os::raw::c_int = 61841;
pub const IONICON__BOOKMARK_SHARP: ::std::os::raw::c_int = 61842;
pub const IONICON__BOOKMARKS: ::std::os::raw::c_int = 61843;
pub const IONICON__BOOKMARKS_OUTLINE: ::std::os::raw::c_int = 61844;
pub const IONICON__BOOKMARKS_SHARP: ::std::os::raw::c_int = 61845;
pub const IONICON__BRIEFCASE: ::std::os::raw::c_int = 61846;
pub const IONICON__BRIEFCASE_OUTLINE: ::std::os::raw::c_int = 61847;
pub const IONICON__BRIEFCASE_SHARP: ::std::os::raw::c_int = 61848;
pub const IONICON__BROWSERS: ::std::os::raw::c_int = 61849;
pub const IONICON__BROWSERS_OUTLINE: ::std::os::raw::c_int = 61850;
pub const IONICON__BROWSERS_SHARP: ::std::os::raw::c_int = 61851;
pub const IONICON__BRUSH: ::std::os::raw::c_int = 61852;
pub const IONICON__BRUSH_OUTLINE: ::std::os::raw::c_int = 61853;
pub const IONICON__BRUSH_SHARP: ::std::os::raw::c_int = 61854;
pub const IONICON__BUG: ::std::os::raw::c_int = 61855;
pub const IONICON__BUG_OUTLINE: ::std::os::raw::c_int = 61856;
pub const IONICON__BUG_SHARP: ::std::os::raw::c_int = 61857;
pub const IONICON__BUILD: ::std::os::raw::c_int = 61858;
pub const IONICON__BUILD_OUTLINE: ::std::os::raw::c_int = 61859;
pub const IONICON__BUILD_SHARP: ::std::os::raw::c_int = 61860;
pub const IONICON__BULB: ::std::os::raw::c_int = 61861;
pub const IONICON__BULB_OUTLINE: ::std::os::raw::c_int = 61862;
pub const IONICON__BULB_SHARP: ::std::os::raw::c_int = 61863;
pub const IONICON__BUS: ::std::os::raw::c_int = 61864;
pub const IONICON__BUS_OUTLINE: ::std::os::raw::c_int = 61865;
pub const IONICON__BUS_SHARP: ::std::os::raw::c_int = 61866;
pub const IONICON__BUSINESS: ::std::os::raw::c_int = 61867;
pub const IONICON__BUSINESS_OUTLINE: ::std::os::raw::c_int = 61868;
pub const IONICON__BUSINESS_SHARP: ::std::os::raw::c_int = 61869;
pub const IONICON__CAFE: ::std::os::raw::c_int = 61870;
pub const IONICON__CAFE_OUTLINE: ::std::os::raw::c_int = 61871;
pub const IONICON__CAFE_SHARP: ::std::os::raw::c_int = 61872;
pub const IONICON__CALCULATOR: ::std::os::raw::c_int = 61873;
pub const IONICON__CALCULATOR_OUTLINE: ::std::os::raw::c_int = 61874;
pub const IONICON__CALCULATOR_SHARP: ::std::os::raw::c_int = 61875;
pub const IONICON__CALENDAR: ::std::os::raw::c_int = 61876;
pub const IONICON__CALENDAR_OUTLINE: ::std::os::raw::c_int = 61877;
pub const IONICON__CALENDAR_SHARP: ::std::os::raw::c_int = 61878;
pub const IONICON__CALL: ::std::os::raw::c_int = 61879;
pub const IONICON__CALL_OUTLINE: ::std::os::raw::c_int = 61880;
pub const IONICON__CALL_SHARP: ::std::os::raw::c_int = 61881;
pub const IONICON__CAMERA: ::std::os::raw::c_int = 61882;
pub const IONICON__CAMERA_OUTLINE: ::std::os::raw::c_int = 61883;
pub const IONICON__CAMERA_REVERSE: ::std::os::raw::c_int = 61884;
pub const IONICON__CAMERA_REVERSE_OUTLINE: ::std::os::raw::c_int = 61885;
pub const IONICON__CAMERA_REVERSE_SHARP: ::std::os::raw::c_int = 61886;
pub const IONICON__CAMERA_SHARP: ::std::os::raw::c_int = 61887;
pub const IONICON__CAR: ::std::os::raw::c_int = 61888;
pub const IONICON__CAR_OUTLINE: ::std::os::raw::c_int = 61889;
pub const IONICON__CAR_SHARP: ::std::os::raw::c_int = 61890;
pub const IONICON__CAR_SPORT: ::std::os::raw::c_int = 61891;
pub const IONICON__CAR_SPORT_OUTLINE: ::std::os::raw::c_int = 61892;
pub const IONICON__CAR_SPORT_SHARP: ::std::os::raw::c_int = 61893;
pub const IONICON__CARD: ::std::os::raw::c_int = 61894;
pub const IONICON__CARD_OUTLINE: ::std::os::raw::c_int = 61895;
pub const IONICON__CARD_SHARP: ::std::os::raw::c_int = 61896;
pub const IONICON__CARET_BACK: ::std::os::raw::c_int = 61897;
pub const IONICON__CARET_BACK_CIRCLE: ::std::os::raw::c_int = 61898;
pub const IONICON__CARET_BACK_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61899;
pub const IONICON__CARET_BACK_CIRCLE_SHARP: ::std::os::raw::c_int = 61900;
pub const IONICON__CARET_BACK_OUTLINE: ::std::os::raw::c_int = 61901;
pub const IONICON__CARET_BACK_SHARP: ::std::os::raw::c_int = 61902;
pub const IONICON__CARET_DOWN: ::std::os::raw::c_int = 61903;
pub const IONICON__CARET_DOWN_CIRCLE: ::std::os::raw::c_int = 61904;
pub const IONICON__CARET_DOWN_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61905;
pub const IONICON__CARET_DOWN_CIRCLE_SHARP: ::std::os::raw::c_int = 61906;
pub const IONICON__CARET_DOWN_OUTLINE: ::std::os::raw::c_int = 61907;
pub const IONICON__CARET_DOWN_SHARP: ::std::os::raw::c_int = 61908;
pub const IONICON__CARET_FORWARD: ::std::os::raw::c_int = 61909;
pub const IONICON__CARET_FORWARD_CIRCLE: ::std::os::raw::c_int = 61910;
pub const IONICON__CARET_FORWARD_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61911;
pub const IONICON__CARET_FORWARD_CIRCLE_SHARP: ::std::os::raw::c_int = 61912;
pub const IONICON__CARET_FORWARD_OUTLINE: ::std::os::raw::c_int = 61913;
pub const IONICON__CARET_FORWARD_SHARP: ::std::os::raw::c_int = 61914;
pub const IONICON__CARET_UP: ::std::os::raw::c_int = 61915;
pub const IONICON__CARET_UP_CIRCLE: ::std::os::raw::c_int = 61916;
pub const IONICON__CARET_UP_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61917;
pub const IONICON__CARET_UP_CIRCLE_SHARP: ::std::os::raw::c_int = 61918;
pub const IONICON__CARET_UP_OUTLINE: ::std::os::raw::c_int = 61919;
pub const IONICON__CARET_UP_SHARP: ::std::os::raw::c_int = 61920;
pub const IONICON__CART: ::std::os::raw::c_int = 61921;
pub const IONICON__CART_OUTLINE: ::std::os::raw::c_int = 61922;
pub const IONICON__CART_SHARP: ::std::os::raw::c_int = 61923;
pub const IONICON__CASH: ::std::os::raw::c_int = 61924;
pub const IONICON__CASH_OUTLINE: ::std::os::raw::c_int = 61925;
pub const IONICON__CASH_SHARP: ::std::os::raw::c_int = 61926;
pub const IONICON__CELLULAR: ::std::os::raw::c_int = 61927;
pub const IONICON__CELLULAR_OUTLINE: ::std::os::raw::c_int = 61928;
pub const IONICON__CELLULAR_SHARP: ::std::os::raw::c_int = 61929;
pub const IONICON__CHATBOX: ::std::os::raw::c_int = 61930;
pub const IONICON__CHATBOX_ELLIPSES: ::std::os::raw::c_int = 61931;
pub const IONICON__CHATBOX_ELLIPSES_OUTLINE: ::std::os::raw::c_int = 61932;
pub const IONICON__CHATBOX_ELLIPSES_SHARP: ::std::os::raw::c_int = 61933;
pub const IONICON__CHATBOX_OUTLINE: ::std::os::raw::c_int = 61934;
pub const IONICON__CHATBOX_SHARP: ::std::os::raw::c_int = 61935;
pub const IONICON__CHATBUBBLE: ::std::os::raw::c_int = 61936;
pub const IONICON__CHATBUBBLE_ELLIPSES: ::std::os::raw::c_int = 61937;
pub const IONICON__CHATBUBBLE_ELLIPSES_OUTLINE: ::std::os::raw::c_int = 61938;
pub const IONICON__CHATBUBBLE_ELLIPSES_SHARP: ::std::os::raw::c_int = 61939;
pub const IONICON__CHATBUBBLE_OUTLINE: ::std::os::raw::c_int = 61940;
pub const IONICON__CHATBUBBLE_SHARP: ::std::os::raw::c_int = 61941;
pub const IONICON__CHATBUBBLES: ::std::os::raw::c_int = 61942;
pub const IONICON__CHATBUBBLES_OUTLINE: ::std::os::raw::c_int = 61943;
pub const IONICON__CHATBUBBLES_SHARP: ::std::os::raw::c_int = 61944;
pub const IONICON__CHECKBOX: ::std::os::raw::c_int = 61945;
pub const IONICON__CHECKBOX_OUTLINE: ::std::os::raw::c_int = 61946;
pub const IONICON__CHECKBOX_SHARP: ::std::os::raw::c_int = 61947;
pub const IONICON__CHECKMARK: ::std::os::raw::c_int = 61948;
pub const IONICON__CHECKMARK_CIRCLE: ::std::os::raw::c_int = 61949;
pub const IONICON__CHECKMARK_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61950;
pub const IONICON__CHECKMARK_CIRCLE_SHARP: ::std::os::raw::c_int = 61951;
pub const IONICON__CHECKMARK_DONE: ::std::os::raw::c_int = 61952;
pub const IONICON__CHECKMARK_DONE_CIRCLE: ::std::os::raw::c_int = 61953;
pub const IONICON__CHECKMARK_DONE_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61954;
pub const IONICON__CHECKMARK_DONE_CIRCLE_SHARP: ::std::os::raw::c_int = 61955;
pub const IONICON__CHECKMARK_DONE_OUTLINE: ::std::os::raw::c_int = 61956;
pub const IONICON__CHECKMARK_DONE_SHARP: ::std::os::raw::c_int = 61957;
pub const IONICON__CHECKMARK_OUTLINE: ::std::os::raw::c_int = 61958;
pub const IONICON__CHECKMARK_SHARP: ::std::os::raw::c_int = 61959;
pub const IONICON__CHEVRON_BACK: ::std::os::raw::c_int = 61960;
pub const IONICON__CHEVRON_BACK_CIRCLE: ::std::os::raw::c_int = 61961;
pub const IONICON__CHEVRON_BACK_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61962;
pub const IONICON__CHEVRON_BACK_CIRCLE_SHARP: ::std::os::raw::c_int = 61963;
pub const IONICON__CHEVRON_BACK_OUTLINE: ::std::os::raw::c_int = 61964;
pub const IONICON__CHEVRON_BACK_SHARP: ::std::os::raw::c_int = 61965;
pub const IONICON__CHEVRON_DOWN: ::std::os::raw::c_int = 61966;
pub const IONICON__CHEVRON_DOWN_CIRCLE: ::std::os::raw::c_int = 61967;
pub const IONICON__CHEVRON_DOWN_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61968;
pub const IONICON__CHEVRON_DOWN_CIRCLE_SHARP: ::std::os::raw::c_int = 61969;
pub const IONICON__CHEVRON_DOWN_OUTLINE: ::std::os::raw::c_int = 61970;
pub const IONICON__CHEVRON_DOWN_SHARP: ::std::os::raw::c_int = 61971;
pub const IONICON__CHEVRON_FORWARD: ::std::os::raw::c_int = 61972;
pub const IONICON__CHEVRON_FORWARD_CIRCLE: ::std::os::raw::c_int = 61973;
pub const IONICON__CHEVRON_FORWARD_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61974;
pub const IONICON__CHEVRON_FORWARD_CIRCLE_SHARP: ::std::os::raw::c_int = 61975;
pub const IONICON__CHEVRON_FORWARD_OUTLINE: ::std::os::raw::c_int = 61976;
pub const IONICON__CHEVRON_FORWARD_SHARP: ::std::os::raw::c_int = 61977;
pub const IONICON__CHEVRON_UP: ::std::os::raw::c_int = 61978;
pub const IONICON__CHEVRON_UP_CIRCLE: ::std::os::raw::c_int = 61979;
pub const IONICON__CHEVRON_UP_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61980;
pub const IONICON__CHEVRON_UP_CIRCLE_SHARP: ::std::os::raw::c_int = 61981;
pub const IONICON__CHEVRON_UP_OUTLINE: ::std::os::raw::c_int = 61982;
pub const IONICON__CHEVRON_UP_SHARP: ::std::os::raw::c_int = 61983;
pub const IONICON__CLIPBOARD: ::std::os::raw::c_int = 61984;
pub const IONICON__CLIPBOARD_OUTLINE: ::std::os::raw::c_int = 61985;
pub const IONICON__CLIPBOARD_SHARP: ::std::os::raw::c_int = 61986;
pub const IONICON__CLOSE: ::std::os::raw::c_int = 61987;
pub const IONICON__CLOSE_CIRCLE: ::std::os::raw::c_int = 61988;
pub const IONICON__CLOSE_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61989;
pub const IONICON__CLOSE_CIRCLE_SHARP: ::std::os::raw::c_int = 61990;
pub const IONICON__CLOSE_OUTLINE: ::std::os::raw::c_int = 61991;
pub const IONICON__CLOSE_SHARP: ::std::os::raw::c_int = 61992;
pub const IONICON__CLOUD: ::std::os::raw::c_int = 61993;
pub const IONICON__CLOUD_CIRCLE: ::std::os::raw::c_int = 61994;
pub const IONICON__CLOUD_CIRCLE_OUTLINE: ::std::os::raw::c_int = 61995;
pub const IONICON__CLOUD_CIRCLE_SHARP: ::std::os::raw::c_int = 61996;
pub const IONICON__CLOUD_DONE: ::std::os::raw::c_int = 61997;
pub const IONICON__CLOUD_DONE_OUTLINE: ::std::os::raw::c_int = 61998;
pub const IONICON__CLOUD_DONE_SHARP: ::std::os::raw::c_int = 61999;
pub const IONICON__CLOUD_DOWNLOAD: ::std::os::raw::c_int = 62000;
pub const IONICON__CLOUD_DOWNLOAD_OUTLINE: ::std::os::raw::c_int = 62001;
pub const IONICON__CLOUD_DOWNLOAD_SHARP: ::std::os::raw::c_int = 62002;
pub const IONICON__CLOUD_OFFLINE: ::std::os::raw::c_int = 62003;
pub const IONICON__CLOUD_OFFLINE_OUTLINE: ::std::os::raw::c_int = 62004;
pub const IONICON__CLOUD_OFFLINE_SHARP: ::std::os::raw::c_int = 62005;
pub const IONICON__CLOUD_OUTLINE: ::std::os::raw::c_int = 62006;
pub const IONICON__CLOUD_SHARP: ::std::os::raw::c_int = 62007;
pub const IONICON__CLOUD_UPLOAD: ::std::os::raw::c_int = 62008;
pub const IONICON__CLOUD_UPLOAD_OUTLINE: ::std::os::raw::c_int = 62009;
pub const IONICON__CLOUD_UPLOAD_SHARP: ::std::os::raw::c_int = 62010;
pub const IONICON__CLOUDY: ::std::os::raw::c_int = 62011;
pub const IONICON__CLOUDY_NIGHT: ::std::os::raw::c_int = 62012;
pub const IONICON__CLOUDY_NIGHT_OUTLINE: ::std::os::raw::c_int = 62013;
pub const IONICON__CLOUDY_NIGHT_SHARP: ::std::os::raw::c_int = 62014;
pub const IONICON__CLOUDY_OUTLINE: ::std::os::raw::c_int = 62015;
pub const IONICON__CLOUDY_SHARP: ::std::os::raw::c_int = 62016;
pub const IONICON__CODE: ::std::os::raw::c_int = 62017;
pub const IONICON__CODE_DOWNLOAD: ::std::os::raw::c_int = 62018;
pub const IONICON__CODE_DOWNLOAD_OUTLINE: ::std::os::raw::c_int = 62019;
pub const IONICON__CODE_DOWNLOAD_SHARP: ::std::os::raw::c_int = 62020;
pub const IONICON__CODE_OUTLINE: ::std::os::raw::c_int = 62021;
pub const IONICON__CODE_SHARP: ::std::os::raw::c_int = 62022;
pub const IONICON__CODE_SLASH: ::std::os::raw::c_int = 62023;
pub const IONICON__CODE_SLASH_OUTLINE: ::std::os::raw::c_int = 62024;
pub const IONICON__CODE_SLASH_SHARP: ::std::os::raw::c_int = 62025;
pub const IONICON__CODE_WORKING: ::std::os::raw::c_int = 62026;
pub const IONICON__CODE_WORKING_OUTLINE: ::std::os::raw::c_int = 62027;
pub const IONICON__CODE_WORKING_SHARP: ::std::os::raw::c_int = 62028;
pub const IONICON__COG: ::std::os::raw::c_int = 62029;
pub const IONICON__COG_OUTLINE: ::std::os::raw::c_int = 62030;
pub const IONICON__COG_SHARP: ::std::os::raw::c_int = 62031;
pub const IONICON__COLOR_FILL: ::std::os::raw::c_int = 62032;
pub const IONICON__COLOR_FILL_OUTLINE: ::std::os::raw::c_int = 62033;
pub const IONICON__COLOR_FILL_SHARP: ::std::os::raw::c_int = 62034;
pub const IONICON__COLOR_FILTER: ::std::os::raw::c_int = 62035;
pub const IONICON__COLOR_FILTER_OUTLINE: ::std::os::raw::c_int = 62036;
pub const IONICON__COLOR_FILTER_SHARP: ::std::os::raw::c_int = 62037;
pub const IONICON__COLOR_PALETTE: ::std::os::raw::c_int = 62038;
pub const IONICON__COLOR_PALETTE_OUTLINE: ::std::os::raw::c_int = 62039;
pub const IONICON__COLOR_PALETTE_SHARP: ::std::os::raw::c_int = 62040;
pub const IONICON__COLOR_WAND: ::std::os::raw::c_int = 62041;
pub const IONICON__COLOR_WAND_OUTLINE: ::std::os::raw::c_int = 62042;
pub const IONICON__COLOR_WAND_SHARP: ::std::os::raw::c_int = 62043;
pub const IONICON__COMPASS: ::std::os::raw::c_int = 62044;
pub const IONICON__COMPASS_OUTLINE: ::std::os::raw::c_int = 62045;
pub const IONICON__COMPASS_SHARP: ::std::os::raw::c_int = 62046;
pub const IONICON__CONSTRUCT: ::std::os::raw::c_int = 62047;
pub const IONICON__CONSTRUCT_OUTLINE: ::std::os::raw::c_int = 62048;
pub const IONICON__CONSTRUCT_SHARP: ::std::os::raw::c_int = 62049;
pub const IONICON__CONTRACT: ::std::os::raw::c_int = 62050;
pub const IONICON__CONTRACT_OUTLINE: ::std::os::raw::c_int = 62051;
pub const IONICON__CONTRACT_SHARP: ::std::os::raw::c_int = 62052;
pub const IONICON__CONTRAST: ::std::os::raw::c_int = 62053;
pub const IONICON__CONTRAST_OUTLINE: ::std::os::raw::c_int = 62054;
pub const IONICON__CONTRAST_SHARP: ::std::os::raw::c_int = 62055;
pub const IONICON__COPY: ::std::os::raw::c_int = 62056;
pub const IONICON__COPY_OUTLINE: ::std::os::raw::c_int = 62057;
pub const IONICON__COPY_SHARP: ::std::os::raw::c_int = 62058;
pub const IONICON__CREATE: ::std::os::raw::c_int = 62059;
pub const IONICON__CREATE_OUTLINE: ::std::os::raw::c_int = 62060;
pub const IONICON__CREATE_SHARP: ::std::os::raw::c_int = 62061;
pub const IONICON__CROP: ::std::os::raw::c_int = 62062;
pub const IONICON__CROP_OUTLINE: ::std::os::raw::c_int = 62063;
pub const IONICON__CROP_SHARP: ::std::os::raw::c_int = 62064;
pub const IONICON__CUBE: ::std::os::raw::c_int = 62065;
pub const IONICON__CUBE_OUTLINE: ::std::os::raw::c_int = 62066;
pub const IONICON__CUBE_SHARP: ::std::os::raw::c_int = 62067;
pub const IONICON__CUT: ::std::os::raw::c_int = 62068;
pub const IONICON__CUT_OUTLINE: ::std::os::raw::c_int = 62069;
pub const IONICON__CUT_SHARP: ::std::os::raw::c_int = 62070;
pub const IONICON__DESKTOP: ::std::os::raw::c_int = 62071;
pub const IONICON__DESKTOP_OUTLINE: ::std::os::raw::c_int = 62072;
pub const IONICON__DESKTOP_SHARP: ::std::os::raw::c_int = 62073;
pub const IONICON__DISC: ::std::os::raw::c_int = 62074;
pub const IONICON__DISC_OUTLINE: ::std::os::raw::c_int = 62075;
pub const IONICON__DISC_SHARP: ::std::os::raw::c_int = 62076;
pub const IONICON__DOCUMENT: ::std::os::raw::c_int = 62077;
pub const IONICON__DOCUMENT_ATTACH: ::std::os::raw::c_int = 62078;
pub const IONICON__DOCUMENT_ATTACH_OUTLINE: ::std::os::raw::c_int = 62079;
pub const IONICON__DOCUMENT_ATTACH_SHARP: ::std::os::raw::c_int = 62080;
pub const IONICON__DOCUMENT_OUTLINE: ::std::os::raw::c_int = 62081;
pub const IONICON__DOCUMENT_SHARP: ::std::os::raw::c_int = 62082;
pub const IONICON__DOCUMENT_TEXT: ::std::os::raw::c_int = 62083;
pub const IONICON__DOCUMENT_TEXT_OUTLINE: ::std::os::raw::c_int = 62084;
pub const IONICON__DOCUMENT_TEXT_SHARP: ::std::os::raw::c_int = 62085;
pub const IONICON__DOCUMENTS: ::std::os::raw::c_int = 62086;
pub const IONICON__DOCUMENTS_OUTLINE: ::std::os::raw::c_int = 62087;
pub const IONICON__DOCUMENTS_SHARP: ::std::os::raw::c_int = 62088;
pub const IONICON__DOWNLOAD: ::std::os::raw::c_int = 62089;
pub const IONICON__DOWNLOAD_OUTLINE: ::std::os::raw::c_int = 62090;
pub const IONICON__DOWNLOAD_SHARP: ::std::os::raw::c_int = 62091;
pub const IONICON__DUPLICATE: ::std::os::raw::c_int = 62092;
pub const IONICON__DUPLICATE_OUTLINE: ::std::os::raw::c_int = 62093;
pub const IONICON__DUPLICATE_SHARP: ::std::os::raw::c_int = 62094;
pub const IONICON__EAR: ::std::os::raw::c_int = 62095;
pub const IONICON__EAR_OUTLINE: ::std::os::raw::c_int = 62096;
pub const IONICON__EAR_SHARP: ::std::os::raw::c_int = 62097;
pub const IONICON__EARTH: ::std::os::raw::c_int = 62098;
pub const IONICON__EARTH_OUTLINE: ::std::os::raw::c_int = 62099;
pub const IONICON__EARTH_SHARP: ::std::os::raw::c_int = 62100;
pub const IONICON__EASEL: ::std::os::raw::c_int = 62101;
pub const IONICON__EASEL_OUTLINE: ::std::os::raw::c_int = 62102;
pub const IONICON__EASEL_SHARP: ::std::os::raw::c_int = 62103;
pub const IONICON__EGG: ::std::os::raw::c_int = 62104;
pub const IONICON__EGG_OUTLINE: ::std::os::raw::c_int = 62105;
pub const IONICON__EGG_SHARP: ::std::os::raw::c_int = 62106;
pub const IONICON__ELLIPSE: ::std::os::raw::c_int = 62107;
pub const IONICON__ELLIPSE_OUTLINE: ::std::os::raw::c_int = 62108;
pub const IONICON__ELLIPSE_SHARP: ::std::os::raw::c_int = 62109;
pub const IONICON__ELLIPSIS_HORIZONTAL: ::std::os::raw::c_int = 62110;
pub const IONICON__ELLIPSIS_HORIZONTAL_CIRCLE: ::std::os::raw::c_int = 62111;
pub const IONICON__ELLIPSIS_HORIZONTAL_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62112;
pub const IONICON__ELLIPSIS_HORIZONTAL_CIRCLE_SHARP: ::std::os::raw::c_int = 62113;
pub const IONICON__ELLIPSIS_HORIZONTAL_OUTLINE: ::std::os::raw::c_int = 62114;
pub const IONICON__ELLIPSIS_HORIZONTAL_SHARP: ::std::os::raw::c_int = 62115;
pub const IONICON__ELLIPSIS_VERTICAL: ::std::os::raw::c_int = 62116;
pub const IONICON__ELLIPSIS_VERTICAL_CIRCLE: ::std::os::raw::c_int = 62117;
pub const IONICON__ELLIPSIS_VERTICAL_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62118;
pub const IONICON__ELLIPSIS_VERTICAL_CIRCLE_SHARP: ::std::os::raw::c_int = 62119;
pub const IONICON__ELLIPSIS_VERTICAL_OUTLINE: ::std::os::raw::c_int = 62120;
pub const IONICON__ELLIPSIS_VERTICAL_SHARP: ::std::os::raw::c_int = 62121;
pub const IONICON__ENTER: ::std::os::raw::c_int = 62122;
pub const IONICON__ENTER_OUTLINE: ::std::os::raw::c_int = 62123;
pub const IONICON__ENTER_SHARP: ::std::os::raw::c_int = 62124;
pub const IONICON__EXIT: ::std::os::raw::c_int = 62125;
pub const IONICON__EXIT_OUTLINE: ::std::os::raw::c_int = 62126;
pub const IONICON__EXIT_SHARP: ::std::os::raw::c_int = 62127;
pub const IONICON__EXPAND: ::std::os::raw::c_int = 62128;
pub const IONICON__EXPAND_OUTLINE: ::std::os::raw::c_int = 62129;
pub const IONICON__EXPAND_SHARP: ::std::os::raw::c_int = 62130;
pub const IONICON__EYE: ::std::os::raw::c_int = 62131;
pub const IONICON__EYE_OFF: ::std::os::raw::c_int = 62132;
pub const IONICON__EYE_OFF_OUTLINE: ::std::os::raw::c_int = 62133;
pub const IONICON__EYE_OFF_SHARP: ::std::os::raw::c_int = 62134;
pub const IONICON__EYE_OUTLINE: ::std::os::raw::c_int = 62135;
pub const IONICON__EYE_SHARP: ::std::os::raw::c_int = 62136;
pub const IONICON__EYEDROP: ::std::os::raw::c_int = 62137;
pub const IONICON__EYEDROP_OUTLINE: ::std::os::raw::c_int = 62138;
pub const IONICON__EYEDROP_SHARP: ::std::os::raw::c_int = 62139;
pub const IONICON__FAST_FOOD: ::std::os::raw::c_int = 62140;
pub const IONICON__FAST_FOOD_OUTLINE: ::std::os::raw::c_int = 62141;
pub const IONICON__FAST_FOOD_SHARP: ::std::os::raw::c_int = 62142;
pub const IONICON__FEMALE: ::std::os::raw::c_int = 62143;
pub const IONICON__FEMALE_OUTLINE: ::std::os::raw::c_int = 62144;
pub const IONICON__FEMALE_SHARP: ::std::os::raw::c_int = 62145;
pub const IONICON__FILE_TRAY: ::std::os::raw::c_int = 62146;
pub const IONICON__FILE_TRAY_FULL: ::std::os::raw::c_int = 62147;
pub const IONICON__FILE_TRAY_FULL_OUTLINE: ::std::os::raw::c_int = 62148;
pub const IONICON__FILE_TRAY_FULL_SHARP: ::std::os::raw::c_int = 62149;
pub const IONICON__FILE_TRAY_OUTLINE: ::std::os::raw::c_int = 62150;
pub const IONICON__FILE_TRAY_SHARP: ::std::os::raw::c_int = 62151;
pub const IONICON__FILE_TRAY_STACKED: ::std::os::raw::c_int = 62152;
pub const IONICON__FILE_TRAY_STACKED_OUTLINE: ::std::os::raw::c_int = 62153;
pub const IONICON__FILE_TRAY_STACKED_SHARP: ::std::os::raw::c_int = 62154;
pub const IONICON__FILM: ::std::os::raw::c_int = 62155;
pub const IONICON__FILM_OUTLINE: ::std::os::raw::c_int = 62156;
pub const IONICON__FILM_SHARP: ::std::os::raw::c_int = 62157;
pub const IONICON__FILTER: ::std::os::raw::c_int = 62158;
pub const IONICON__FILTER_OUTLINE: ::std::os::raw::c_int = 62159;
pub const IONICON__FILTER_SHARP: ::std::os::raw::c_int = 62160;
pub const IONICON__FINGER_PRINT: ::std::os::raw::c_int = 62161;
pub const IONICON__FINGER_PRINT_OUTLINE: ::std::os::raw::c_int = 62162;
pub const IONICON__FINGER_PRINT_SHARP: ::std::os::raw::c_int = 62163;
pub const IONICON__FITNESS: ::std::os::raw::c_int = 62164;
pub const IONICON__FITNESS_OUTLINE: ::std::os::raw::c_int = 62165;
pub const IONICON__FITNESS_SHARP: ::std::os::raw::c_int = 62166;
pub const IONICON__FLAG: ::std::os::raw::c_int = 62167;
pub const IONICON__FLAG_OUTLINE: ::std::os::raw::c_int = 62168;
pub const IONICON__FLAG_SHARP: ::std::os::raw::c_int = 62169;
pub const IONICON__FLAME: ::std::os::raw::c_int = 62170;
pub const IONICON__FLAME_OUTLINE: ::std::os::raw::c_int = 62171;
pub const IONICON__FLAME_SHARP: ::std::os::raw::c_int = 62172;
pub const IONICON__FLASH: ::std::os::raw::c_int = 62173;
pub const IONICON__FLASH_OFF: ::std::os::raw::c_int = 62174;
pub const IONICON__FLASH_OFF_OUTLINE: ::std::os::raw::c_int = 62175;
pub const IONICON__FLASH_OFF_SHARP: ::std::os::raw::c_int = 62176;
pub const IONICON__FLASH_OUTLINE: ::std::os::raw::c_int = 62177;
pub const IONICON__FLASH_SHARP: ::std::os::raw::c_int = 62178;
pub const IONICON__FLASHLIGHT: ::std::os::raw::c_int = 62179;
pub const IONICON__FLASHLIGHT_OUTLINE: ::std::os::raw::c_int = 62180;
pub const IONICON__FLASHLIGHT_SHARP: ::std::os::raw::c_int = 62181;
pub const IONICON__FLASK: ::std::os::raw::c_int = 62182;
pub const IONICON__FLASK_OUTLINE: ::std::os::raw::c_int = 62183;
pub const IONICON__FLASK_SHARP: ::std::os::raw::c_int = 62184;
pub const IONICON__FLOWER: ::std::os::raw::c_int = 62185;
pub const IONICON__FLOWER_OUTLINE: ::std::os::raw::c_int = 62186;
pub const IONICON__FLOWER_SHARP: ::std::os::raw::c_int = 62187;
pub const IONICON__FOLDER: ::std::os::raw::c_int = 62188;
pub const IONICON__FOLDER_OPEN: ::std::os::raw::c_int = 62189;
pub const IONICON__FOLDER_OPEN_OUTLINE: ::std::os::raw::c_int = 62190;
pub const IONICON__FOLDER_OPEN_SHARP: ::std::os::raw::c_int = 62191;
pub const IONICON__FOLDER_OUTLINE: ::std::os::raw::c_int = 62192;
pub const IONICON__FOLDER_SHARP: ::std::os::raw::c_int = 62193;
pub const IONICON__FOOTBALL: ::std::os::raw::c_int = 62194;
pub const IONICON__FOOTBALL_OUTLINE: ::std::os::raw::c_int = 62195;
pub const IONICON__FOOTBALL_SHARP: ::std::os::raw::c_int = 62196;
pub const IONICON__FUNNEL: ::std::os::raw::c_int = 62197;
pub const IONICON__FUNNEL_OUTLINE: ::std::os::raw::c_int = 62198;
pub const IONICON__FUNNEL_SHARP: ::std::os::raw::c_int = 62199;
pub const IONICON__GAME_CONTROLLER: ::std::os::raw::c_int = 62200;
pub const IONICON__GAME_CONTROLLER_OUTLINE: ::std::os::raw::c_int = 62201;
pub const IONICON__GAME_CONTROLLER_SHARP: ::std::os::raw::c_int = 62202;
pub const IONICON__GIFT: ::std::os::raw::c_int = 62203;
pub const IONICON__GIFT_OUTLINE: ::std::os::raw::c_int = 62204;
pub const IONICON__GIFT_SHARP: ::std::os::raw::c_int = 62205;
pub const IONICON__GIT_BRANCH: ::std::os::raw::c_int = 62206;
pub const IONICON__GIT_BRANCH_OUTLINE: ::std::os::raw::c_int = 62207;
pub const IONICON__GIT_BRANCH_SHARP: ::std::os::raw::c_int = 62208;
pub const IONICON__GIT_COMMIT: ::std::os::raw::c_int = 62209;
pub const IONICON__GIT_COMMIT_OUTLINE: ::std::os::raw::c_int = 62210;
pub const IONICON__GIT_COMMIT_SHARP: ::std::os::raw::c_int = 62211;
pub const IONICON__GIT_COMPARE: ::std::os::raw::c_int = 62212;
pub const IONICON__GIT_COMPARE_OUTLINE: ::std::os::raw::c_int = 62213;
pub const IONICON__GIT_COMPARE_SHARP: ::std::os::raw::c_int = 62214;
pub const IONICON__GIT_MERGE: ::std::os::raw::c_int = 62215;
pub const IONICON__GIT_MERGE_OUTLINE: ::std::os::raw::c_int = 62216;
pub const IONICON__GIT_MERGE_SHARP: ::std::os::raw::c_int = 62217;
pub const IONICON__GIT_NETWORK: ::std::os::raw::c_int = 62218;
pub const IONICON__GIT_NETWORK_OUTLINE: ::std::os::raw::c_int = 62219;
pub const IONICON__GIT_NETWORK_SHARP: ::std::os::raw::c_int = 62220;
pub const IONICON__GIT_PULL_REQUEST: ::std::os::raw::c_int = 62221;
pub const IONICON__GIT_PULL_REQUEST_OUTLINE: ::std::os::raw::c_int = 62222;
pub const IONICON__GIT_PULL_REQUEST_SHARP: ::std::os::raw::c_int = 62223;
pub const IONICON__GLASSES: ::std::os::raw::c_int = 62224;
pub const IONICON__GLASSES_OUTLINE: ::std::os::raw::c_int = 62225;
pub const IONICON__GLASSES_SHARP: ::std::os::raw::c_int = 62226;
pub const IONICON__GLOBE: ::std::os::raw::c_int = 62227;
pub const IONICON__GLOBE_OUTLINE: ::std::os::raw::c_int = 62228;
pub const IONICON__GLOBE_SHARP: ::std::os::raw::c_int = 62229;
pub const IONICON__GOLF: ::std::os::raw::c_int = 62230;
pub const IONICON__GOLF_OUTLINE: ::std::os::raw::c_int = 62231;
pub const IONICON__GOLF_SHARP: ::std::os::raw::c_int = 62232;
pub const IONICON__GRID: ::std::os::raw::c_int = 62233;
pub const IONICON__GRID_OUTLINE: ::std::os::raw::c_int = 62234;
pub const IONICON__GRID_SHARP: ::std::os::raw::c_int = 62235;
pub const IONICON__HAMMER: ::std::os::raw::c_int = 62236;
pub const IONICON__HAMMER_OUTLINE: ::std::os::raw::c_int = 62237;
pub const IONICON__HAMMER_SHARP: ::std::os::raw::c_int = 62238;
pub const IONICON__HAND_LEFT: ::std::os::raw::c_int = 62239;
pub const IONICON__HAND_LEFT_OUTLINE: ::std::os::raw::c_int = 62240;
pub const IONICON__HAND_LEFT_SHARP: ::std::os::raw::c_int = 62241;
pub const IONICON__HAND_RIGHT: ::std::os::raw::c_int = 62242;
pub const IONICON__HAND_RIGHT_OUTLINE: ::std::os::raw::c_int = 62243;
pub const IONICON__HAND_RIGHT_SHARP: ::std::os::raw::c_int = 62244;
pub const IONICON__HAPPY: ::std::os::raw::c_int = 62245;
pub const IONICON__HAPPY_OUTLINE: ::std::os::raw::c_int = 62246;
pub const IONICON__HAPPY_SHARP: ::std::os::raw::c_int = 62247;
pub const IONICON__HARDWARE_CHIP: ::std::os::raw::c_int = 62248;
pub const IONICON__HARDWARE_CHIP_OUTLINE: ::std::os::raw::c_int = 62249;
pub const IONICON__HARDWARE_CHIP_SHARP: ::std::os::raw::c_int = 62250;
pub const IONICON__HEADSET: ::std::os::raw::c_int = 62251;
pub const IONICON__HEADSET_OUTLINE: ::std::os::raw::c_int = 62252;
pub const IONICON__HEADSET_SHARP: ::std::os::raw::c_int = 62253;
pub const IONICON__HEART: ::std::os::raw::c_int = 62254;
pub const IONICON__HEART_CIRCLE: ::std::os::raw::c_int = 62255;
pub const IONICON__HEART_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62256;
pub const IONICON__HEART_CIRCLE_SHARP: ::std::os::raw::c_int = 62257;
pub const IONICON__HEART_DISLIKE: ::std::os::raw::c_int = 62258;
pub const IONICON__HEART_DISLIKE_CIRCLE: ::std::os::raw::c_int = 62259;
pub const IONICON__HEART_DISLIKE_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62260;
pub const IONICON__HEART_DISLIKE_CIRCLE_SHARP: ::std::os::raw::c_int = 62261;
pub const IONICON__HEART_DISLIKE_OUTLINE: ::std::os::raw::c_int = 62262;
pub const IONICON__HEART_DISLIKE_SHARP: ::std::os::raw::c_int = 62263;
pub const IONICON__HEART_HALF: ::std::os::raw::c_int = 62264;
pub const IONICON__HEART_HALF_OUTLINE: ::std::os::raw::c_int = 62265;
pub const IONICON__HEART_HALF_SHARP: ::std::os::raw::c_int = 62266;
pub const IONICON__HEART_OUTLINE: ::std::os::raw::c_int = 62267;
pub const IONICON__HEART_SHARP: ::std::os::raw::c_int = 62268;
pub const IONICON__HELP: ::std::os::raw::c_int = 62269;
pub const IONICON__HELP_BUOY: ::std::os::raw::c_int = 62270;
pub const IONICON__HELP_BUOY_OUTLINE: ::std::os::raw::c_int = 62271;
pub const IONICON__HELP_BUOY_SHARP: ::std::os::raw::c_int = 62272;
pub const IONICON__HELP_CIRCLE: ::std::os::raw::c_int = 62273;
pub const IONICON__HELP_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62274;
pub const IONICON__HELP_CIRCLE_SHARP: ::std::os::raw::c_int = 62275;
pub const IONICON__HELP_OUTLINE: ::std::os::raw::c_int = 62276;
pub const IONICON__HELP_SHARP: ::std::os::raw::c_int = 62277;
pub const IONICON__HOME: ::std::os::raw::c_int = 62278;
pub const IONICON__HOME_OUTLINE: ::std::os::raw::c_int = 62279;
pub const IONICON__HOME_SHARP: ::std::os::raw::c_int = 62280;
pub const IONICON__HOURGLASS: ::std::os::raw::c_int = 62281;
pub const IONICON__HOURGLASS_OUTLINE: ::std::os::raw::c_int = 62282;
pub const IONICON__HOURGLASS_SHARP: ::std::os::raw::c_int = 62283;
pub const IONICON__ICE_CREAM: ::std::os::raw::c_int = 62284;
pub const IONICON__ICE_CREAM_OUTLINE: ::std::os::raw::c_int = 62285;
pub const IONICON__ICE_CREAM_SHARP: ::std::os::raw::c_int = 62286;
pub const IONICON__IMAGE: ::std::os::raw::c_int = 62287;
pub const IONICON__IMAGE_OUTLINE: ::std::os::raw::c_int = 62288;
pub const IONICON__IMAGE_SHARP: ::std::os::raw::c_int = 62289;
pub const IONICON__IMAGES: ::std::os::raw::c_int = 62290;
pub const IONICON__IMAGES_OUTLINE: ::std::os::raw::c_int = 62291;
pub const IONICON__IMAGES_SHARP: ::std::os::raw::c_int = 62292;
pub const IONICON__INFINITE: ::std::os::raw::c_int = 62293;
pub const IONICON__INFINITE_OUTLINE: ::std::os::raw::c_int = 62294;
pub const IONICON__INFINITE_SHARP: ::std::os::raw::c_int = 62295;
pub const IONICON__INFORMATION: ::std::os::raw::c_int = 62296;
pub const IONICON__INFORMATION_CIRCLE: ::std::os::raw::c_int = 62297;
pub const IONICON__INFORMATION_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62298;
pub const IONICON__INFORMATION_CIRCLE_SHARP: ::std::os::raw::c_int = 62299;
pub const IONICON__INFORMATION_OUTLINE: ::std::os::raw::c_int = 62300;
pub const IONICON__INFORMATION_SHARP: ::std::os::raw::c_int = 62301;
pub const IONICON__JOURNAL: ::std::os::raw::c_int = 62302;
pub const IONICON__JOURNAL_OUTLINE: ::std::os::raw::c_int = 62303;
pub const IONICON__JOURNAL_SHARP: ::std::os::raw::c_int = 62304;
pub const IONICON__KEY: ::std::os::raw::c_int = 62305;
pub const IONICON__KEY_OUTLINE: ::std::os::raw::c_int = 62306;
pub const IONICON__KEY_SHARP: ::std::os::raw::c_int = 62307;
pub const IONICON__KEYPAD: ::std::os::raw::c_int = 62308;
pub const IONICON__KEYPAD_OUTLINE: ::std::os::raw::c_int = 62309;
pub const IONICON__KEYPAD_SHARP: ::std::os::raw::c_int = 62310;
pub const IONICON__LANGUAGE: ::std::os::raw::c_int = 62311;
pub const IONICON__LANGUAGE_OUTLINE: ::std::os::raw::c_int = 62312;
pub const IONICON__LANGUAGE_SHARP: ::std::os::raw::c_int = 62313;
pub const IONICON__LAPTOP: ::std::os::raw::c_int = 62314;
pub const IONICON__LAPTOP_OUTLINE: ::std::os::raw::c_int = 62315;
pub const IONICON__LAPTOP_SHARP: ::std::os::raw::c_int = 62316;
pub const IONICON__LAYERS: ::std::os::raw::c_int = 62317;
pub const IONICON__LAYERS_OUTLINE: ::std::os::raw::c_int = 62318;
pub const IONICON__LAYERS_SHARP: ::std::os::raw::c_int = 62319;
pub const IONICON__LEAF: ::std::os::raw::c_int = 62320;
pub const IONICON__LEAF_OUTLINE: ::std::os::raw::c_int = 62321;
pub const IONICON__LEAF_SHARP: ::std::os::raw::c_int = 62322;
pub const IONICON__LIBRARY: ::std::os::raw::c_int = 62323;
pub const IONICON__LIBRARY_OUTLINE: ::std::os::raw::c_int = 62324;
pub const IONICON__LIBRARY_SHARP: ::std::os::raw::c_int = 62325;
pub const IONICON__LINK: ::std::os::raw::c_int = 62326;
pub const IONICON__LINK_OUTLINE: ::std::os::raw::c_int = 62327;
pub const IONICON__LINK_SHARP: ::std::os::raw::c_int = 62328;
pub const IONICON__LIST: ::std::os::raw::c_int = 62329;
pub const IONICON__LIST_CIRCLE: ::std::os::raw::c_int = 62330;
pub const IONICON__LIST_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62331;
pub const IONICON__LIST_CIRCLE_SHARP: ::std::os::raw::c_int = 62332;
pub const IONICON__LIST_OUTLINE: ::std::os::raw::c_int = 62333;
pub const IONICON__LIST_SHARP: ::std::os::raw::c_int = 62334;
pub const IONICON__LOCATE: ::std::os::raw::c_int = 62335;
pub const IONICON__LOCATE_OUTLINE: ::std::os::raw::c_int = 62336;
pub const IONICON__LOCATE_SHARP: ::std::os::raw::c_int = 62337;
pub const IONICON__LOCATION: ::std::os::raw::c_int = 62338;
pub const IONICON__LOCATION_OUTLINE: ::std::os::raw::c_int = 62339;
pub const IONICON__LOCATION_SHARP: ::std::os::raw::c_int = 62340;
pub const IONICON__LOCK_CLOSED: ::std::os::raw::c_int = 62341;
pub const IONICON__LOCK_CLOSED_OUTLINE: ::std::os::raw::c_int = 62342;
pub const IONICON__LOCK_CLOSED_SHARP: ::std::os::raw::c_int = 62343;
pub const IONICON__LOCK_OPEN: ::std::os::raw::c_int = 62344;
pub const IONICON__LOCK_OPEN_OUTLINE: ::std::os::raw::c_int = 62345;
pub const IONICON__LOCK_OPEN_SHARP: ::std::os::raw::c_int = 62346;
pub const IONICON__LOG_IN: ::std::os::raw::c_int = 62347;
pub const IONICON__LOG_IN_OUTLINE: ::std::os::raw::c_int = 62348;
pub const IONICON__LOG_IN_SHARP: ::std::os::raw::c_int = 62349;
pub const IONICON__LOG_OUT: ::std::os::raw::c_int = 62350;
pub const IONICON__LOG_OUT_OUTLINE: ::std::os::raw::c_int = 62351;
pub const IONICON__LOG_OUT_SHARP: ::std::os::raw::c_int = 62352;
pub const IONICON__LOGO_AMAZON: ::std::os::raw::c_int = 62353;
pub const IONICON__LOGO_AMPLIFY: ::std::os::raw::c_int = 62354;
pub const IONICON__LOGO_ANDROID: ::std::os::raw::c_int = 62355;
pub const IONICON__LOGO_ANGULAR: ::std::os::raw::c_int = 62356;
pub const IONICON__LOGO_APPLE: ::std::os::raw::c_int = 62357;
pub const IONICON__LOGO_APPLE_APPSTORE: ::std::os::raw::c_int = 62358;
pub const IONICON__LOGO_BITBUCKET: ::std::os::raw::c_int = 62359;
pub const IONICON__LOGO_BITCOIN: ::std::os::raw::c_int = 62360;
pub const IONICON__LOGO_BUFFER: ::std::os::raw::c_int = 62361;
pub const IONICON__LOGO_CAPACITOR: ::std::os::raw::c_int = 62362;
pub const IONICON__LOGO_CHROME: ::std::os::raw::c_int = 62363;
pub const IONICON__LOGO_CLOSED_CAPTIONING: ::std::os::raw::c_int = 62364;
pub const IONICON__LOGO_CODEPEN: ::std::os::raw::c_int = 62365;
pub const IONICON__LOGO_CSS3: ::std::os::raw::c_int = 62366;
pub const IONICON__LOGO_DESIGNERNEWS: ::std::os::raw::c_int = 62367;
pub const IONICON__LOGO_DRIBBBLE: ::std::os::raw::c_int = 62368;
pub const IONICON__LOGO_DROPBOX: ::std::os::raw::c_int = 62369;
pub const IONICON__LOGO_EDGE: ::std::os::raw::c_int = 62370;
pub const IONICON__LOGO_ELECTRON: ::std::os::raw::c_int = 62371;
pub const IONICON__LOGO_EURO: ::std::os::raw::c_int = 62372;
pub const IONICON__LOGO_FACEBOOK: ::std::os::raw::c_int = 62373;
pub const IONICON__LOGO_FIREBASE: ::std::os::raw::c_int = 62374;
pub const IONICON__LOGO_FIREFOX: ::std::os::raw::c_int = 62375;
pub const IONICON__LOGO_FLICKR: ::std::os::raw::c_int = 62376;
pub const IONICON__LOGO_FOURSQUARE: ::std::os::raw::c_int = 62377;
pub const IONICON__LOGO_GITHUB: ::std::os::raw::c_int = 62378;
pub const IONICON__LOGO_GOOGLE: ::std::os::raw::c_int = 62379;
pub const IONICON__LOGO_GOOGLE_PLAYSTORE: ::std::os::raw::c_int = 62380;
pub const IONICON__LOGO_HACKERNEWS: ::std::os::raw::c_int = 62381;
pub const IONICON__LOGO_HTML5: ::std::os::raw::c_int = 62382;
pub const IONICON__LOGO_INSTAGRAM: ::std::os::raw::c_int = 62383;
pub const IONICON__LOGO_IONIC: ::std::os::raw::c_int = 62384;
pub const IONICON__LOGO_IONITRON: ::std::os::raw::c_int = 62385;
pub const IONICON__LOGO_JAVASCRIPT: ::std::os::raw::c_int = 62386;
pub const IONICON__LOGO_LARAVEL: ::std::os::raw::c_int = 62387;
pub const IONICON__LOGO_LINKEDIN: ::std::os::raw::c_int = 62388;
pub const IONICON__LOGO_MARKDOWN: ::std::os::raw::c_int = 62389;
pub const IONICON__LOGO_NO_SMOKING: ::std::os::raw::c_int = 62390;
pub const IONICON__LOGO_NODEJS: ::std::os::raw::c_int = 62391;
pub const IONICON__LOGO_NPM: ::std::os::raw::c_int = 62392;
pub const IONICON__LOGO_OCTOCAT: ::std::os::raw::c_int = 62393;
pub const IONICON__LOGO_PINTEREST: ::std::os::raw::c_int = 62394;
pub const IONICON__LOGO_PLAYSTATION: ::std::os::raw::c_int = 62395;
pub const IONICON__LOGO_PWA: ::std::os::raw::c_int = 62396;
pub const IONICON__LOGO_PYTHON: ::std::os::raw::c_int = 62397;
pub const IONICON__LOGO_REACT: ::std::os::raw::c_int = 62398;
pub const IONICON__LOGO_REDDIT: ::std::os::raw::c_int = 62399;
pub const IONICON__LOGO_RSS: ::std::os::raw::c_int = 62400;
pub const IONICON__LOGO_SASS: ::std::os::raw::c_int = 62401;
pub const IONICON__LOGO_SKYPE: ::std::os::raw::c_int = 62402;
pub const IONICON__LOGO_SLACK: ::std::os::raw::c_int = 62403;
pub const IONICON__LOGO_SNAPCHAT: ::std::os::raw::c_int = 62404;
pub const IONICON__LOGO_STACKOVERFLOW: ::std::os::raw::c_int = 62405;
pub const IONICON__LOGO_STEAM: ::std::os::raw::c_int = 62406;
pub const IONICON__LOGO_STENCIL: ::std::os::raw::c_int = 62407;
pub const IONICON__LOGO_TUMBLR: ::std::os::raw::c_int = 62408;
pub const IONICON__LOGO_TUX: ::std::os::raw::c_int = 62409;
pub const IONICON__LOGO_TWITCH: ::std::os::raw::c_int = 62410;
pub const IONICON__LOGO_TWITTER: ::std::os::raw::c_int = 62411;
pub const IONICON__LOGO_USD: ::std::os::raw::c_int = 62412;
pub const IONICON__LOGO_VIMEO: ::std::os::raw::c_int = 62413;
pub const IONICON__LOGO_VK: ::std::os::raw::c_int = 62414;
pub const IONICON__LOGO_VUE: ::std::os::raw::c_int = 62415;
pub const IONICON__LOGO_WEB_COMPONENT: ::std::os::raw::c_int = 62416;
pub const IONICON__LOGO_WHATSAPP: ::std::os::raw::c_int = 62417;
pub const IONICON__LOGO_WINDOWS: ::std::os::raw::c_int = 62418;
pub const IONICON__LOGO_WORDPRESS: ::std::os::raw::c_int = 62419;
pub const IONICON__LOGO_XBOX: ::std::os::raw::c_int = 62420;
pub const IONICON__LOGO_XING: ::std::os::raw::c_int = 62421;
pub const IONICON__LOGO_YAHOO: ::std::os::raw::c_int = 62422;
pub const IONICON__LOGO_YEN: ::std::os::raw::c_int = 62423;
pub const IONICON__LOGO_YOUTUBE: ::std::os::raw::c_int = 62424;
pub const IONICON__MAGNET: ::std::os::raw::c_int = 62425;
pub const IONICON__MAGNET_OUTLINE: ::std::os::raw::c_int = 62426;
pub const IONICON__MAGNET_SHARP: ::std::os::raw::c_int = 62427;
pub const IONICON__MAIL: ::std::os::raw::c_int = 62428;
pub const IONICON__MAIL_OPEN: ::std::os::raw::c_int = 62429;
pub const IONICON__MAIL_OPEN_OUTLINE: ::std::os::raw::c_int = 62430;
pub const IONICON__MAIL_OPEN_SHARP: ::std::os::raw::c_int = 62431;
pub const IONICON__MAIL_OUTLINE: ::std::os::raw::c_int = 62432;
pub const IONICON__MAIL_SHARP: ::std::os::raw::c_int = 62433;
pub const IONICON__MAIL_UNREAD: ::std::os::raw::c_int = 62434;
pub const IONICON__MAIL_UNREAD_OUTLINE: ::std::os::raw::c_int = 62435;
pub const IONICON__MAIL_UNREAD_SHARP: ::std::os::raw::c_int = 62436;
pub const IONICON__MALE: ::std::os::raw::c_int = 62437;
pub const IONICON__MALE_FEMALE: ::std::os::raw::c_int = 62438;
pub const IONICON__MALE_FEMALE_OUTLINE: ::std::os::raw::c_int = 62439;
pub const IONICON__MALE_FEMALE_SHARP: ::std::os::raw::c_int = 62440;
pub const IONICON__MALE_OUTLINE: ::std::os::raw::c_int = 62441;
pub const IONICON__MALE_SHARP: ::std::os::raw::c_int = 62442;
pub const IONICON__MAN: ::std::os::raw::c_int = 62443;
pub const IONICON__MAN_OUTLINE: ::std::os::raw::c_int = 62444;
pub const IONICON__MAN_SHARP: ::std::os::raw::c_int = 62445;
pub const IONICON__MAP: ::std::os::raw::c_int = 62446;
pub const IONICON__MAP_OUTLINE: ::std::os::raw::c_int = 62447;
pub const IONICON__MAP_SHARP: ::std::os::raw::c_int = 62448;
pub const IONICON__MEDAL: ::std::os::raw::c_int = 62449;
pub const IONICON__MEDAL_OUTLINE: ::std::os::raw::c_int = 62450;
pub const IONICON__MEDAL_SHARP: ::std::os::raw::c_int = 62451;
pub const IONICON__MEDICAL: ::std::os::raw::c_int = 62452;
pub const IONICON__MEDICAL_OUTLINE: ::std::os::raw::c_int = 62453;
pub const IONICON__MEDICAL_SHARP: ::std::os::raw::c_int = 62454;
pub const IONICON__MEDKIT: ::std::os::raw::c_int = 62455;
pub const IONICON__MEDKIT_OUTLINE: ::std::os::raw::c_int = 62456;
pub const IONICON__MEDKIT_SHARP: ::std::os::raw::c_int = 62457;
pub const IONICON__MEGAPHONE: ::std::os::raw::c_int = 62458;
pub const IONICON__MEGAPHONE_OUTLINE: ::std::os::raw::c_int = 62459;
pub const IONICON__MEGAPHONE_SHARP: ::std::os::raw::c_int = 62460;
pub const IONICON__MENU: ::std::os::raw::c_int = 62461;
pub const IONICON__MENU_OUTLINE: ::std::os::raw::c_int = 62462;
pub const IONICON__MENU_SHARP: ::std::os::raw::c_int = 62463;
pub const IONICON__MIC: ::std::os::raw::c_int = 62464;
pub const IONICON__MIC_CIRCLE: ::std::os::raw::c_int = 62465;
pub const IONICON__MIC_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62466;
pub const IONICON__MIC_CIRCLE_SHARP: ::std::os::raw::c_int = 62467;
pub const IONICON__MIC_OFF: ::std::os::raw::c_int = 62468;
pub const IONICON__MIC_OFF_CIRCLE: ::std::os::raw::c_int = 62469;
pub const IONICON__MIC_OFF_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62470;
pub const IONICON__MIC_OFF_CIRCLE_SHARP: ::std::os::raw::c_int = 62471;
pub const IONICON__MIC_OFF_OUTLINE: ::std::os::raw::c_int = 62472;
pub const IONICON__MIC_OFF_SHARP: ::std::os::raw::c_int = 62473;
pub const IONICON__MIC_OUTLINE: ::std::os::raw::c_int = 62474;
pub const IONICON__MIC_SHARP: ::std::os::raw::c_int = 62475;
pub const IONICON__MOON: ::std::os::raw::c_int = 62476;
pub const IONICON__MOON_OUTLINE: ::std::os::raw::c_int = 62477;
pub const IONICON__MOON_SHARP: ::std::os::raw::c_int = 62478;
pub const IONICON__MOVE: ::std::os::raw::c_int = 62479;
pub const IONICON__MOVE_OUTLINE: ::std::os::raw::c_int = 62480;
pub const IONICON__MOVE_SHARP: ::std::os::raw::c_int = 62481;
pub const IONICON__MUSICAL_NOTE: ::std::os::raw::c_int = 62482;
pub const IONICON__MUSICAL_NOTE_OUTLINE: ::std::os::raw::c_int = 62483;
pub const IONICON__MUSICAL_NOTE_SHARP: ::std::os::raw::c_int = 62484;
pub const IONICON__MUSICAL_NOTES: ::std::os::raw::c_int = 62485;
pub const IONICON__MUSICAL_NOTES_OUTLINE: ::std::os::raw::c_int = 62486;
pub const IONICON__MUSICAL_NOTES_SHARP: ::std::os::raw::c_int = 62487;
pub const IONICON__NAVIGATE: ::std::os::raw::c_int = 62488;
pub const IONICON__NAVIGATE_CIRCLE: ::std::os::raw::c_int = 62489;
pub const IONICON__NAVIGATE_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62490;
pub const IONICON__NAVIGATE_CIRCLE_SHARP: ::std::os::raw::c_int = 62491;
pub const IONICON__NAVIGATE_OUTLINE: ::std::os::raw::c_int = 62492;
pub const IONICON__NAVIGATE_SHARP: ::std::os::raw::c_int = 62493;
pub const IONICON__NEWSPAPER: ::std::os::raw::c_int = 62494;
pub const IONICON__NEWSPAPER_OUTLINE: ::std::os::raw::c_int = 62495;
pub const IONICON__NEWSPAPER_SHARP: ::std::os::raw::c_int = 62496;
pub const IONICON__NOTIFICATIONS: ::std::os::raw::c_int = 62497;
pub const IONICON__NOTIFICATIONS_CIRCLE: ::std::os::raw::c_int = 62498;
pub const IONICON__NOTIFICATIONS_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62499;
pub const IONICON__NOTIFICATIONS_CIRCLE_SHARP: ::std::os::raw::c_int = 62500;
pub const IONICON__NOTIFICATIONS_OFF: ::std::os::raw::c_int = 62501;
pub const IONICON__NOTIFICATIONS_OFF_CIRCLE: ::std::os::raw::c_int = 62502;
pub const IONICON__NOTIFICATIONS_OFF_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62503;
pub const IONICON__NOTIFICATIONS_OFF_CIRCLE_SHARP: ::std::os::raw::c_int = 62504;
pub const IONICON__NOTIFICATIONS_OFF_OUTLINE: ::std::os::raw::c_int = 62505;
pub const IONICON__NOTIFICATIONS_OFF_SHARP: ::std::os::raw::c_int = 62506;
pub const IONICON__NOTIFICATIONS_OUTLINE: ::std::os::raw::c_int = 62507;
pub const IONICON__NOTIFICATIONS_SHARP: ::std::os::raw::c_int = 62508;
pub const IONICON__NUCLEAR: ::std::os::raw::c_int = 62509;
pub const IONICON__NUCLEAR_OUTLINE: ::std::os::raw::c_int = 62510;
pub const IONICON__NUCLEAR_SHARP: ::std::os::raw::c_int = 62511;
pub const IONICON__NUTRITION: ::std::os::raw::c_int = 62512;
pub const IONICON__NUTRITION_OUTLINE: ::std::os::raw::c_int = 62513;
pub const IONICON__NUTRITION_SHARP: ::std::os::raw::c_int = 62514;
pub const IONICON__OPEN: ::std::os::raw::c_int = 62515;
pub const IONICON__OPEN_OUTLINE: ::std::os::raw::c_int = 62516;
pub const IONICON__OPEN_SHARP: ::std::os::raw::c_int = 62517;
pub const IONICON__OPTIONS: ::std::os::raw::c_int = 62518;
pub const IONICON__OPTIONS_OUTLINE: ::std::os::raw::c_int = 62519;
pub const IONICON__OPTIONS_SHARP: ::std::os::raw::c_int = 62520;
pub const IONICON__PAPER_PLANE: ::std::os::raw::c_int = 62521;
pub const IONICON__PAPER_PLANE_OUTLINE: ::std::os::raw::c_int = 62522;
pub const IONICON__PAPER_PLANE_SHARP: ::std::os::raw::c_int = 62523;
pub const IONICON__PARTLY_SUNNY: ::std::os::raw::c_int = 62524;
pub const IONICON__PARTLY_SUNNY_OUTLINE: ::std::os::raw::c_int = 62525;
pub const IONICON__PARTLY_SUNNY_SHARP: ::std::os::raw::c_int = 62526;
pub const IONICON__PAUSE: ::std::os::raw::c_int = 62527;
pub const IONICON__PAUSE_CIRCLE: ::std::os::raw::c_int = 62528;
pub const IONICON__PAUSE_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62529;
pub const IONICON__PAUSE_CIRCLE_SHARP: ::std::os::raw::c_int = 62530;
pub const IONICON__PAUSE_OUTLINE: ::std::os::raw::c_int = 62531;
pub const IONICON__PAUSE_SHARP: ::std::os::raw::c_int = 62532;
pub const IONICON__PAW: ::std::os::raw::c_int = 62533;
pub const IONICON__PAW_OUTLINE: ::std::os::raw::c_int = 62534;
pub const IONICON__PAW_SHARP: ::std::os::raw::c_int = 62535;
pub const IONICON__PENCIL: ::std::os::raw::c_int = 62536;
pub const IONICON__PENCIL_OUTLINE: ::std::os::raw::c_int = 62537;
pub const IONICON__PENCIL_SHARP: ::std::os::raw::c_int = 62538;
pub const IONICON__PEOPLE: ::std::os::raw::c_int = 62539;
pub const IONICON__PEOPLE_CIRCLE: ::std::os::raw::c_int = 62540;
pub const IONICON__PEOPLE_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62541;
pub const IONICON__PEOPLE_CIRCLE_SHARP: ::std::os::raw::c_int = 62542;
pub const IONICON__PEOPLE_OUTLINE: ::std::os::raw::c_int = 62543;
pub const IONICON__PEOPLE_SHARP: ::std::os::raw::c_int = 62544;
pub const IONICON__PERSON: ::std::os::raw::c_int = 62545;
pub const IONICON__PERSON_ADD: ::std::os::raw::c_int = 62546;
pub const IONICON__PERSON_ADD_OUTLINE: ::std::os::raw::c_int = 62547;
pub const IONICON__PERSON_ADD_SHARP: ::std::os::raw::c_int = 62548;
pub const IONICON__PERSON_CIRCLE: ::std::os::raw::c_int = 62549;
pub const IONICON__PERSON_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62550;
pub const IONICON__PERSON_CIRCLE_SHARP: ::std::os::raw::c_int = 62551;
pub const IONICON__PERSON_OUTLINE: ::std::os::raw::c_int = 62552;
pub const IONICON__PERSON_REMOVE: ::std::os::raw::c_int = 62553;
pub const IONICON__PERSON_REMOVE_OUTLINE: ::std::os::raw::c_int = 62554;
pub const IONICON__PERSON_REMOVE_SHARP: ::std::os::raw::c_int = 62555;
pub const IONICON__PERSON_SHARP: ::std::os::raw::c_int = 62556;
pub const IONICON__PHONE_LANDSCAPE: ::std::os::raw::c_int = 62557;
pub const IONICON__PHONE_LANDSCAPE_OUTLINE: ::std::os::raw::c_int = 62558;
pub const IONICON__PHONE_LANDSCAPE_SHARP: ::std::os::raw::c_int = 62559;
pub const IONICON__PHONE_PORTRAIT: ::std::os::raw::c_int = 62560;
pub const IONICON__PHONE_PORTRAIT_OUTLINE: ::std::os::raw::c_int = 62561;
pub const IONICON__PHONE_PORTRAIT_SHARP: ::std::os::raw::c_int = 62562;
pub const IONICON__PIE_CHART: ::std::os::raw::c_int = 62563;
pub const IONICON__PIE_CHART_OUTLINE: ::std::os::raw::c_int = 62564;
pub const IONICON__PIE_CHART_SHARP: ::std::os::raw::c_int = 62565;
pub const IONICON__PIN: ::std::os::raw::c_int = 62566;
pub const IONICON__PIN_OUTLINE: ::std::os::raw::c_int = 62567;
pub const IONICON__PIN_SHARP: ::std::os::raw::c_int = 62568;
pub const IONICON__PINT: ::std::os::raw::c_int = 62569;
pub const IONICON__PINT_OUTLINE: ::std::os::raw::c_int = 62570;
pub const IONICON__PINT_SHARP: ::std::os::raw::c_int = 62571;
pub const IONICON__PIZZA: ::std::os::raw::c_int = 62572;
pub const IONICON__PIZZA_OUTLINE: ::std::os::raw::c_int = 62573;
pub const IONICON__PIZZA_SHARP: ::std::os::raw::c_int = 62574;
pub const IONICON__PLANET: ::std::os::raw::c_int = 62575;
pub const IONICON__PLANET_OUTLINE: ::std::os::raw::c_int = 62576;
pub const IONICON__PLANET_SHARP: ::std::os::raw::c_int = 62577;
pub const IONICON__PLAY: ::std::os::raw::c_int = 62578;
pub const IONICON__PLAY_BACK: ::std::os::raw::c_int = 62579;
pub const IONICON__PLAY_BACK_CIRCLE: ::std::os::raw::c_int = 62580;
pub const IONICON__PLAY_BACK_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62581;
pub const IONICON__PLAY_BACK_CIRCLE_SHARP: ::std::os::raw::c_int = 62582;
pub const IONICON__PLAY_BACK_OUTLINE: ::std::os::raw::c_int = 62583;
pub const IONICON__PLAY_BACK_SHARP: ::std::os::raw::c_int = 62584;
pub const IONICON__PLAY_CIRCLE: ::std::os::raw::c_int = 62585;
pub const IONICON__PLAY_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62586;
pub const IONICON__PLAY_CIRCLE_SHARP: ::std::os::raw::c_int = 62587;
pub const IONICON__PLAY_FORWARD: ::std::os::raw::c_int = 62588;
pub const IONICON__PLAY_FORWARD_CIRCLE: ::std::os::raw::c_int = 62589;
pub const IONICON__PLAY_FORWARD_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62590;
pub const IONICON__PLAY_FORWARD_CIRCLE_SHARP: ::std::os::raw::c_int = 62591;
pub const IONICON__PLAY_FORWARD_OUTLINE: ::std::os::raw::c_int = 62592;
pub const IONICON__PLAY_FORWARD_SHARP: ::std::os::raw::c_int = 62593;
pub const IONICON__PLAY_OUTLINE: ::std::os::raw::c_int = 62594;
pub const IONICON__PLAY_SHARP: ::std::os::raw::c_int = 62595;
pub const IONICON__PLAY_SKIP_BACK: ::std::os::raw::c_int = 62596;
pub const IONICON__PLAY_SKIP_BACK_CIRCLE: ::std::os::raw::c_int = 62597;
pub const IONICON__PLAY_SKIP_BACK_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62598;
pub const IONICON__PLAY_SKIP_BACK_CIRCLE_SHARP: ::std::os::raw::c_int = 62599;
pub const IONICON__PLAY_SKIP_BACK_OUTLINE: ::std::os::raw::c_int = 62600;
pub const IONICON__PLAY_SKIP_BACK_SHARP: ::std::os::raw::c_int = 62601;
pub const IONICON__PLAY_SKIP_FORWARD: ::std::os::raw::c_int = 62602;
pub const IONICON__PLAY_SKIP_FORWARD_CIRCLE: ::std::os::raw::c_int = 62603;
pub const IONICON__PLAY_SKIP_FORWARD_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62604;
pub const IONICON__PLAY_SKIP_FORWARD_CIRCLE_SHARP: ::std::os::raw::c_int = 62605;
pub const IONICON__PLAY_SKIP_FORWARD_OUTLINE: ::std::os::raw::c_int = 62606;
pub const IONICON__PLAY_SKIP_FORWARD_SHARP: ::std::os::raw::c_int = 62607;
pub const IONICON__PODIUM: ::std::os::raw::c_int = 62608;
pub const IONICON__PODIUM_OUTLINE: ::std::os::raw::c_int = 62609;
pub const IONICON__PODIUM_SHARP: ::std::os::raw::c_int = 62610;
pub const IONICON__POWER: ::std::os::raw::c_int = 62611;
pub const IONICON__POWER_OUTLINE: ::std::os::raw::c_int = 62612;
pub const IONICON__POWER_SHARP: ::std::os::raw::c_int = 62613;
pub const IONICON__PRICETAG: ::std::os::raw::c_int = 62614;
pub const IONICON__PRICETAG_OUTLINE: ::std::os::raw::c_int = 62615;
pub const IONICON__PRICETAG_SHARP: ::std::os::raw::c_int = 62616;
pub const IONICON__PRICETAGS: ::std::os::raw::c_int = 62617;
pub const IONICON__PRICETAGS_OUTLINE: ::std::os::raw::c_int = 62618;
pub const IONICON__PRICETAGS_SHARP: ::std::os::raw::c_int = 62619;
pub const IONICON__PRINT: ::std::os::raw::c_int = 62620;
pub const IONICON__PRINT_OUTLINE: ::std::os::raw::c_int = 62621;
pub const IONICON__PRINT_SHARP: ::std::os::raw::c_int = 62622;
pub const IONICON__PULSE: ::std::os::raw::c_int = 62623;
pub const IONICON__PULSE_OUTLINE: ::std::os::raw::c_int = 62624;
pub const IONICON__PULSE_SHARP: ::std::os::raw::c_int = 62625;
pub const IONICON__PUSH: ::std::os::raw::c_int = 62626;
pub const IONICON__PUSH_OUTLINE: ::std::os::raw::c_int = 62627;
pub const IONICON__PUSH_SHARP: ::std::os::raw::c_int = 62628;
pub const IONICON__QR_CODE: ::std::os::raw::c_int = 62629;
pub const IONICON__QR_CODE_OUTLINE: ::std::os::raw::c_int = 62630;
pub const IONICON__QR_CODE_SHARP: ::std::os::raw::c_int = 62631;
pub const IONICON__RADIO: ::std::os::raw::c_int = 62632;
pub const IONICON__RADIO_BUTTON_OFF: ::std::os::raw::c_int = 62633;
pub const IONICON__RADIO_BUTTON_OFF_OUTLINE: ::std::os::raw::c_int = 62634;
pub const IONICON__RADIO_BUTTON_OFF_SHARP: ::std::os::raw::c_int = 62635;
pub const IONICON__RADIO_BUTTON_ON: ::std::os::raw::c_int = 62636;
pub const IONICON__RADIO_BUTTON_ON_OUTLINE: ::std::os::raw::c_int = 62637;
pub const IONICON__RADIO_BUTTON_ON_SHARP: ::std::os::raw::c_int = 62638;
pub const IONICON__RADIO_OUTLINE: ::std::os::raw::c_int = 62639;
pub const IONICON__RADIO_SHARP: ::std::os::raw::c_int = 62640;
pub const IONICON__RAINY: ::std::os::raw::c_int = 62641;
pub const IONICON__RAINY_OUTLINE: ::std::os::raw::c_int = 62642;
pub const IONICON__RAINY_SHARP: ::std::os::raw::c_int = 62643;
pub const IONICON__READER: ::std::os::raw::c_int = 62644;
pub const IONICON__READER_OUTLINE: ::std::os::raw::c_int = 62645;
pub const IONICON__READER_SHARP: ::std::os::raw::c_int = 62646;
pub const IONICON__RECEIPT: ::std::os::raw::c_int = 62647;
pub const IONICON__RECEIPT_OUTLINE: ::std::os::raw::c_int = 62648;
pub const IONICON__RECEIPT_SHARP: ::std::os::raw::c_int = 62649;
pub const IONICON__RECORDING: ::std::os::raw::c_int = 62650;
pub const IONICON__RECORDING_OUTLINE: ::std::os::raw::c_int = 62651;
pub const IONICON__RECORDING_SHARP: ::std::os::raw::c_int = 62652;
pub const IONICON__REFRESH: ::std::os::raw::c_int = 62653;
pub const IONICON__REFRESH_CIRCLE: ::std::os::raw::c_int = 62654;
pub const IONICON__REFRESH_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62655;
pub const IONICON__REFRESH_CIRCLE_SHARP: ::std::os::raw::c_int = 62656;
pub const IONICON__REFRESH_OUTLINE: ::std::os::raw::c_int = 62657;
pub const IONICON__REFRESH_SHARP: ::std::os::raw::c_int = 62658;
pub const IONICON__RELOAD: ::std::os::raw::c_int = 62659;
pub const IONICON__RELOAD_CIRCLE: ::std::os::raw::c_int = 62660;
pub const IONICON__RELOAD_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62661;
pub const IONICON__RELOAD_CIRCLE_SHARP: ::std::os::raw::c_int = 62662;
pub const IONICON__RELOAD_OUTLINE: ::std::os::raw::c_int = 62663;
pub const IONICON__RELOAD_SHARP: ::std::os::raw::c_int = 62664;
pub const IONICON__REMOVE: ::std::os::raw::c_int = 62665;
pub const IONICON__REMOVE_CIRCLE: ::std::os::raw::c_int = 62666;
pub const IONICON__REMOVE_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62667;
pub const IONICON__REMOVE_CIRCLE_SHARP: ::std::os::raw::c_int = 62668;
pub const IONICON__REMOVE_OUTLINE: ::std::os::raw::c_int = 62669;
pub const IONICON__REMOVE_SHARP: ::std::os::raw::c_int = 62670;
pub const IONICON__REORDER_FOUR: ::std::os::raw::c_int = 62671;
pub const IONICON__REORDER_FOUR_OUTLINE: ::std::os::raw::c_int = 62672;
pub const IONICON__REORDER_FOUR_SHARP: ::std::os::raw::c_int = 62673;
pub const IONICON__REORDER_THREE: ::std::os::raw::c_int = 62674;
pub const IONICON__REORDER_THREE_OUTLINE: ::std::os::raw::c_int = 62675;
pub const IONICON__REORDER_THREE_SHARP: ::std::os::raw::c_int = 62676;
pub const IONICON__REORDER_TWO: ::std::os::raw::c_int = 62677;
pub const IONICON__REORDER_TWO_OUTLINE: ::std::os::raw::c_int = 62678;
pub const IONICON__REORDER_TWO_SHARP: ::std::os::raw::c_int = 62679;
pub const IONICON__REPEAT: ::std::os::raw::c_int = 62680;
pub const IONICON__REPEAT_OUTLINE: ::std::os::raw::c_int = 62681;
pub const IONICON__REPEAT_SHARP: ::std::os::raw::c_int = 62682;
pub const IONICON__RESIZE: ::std::os::raw::c_int = 62683;
pub const IONICON__RESIZE_OUTLINE: ::std::os::raw::c_int = 62684;
pub const IONICON__RESIZE_SHARP: ::std::os::raw::c_int = 62685;
pub const IONICON__RESTAURANT: ::std::os::raw::c_int = 62686;
pub const IONICON__RESTAURANT_OUTLINE: ::std::os::raw::c_int = 62687;
pub const IONICON__RESTAURANT_SHARP: ::std::os::raw::c_int = 62688;
pub const IONICON__RETURN_DOWN_BACK: ::std::os::raw::c_int = 62689;
pub const IONICON__RETURN_DOWN_BACK_OUTLINE: ::std::os::raw::c_int = 62690;
pub const IONICON__RETURN_DOWN_BACK_SHARP: ::std::os::raw::c_int = 62691;
pub const IONICON__RETURN_DOWN_FORWARD: ::std::os::raw::c_int = 62692;
pub const IONICON__RETURN_DOWN_FORWARD_OUTLINE: ::std::os::raw::c_int = 62693;
pub const IONICON__RETURN_DOWN_FORWARD_SHARP: ::std::os::raw::c_int = 62694;
pub const IONICON__RETURN_UP_BACK: ::std::os::raw::c_int = 62695;
pub const IONICON__RETURN_UP_BACK_OUTLINE: ::std::os::raw::c_int = 62696;
pub const IONICON__RETURN_UP_BACK_SHARP: ::std::os::raw::c_int = 62697;
pub const IONICON__RETURN_UP_FORWARD: ::std::os::raw::c_int = 62698;
pub const IONICON__RETURN_UP_FORWARD_OUTLINE: ::std::os::raw::c_int = 62699;
pub const IONICON__RETURN_UP_FORWARD_SHARP: ::std::os::raw::c_int = 62700;
pub const IONICON__RIBBON: ::std::os::raw::c_int = 62701;
pub const IONICON__RIBBON_OUTLINE: ::std::os::raw::c_int = 62702;
pub const IONICON__RIBBON_SHARP: ::std::os::raw::c_int = 62703;
pub const IONICON__ROCKET: ::std::os::raw::c_int = 62704;
pub const IONICON__ROCKET_OUTLINE: ::std::os::raw::c_int = 62705;
pub const IONICON__ROCKET_SHARP: ::std::os::raw::c_int = 62706;
pub const IONICON__ROSE: ::std::os::raw::c_int = 62707;
pub const IONICON__ROSE_OUTLINE: ::std::os::raw::c_int = 62708;
pub const IONICON__ROSE_SHARP: ::std::os::raw::c_int = 62709;
pub const IONICON__SAD: ::std::os::raw::c_int = 62710;
pub const IONICON__SAD_OUTLINE: ::std::os::raw::c_int = 62711;
pub const IONICON__SAD_SHARP: ::std::os::raw::c_int = 62712;
pub const IONICON__SAVE: ::std::os::raw::c_int = 62713;
pub const IONICON__SAVE_OUTLINE: ::std::os::raw::c_int = 62714;
pub const IONICON__SAVE_SHARP: ::std::os::raw::c_int = 62715;
pub const IONICON__SCAN: ::std::os::raw::c_int = 62716;
pub const IONICON__SCAN_CIRCLE: ::std::os::raw::c_int = 62717;
pub const IONICON__SCAN_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62718;
pub const IONICON__SCAN_CIRCLE_SHARP: ::std::os::raw::c_int = 62719;
pub const IONICON__SCAN_OUTLINE: ::std::os::raw::c_int = 62720;
pub const IONICON__SCAN_SHARP: ::std::os::raw::c_int = 62721;
pub const IONICON__SCHOOL: ::std::os::raw::c_int = 62722;
pub const IONICON__SCHOOL_OUTLINE: ::std::os::raw::c_int = 62723;
pub const IONICON__SCHOOL_SHARP: ::std::os::raw::c_int = 62724;
pub const IONICON__SEARCH: ::std::os::raw::c_int = 62725;
pub const IONICON__SEARCH_CIRCLE: ::std::os::raw::c_int = 62726;
pub const IONICON__SEARCH_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62727;
pub const IONICON__SEARCH_CIRCLE_SHARP: ::std::os::raw::c_int = 62728;
pub const IONICON__SEARCH_OUTLINE: ::std::os::raw::c_int = 62729;
pub const IONICON__SEARCH_SHARP: ::std::os::raw::c_int = 62730;
pub const IONICON__SEND: ::std::os::raw::c_int = 62731;
pub const IONICON__SEND_OUTLINE: ::std::os::raw::c_int = 62732;
pub const IONICON__SEND_SHARP: ::std::os::raw::c_int = 62733;
pub const IONICON__SERVER: ::std::os::raw::c_int = 62734;
pub const IONICON__SERVER_OUTLINE: ::std::os::raw::c_int = 62735;
pub const IONICON__SERVER_SHARP: ::std::os::raw::c_int = 62736;
pub const IONICON__SETTINGS: ::std::os::raw::c_int = 62737;
pub const IONICON__SETTINGS_OUTLINE: ::std::os::raw::c_int = 62738;
pub const IONICON__SETTINGS_SHARP: ::std::os::raw::c_int = 62739;
pub const IONICON__SHAPES: ::std::os::raw::c_int = 62740;
pub const IONICON__SHAPES_OUTLINE: ::std::os::raw::c_int = 62741;
pub const IONICON__SHAPES_SHARP: ::std::os::raw::c_int = 62742;
pub const IONICON__SHARE: ::std::os::raw::c_int = 62743;
pub const IONICON__SHARE_OUTLINE: ::std::os::raw::c_int = 62744;
pub const IONICON__SHARE_SHARP: ::std::os::raw::c_int = 62745;
pub const IONICON__SHARE_SOCIAL: ::std::os::raw::c_int = 62746;
pub const IONICON__SHARE_SOCIAL_OUTLINE: ::std::os::raw::c_int = 62747;
pub const IONICON__SHARE_SOCIAL_SHARP: ::std::os::raw::c_int = 62748;
pub const IONICON__SHIELD: ::std::os::raw::c_int = 62749;
pub const IONICON__SHIELD_CHECKMARK: ::std::os::raw::c_int = 62750;
pub const IONICON__SHIELD_CHECKMARK_OUTLINE: ::std::os::raw::c_int = 62751;
pub const IONICON__SHIELD_CHECKMARK_SHARP: ::std::os::raw::c_int = 62752;
pub const IONICON__SHIELD_OUTLINE: ::std::os::raw::c_int = 62753;
pub const IONICON__SHIELD_SHARP: ::std::os::raw::c_int = 62754;
pub const IONICON__SHIRT: ::std::os::raw::c_int = 62755;
pub const IONICON__SHIRT_OUTLINE: ::std::os::raw::c_int = 62756;
pub const IONICON__SHIRT_SHARP: ::std::os::raw::c_int = 62757;
pub const IONICON__SHUFFLE: ::std::os::raw::c_int = 62758;
pub const IONICON__SHUFFLE_OUTLINE: ::std::os::raw::c_int = 62759;
pub const IONICON__SHUFFLE_SHARP: ::std::os::raw::c_int = 62760;
pub const IONICON__SKULL: ::std::os::raw::c_int = 62761;
pub const IONICON__SKULL_OUTLINE: ::std::os::raw::c_int = 62762;
pub const IONICON__SKULL_SHARP: ::std::os::raw::c_int = 62763;
pub const IONICON__SNOW: ::std::os::raw::c_int = 62764;
pub const IONICON__SNOW_OUTLINE: ::std::os::raw::c_int = 62765;
pub const IONICON__SNOW_SHARP: ::std::os::raw::c_int = 62766;
pub const IONICON__SPEEDOMETER: ::std::os::raw::c_int = 62767;
pub const IONICON__SPEEDOMETER_OUTLINE: ::std::os::raw::c_int = 62768;
pub const IONICON__SPEEDOMETER_SHARP: ::std::os::raw::c_int = 62769;
pub const IONICON__SQUARE: ::std::os::raw::c_int = 62770;
pub const IONICON__SQUARE_OUTLINE: ::std::os::raw::c_int = 62771;
pub const IONICON__SQUARE_SHARP: ::std::os::raw::c_int = 62772;
pub const IONICON__STAR: ::std::os::raw::c_int = 62773;
pub const IONICON__STAR_HALF: ::std::os::raw::c_int = 62774;
pub const IONICON__STAR_HALF_OUTLINE: ::std::os::raw::c_int = 62775;
pub const IONICON__STAR_HALF_SHARP: ::std::os::raw::c_int = 62776;
pub const IONICON__STAR_OUTLINE: ::std::os::raw::c_int = 62777;
pub const IONICON__STAR_SHARP: ::std::os::raw::c_int = 62778;
pub const IONICON__STATS_CHART: ::std::os::raw::c_int = 62779;
pub const IONICON__STATS_CHART_OUTLINE: ::std::os::raw::c_int = 62780;
pub const IONICON__STATS_CHART_SHARP: ::std::os::raw::c_int = 62781;
pub const IONICON__STOP: ::std::os::raw::c_int = 62782;
pub const IONICON__STOP_CIRCLE: ::std::os::raw::c_int = 62783;
pub const IONICON__STOP_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62784;
pub const IONICON__STOP_CIRCLE_SHARP: ::std::os::raw::c_int = 62785;
pub const IONICON__STOP_OUTLINE: ::std::os::raw::c_int = 62786;
pub const IONICON__STOP_SHARP: ::std::os::raw::c_int = 62787;
pub const IONICON__STOPWATCH: ::std::os::raw::c_int = 62788;
pub const IONICON__STOPWATCH_OUTLINE: ::std::os::raw::c_int = 62789;
pub const IONICON__STOPWATCH_SHARP: ::std::os::raw::c_int = 62790;
pub const IONICON__SUBWAY: ::std::os::raw::c_int = 62791;
pub const IONICON__SUBWAY_OUTLINE: ::std::os::raw::c_int = 62792;
pub const IONICON__SUBWAY_SHARP: ::std::os::raw::c_int = 62793;
pub const IONICON__SUNNY: ::std::os::raw::c_int = 62794;
pub const IONICON__SUNNY_OUTLINE: ::std::os::raw::c_int = 62795;
pub const IONICON__SUNNY_SHARP: ::std::os::raw::c_int = 62796;
pub const IONICON__SWAP_HORIZONTAL: ::std::os::raw::c_int = 62797;
pub const IONICON__SWAP_HORIZONTAL_OUTLINE: ::std::os::raw::c_int = 62798;
pub const IONICON__SWAP_HORIZONTAL_SHARP: ::std::os::raw::c_int = 62799;
pub const IONICON__SWAP_VERTICAL: ::std::os::raw::c_int = 62800;
pub const IONICON__SWAP_VERTICAL_OUTLINE: ::std::os::raw::c_int = 62801;
pub const IONICON__SWAP_VERTICAL_SHARP: ::std::os::raw::c_int = 62802;
pub const IONICON__SYNC: ::std::os::raw::c_int = 62803;
pub const IONICON__SYNC_CIRCLE: ::std::os::raw::c_int = 62804;
pub const IONICON__SYNC_CIRCLE_OUTLINE: ::std::os::raw::c_int = 62805;
pub const IONICON__SYNC_CIRCLE_SHARP: ::std::os::raw::c_int = 62806;
pub const IONICON__SYNC_OUTLINE: ::std::os::raw::c_int = 62807;
pub const IONICON__SYNC_SHARP: ::std::os::raw::c_int = 62808;
pub const IONICON__TABLET_LANDSCAPE: ::std::os::raw::c_int = 62809;
pub const IONICON__TABLET_LANDSCAPE_OUTLINE: ::std::os::raw::c_int = 62810;
pub const IONICON__TABLET_LANDSCAPE_SHARP: ::std::os::raw::c_int = 62811;
pub const IONICON__TABLET_PORTRAIT: ::std::os::raw::c_int = 62812;
pub const IONICON__TABLET_PORTRAIT_OUTLINE: ::std::os::raw::c_int = 62813;
pub const IONICON__TABLET_PORTRAIT_SHARP: ::std::os::raw::c_int = 62814;
pub const IONICON__TENNISBALL: ::std::os::raw::c_int = 62815;
pub const IONICON__TENNISBALL_OUTLINE: ::std::os::raw::c_int = 62816;
pub const IONICON__TENNISBALL_SHARP: ::std::os::raw::c_int = 62817;
pub const IONICON__TERMINAL: ::std::os::raw::c_int = 62818;
pub const IONICON__TERMINAL_OUTLINE: ::std::os::raw::c_int = 62819;
pub const IONICON__TERMINAL_SHARP: ::std::os::raw::c_int = 62820;
pub const IONICON__TEXT: ::std::os::raw::c_int = 62821;
pub const IONICON__TEXT_OUTLINE: ::std::os::raw::c_int = 62822;
pub const IONICON__TEXT_SHARP: ::std::os::raw::c_int = 62823;
pub const IONICON__THERMOMETER: ::std::os::raw::c_int = 62824;
pub const IONICON__THERMOMETER_OUTLINE: ::std::os::raw::c_int = 62825;
pub const IONICON__THERMOMETER_SHARP: ::std::os::raw::c_int = 62826;
pub const IONICON__THUMBS_DOWN: ::std::os::raw::c_int = 62827;
pub const IONICON__THUMBS_DOWN_OUTLINE: ::std::os::raw::c_int = 62828;
pub const IONICON__THUMBS_DOWN_SHARP: ::std::os::raw::c_int = 62829;
pub const IONICON__THUMBS_UP: ::std::os::raw::c_int = 62830;
pub const IONICON__THUMBS_UP_OUTLINE: ::std::os::raw::c_int = 62831;
pub const IONICON__THUMBS_UP_SHARP: ::std::os::raw::c_int = 62832;
pub const IONICON__THUNDERSTORM: ::std::os::raw::c_int = 62833;
pub const IONICON__THUNDERSTORM_OUTLINE: ::std::os::raw::c_int = 62834;
pub const IONICON__THUNDERSTORM_SHARP: ::std::os::raw::c_int = 62835;
pub const IONICON__TIME: ::std::os::raw::c_int = 62836;
pub const IONICON__TIME_OUTLINE: ::std::os::raw::c_int = 62837;
pub const IONICON__TIME_SHARP: ::std::os::raw::c_int = 62838;
pub const IONICON__TIMER: ::std::os::raw::c_int = 62839;
pub const IONICON__TIMER_OUTLINE: ::std::os::raw::c_int = 62840;
pub const IONICON__TIMER_SHARP: ::std::os::raw::c_int = 62841;
pub const IONICON__TODAY: ::std::os::raw::c_int = 62842;
pub const IONICON__TODAY_OUTLINE: ::std::os::raw::c_int = 62843;
pub const IONICON__TODAY_SHARP: ::std::os::raw::c_int = 62844;
pub const IONICON__TOGGLE: ::std::os::raw::c_int = 62845;
pub const IONICON__TOGGLE_OUTLINE: ::std::os::raw::c_int = 62846;
pub const IONICON__TOGGLE_SHARP: ::std::os::raw::c_int = 62847;
pub const IONICON__TRAIL_SIGN: ::std::os::raw::c_int = 62848;
pub const IONICON__TRAIL_SIGN_OUTLINE: ::std::os::raw::c_int = 62849;
pub const IONICON__TRAIL_SIGN_SHARP: ::std::os::raw::c_int = 62850;
pub const IONICON__TRAIN: ::std::os::raw::c_int = 62851;
pub const IONICON__TRAIN_OUTLINE: ::std::os::raw::c_int = 62852;
pub const IONICON__TRAIN_SHARP: ::std::os::raw::c_int = 62853;
pub const IONICON__TRANSGENDER: ::std::os::raw::c_int = 62854;
pub const IONICON__TRANSGENDER_OUTLINE: ::std::os::raw::c_int = 62855;
pub const IONICON__TRANSGENDER_SHARP: ::std::os::raw::c_int = 62856;
pub const IONICON__TRASH: ::std::os::raw::c_int = 62857;
pub const IONICON__TRASH_BIN: ::std::os::raw::c_int = 62858;
pub const IONICON__TRASH_BIN_OUTLINE: ::std::os::raw::c_int = 62859;
pub const IONICON__TRASH_BIN_SHARP: ::std::os::raw::c_int = 62860;
pub const IONICON__TRASH_OUTLINE: ::std::os::raw::c_int = 62861;
pub const IONICON__TRASH_SHARP: ::std::os::raw::c_int = 62862;
pub const IONICON__TRENDING_DOWN: ::std::os::raw::c_int = 62863;
pub const IONICON__TRENDING_DOWN_OUTLINE: ::std::os::raw::c_int = 62864;
pub const IONICON__TRENDING_DOWN_SHARP: ::std::os::raw::c_int = 62865;
pub const IONICON__TRENDING_UP: ::std::os::raw::c_int = 62866;
pub const IONICON__TRENDING_UP_OUTLINE: ::std::os::raw::c_int = 62867;
pub const IONICON__TRENDING_UP_SHARP: ::std::os::raw::c_int = 62868;
pub const IONICON__TRIANGLE: ::std::os::raw::c_int = 62869;
pub const IONICON__TRIANGLE_OUTLINE: ::std::os::raw::c_int = 62870;
pub const IONICON__TRIANGLE_SHARP: ::std::os::raw::c_int = 62871;
pub const IONICON__TROPHY: ::std::os::raw::c_int = 62872;
pub const IONICON__TROPHY_OUTLINE: ::std::os::raw::c_int = 62873;
pub const IONICON__TROPHY_SHARP: ::std::os::raw::c_int = 62874;
pub const IONICON__TV: ::std::os::raw::c_int = 62875;
pub const IONICON__TV_OUTLINE: ::std::os::raw::c_int = 62876;
pub const IONICON__TV_SHARP: ::std::os::raw::c_int = 62877;
pub const IONICON__UMBRELLA: ::std::os::raw::c_int = 62878;
pub const IONICON__UMBRELLA_OUTLINE: ::std::os::raw::c_int = 62879;
pub const IONICON__UMBRELLA_SHARP: ::std::os::raw::c_int = 62880;
pub const IONICON__VIDEOCAM: ::std::os::raw::c_int = 62881;
pub const IONICON__VIDEOCAM_OUTLINE: ::std::os::raw::c_int = 62882;
pub const IONICON__VIDEOCAM_SHARP: ::std::os::raw::c_int = 62883;
pub const IONICON__VOLUME_HIGH: ::std::os::raw::c_int = 62884;
pub const IONICON__VOLUME_HIGH_OUTLINE: ::std::os::raw::c_int = 62885;
pub const IONICON__VOLUME_HIGH_SHARP: ::std::os::raw::c_int = 62886;
pub const IONICON__VOLUME_LOW: ::std::os::raw::c_int = 62887;
pub const IONICON__VOLUME_LOW_OUTLINE: ::std::os::raw::c_int = 62888;
pub const IONICON__VOLUME_LOW_SHARP: ::std::os::raw::c_int = 62889;
pub const IONICON__VOLUME_MEDIUM: ::std::os::raw::c_int = 62890;
pub const IONICON__VOLUME_MEDIUM_OUTLINE: ::std::os::raw::c_int = 62891;
pub const IONICON__VOLUME_MEDIUM_SHARP: ::std::os::raw::c_int = 62892;
pub const IONICON__VOLUME_MUTE: ::std::os::raw::c_int = 62893;
pub const IONICON__VOLUME_MUTE_OUTLINE: ::std::os::raw::c_int = 62894;
pub const IONICON__VOLUME_MUTE_SHARP: ::std::os::raw::c_int = 62895;
pub const IONICON__VOLUME_OFF: ::std::os::raw::c_int = 62896;
pub const IONICON__VOLUME_OFF_OUTLINE: ::std::os::raw::c_int = 62897;
pub const IONICON__VOLUME_OFF_SHARP: ::std::os::raw::c_int = 62898;
pub const IONICON__WALK: ::std::os::raw::c_int = 62899;
pub const IONICON__WALK_OUTLINE: ::std::os::raw::c_int = 62900;
pub const IONICON__WALK_SHARP: ::std::os::raw::c_int = 62901;
pub const IONICON__WALLET: ::std::os::raw::c_int = 62902;
pub const IONICON__WALLET_OUTLINE: ::std::os::raw::c_int = 62903;
pub const IONICON__WALLET_SHARP: ::std::os::raw::c_int = 62904;
pub const IONICON__WARNING: ::std::os::raw::c_int = 62905;
pub const IONICON__WARNING_OUTLINE: ::std::os::raw::c_int = 62906;
pub const IONICON__WARNING_SHARP: ::std::os::raw::c_int = 62907;
pub const IONICON__WATCH: ::std::os::raw::c_int = 62908;
pub const IONICON__WATCH_OUTLINE: ::std::os::raw::c_int = 62909;
pub const IONICON__WATCH_SHARP: ::std::os::raw::c_int = 62910;
pub const IONICON__WATER: ::std::os::raw::c_int = 62911;
pub const IONICON__WATER_OUTLINE: ::std::os::raw::c_int = 62912;
pub const IONICON__WATER_SHARP: ::std::os::raw::c_int = 62913;
pub const IONICON__WIFI: ::std::os::raw::c_int = 62914;
pub const IONICON__WIFI_OUTLINE: ::std::os::raw::c_int = 62915;
pub const IONICON__WIFI_SHARP: ::std::os::raw::c_int = 62916;
pub const IONICON__WINE: ::std::os::raw::c_int = 62917;
pub const IONICON__WINE_OUTLINE: ::std::os::raw::c_int = 62918;
pub const IONICON__WINE_SHARP: ::std::os::raw::c_int = 62919;
pub const IONICON__WOMAN: ::std::os::raw::c_int = 62920;
pub const IONICON__WOMAN_OUTLINE: ::std::os::raw::c_int = 62921;
pub const IONICON__WOMAN_SHARP: ::std::os::raw::c_int = 62922;
pub type _bindgen_ty_6 = ::std::os::raw::c_int;
pub const IONICON_ADD_ON__MOUSE: ::std::os::raw::c_int = 57344;
pub const IONICON_ADD_ON__MOUSE_LEFT: ::std::os::raw::c_int = 57345;
pub const IONICON_ADD_ON__MOUSE_RIGHT: ::std::os::raw::c_int = 57346;
pub const IONICON_ADD_ON__MOUSE_MIDDLE: ::std::os::raw::c_int = 57347;
pub const IONICON_ADD_ON__MOUSE_DOUBLE: ::std::os::raw::c_int = 57348;
pub const IONICON_ADD_ON__CHILDREN_TREE: ::std::os::raw::c_int = 57360;
pub const IONICON_ADD_ON__CREATION_GRAPH: ::std::os::raw::c_int = 57361;
pub const IONICON_ADD_ON__DETAILED_LIST: ::std::os::raw::c_int = 57362;
pub const IONICON_ADD_ON__INHERIT_FROM_PARENT: ::std::os::raw::c_int = 57363;
pub const IONICON_ADD_ON__PROPAGATE_TO_PARENT: ::std::os::raw::c_int = 57364;
pub const IONICON_ADD_ON__ATTACH_TO_PROCESS: ::std::os::raw::c_int = 57365;
pub const IONICON_ADD_ON__CAPTURE_RENDERDOC: ::std::os::raw::c_int = 57366;
pub const IONICON_ADD_ON__TRANSFORM: ::std::os::raw::c_int = 57376;
pub const IONICON_ADD_ON__BOX: ::std::os::raw::c_int = 57377;
pub const IONICON_ADD_ON__BOX_OUTLINE: ::std::os::raw::c_int = 57378;
pub const IONICON_ADD_ON__BOX_DASHED: ::std::os::raw::c_int = 57379;
pub const IONICON_ADD_ON__PUZZLE: ::std::os::raw::c_int = 57380;
pub const IONICON_ADD_ON__PUZZLE_OUTLINE: ::std::os::raw::c_int = 57381;
pub const IONICON_ADD_ON__TETRAHEDRON: ::std::os::raw::c_int = 57382;
pub const IONICON_ADD_ON__PHYSICS_SHAPE: ::std::os::raw::c_int = 57383;
pub const IONICON_ADD_ON__PHYSICS_BODY: ::std::os::raw::c_int = 57384;
pub const IONICON_ADD_ON__PHYSICS_JOINT: ::std::os::raw::c_int = 57385;
pub type _bindgen_ty_7 = ::std::os::raw::c_int;
pub const TM_TT_PROP__TABWELL__LEFT: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__TABWELL__RIGHT: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__TABWELL__TOP: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__TABWELL__BOTTOM: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__TABWELL__BIAS: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__TABWELL__VIEWS: ::std::os::raw::c_int = 5;
pub type _bindgen_ty_8 = ::std::os::raw::c_int;
pub const TM_TT_PROP__TAB_VIEW__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__TAB_VIEW__ROOT_UUID: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__TAB_VIEW__ROOT_TYPE: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__TAB_VIEW__SORT_INDEX: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__TAB_VIEW__SETTINGS_VIEW: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__TAB_VIEW__PINNED: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__TAB_VIEW__SETTINGS_STATE: ::std::os::raw::c_int = 6;
pub type _bindgen_ty_9 = ::std::os::raw::c_int;
pub const TM_TAB_LAYOUT_MAX_TABS_PER_TABWELL: ::std::os::raw::c_int = 3;
pub type _bindgen_ty_10 = ::std::os::raw::c_int;
pub const TM_TAB_LAYOUT_SPLIT_TYPE__NONE: TabLayoutSplitType = 0;
pub const TM_TAB_LAYOUT_SPLIT_TYPE__VERTICAL: TabLayoutSplitType = 1;
pub const TM_TAB_LAYOUT_SPLIT_TYPE__HORIZONTAL: TabLayoutSplitType = 2;
pub type TabLayoutSplitType = ::std::os::raw::c_int;
#[repr(C)]
pub struct TabLayoutT {
    pub split: TabLayoutSplitType,
    pub bias: f32,
    pub __bindgen_anon_1: TabLayoutTBindgenTy1,
}
#[repr(C)]
pub struct TabLayoutTBindgenTy1 {
    pub __bindgen_anon_1: __BindgenUnionField<TabLayoutTBindgenTy1BindgenTy1>,
    pub __bindgen_anon_2: __BindgenUnionField<TabLayoutTBindgenTy1BindgenTy2>,
    pub __bindgen_anon_3: __BindgenUnionField<TabLayoutTBindgenTy1BindgenTy3>,
    pub bindgen_union_field: [u64; 9usize],
}
#[repr(C)]
pub struct TabLayoutTBindgenTy1BindgenTy1 {
    pub tab: [StrhashT; 3usize],
    pub out_tab: [*mut TabI; 3usize],
    pub settings: [*const ::std::os::raw::c_void; 3usize],
}
impl Default for TabLayoutTBindgenTy1BindgenTy1 {
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
pub struct TabLayoutTBindgenTy1BindgenTy2 {
    pub left: *mut TabLayoutT,
    pub right: *mut TabLayoutT,
}
impl Default for TabLayoutTBindgenTy1BindgenTy2 {
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
pub struct TabLayoutTBindgenTy1BindgenTy3 {
    pub top: *mut TabLayoutT,
    pub bottom: *mut TabLayoutT,
}
impl Default for TabLayoutTBindgenTy1BindgenTy3 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for TabLayoutTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for TabLayoutT {
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
pub struct TabLayoutApi {
    pub instantiate_layout: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            tabwell: *mut DockingTabwellO,
            layout: *mut TabLayoutT,
            context: *mut TabCreateContextT,
            reuse_old_tabs: bool,
        ),
    >,
    pub load_and_instantiate_layout: ::std::option::Option<
        unsafe extern "C" fn(
            settings_tt: *mut TheTruthO,
            app_tt: *mut TheTruthO,
            tabwell_id: TtIdT,
            ui: *mut UiO,
            tabwell: *mut DockingTabwellO,
            context: *mut TabCreateContextT,
            restore_roots: bool,
            reuse_old_tabs: bool,
        ),
    >,
    pub save_instantiated_layout: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            root_tabwell: *mut DockingTabwellO,
            only_save_restorable_tabs: bool,
            undo_scope: TtUndoScopeT,
        ) -> TtIdT,
    >,
    pub save_layout: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            layout: *const TabLayoutT,
            only_save_restorable_tabs: bool,
            undo_scope: TtUndoScopeT,
        ) -> TtIdT,
    >,
}
pub type UiModalCustomCallback = ::std::option::Option<
    unsafe extern "C" fn(
        data: *mut ::std::os::raw::c_void,
        ui: *mut UiO,
        uistyle: *mut UiStyleT,
        rect: RectT,
        delta_time: f32,
    ),
>;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct UiModalApi {
    pub message_box: ::std::option::Option<
        unsafe extern "C" fn(
            title: *const ::std::os::raw::c_char,
            text: *const ::std::os::raw::c_char,
            buttons: *mut *const ::std::os::raw::c_char,
            num_buttons: u32,
        ) -> u32,
    >,
    pub message_box_with_checkboxes: ::std::option::Option<
        unsafe extern "C" fn(
            title: *const ::std::os::raw::c_char,
            text: *const ::std::os::raw::c_char,
            checkboxes: *mut *const ::std::os::raw::c_char,
            checkbox_values: *mut bool,
            num_checkboxes: u32,
            buttons: *mut *const ::std::os::raw::c_char,
            num_buttons: u32,
        ) -> u32,
    >,
    pub progress_box: ::std::option::Option<
        unsafe extern "C" fn(
            title: *const ::std::os::raw::c_char,
            text: *const ::std::os::raw::c_char,
            buttons: *mut *const ::std::os::raw::c_char,
            num_buttons: u32,
            callback: ::std::option::Option<
                unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> f32,
            >,
            callback_data: *mut ::std::os::raw::c_void,
        ) -> u32,
    >,
    pub infinite_progress_box: ::std::option::Option<
        unsafe extern "C" fn(
            title: *const ::std::os::raw::c_char,
            text: *const ::std::os::raw::c_char,
            callback: ::std::option::Option<
                unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> f32,
            >,
            callback_data: *mut ::std::os::raw::c_void,
        ),
    >,
    pub textinput_box: ::std::option::Option<
        unsafe extern "C" fn(
            title: *const ::std::os::raw::c_char,
            text: *const ::std::os::raw::c_char,
            buttons: *mut *const ::std::os::raw::c_char,
            num_buttons: u32,
            output_text: *mut ::std::os::raw::c_char,
            output_text_bytes: u32,
        ) -> u32,
    >,
    pub message_box_with_custom_ui_callback: ::std::option::Option<
        unsafe extern "C" fn(
            title: *const ::std::os::raw::c_char,
            text: *const ::std::os::raw::c_char,
            callback: UiModalCustomCallback,
            callback_data: *mut ::std::os::raw::c_void,
            custom_ui_height: f32,
            buttons: *mut *const ::std::os::raw::c_char,
            num_buttons: u32,
        ) -> u32,
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Shape3dApi {
    pub circle: ::std::option::Option<
        unsafe extern "C" fn(w: *mut Vec3T, c: Vec3T, n: Vec3T, r: f32, num: u32),
    >,
    pub arc: ::std::option::Option<
        unsafe extern "C" fn(
            w: *mut Vec3T,
            c: Vec3T,
            n: Vec3T,
            r: f32,
            x: Vec3T,
            start_angle: f32,
            stop_angle: f32,
            num: u32,
        ),
    >,
    pub box_: ::std::option::Option<
        unsafe extern "C" fn(w: *mut Vec3T, c: Vec3T, x: Vec3T, y: Vec3T, z: Vec3T),
    >,
    pub box_ib_n: u32,
    pub _padding_25: [::std::os::raw::c_char; 4usize],
    pub box_ib: *mut u32,
}
impl Default for Shape3dApi {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_SHORTCUT_TYPE_PRESSED: ShortcutType = 0;
pub const TM_SHORTCUT_TYPE_DOWN: ShortcutType = 1;
pub type ShortcutType = ::std::os::raw::c_int;
#[repr(C)]
pub struct ShortcutI {
    pub name: *const ::std::os::raw::c_char,
    pub default_binding: *const ::std::os::raw::c_char,
    pub default_secondary_binding: *const ::std::os::raw::c_char,
    pub tooltip: *const ::std::os::raw::c_char,
    pub type_: ShortcutType,
    pub ignore_modifiers: bool,
    pub ignore_key: bool,
    pub _padding_98: [::std::os::raw::c_char; 2usize],
    pub name_hash: StrhashT,
}
impl Default for ShortcutI {
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
pub struct ShortcutManagerApi {
    pub load_settings:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO, index: TtIdT)>,
    pub accelerator_text: ::std::option::Option<
        unsafe extern "C" fn(shortcut: *const ShortcutI) -> *const ::std::os::raw::c_char,
    >,
    pub is_shortcut_triggered: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, shortcut: *const ShortcutI) -> bool,
    >,
    pub is_shortcut_triggered_in_input: ::std::option::Option<
        unsafe extern "C" fn(input_state: *const UiInputStateT, shortcut: *const ShortcutI) -> bool,
    >,
    pub disable_shortcut_processing: ::std::option::Option<unsafe extern "C" fn()>,
}
pub const TM_TT_PROP__SHORTCUTS_ENTRY__NAME: TM_TT_PROP__SHORTCUTS_ENTRY = 0;
pub const TM_TT_PROP__SHORTCUTS_ENTRY__PRIMARY_KEY: TM_TT_PROP__SHORTCUTS_ENTRY = 1;
pub const TM_TT_PROP__SHORTCUTS_ENTRY__PRIMARY_MODIFIERS: TM_TT_PROP__SHORTCUTS_ENTRY = 2;
pub const TM_TT_PROP__SHORTCUTS_ENTRY__SECONDARY_KEY: TM_TT_PROP__SHORTCUTS_ENTRY = 3;
pub const TM_TT_PROP__SHORTCUTS_ENTRY__SECONDARY_MODIFIERS: TM_TT_PROP__SHORTCUTS_ENTRY = 4;
pub type TM_TT_PROP__SHORTCUTS_ENTRY = ::std::os::raw::c_int;
pub const TM_TT_PROP__SHORTCUTS_INDEX__ENTRIES: TM_TT_PROP__SHORTCUTS_INDEX = 0;
pub type TM_TT_PROP__SHORTCUTS_INDEX = ::std::os::raw::c_int;
pub const TM_TT_PROP__TOOLBAR_SETTINGS__ID: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__TOOLBAR_SETTINGS__CONTAINER: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__TOOLBAR_SETTINGS__ANCHOR: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__TOOLBAR_SETTINGS__ANCHOR_ORDER: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__TOOLBAR_SETTINGS__POSITION_X: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__TOOLBAR_SETTINGS__POSITION_Y: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__TOOLBAR_SETTINGS__WIDTH: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__TOOLBAR_SETTINGS__HEIGHT: ::std::os::raw::c_int = 7;
pub const TM_TT_PROP__TOOLBAR_SETTINGS__DRAW_MODE: ::std::os::raw::c_int = 8;
pub type _bindgen_ty_11 = ::std::os::raw::c_int;
pub const TM_TOOLBAR_CONTAINER_TOP: ToolbarContainer = 0;
pub const TM_TOOLBAR_CONTAINER_BOTTOM: ToolbarContainer = 1;
pub const TM_TOOLBAR_CONTAINER_LEFT: ToolbarContainer = 2;
pub const TM_TOOLBAR_CONTAINER_RIGHT: ToolbarContainer = 3;
pub const TM_TOOLBAR_CONTAINER_OVERLAY: ToolbarContainer = 4;
pub const TM_TOOLBAR_CONTAINER_COUNT: ToolbarContainer = 5;
pub type ToolbarContainer = ::std::os::raw::c_int;
pub const TM_TOOLBAR_ANCHOR_NONE: ToolbarAnchor = 0;
pub const TM_TOOLBAR_ANCHOR_END: ToolbarAnchor = 1;
pub const TM_TOOLBAR_ANCHOR_BEGINNING: ToolbarAnchor = 2;
pub type ToolbarAnchor = ::std::os::raw::c_int;
pub const TM_TOOLBAR_FLAG_FILL: ToolbarFlags = 1;
pub const TM_TOOLBAR_FLAG_OVERLAY_RESIZE_X: ToolbarFlags = 2;
pub const TM_TOOLBAR_FLAG_OVERLAY_RESIZE_Y: ToolbarFlags = 4;
pub const TM_TOOLBAR_FLAG_FORCE_ANCHOR: ToolbarFlags = 8;
pub type ToolbarFlags = ::std::os::raw::c_int;
pub const TM_TOOLBAR_DRAW_MODE_HORIZONTAL: ToolbarDrawMode = 1;
pub const TM_TOOLBAR_DRAW_MODE_VERTICAL: ToolbarDrawMode = 2;
pub const TM_TOOLBAR_DRAW_MODE_WIDGET: ToolbarDrawMode = 4;
pub const TM_TOOLBAR_DRAW_MODE_ALL: ToolbarDrawMode = 7;
pub type ToolbarDrawMode = ::std::os::raw::c_int;
#[repr(C)]
pub struct ToolbarI {
    pub id: u64,
    pub owner: *mut ::std::os::raw::c_void,
    pub display_name: ::std::option::Option<
        unsafe extern "C" fn(toolbar: *mut ToolbarI) -> *const ::std::os::raw::c_char,
    >,
    pub ui: ::std::option::Option<
        unsafe extern "C" fn(
            toolbar: *mut ToolbarI,
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            toolbar_r: RectT,
            draw_mode: ToolbarDrawMode,
        ) -> RectT,
    >,
    pub close: ::std::option::Option<unsafe extern "C" fn(toolbar: *mut ToolbarI)>,
    pub custom_settings:
        ::std::option::Option<unsafe extern "C" fn(toolbar: *mut ToolbarI, tm_ui_o: *mut UiO)>,
    pub default_container: ToolbarContainer,
    pub default_anchor: ToolbarAnchor,
    pub tool_ids: *const StrhashT,
    pub num_tool_ids: u32,
    pub flags: ToolbarFlags,
    pub overlay_default_size: Vec2T,
    pub fill_min_size: f32,
    pub draw_mode_mask: ToolbarDrawMode,
}
impl Default for ToolbarI {
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
pub struct ToolbarApi {
    pub create_state: ::std::option::Option<
        unsafe extern "C" fn(allocator: *mut AllocatorI) -> *mut ToolbarsStateO,
    >,
    pub destroy_state: ::std::option::Option<unsafe extern "C" fn(state: *mut ToolbarsStateO)>,
    pub calculate_rect: ::std::option::Option<
        unsafe extern "C" fn(state: *const ToolbarsStateO, ui: *mut UiO, full_rect: RectT) -> RectT,
    >,
    pub ui: ::std::option::Option<
        unsafe extern "C" fn(
            toolbars: *mut ToolbarI,
            num_toolbars: u32,
            state: *mut ToolbarsStateO,
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            full_rect: RectT,
            settings_tt: *mut TheTruthO,
            settings_obj: TtIdT,
        ),
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct TtfRangeT {
    pub start: u32,
    pub n: u32,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct TtfBakerApi {
    pub bake: ::std::option::Option<
        unsafe extern "C" fn(
            ttf: *const u8,
            font_index: u32,
            font_size: f32,
            font_scales: *const f32,
            num_scales: u32,
            texture_identifier: u32,
            pixels: *mut u8,
            width: u32,
            height: u32,
            ranges: *const TtfRangeT,
            num_ranges: u32,
            extract_glyph_segments: bool,
            allocator: *mut AllocatorI,
            font_bytes: *mut u64,
        ) -> *mut FontT,
    >,
}
pub const TM_UI_COLOR_CHROME_BACKGROUND: UiColor = 0;
pub const TM_UI_COLOR_WINDOW_BACKGROUND: UiColor = 1;
pub const TM_UI_COLOR_WINDOW_SELECTION: UiColor = 2;
pub const TM_UI_COLOR_WINDOW_STATUS_BAR: UiColor = 3;
pub const TM_UI_COLOR_WINDOW_STATUS_BAR_TEXT: UiColor = 4;
pub const TM_UI_COLOR_SELECTION: UiColor = 5;
pub const TM_UI_COLOR_SELECTION_NO_FOCUS: UiColor = 6;
pub const TM_UI_COLOR_SELECTION_HOVER: UiColor = 7;
pub const TM_UI_COLOR_TEXT: UiColor = 8;
pub const TM_UI_COLOR_DISABLED_TEXT: UiColor = 9;
pub const TM_UI_COLOR_SELECTED_TEXT_NO_FOCUS: UiColor = 10;
pub const TM_UI_COLOR_SELECTED_TEXT: UiColor = 11;
pub const TM_UI_COLOR_ERROR_TEXT: UiColor = 12;
pub const TM_UI_COLOR_FILTERED_TEXT: UiColor = 13;
pub const TM_UI_COLOR_PROTOTYPE_RELATION_BASE: UiColor = 14;
pub const TM_UI_COLOR_PROTOTYPE_RELATION_ADDED: UiColor = 14;
pub const TM_UI_COLOR_PROTOTYPE_RELATION_ASSET: UiColor = 15;
pub const TM_UI_COLOR_PROTOTYPE_RELATION_INHERITED: UiColor = 16;
pub const TM_UI_COLOR_PROTOTYPE_RELATION_INSTANTIATED: UiColor = 17;
pub const TM_UI_COLOR_PROTOTYPE_RELATION_REMOVED: UiColor = 18;
pub const TM_UI_COLOR_THIN_LINES: UiColor = 19;
pub const TM_UI_COLOR_THIN_LINES_HOVER: UiColor = 20;
pub const TM_UI_COLOR_ICONS: UiColor = 21;
pub const TM_UI_COLOR_ICONS_HOVER: UiColor = 22;
pub const TM_UI_COLOR_ICONS_ACTIVE: UiColor = 23;
pub const TM_UI_COLOR_ICON_DIRECTORY: UiColor = 24;
pub const TM_UI_COLOR_ICON_ASSET: UiColor = 25;
pub const TM_UI_COLOR_ICON_ENTITY: UiColor = 26;
pub const TM_UI_COLOR_ICON_COMPONENT: UiColor = 27;
pub const TM_UI_COLOR_ICON_IMAGE: UiColor = 28;
pub const TM_UI_COLOR_ICON_MATERIAL: UiColor = 29;
pub const TM_UI_COLOR_BUTTON_BACKGROUND: UiColor = 30;
pub const TM_UI_COLOR_BUTTON_BACKGROUND_HOVER: UiColor = 31;
pub const TM_UI_COLOR_BUTTON_BACKGROUND_ACTIVE: UiColor = 32;
pub const TM_UI_COLOR_BUTTON_FOREGROUND_ACTIVE: UiColor = 33;
pub const TM_UI_COLOR_BUTTON_BORDER: UiColor = 34;
pub const TM_UI_COLOR_BUTTON_BORDER_HOVER: UiColor = 35;
pub const TM_UI_COLOR_BUTTON_BORDER_ACTIVE: UiColor = 36;
pub const TM_UI_COLOR_CONTROL_BACKGROUND: UiColor = 37;
pub const TM_UI_COLOR_CONTROL_BACKGROUND_HOVER: UiColor = 38;
pub const TM_UI_COLOR_CONTROL_BACKGROUND_ACTIVE: UiColor = 39;
pub const TM_UI_COLOR_CONTROL_BORDER: UiColor = 40;
pub const TM_UI_COLOR_CONTROL_BORDER_HOVER: UiColor = 41;
pub const TM_UI_COLOR_CONTROL_BORDER_ACTIVE: UiColor = 42;
pub const TM_UI_COLOR_MENU_BACKGROUND: UiColor = 43;
pub const TM_UI_COLOR_MENU_SELECTED: UiColor = 44;
pub const TM_UI_COLOR_MENUBAR_HOVER: UiColor = 45;
pub const TM_UI_COLOR_TOOLTIP_BACKGROUND: UiColor = 46;
pub const TM_UI_COLOR_TOOLTIP_BORDER: UiColor = 47;
pub const TM_UI_COLOR_TOOLTIP_TEXT: UiColor = 48;
pub const TM_UI_COLOR_VIEWPORT_BACKGROUND: UiColor = 49;
pub const TM_UI_COLOR_VIEWPORT_SELECTION: UiColor = 50;
pub const TM_UI_COLOR_LINK_COLOR: UiColor = 51;
pub const TM_UI_COLOR_LINK_COLOR_HOVER: UiColor = 52;
pub const TM_UI_COLOR_NODE_BACKGROUND: UiColor = 53;
pub const TM_UI_COLOR_NODE_GPU_BACKGROUND: UiColor = 54;
pub const TM_UI_COLOR_NODE_GPU_BACKGROUND_WATERMARK_TEXT: UiColor = 55;
pub const TM_UI_COLOR_NODE_EXECUTION_COLOR: UiColor = 56;
pub const TM_UI_COLOR_GRAPH_GRID_THIN_LINE: UiColor = 57;
pub const TM_UI_COLOR_GRAPH_GRID_THICK_LINE: UiColor = 58;
pub const TM_UI_COLOR_SCROLLBAR_BACKGROUND: UiColor = 59;
pub const TM_UI_COLOR_SCROLLBAR: UiColor = 60;
pub const TM_UI_COLOR_SCROLLBAR_HOVER: UiColor = 61;
pub const TM_UI_COLOR_SCROLLBAR_ACTIVE: UiColor = 62;
pub const TM_UI_COLOR_TOOLBAR_BACKGROUND: UiColor = 63;
pub const TM_UI_COLOR_TOOLBAR_DRAG_HANDLE: UiColor = 64;
pub const TM_UI_COLOR_TOOLBAR_DROP_ZONE: UiColor = 65;
pub const TM_UI_COLOR_TOOLBAR_THIN_LINES: UiColor = 66;
pub const TM_UI_COLOR_TOOLBAR_THIN_LINES_HOVER: UiColor = 67;
pub const TM_UI_COLOR_TOOLBAR_CONTROL_BACKGROUND: UiColor = 68;
pub const TM_UI_COLOR_TOOLBAR_CONTROL_BACKGROUND_HOVER: UiColor = 69;
pub const TM_UI_COLOR_TOOLBAR_CONTROL_BACKGROUND_ACTIVE: UiColor = 70;
pub const TM_UI_COLOR_GIZMO_AXIS_X: UiColor = 71;
pub const TM_UI_COLOR_GIZMO_AXIS_Y: UiColor = 72;
pub const TM_UI_COLOR_GIZMO_AXIS_Z: UiColor = 73;
pub const TM_UI_COLOR_GIZMO_AXIS_SELECTED: UiColor = 74;
pub const TM_UI_COLOR_COUNT: UiColor = 75;
pub type UiColor = ::std::os::raw::c_int;
pub const TM_UI_METRIC_LINE_WIDTH: UiMetric = 0;
pub const TM_UI_METRIC_INPUT_CORNER_RADIUS: UiMetric = 1;
pub const TM_UI_METRIC_WINDOW_TITLE_HEIGHT: UiMetric = 2;
pub const TM_UI_METRIC_WINDOW_MIN_SIZE: UiMetric = 3;
pub const TM_UI_METRIC_MENUBAR_HEIGHT: UiMetric = 4;
pub const TM_UI_METRIC_MENU_ITEM_HEIGHT: UiMetric = 5;
pub const TM_UI_METRIC_SCROLLBAR_WIDTH: UiMetric = 6;
pub const TM_UI_METRIC_SCROLLBAR_PADDING: UiMetric = 7;
pub const TM_UI_METRIC_MARGIN: UiMetric = 8;
pub const TM_UI_METRIC_TEXT_MARGIN: UiMetric = 9;
pub const TM_UI_METRIC_SPLITTER_SIZE: UiMetric = 10;
pub const TM_UI_METRIC_TOOLTIP_HOVER_TIME: UiMetric = 11;
pub const TM_UI_METRIC_TOOLTIP_WIDTH: UiMetric = 12;
pub const TM_UI_METRIC_TREE_ARROW_SCALE: UiMetric = 13;
pub const TM_UI_METRIC_TREE_ICON_WIDTH: UiMetric = 14;
pub const TM_UI_METRIC_TOOLBAR_HEIGHT: UiMetric = 15;
pub const TM_UI_METRIC_TOOLBAR_OUTER_MARGIN: UiMetric = 16;
pub const TM_UI_METRIC_TOOLBAR_INNER_MARGIN: UiMetric = 17;
pub const TM_UI_METRIC_TOOLBAR_WIDGET_MARGIN: UiMetric = 18;
pub const TM_UI_METRIC_COUNT: UiMetric = 19;
pub type UiMetric = ::std::os::raw::c_int;
pub const TM_TT_PROP__UI_THEME__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__UI_THEME__BASED_ON: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__UI_THEME__COLORS: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_12 = ::std::os::raw::c_int;
pub const TM_TT_PROP__UI_THEME_COLOR__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__UI_THEME_COLOR__COLOR: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_13 = ::std::os::raw::c_int;
pub const TM_UI__DEFAULT_THEME__DARK: UiDefaultTheme = 0;
pub const TM_UI__DEFAULT_THEME__LIGHT: UiDefaultTheme = 1;
pub const TM_UI__DEFAULT_THEME__DARK_HIGH_CONTRAST: UiDefaultTheme = 2;
pub const TM_UI__DEFAULT_THEME__LIGHT_HIGH_CONTRAST: UiDefaultTheme = 3;
pub type UiDefaultTheme = ::std::os::raw::c_int;
#[repr(C)]
pub struct UiThemeT {
    pub based_on: UiDefaultTheme,
    pub _padding_281: [::std::os::raw::c_char; 4usize],
    pub tt: *mut TheTruthO,
    pub id: TtIdT,
}
impl Default for UiThemeT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_UI_BUFFER_MAIN: UiBuffer = 0;
pub const TM_UI_BUFFER_OVERLAY: UiBuffer = 1;
pub type UiBuffer = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UiStyleT {
    pub clip: u32,
    pub _padding_301: [::std::os::raw::c_char; 4usize],
    pub font: *const Draw2dFontT,
    pub font_scale: f32,
    pub buffer: UiBuffer,
    pub feather_width: f32,
    pub _padding_314: [::std::os::raw::c_char; 4usize],
}
impl Default for UiStyleT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiScrollbarT {
    pub id: u64,
    pub rect: RectT,
    pub min: f32,
    pub max: f32,
    pub size: f32,
    pub _padding_327: [::std::os::raw::c_char; 4usize],
}
impl Default for UiScrollbarT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_UI_SCROLLBAR_VISIBILITY_WHEN_NEEDED: UiScrollbarVisibility = 0;
pub const TM_UI_SCROLLBAR_VISIBILITY_NEVER: UiScrollbarVisibility = 1;
pub const TM_UI_SCROLLBAR_VISIBILITY_ALWAYS: UiScrollbarVisibility = 2;
pub type UiScrollbarVisibility = ::std::os::raw::c_int;
#[repr(C)]
pub struct UiScrollviewT {
    pub id: u64,
    pub rect: RectT,
    pub canvas: RectT,
    pub visibility_x: UiScrollbarVisibility,
    pub visibility_y: UiScrollbarVisibility,
}
impl Default for UiScrollviewT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiLabelT {
    pub id: u64,
    pub rect: RectT,
    pub icon: u32,
    pub _padding_364: [::std::os::raw::c_char; 4usize],
    pub text: *const ::std::os::raw::c_char,
    pub tooltip: *const ::std::os::raw::c_char,
}
impl Default for UiLabelT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_UI_ALIGN_LEFT: UiAlign = 0;
pub const TM_UI_ALIGN_CENTER: UiAlign = 1;
pub const TM_UI_ALIGN_RIGHT: UiAlign = 2;
pub type UiAlign = ::std::os::raw::c_int;
#[repr(C)]
pub struct UiTextT {
    pub id: u64,
    pub rect: RectT,
    pub text: *const ::std::os::raw::c_char,
    pub tooltip: *const ::std::os::raw::c_char,
    pub color: *const ColorSrgbT,
    pub align: UiAlign,
    pub _padding_388: [::std::os::raw::c_char; 4usize],
}
impl Default for UiTextT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiLinkT {
    pub id: u64,
    pub rect: RectT,
    pub text: *const ::std::os::raw::c_char,
    pub tooltip: *const ::std::os::raw::c_char,
    pub color: *const ColorSrgbT,
    pub align: UiAlign,
    pub _padding_404: [::std::os::raw::c_char; 4usize],
}
impl Default for UiLinkT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiButtonT {
    pub id: u64,
    pub rect: RectT,
    pub visible_rect: RectT,
    pub icon: u32,
    pub _padding_416: [::std::os::raw::c_char; 4usize],
    pub text: *const ::std::os::raw::c_char,
    pub text_color: *const ColorSrgbT,
    pub tooltip: *const ::std::os::raw::c_char,
    pub text_offset_y: f32,
    pub is_disabled: bool,
    pub hide_background: bool,
    pub hide_margins: bool,
    pub _padding_426: [::std::os::raw::c_char; 1usize],
}
impl Default for UiButtonT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiCheckboxT {
    pub id: u64,
    pub rect: RectT,
    pub is_disabled: bool,
    pub _padding_435: [::std::os::raw::c_char; 7usize],
    pub text: *const ::std::os::raw::c_char,
    pub tooltip: *const ::std::os::raw::c_char,
}
impl Default for UiCheckboxT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiRadioT {
    pub id: u64,
    pub rect: RectT,
    pub is_disabled: bool,
    pub _padding_447: [::std::os::raw::c_char; 7usize],
    pub text: *const ::std::os::raw::c_char,
    pub tooltip: *const ::std::os::raw::c_char,
}
impl Default for UiRadioT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiProgressT {
    pub id: u64,
    pub rect: RectT,
    pub text: *const ::std::os::raw::c_char,
}
impl Default for UiProgressT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiSliderT {
    pub id: u64,
    pub rect: RectT,
    pub is_disabled: bool,
    pub _padding_468: [::std::os::raw::c_char; 3usize],
    pub min: f32,
    pub max: f32,
    pub step: f32,
}
impl Default for UiSliderT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct Ui2dSliderT {
    pub id: u64,
    pub rect: RectT,
    pub is_disabled: bool,
    pub is_circular: bool,
    pub _padding_487: [::std::os::raw::c_char; 2usize],
    pub min: Vec2T,
    pub max: Vec2T,
    pub step: f32,
}
impl Default for Ui2dSliderT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_UI_SPINNER__DECIMALS__DEFAULT: ::std::os::raw::c_int = 0;
pub const TM_UI_SPINNER__DECIMALS__0: ::std::os::raw::c_int = 1;
pub const TM_UI_SPINNER__DECIMALS__1: ::std::os::raw::c_int = 2;
pub const TM_UI_SPINNER__DECIMALS__2: ::std::os::raw::c_int = 3;
pub const TM_UI_SPINNER__DECIMALS__3: ::std::os::raw::c_int = 4;
pub const TM_UI_SPINNER__DECIMALS__4: ::std::os::raw::c_int = 5;
pub const TM_UI_SPINNER__DECIMALS__5: ::std::os::raw::c_int = 6;
pub type _bindgen_ty_14 = ::std::os::raw::c_int;
#[repr(C)]
pub struct UiSpinnerT {
    pub id: u64,
    pub rect: RectT,
    pub is_disabled: bool,
    pub _padding_519: [::std::os::raw::c_char; 7usize],
    pub min: f64,
    pub max: f64,
    pub value_per_pixel: f64,
    pub decimals: u32,
    pub _padding_529: [::std::os::raw::c_char; 4usize],
    pub tooltip: *const ::std::os::raw::c_char,
}
impl Default for UiSpinnerT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiDropdownT {
    pub id: u64,
    pub rect: RectT,
    pub is_disabled: bool,
    pub _padding_539: [::std::os::raw::c_char; 7usize],
    pub items: *mut *const ::std::os::raw::c_char,
    pub tooltips: *mut *const ::std::os::raw::c_char,
    pub num_items: u32,
    pub _padding_547: [::std::os::raw::c_char; 4usize],
}
impl Default for UiDropdownT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiTexteditT {
    pub id: u64,
    pub rect: RectT,
    pub is_disabled: bool,
    pub is_password: bool,
    pub _padding_557: [::std::os::raw::c_char; 6usize],
    pub default_text: *const ::std::os::raw::c_char,
    pub select_all_on_mouse_activate: bool,
    pub scroll_to_end: bool,
    pub select_all_on_startup: bool,
    pub _padding_574: [::std::os::raw::c_char; 1usize],
    pub select: UiTexteditTBindgenTy1,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct UiTexteditTBindgenTy1 {
    pub all: bool,
    pub range: bool,
    pub _padding_584: [::std::os::raw::c_char; 2usize],
    pub start: u32,
    pub end: u32,
}
impl Default for UiTexteditT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_UI_MENU_DEFAULT_ID_BASE: ::std::os::raw::c_int = -268435456;
pub type _bindgen_ty_15 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UiMenuItemT {
    pub text: *const ::std::os::raw::c_char,
    pub shortcut: *mut ShortcutI,
    pub accelerator: *const ::std::os::raw::c_char,
    pub tooltip: *const ::std::os::raw::c_char,
    pub item_id: u64,
    pub toggle: *mut bool,
    pub is_checked: bool,
    pub has_submenu: bool,
    pub is_disabled: bool,
    pub is_hidden: bool,
    pub icon: u32,
}
impl Default for UiMenuItemT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiMenubarT {
    pub id: u64,
    pub pos: Vec2T,
    pub width: f32,
    pub padding_left_right: f32,
    pub items: *const UiMenuItemT,
    pub num_items: u32,
    pub _padding_665: [::std::os::raw::c_char; 4usize],
}
impl Default for UiMenubarT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiMenuT {
    pub pos: Vec2T,
    pub items: *const UiMenuItemT,
    pub num_items: u32,
    pub _padding_679: [::std::os::raw::c_char; 4usize],
}
impl Default for UiMenuT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiMenuResultT {
    pub selected_item_id: u64,
    pub selected_text: *const ::std::os::raw::c_char,
    pub highlighted_item_id: u64,
    pub submenu_pos: Vec2T,
}
impl Default for UiMenuResultT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiTreeitemT {
    pub id: u64,
    pub rect: RectT,
    pub text: *const ::std::os::raw::c_char,
}
impl Default for UiTreeitemT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_UI_TAB_PIN_TYPE__NONE: UiTabPinType = 0;
pub const TM_UI_TAB_PIN_TYPE__OBJECT: UiTabPinType = 1;
pub const TM_UI_TAB_PIN_TYPE__TABS: UiTabPinType = 2;
pub const TM_UI_TAB_PIN_TYPE__WINDOWS: UiTabPinType = 3;
pub type UiTabPinType = ::std::os::raw::c_int;
#[repr(C)]
pub struct UiTabbarItemT {
    pub text: *const ::std::os::raw::c_char,
    pub id: u64,
    pub has_close_box: bool,
    pub has_pin: bool,
    pub _padding_736: [::std::os::raw::c_char; 2usize],
    pub pin_type: UiTabPinType,
    pub has_border_color: bool,
    pub _padding_741: [::std::os::raw::c_char; 3usize],
    pub border_color: ColorSrgbT,
    pub disable_drag: bool,
    pub _padding_746: [::std::os::raw::c_char; 3usize],
    pub out_text_rect: RectT,
    pub _padding_750: [::std::os::raw::c_char; 4usize],
    pub icon: u32,
    pub _padding_754: [::std::os::raw::c_char; 4usize],
}
impl Default for UiTabbarItemT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiTabbarT {
    pub id: u64,
    pub rect: RectT,
    pub items: *mut UiTabbarItemT,
    pub num_items: u32,
    pub num_pseudo_items: u32,
    pub _padding_770: [::std::os::raw::c_char; 4usize],
    pub can_drag: bool,
    pub can_drag_off: bool,
    pub is_dragging_external_tab: bool,
    pub _padding_784: [::std::os::raw::c_char; 1usize],
    pub dragged_external_item: *mut *const ::std::os::raw::c_char,
    pub vertical: bool,
    pub _padding_793: [::std::os::raw::c_char; 7usize],
}
impl Default for UiTabbarT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_UI_TABBAR_EVENT_NONE: UiTabbarEvent = 0;
pub const TM_UI_TABBAR_EVENT_SELECT: UiTabbarEvent = 1;
pub const TM_UI_TABBAR_EVENT_CLOSE: UiTabbarEvent = 2;
pub const TM_UI_TABBAR_EVENT_REORDER: UiTabbarEvent = 3;
pub const TM_UI_TABBAR_EVENT_DRAG_OFF: UiTabbarEvent = 4;
pub const TM_UI_TABBAR_EVENT_EXTERNAL_DROP: UiTabbarEvent = 5;
pub const TM_UI_TABBAR_EVENT_CONTEXT_MENU: UiTabbarEvent = 6;
pub const TM_UI_TABBAR_EVENT_PIN: UiTabbarEvent = 7;
pub type UiTabbarEvent = ::std::os::raw::c_int;
#[repr(C)]
pub struct UiTabbarResultT {
    pub event: UiTabbarEvent,
    pub item: u32,
    pub new_position: u32,
    pub drag_offset: Vec2T,
    pub hovered_item: u32,
    pub hover: bool,
    pub _padding_839: [::std::os::raw::c_char; 3usize],
}
impl Default for UiTabbarResultT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiDraggedtabT {
    pub id: u64,
    pub rect: RectT,
    pub text: *const ::std::os::raw::c_char,
    pub has_close_box: bool,
    pub has_pin: bool,
    pub _padding_854: [::std::os::raw::c_char; 2usize],
    pub pin_type: UiTabPinType,
}
impl Default for UiDraggedtabT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiSplitterT {
    pub id: u64,
    pub rect: RectT,
    pub min_size: f32,
    pub _padding_865: [::std::os::raw::c_char; 4usize],
}
impl Default for UiSplitterT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiTitlebarT {
    pub id: u64,
    pub rect: RectT,
    pub has_focus: bool,
    pub is_maximized: bool,
    pub _padding_881: [::std::os::raw::c_char; 6usize],
    pub caption: *const ::std::os::raw::c_char,
    pub border_width: f32,
    pub caption_height: f32,
    pub caption_padding: f32,
    pub _padding_889: [::std::os::raw::c_char; 4usize],
    pub icon_texture: u32,
    pub _padding_893: [::std::os::raw::c_char; 4usize],
}
impl Default for UiTitlebarT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiTitlebarResultT {
    pub close_window: bool,
    pub restore_window: bool,
    pub maximize_window: bool,
    pub minimize_window: bool,
    pub content_r: RectT,
}
impl Default for UiTitlebarResultT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_UI_CURSOR_DEFAULT: UiCursor = 0;
pub const TM_UI_CURSOR_POINTER: UiCursor = 1;
pub const TM_UI_CURSOR_TEXT: UiCursor = 2;
pub const TM_UI_CURSOR_MOVE: UiCursor = 3;
pub const TM_UI_CURSOR_ALL_SCROLL: UiCursor = 4;
pub const TM_UI_CURSOR_COL_RESIZE: UiCursor = 5;
pub const TM_UI_CURSOR_ROW_RESIZE: UiCursor = 6;
pub const TM_UI_CURSOR_EW_RESIZE: UiCursor = 7;
pub const TM_UI_CURSOR_NS_RESIZE: UiCursor = 8;
pub const TM_UI_CURSOR_NESW_RESIZE: UiCursor = 9;
pub const TM_UI_CURSOR_NWSE_RESIZE: UiCursor = 10;
pub const TM_UI_CURSOR_DRAG_AND_DROP: UiCursor = 11;
pub const TM_UI_CURSOR_NONE: UiCursor = 12;
pub type UiCursor = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UiBuffersT {
    pub vbuffer: *mut Draw2dVbufferT,
    pub ibuffers: *mut *mut Draw2dIbufferT,
    pub input: *mut UiInputStateT,
    pub activation: *mut UiActivationT,
    pub metrics: *mut f32,
    pub colors: *mut ColorSrgbT,
    pub allocator: *mut AllocatorI,
}
impl Default for UiBuffersT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_UI_INTERACTION_RESULT_NO_CHANGE: UiInteractionResultT = 0;
pub const TM_UI_INTERACTION_RESULT_TRANSIENT_CHANGE: UiInteractionResultT = 1;
pub const TM_UI_INTERACTION_RESULT_COMMIT: UiInteractionResultT = 2;
pub const TM_UI_INTERACTION_RESULT_ABORT: UiInteractionResultT = 3;
pub type UiInteractionResultT = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UiMouseHelpTextsT {
    pub left_mouse_pressed: *const ::std::os::raw::c_char,
    pub middle_mouse_pressed: *const ::std::os::raw::c_char,
    pub right_mouse_pressed: *const ::std::os::raw::c_char,
    pub double_click: *const ::std::os::raw::c_char,
    pub triple_click: *const ::std::os::raw::c_char,
}
impl Default for UiMouseHelpTextsT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiFontT {
    pub id: StrhashT,
    pub size: u32,
    pub _padding_1024: [::std::os::raw::c_char; 4usize],
    pub font: *mut Draw2dFontT,
}
impl Default for UiFontT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_UI_SHORTCUT_UNDO: UiShortcut = 0;
pub const TM_UI_SHORTCUT_REDO: UiShortcut = 1;
pub const TM_UI_SHORTCUT_CUT: UiShortcut = 2;
pub const TM_UI_SHORTCUT_COPY: UiShortcut = 3;
pub const TM_UI_SHORTCUT_PASTE: UiShortcut = 4;
pub const TM_UI_SHORTCUT_DUPLICATE: UiShortcut = 5;
pub const TM_UI_SHORTCUT_DELETE: UiShortcut = 6;
pub const TM_UI_SHORTCUT_SELECT_ALL: UiShortcut = 7;
pub const TM_UI_SHORTCUT_COUNT: UiShortcut = 8;
pub type UiShortcut = ::std::os::raw::c_int;
#[repr(C)]
pub struct UiAutomationControlT {
    pub role: StrhashT,
    pub title_hash: StrhashT,
    pub title: [::std::os::raw::c_char; 128usize],
    pub rect: RectT,
}
impl Default for UiAutomationControlT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_UI_VISUALIZE_FLAG__AUTOMATION_CONTROLS: UiVisualizeFlag = 1;
pub type UiVisualizeFlag = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct UiApi {
    pub create: ::std::option::Option<unsafe extern "C" fn(a: *mut AllocatorI) -> *mut UiO>,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(i: *mut UiO)>,
    pub clear: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO)>,
    pub release_held_state: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO)>,
    pub set_window_status: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, rect: RectT, has_focus: bool, is_under_cursor: bool),
    >,
    pub window_rect: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> RectT>,
    pub window_has_focus: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> bool>,
    pub window_is_under_cursor: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> bool>,
    pub set_feather_width:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, feather_width: f32)>,
    pub feather_width: ::std::option::Option<unsafe extern "C" fn(ui: *const UiO) -> f32>,
    pub set_scroll_wheel_lines:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, scroll_speed: f32)>,
    pub feed_events: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            events: *const InputEventT,
            count: u32,
            offset: Vec2T,
            scale: Vec2T,
        ),
    >,
    pub feed_external_edit_key: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, key: u32)>,
    pub merge_overlay: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO)>,
    pub begin_overlay_draw_scope:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, id: u64)>,
    pub end_overlay_draw_scope: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, id: u64)>,
    pub drawing_in_overlay: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> bool>,
    pub cursor: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> UiCursor>,
    pub pane: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, style: *const UiStyleT, rect: RectT),
    >,
    pub scrollbar_x: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            c: *const UiScrollbarT,
            scroll: *mut f32,
        ) -> bool,
    >,
    pub scrollbar_y: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            c: *const UiScrollbarT,
            scroll: *mut f32,
        ) -> bool,
    >,
    pub begin_scrollview: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            style: *const UiStyleT,
            c: *const UiScrollviewT,
            scroll_x: *mut f32,
            scroll_y: *mut f32,
            content_rect: *mut RectT,
        ) -> bool,
    >,
    pub end_scrollview: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            scroll_x: *mut f32,
            scroll_y: *mut f32,
            can_respond_to_keyboard: bool,
        ) -> bool,
    >,
    pub label: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, style: *const UiStyleT, c: *const UiLabelT) -> RectT,
    >,
    pub text: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, style: *const UiStyleT, c: *const UiTextT) -> RectT,
    >,
    pub text_metrics: ::std::option::Option<
        unsafe extern "C" fn(style: *const UiStyleT, text: *const ::std::os::raw::c_char) -> RectT,
    >,
    pub wrapped_text: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, style: *const UiStyleT, c: *const UiTextT) -> RectT,
    >,
    pub link: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, style: *const UiStyleT, c: *const UiLinkT) -> bool,
    >,
    pub tooltip: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            style: *const UiStyleT,
            text: *const ::std::os::raw::c_char,
        ),
    >,
    pub button: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, style: *const UiStyleT, c: *const UiButtonT) -> bool,
    >,
    pub pushbutton: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            style: *const UiStyleT,
            c: *const UiButtonT,
            pressed: *mut bool,
        ) -> bool,
    >,
    pub checkbox: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            style: *const UiStyleT,
            c: *const UiCheckboxT,
            checked: *mut bool,
        ) -> bool,
    >,
    pub radio: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            style: *const UiStyleT,
            c: *const UiRadioT,
            checked: bool,
        ) -> bool,
    >,
    pub progress: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            style: *const UiStyleT,
            c: *const UiProgressT,
            fraction: f32,
        ),
    >,
    pub slider: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            style: *const UiStyleT,
            c: *const UiSliderT,
            val: *mut f32,
            initial: *mut f32,
        ) -> UiInteractionResultT,
    >,
    pub slider_2d: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            style: *const UiStyleT,
            c: *const Ui2dSliderT,
            val: *mut Vec2T,
            initial: *mut Vec2T,
        ) -> UiInteractionResultT,
    >,
    pub spinner: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            style: *const UiStyleT,
            c: *const UiSpinnerT,
            val: *mut f64,
            initial: *mut f64,
        ) -> UiInteractionResultT,
    >,
    pub dropdown: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            style: *const UiStyleT,
            c: *const UiDropdownT,
            selected: *mut u32,
        ) -> bool,
    >,
    pub textedit: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            style: *const UiStyleT,
            c: *const UiTexteditT,
            buffer: *mut ::std::os::raw::c_char,
            buffer_bytes: u32,
        ) -> bool,
    >,
    pub multiline_textedit: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            style: *const UiStyleT,
            c: *const UiTexteditT,
            buffer: *mut *mut ::std::os::raw::c_char,
            a: *mut AllocatorI,
            caret_rect: *mut RectT,
        ) -> bool,
    >,
    pub menubar: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            style: *const UiStyleT,
            c: *const UiMenubarT,
        ) -> UiMenuResultT,
    >,
    pub menu: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            style: *const UiStyleT,
            c: *const UiMenuT,
        ) -> UiMenuResultT,
    >,
    pub sort_menu_items:
        ::std::option::Option<unsafe extern "C" fn(items: *mut UiMenuItemT, count: u32)>,
    pub tabbar: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            style: *const UiStyleT,
            c: *const UiTabbarT,
            selected: *mut u32,
        ) -> UiTabbarResultT,
    >,
    pub draggedtab: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, style: *const UiStyleT, c: *const UiDraggedtabT),
    >,
    pub splitter_x: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            c: *const UiSplitterT,
            bias: *mut f32,
            content_left: *mut RectT,
            content_right: *mut RectT,
        ) -> bool,
    >,
    pub splitter_x_rects: ::std::option::Option<
        unsafe extern "C" fn(
            c: *const UiSplitterT,
            bias: f32,
            content_left: *mut RectT,
            content_right: *mut RectT,
        ),
    >,
    pub splitter_y: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            c: *const UiSplitterT,
            bias: *mut f32,
            content_top: *mut RectT,
            content_bottom: *mut RectT,
        ) -> bool,
    >,
    pub splitter_y_rects: ::std::option::Option<
        unsafe extern "C" fn(
            c: *const UiSplitterT,
            bias: f32,
            content_top: *mut RectT,
            content_bottom: *mut RectT,
        ),
    >,
    pub titlebar: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            c: *const UiTitlebarT,
        ) -> UiTitlebarResultT,
    >,
    pub buffers: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> UiBuffersT>,
    pub shortcuts: ::std::option::Option<unsafe extern "C" fn() -> *mut *mut ShortcutI>,
    pub reserve_draw_memory: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO)>,
    pub reserve_draw_memory_detailed: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            primitive_bytes: u32,
            main_index_bytes: u32,
            overlay_index_bytes: u32,
        ),
    >,
    pub make_id: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> u64>,
    pub last_id: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> u64>,
    pub create_fixed_id_range:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, size: u64) -> u64>,
    pub set_id: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, id: u64) -> u64>,
    pub set_cursor: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, cursor: UiCursor)>,
    pub is_hovering:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, r: RectT, clip: u32) -> bool>,
    pub set_responder_chain_root:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, id: u64)>,
    pub begin_responder_scope: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, id: u64)>,
    pub end_responder_scope: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, id: u64)>,
    pub in_responder_chain:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, id: u64) -> bool>,
    pub is_first_responder:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, id: u64) -> bool>,
    pub set_responder_chain: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, id: u64)>,
    pub pop_responder_chain: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, id: u64)>,
    pub responder_chain:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, count: *mut u32) -> *mut u64>,
    pub is_responder_chain_empty: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> bool>,
    pub focus_on_mouse_press:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, r: RectT, id: u64) -> bool>,
    pub consume_key: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, keyboard_item: u32)>,
    pub begin_tab_scope: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, id: u64) -> bool>,
    pub end_tab_scope: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO)>,
    pub focus_on_tab:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, r: RectT, id: u64) -> bool>,
    pub suppress_next_tab_focus: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO)>,
    pub clear_active: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO)>,
    pub set_active: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            id: u64,
            active_data_format: StrhashT,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub is_active: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            id: u64,
            active_data_format: StrhashT,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub lost_active: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            id: u64,
            active_data_format: StrhashT,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub clear_lost_active: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO)>,
    pub save_active_state: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, ta: *mut TempAllocatorI) -> *mut ::std::os::raw::c_void,
    >,
    pub restore_active_state: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, state: *const ::std::os::raw::c_void),
    >,
    pub to_draw_style: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            style: *mut Draw2dStyleT,
            uistyle: *const UiStyleT,
        ) -> *mut Draw2dStyleT,
    >,
    pub set_cache: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, key: u64, carray: *mut ::std::os::raw::c_char),
    >,
    pub lookup_cache: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, key: u64) -> *mut ::std::os::raw::c_char,
    >,
    pub get_cache: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, key: u64, size: u32) -> *mut ::std::os::raw::c_void,
    >,
    pub left_mouse_pressed: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, help_text: *const ::std::os::raw::c_char) -> bool,
    >,
    pub middle_mouse_pressed: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, help_text: *const ::std::os::raw::c_char) -> bool,
    >,
    pub right_mouse_pressed: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, help_text: *const ::std::os::raw::c_char) -> bool,
    >,
    pub left_mouse_released: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, help_text: *const ::std::os::raw::c_char) -> bool,
    >,
    pub middle_mouse_released: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, help_text: *const ::std::os::raw::c_char) -> bool,
    >,
    pub right_mouse_released: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, help_text: *const ::std::os::raw::c_char) -> bool,
    >,
    pub double_click: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, help_text: *const ::std::os::raw::c_char) -> bool,
    >,
    pub triple_click: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, help_text: *const ::std::os::raw::c_char) -> bool,
    >,
    pub get_mouse_help_texts:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> UiMouseHelpTextsT>,
    pub theme: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> UiThemeT>,
    pub get_theme_colors: ::std::option::Option<
        unsafe extern "C" fn(
            theme: UiThemeT,
            theme_colors: *mut *mut ColorSrgbT,
            a: *mut AllocatorI,
        ),
    >,
    pub set_theme: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, theme: UiThemeT)>,
    pub set_theme_colors:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, colors: *const ColorSrgbT)>,
    pub create_custom_theme:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, tt: *mut TheTruthO) -> UiThemeT>,
    pub set_parent_ui:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, parent_ui: *mut UiO)>,
    pub fork: ::std::option::Option<unsafe extern "C" fn(main: *mut UiO) -> *mut UiO>,
    pub join: ::std::option::Option<unsafe extern "C" fn(main: *mut UiO, fork: *mut UiO)>,
    pub merge_render_buffers:
        ::std::option::Option<unsafe extern "C" fn(main: *mut UiO, fork: *mut UiO)>,
    pub main_ui: ::std::option::Option<unsafe extern "C" fn(ui: *const UiO) -> *mut UiO>,
    pub add_font: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            font_id: StrhashT,
            size: u32,
            font: *const FontT,
        ) -> *const Draw2dFontT,
    >,
    pub font: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, font_id: StrhashT, size: u32) -> UiFontT,
    >,
    pub default_style: ::std::option::Option<unsafe extern "C" fn(ui: *const UiO) -> UiStyleT>,
    pub create_truth_types: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
    pub register_control: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            role: StrhashT,
            title: *const ::std::os::raw::c_char,
            rect: RectT,
        ),
    >,
    pub find_control: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            role: StrhashT,
            title: *const ::std::os::raw::c_char,
        ) -> RectT,
    >,
    pub automation_controls: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, ta: *mut TempAllocatorI) -> *mut UiAutomationControlT,
    >,
    pub mouse_move: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, pos: Vec2T)>,
    pub mouse_button_state:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, mouse_item: u32, down: bool)>,
    pub keyboard_key_state:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, keyboard_item: u32, down: bool)>,
    pub text_input: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, text: *const ::std::os::raw::c_char),
    >,
    pub visualize_flag:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, flag: UiVisualizeFlag) -> bool>,
    pub set_visualize_flag:
        ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, flag: UiVisualizeFlag, on: bool)>,
    pub visualize: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO)>,
}
#[repr(C)]
pub struct UiActivationT {
    pub active: u64,
    pub hover: u64,
    pub hover_in_overlay: bool,
    pub _padding_35: [::std::os::raw::c_char; 7usize],
    pub next_hover: u64,
    pub next_hover_in_overlay: bool,
    pub _padding_38: [::std::os::raw::c_char; 7usize],
    pub sub_hover: u64,
    pub next_sub_hover: u64,
    pub active_mouse_pos: Vec2T,
    pub hover_time: f32,
    pub _padding_47: [::std::os::raw::c_char; 4usize],
    pub hover_at_time: f64,
    pub tooltip: u64,
    pub tooltip_position: Vec2T,
    pub tooltip_time: f64,
    pub next_hover_window: u64,
    pub hover_window: u64,
    pub next_hover_scrollview: u64,
    pub hover_scrollview: u64,
    pub menu_level: u32,
    pub _padding_86: [::std::os::raw::c_char; 4usize],
}
impl Default for UiActivationT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_UI_EDIT_KEY_NONE: UiEditKey = 0;
pub const TM_UI_EDIT_KEY_LEFT: UiEditKey = 1;
pub const TM_UI_EDIT_KEY_RIGHT: UiEditKey = 2;
pub const TM_UI_EDIT_KEY_UP: UiEditKey = 3;
pub const TM_UI_EDIT_KEY_DOWN: UiEditKey = 4;
pub const TM_UI_EDIT_KEY_TAB: UiEditKey = 5;
pub const TM_UI_EDIT_KEY_ENTER: UiEditKey = 6;
pub const TM_UI_EDIT_KEY_END: UiEditKey = 7;
pub const TM_UI_EDIT_KEY_HOME: UiEditKey = 8;
pub const TM_UI_EDIT_KEY_DELETE: UiEditKey = 9;
pub const TM_UI_EDIT_KEY_BACKSPACE: UiEditKey = 10;
pub const TM_UI_EDIT_KEY_ESCAPE: UiEditKey = 11;
pub const TM_UI_EDIT_KEY_SELECT_ALL: UiEditKey = 12;
pub const TM_UI_EDIT_KEY_CUT: UiEditKey = 13;
pub const TM_UI_EDIT_KEY_COPY: UiEditKey = 14;
pub const TM_UI_EDIT_KEY_PASTE: UiEditKey = 15;
pub const TM_UI_EDIT_KEY_DUPLICATE: UiEditKey = 16;
pub const TM_UI_EDIT_KEY_UNDO: UiEditKey = 17;
pub const TM_UI_EDIT_KEY_REDO: UiEditKey = 18;
pub const TM_UI_EDIT_KEY_COUNT: UiEditKey = 19;
pub type UiEditKey = ::std::os::raw::c_int;
pub const TM_UI_MAX_TEXT_INPUT: ::std::os::raw::c_int = 256;
pub type _bindgen_ty_16 = ::std::os::raw::c_int;
pub const TM_UI_MODIFIERS_NONE: ::std::os::raw::c_int = 0;
pub const TM_UI_MODIFIERS_SHIFT: ::std::os::raw::c_int = 1;
pub const TM_UI_MODIFIERS_ALT: ::std::os::raw::c_int = 2;
pub const TM_UI_MODIFIERS_SHIFT_ALT: ::std::os::raw::c_int = 3;
pub const TM_UI_MODIFIERS_CTRL: ::std::os::raw::c_int = 4;
pub const TM_UI_MODIFIERS_SHIFT_CTRL: ::std::os::raw::c_int = 5;
pub const TM_UI_MODIFIERS_ALT_CTRL: ::std::os::raw::c_int = 6;
pub const TM_UI_MODIFIERS_SHIFT_ALT_CTRL: ::std::os::raw::c_int = 7;
pub type _bindgen_ty_17 = ::std::os::raw::c_int;
#[repr(C)]
pub struct UiInputStateT {
    pub time: f64,
    pub scale: Vec2T,
    pub offset: Vec2T,
    pub mouse_pos: Vec2T,
    pub mouse_move: Vec2T,
    pub mouse_wheel: f32,
    pub left_mouse_pressed: bool,
    pub left_mouse_released: bool,
    pub left_mouse_is_down: bool,
    pub right_mouse_pressed: bool,
    pub right_mouse_released: bool,
    pub right_mouse_is_down: bool,
    pub middle_mouse_pressed: bool,
    pub middle_mouse_released: bool,
    pub middle_mouse_is_down: bool,
    pub back_mouse_pressed: bool,
    pub back_mouse_released: bool,
    pub back_mouse_is_down: bool,
    pub forward_mouse_pressed: bool,
    pub forward_mouse_released: bool,
    pub forward_mouse_is_down: bool,
    pub _padding_166: [::std::os::raw::c_char; 5usize],
    pub left_mouse_pressed_at_time: f64,
    pub mouse_move_at_time: f64,
    pub double_click: bool,
    pub triple_click: bool,
    pub pen_pressed: bool,
    pub pen_released: bool,
    pub pen_is_down: bool,
    pub touch_pressed: bool,
    pub touch_released: bool,
    pub touch_is_down: bool,
    pub pressure: f32,
    pub pen_erase: bool,
    pub _padding_194: [::std::os::raw::c_char; 3usize],
    pub key_is_down: *mut bool,
    pub key_pressed: *mut bool,
    pub key_released: *mut bool,
    pub key_repeated: *mut bool,
    pub modifiers: u32,
    pub edit_key_pressed: [bool; 19usize],
    pub _padding_210: [::std::os::raw::c_char; 1usize],
    pub num_text_input: u32,
    pub text_input: [u32; 256usize],
    pub _padding_215: [::std::os::raw::c_char; 4usize],
}
impl Default for UiInputStateT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_UI_ACTIVE_DATA_BYTES: ::std::os::raw::c_int = 16384;
pub type _bindgen_ty_18 = ::std::os::raw::c_int;
pub const TM_UI_ICON__NONE: UiIcon = 0;
pub const TM_UI_ICON__PLAY: UiIcon = 1;
pub const TM_UI_ICON__PAUSE: UiIcon = 2;
pub const TM_UI_ICON__RESTART: UiIcon = 3;
pub const TM_UI_ICON__STOP: UiIcon = 4;
pub const TM_UI_ICON__VOLUME: UiIcon = 5;
pub const TM_UI_ICON__PITCH: UiIcon = 6;
pub const TM_UI_ICON__PAN: UiIcon = 7;
pub const TM_UI_ICON__PIN: UiIcon = 8;
pub const TM_UI_ICON__WINDOW: UiIcon = 9;
pub const TM_UI_ICON__TAB: UiIcon = 10;
pub const TM_UI_ICON__SHUFFLE: UiIcon = 11;
pub const TM_UI_ICON__TRIANGLE_LEFT: UiIcon = 12;
pub const TM_UI_ICON__TRIANGLE_RIGHT: UiIcon = 13;
pub const TM_UI_ICON__TRIANGLE_UP: UiIcon = 14;
pub const TM_UI_ICON__TRIANGLE_DOWN: UiIcon = 15;
pub const TM_UI_ICON__OPTIONS: UiIcon = 16;
pub const TM_UI_ICON__NULL: UiIcon = 17;
pub const TM_UI_ICON__ARROW_LEFT: UiIcon = 18;
pub const TM_UI_ICON__ARROW_RIGHT: UiIcon = 19;
pub const TM_UI_ICON__ARROW_UP: UiIcon = 20;
pub const TM_UI_ICON__ARROW_DOWN: UiIcon = 21;
pub const TM_UI_ICON__GEAR: UiIcon = 22;
pub const TM_UI_ICON__BULB: UiIcon = 23;
pub const TM_UI_ICON__CAMERA: UiIcon = 24;
pub const TM_UI_ICON__LIGHT: UiIcon = 25;
pub const TM_UI_ICON__LOCAL: UiIcon = 26;
pub const TM_UI_ICON__WORLD: UiIcon = 27;
pub const TM_UI_ICON__LOCK: UiIcon = 28;
pub const TM_UI_ICON__UNLOCK: UiIcon = 29;
pub const TM_UI_ICON__ANGLE: UiIcon = 30;
pub const TM_UI_ICON__PREV: UiIcon = 31;
pub const TM_UI_ICON__NEXT: UiIcon = 32;
pub const TM_UI_ICON__PIVOT_FIRST: UiIcon = 33;
pub const TM_UI_ICON__PIVOT_LAST: UiIcon = 34;
pub const TM_UI_ICON__PIVOT_CENTER: UiIcon = 35;
pub const TM_UI_ICON__ALIGN: UiIcon = 36;
pub const TM_UI_ICON__FILTER: UiIcon = 37;
pub const TM_UI_ICON__SLEEP: UiIcon = 38;
pub const TM_UI_ICON__EYE: UiIcon = 39;
pub const TM_UI_ICON__EYE_OFF: UiIcon = 40;
pub const TM_UI_ICON__SAVE: UiIcon = 41;
pub const TM_UI_ICON__REFRESH: UiIcon = 42;
pub const TM_UI_ICON__GRID: UiIcon = 43;
pub const TM_UI_ICON__LIST: UiIcon = 44;
pub const TM_UI_ICON__DETAILS: UiIcon = 45;
pub const TM_UI_ICON__REMOVE: UiIcon = 46;
pub const TM_UI_ICON__ADD: UiIcon = 47;
pub const TM_UI_ICON__SORT: UiIcon = 48;
pub const TM_UI_ICON__CHECKMARK: UiIcon = 49;
pub const TM_UI_ICON__CLOSE: UiIcon = 50;
pub const TM_UI_ICON__IMAGE: UiIcon = 51;
pub const TM_UI_ICON__MATERIAL: UiIcon = 52;
pub const TM_UI_ICON__MOUSE_LEFT: UiIcon = 53;
pub const TM_UI_ICON__MOUSE_MIDDLE: UiIcon = 54;
pub const TM_UI_ICON__MOUSE_RIGHT: UiIcon = 55;
pub const TM_UI_ICON__MOUSE_DOUBLE: UiIcon = 56;
pub const TM_UI_ICON__BUG: UiIcon = 57;
pub const TM_UI_ICON__COPY: UiIcon = 58;
pub const TM_UI_ICON__DOCUMENT: UiIcon = 59;
pub const TM_UI_ICON__TRASH_BIN: UiIcon = 60;
pub const TM_UI_ICON__C: UiIcon = 61;
pub const TM_UI_ICON__NOTIFICATIONS: UiIcon = 62;
pub const TM_UI_ICON__NOTIFICATIONS_OFF: UiIcon = 63;
pub const TM_UI_ICON__LOGIN: UiIcon = 64;
pub const TM_UI_ICON__LOGOUT: UiIcon = 65;
pub const TM_UI_ICON__FOLDER: UiIcon = 66;
pub const TM_UI_ICON__FOLDER_OPEN: UiIcon = 67;
pub const TM_UI_ICON__COMPASS: UiIcon = 68;
pub const TM_UI_ICON__COLOR_WAND: UiIcon = 69;
pub const TM_UI_ICON__CHILDREN_TREE: UiIcon = 70;
pub const TM_UI_ICON__CREATION_GRAPH: UiIcon = 71;
pub const TM_UI_ICON__INHERIT_FROM_PARENT: UiIcon = 72;
pub const TM_UI_ICON__PROPAGATE_TO_PARENT: UiIcon = 73;
pub const TM_UI_ICON__ATTACH_TO_PROCESS: UiIcon = 74;
pub const TM_UI_ICON__CAPTURE_RENDERDOC: UiIcon = 75;
pub const TM_UI_ICON__TRANSFORM: UiIcon = 76;
pub const TM_UI_ICON__ENTITY: UiIcon = 77;
pub const TM_UI_ICON__ENTITY_OUTLINE: UiIcon = 78;
pub const TM_UI_ICON__ENTITY_DASHED: UiIcon = 79;
pub const TM_UI_ICON__COMPONENT: UiIcon = 80;
pub const TM_UI_ICON__COMPONENT_OUTLINE: UiIcon = 81;
pub const TM_UI_ICON__RENDER_COMPONENT: UiIcon = 82;
pub const TM_UI_ICON__PHYSICS_SHAPE: UiIcon = 83;
pub const TM_UI_ICON__PHYSICS_BODY: UiIcon = 84;
pub const TM_UI_ICON__PHYSICS_JOINT: UiIcon = 85;
pub const TM_UI_ICON__SEARCH: UiIcon = 86;
pub const TM_UI_ICON__OPEN: UiIcon = 87;
pub type UiIcon = ::std::os::raw::c_int;
#[repr(C)]
pub struct UiIconLabelT {
    pub id: u64,
    pub rect: RectT,
    pub icon: UiIcon,
    pub _padding_115: [::std::os::raw::c_char; 4usize],
    pub tooltip: *const ::std::os::raw::c_char,
}
impl Default for UiIconLabelT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiIconTextT {
    pub id: u64,
    pub rect: RectT,
    pub icon: UiIcon,
    pub _padding_125: [::std::os::raw::c_char; 4usize],
    pub tooltip: *const ::std::os::raw::c_char,
    pub color: *const ColorSrgbT,
    pub align: u32,
    pub _padding_129: [::std::os::raw::c_char; 4usize],
}
impl Default for UiIconTextT {
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
pub struct UiIconApi {
    pub label: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, style: *const UiStyleT, c: *const UiIconLabelT) -> RectT,
    >,
    pub text: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, style: *const UiStyleT, c: *const UiIconTextT) -> RectT,
    >,
    pub metrics: ::std::option::Option<
        unsafe extern "C" fn(ui: *mut UiO, font_scale: f32, icon: UiIcon) -> RectT,
    >,
    pub codepoint: ::std::option::Option<unsafe extern "C" fn(icon: UiIcon) -> u32>,
    pub glyph: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO, icon: UiIcon) -> u16>,
    pub draw: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            style: *const UiStyleT,
            color: *mut ColorSrgbT,
            pos: Vec2T,
            icon: UiIcon,
        ) -> f32,
    >,
    pub title:
        ::std::option::Option<unsafe extern "C" fn(icon: UiIcon) -> *const ::std::os::raw::c_char>,
}
#[repr(C)]
pub struct FontProviderT {
    pub font_id: StrhashT,
    pub font_size: u32,
    pub _padding_36: [::std::os::raw::c_char; 4usize],
    pub descriptor: *const FontDescriptorT,
}
impl Default for FontProviderT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type FontProviderF =
    ::std::option::Option<unsafe extern "C" fn(ta: *mut TempAllocatorI) -> *const FontProviderT>;
#[repr(C)]
pub struct FontRequestGlyphsT {
    pub font_id: StrhashT,
    pub num_ranges: u32,
    pub _padding_77: [::std::os::raw::c_char; 4usize],
    pub ranges: *const TtfRangeT,
}
impl Default for FontRequestGlyphsT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct AddedFontT {
    pub font_identifier: StrhashT,
    pub font_size: u32,
    pub _padding_95: [::std::os::raw::c_char; 4usize],
    pub font: *const FontT,
}
impl Default for AddedFontT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const TM_UI_CANVAS_SCREEN_SPACE: UiCanvasType = 0;
pub const TM_UI_CANVAS_WORLD_SPACE: UiCanvasType = 1;
pub type UiCanvasType = ::std::os::raw::c_int;
#[repr(C)]
pub struct UiCanvasT {
    pub entity_id: u64,
    pub gpu_picking: *mut GpuPickingO,
    pub type_: UiCanvasType,
    pub dpi_scale_factor: f32,
    pub rect: RectT,
    pub transform: TransformT,
    pub cam_view: Mat44T,
    pub cam_projection: Mat44T,
}
impl Default for UiCanvasT {
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
pub struct UiRendererApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            res_buf: *mut RendererResourceCommandBufferO,
            shader_repository: *mut ShaderRepositoryO,
            allocator: *mut AllocatorI,
            device_affinity: u32,
        ) -> *mut UiRendererO,
    >,
    pub destroy: ::std::option::Option<
        unsafe extern "C" fn(
            ui_renderer: *mut UiRendererO,
            res_buf: *mut RendererResourceCommandBufferO,
        ),
    >,
    pub render: ::std::option::Option<
        unsafe extern "C" fn(
            ui_renderer: *mut UiRendererO,
            ui: *mut UiO,
            canvas: *const UiCanvasT,
            sort_key: u64,
            res_buf: *mut RendererResourceCommandBufferO,
            cmd_buf: *mut RendererCommandBufferO,
            color_space: *const ColorSpaceDescT,
        ),
    >,
    pub default_font: ::std::option::Option<
        unsafe extern "C" fn(
            ui_renderer: *mut UiRendererO,
            font_size: u32,
            window_dpi: f32,
            res_buf: *mut RendererResourceCommandBufferO,
        ) -> *const FontT,
    >,
    pub custom_font: ::std::option::Option<
        unsafe extern "C" fn(
            ui_renderer: *mut UiRendererO,
            font_id: StrhashT,
            desc: *const FontDescriptorT,
            font_size: u32,
            window_dpi: f32,
            res_buf: *mut RendererResourceCommandBufferO,
        ) -> *const FontT,
    >,
    pub add_all_font_providers: ::std::option::Option<
        unsafe extern "C" fn(
            ui_renderer: *mut UiRendererO,
            window_dpi: f32,
            res_buf: *mut RendererResourceCommandBufferO,
            ta: *mut TempAllocatorI,
        ) -> *mut AddedFontT,
    >,
    pub allocate_image_slot:
        ::std::option::Option<unsafe extern "C" fn(ui_renderer: *mut UiRendererO) -> u32>,
    pub set_image: ::std::option::Option<
        unsafe extern "C" fn(
            ui_renderer: *mut UiRendererO,
            slot: u32,
            image_handle: RendererHandleT,
        ),
    >,
}
#[repr(C)]
pub struct UiTreeT {
    pub ud: *mut ::std::os::raw::c_void,
    pub root: u64,
    pub item_height: f32,
    pub rect: RectT,
    pub _padding_26: [::std::os::raw::c_char; 4usize],
    pub node_ui: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            item_id: u64,
            rect: RectT,
            ud: *mut ::std::os::raw::c_void,
            node: u64,
            parent: u64,
            child_idx: u64,
        ),
    >,
    pub node_children: ::std::option::Option<
        unsafe extern "C" fn(
            ud: *mut ::std::os::raw::c_void,
            node: u64,
            children: *mut u64,
            n: u32,
        ) -> u32,
    >,
}
impl Default for UiTreeT {
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
pub struct UiTreeItemMetricsT {
    pub have_been_filled: bool,
    pub _padding_45: [::std::os::raw::c_char; 7usize],
    pub tooltip_hover_text: *const ::std::os::raw::c_char,
    pub toolbar_ui_elements: *mut u64,
    pub toolbar_element_count: u32,
    pub dont_auto_expand: bool,
    pub _padding_52: [::std::os::raw::c_char; 3usize],
}
impl Default for UiTreeItemMetricsT {
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
pub struct UiTreeStateT {
    pub expanded: *mut SetT,
    pub selected: *mut SetT,
    pub total_height: f32,
    pub scroll_y: f32,
}
impl Default for UiTreeStateT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct UiTreeItemStateT {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
}
impl UiTreeItemStateT {
    #[inline]
    pub fn can_expand(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_can_expand(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn expanded(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_expanded(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn can_select(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_can_select(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn selected(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_selected(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
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
    pub fn new_bitfield_1(
        can_expand: bool,
        expanded: bool,
        can_select: bool,
        selected: bool,
        padding: u8,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let can_expand: u8 = unsafe { ::std::mem::transmute(can_expand) };
            can_expand as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let expanded: u8 = unsafe { ::std::mem::transmute(expanded) };
            expanded as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let can_select: u8 = unsafe { ::std::mem::transmute(can_select) };
            can_select as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let selected: u8 = unsafe { ::std::mem::transmute(selected) };
            selected as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let padding: u8 = unsafe { ::std::mem::transmute(padding) };
            padding as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
pub struct UiTreeItemResT {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
    pub _padding_109: [::std::os::raw::c_char; 2usize],
    pub item_rect: RectT,
    pub _padding_113: [::std::os::raw::c_char; 4usize],
    pub item_id: u64,
}
impl Default for UiTreeItemResT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl UiTreeItemResT {
    #[inline]
    pub fn expanded(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_expanded(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn expanded_changed(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_expanded_changed(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn selected(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_selected(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn selected_changed(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_selected_changed(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn clicked(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_clicked(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn double_clicked(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_double_clicked(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn visible(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_visible(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn deselect_others(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_deselect_others(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn select_range(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_select_range(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hovered(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_hovered(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn padding(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 6u8) as u8) }
    }
    #[inline]
    pub fn set_padding(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(10usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        expanded: bool,
        expanded_changed: bool,
        selected: bool,
        selected_changed: bool,
        clicked: bool,
        double_clicked: bool,
        visible: bool,
        deselect_others: bool,
        select_range: bool,
        hovered: bool,
        padding: u8,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let expanded: u8 = unsafe { ::std::mem::transmute(expanded) };
            expanded as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let expanded_changed: u8 = unsafe { ::std::mem::transmute(expanded_changed) };
            expanded_changed as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let selected: u8 = unsafe { ::std::mem::transmute(selected) };
            selected as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let selected_changed: u8 = unsafe { ::std::mem::transmute(selected_changed) };
            selected_changed as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let clicked: u8 = unsafe { ::std::mem::transmute(clicked) };
            clicked as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let double_clicked: u8 = unsafe { ::std::mem::transmute(double_clicked) };
            double_clicked as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let visible: u8 = unsafe { ::std::mem::transmute(visible) };
            visible as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let deselect_others: u8 = unsafe { ::std::mem::transmute(deselect_others) };
            deselect_others as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let select_range: u8 = unsafe { ::std::mem::transmute(select_range) };
            select_range as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let hovered: u8 = unsafe { ::std::mem::transmute(hovered) };
            hovered as u64
        });
        __bindgen_bitfield_unit.set(10usize, 6u8, {
            let padding: u8 = unsafe { ::std::mem::transmute(padding) };
            padding as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct UiTreeApi {
    pub tree: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            tree: *const UiTreeT,
            state: *mut UiTreeStateT,
        ),
    >,
    pub tree_item: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            rect: RectT,
            state: UiTreeItemStateT,
            tree_has_focus: bool,
            metrics: *const UiTreeItemMetricsT,
            ui_id: u64,
        ) -> UiTreeItemResT,
    >,
    pub tree_item_with_sets: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            rect: RectT,
            key: u64,
            expanded_set: *mut SetT,
            selected_set: *mut SetT,
            tree_has_focus: bool,
            ui_id: u64,
        ) -> UiTreeItemResT,
    >,
    pub tree_item_with_metrics: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            rect: RectT,
            key: u64,
            expanded_set: *mut SetT,
            selected_set: *mut SetT,
            tree_has_focus: bool,
            metrics: *const UiTreeItemMetricsT,
            ui_id: u64,
        ) -> UiTreeItemResT,
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct GpuPickingO {
    pub _address: u8,
}

// Extensions generated by machinery-generator

use const_cstr::{const_cstr, ConstCStr};

use crate::foundation::VersionT;

use crate::foundation::*;
use crate::plugins::entity::*;
use crate::plugins::renderer::*;
use crate::plugins::shader_system::*;
use crate::the_machinery::TabCreateContextT;

impl UiClipboardApi {
    pub unsafe fn cut(
        &self,
        tt: *mut TheTruthO,
        objects: *const TtIdT,
        count: u32,
        undo_stack: *mut UndoStackI,
    ) {
        self.cut.unwrap()(tt, objects, count, undo_stack)
    }

    pub unsafe fn copy(
        &self,
        tt: *mut TheTruthO,
        objects: *const TtIdT,
        count: u32,
        undo_stack: *mut UndoStackI,
    ) {
        self.copy.unwrap()(tt, objects, count, undo_stack)
    }

    pub unsafe fn empty(&self, tt: *mut TheTruthO, undo_scope: TtUndoScopeT) {
        self.empty.unwrap()(tt, undo_scope)
    }

    pub unsafe fn cut_objects(
        &self,
        tt: *const TheTruthO,
        ta: *mut TempAllocatorI,
    ) -> *const TtIdT {
        self.cut_objects.unwrap()(tt, ta)
    }

    pub unsafe fn copied_objects(
        &self,
        tt: *const TheTruthO,
        ta: *mut TempAllocatorI,
    ) -> *const TtIdT {
        self.copied_objects.unwrap()(tt, ta)
    }

    pub unsafe fn truth(&self) -> *const TheTruthO {
        self.truth.unwrap()()
    }
}

impl crate::Api for UiClipboardApi {
    const NAME: ConstCStr = const_cstr!("tm_ui_clipboard_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl DockingApi {
    pub unsafe fn create_truth_types(&self, tt: *mut TheTruthO) {
        self.create_truth_types.unwrap()(tt)
    }

    pub unsafe fn add_ui(&self, ui: *mut UiO, r: RectT) {
        self.add_ui.unwrap()(ui, r)
    }

    pub unsafe fn remove_ui(&self, ui: *mut UiO) {
        self.remove_ui.unwrap()(ui)
    }

    pub unsafe fn add_workspace(&self, ui: *mut UiO, name: *const ::std::os::raw::c_char) {
        self.add_workspace.unwrap()(ui, name)
    }

    pub unsafe fn close_focused_workspace(&self, ui: *mut UiO) -> bool {
        self.close_focused_workspace.unwrap()(ui)
    }

    pub unsafe fn num_workspaces(&self, ui: *mut UiO) -> u32 {
        self.num_workspaces.unwrap()(ui)
    }

    pub unsafe fn current_workspace(&self, ui: *mut UiO) -> u32 {
        self.current_workspace.unwrap()(ui)
    }

    pub unsafe fn workspace_name(&self, ui: *mut UiO, idx: u32) -> *const ::std::os::raw::c_char {
        self.workspace_name.unwrap()(ui, idx)
    }

    pub unsafe fn set_workspace_name(
        &self,
        ui: *mut UiO,
        idx: u32,
        name: *const ::std::os::raw::c_char,
    ) {
        self.set_workspace_name.unwrap()(ui, idx, name)
    }

    pub unsafe fn workspace_icon(&self, ui: *mut UiO, idx: u32) -> u32 {
        self.workspace_icon.unwrap()(ui, idx)
    }

    pub unsafe fn set_workspace_icon(&self, ui: *mut UiO, idx: u32, icon: u32) {
        self.set_workspace_icon.unwrap()(ui, idx, icon)
    }

    pub unsafe fn workspace_root(&self, ui: *mut UiO, idx: u32) -> *mut DockingTabwellO {
        self.workspace_root.unwrap()(ui, idx)
    }

    pub unsafe fn hot_reload(&self) {
        self.hot_reload.unwrap()()
    }

    pub unsafe fn root(&self, ui: *mut UiO) -> *mut DockingTabwellO {
        self.root.unwrap()(ui)
    }

    pub unsafe fn split_tabwell(
        &self,
        parent: *mut DockingTabwellO,
        split: DockingTabwellSplit,
        bias: f32,
        sibling: *mut *mut DockingTabwellO,
    ) -> *mut DockingTabwellO {
        self.split_tabwell.unwrap()(parent, split, bias, sibling)
    }

    pub unsafe fn add_tab(&self, ui: *mut UiO, tabwell: *mut DockingTabwellO, tab: *mut TabI) {
        self.add_tab.unwrap()(ui, tabwell, tab)
    }

    pub unsafe fn remove_tab(&self, tab: *mut TabI) {
        self.remove_tab.unwrap()(tab)
    }

    pub unsafe fn move_tab(&self, tab: *mut TabI, ui: *mut UiO, tabwell: *mut DockingTabwellO) {
        self.move_tab.unwrap()(tab, ui, tabwell)
    }

    pub unsafe fn clear(&self) {
        self.clear.unwrap()()
    }

    pub unsafe fn feed_events(
        &self,
        events: *const InputEventT,
        count: u32,
        args: *mut DockingFeedEventsArgsT,
    ) {
        self.feed_events.unwrap()(events, count, args)
    }

    pub unsafe fn ui(
        &self,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        ui_renderer: *mut UiRendererO,
        rect: RectT,
        ui_has_focus: bool,
        dropped_files: *const OsDroppedFileT,
        num_dropped_files: u32,
        context: *mut TabCreateContextT,
        settings_tt: *mut TheTruthO,
        settings_obj: TtIdT,
        default_padding: f32,
        workspaces_mode: DockingWorkspacesBarMode,
    ) {
        self.ui.unwrap()(
            ui,
            uistyle,
            ui_renderer,
            rect,
            ui_has_focus,
            dropped_files,
            num_dropped_files,
            context,
            settings_tt,
            settings_obj,
            default_padding,
            workspaces_mode,
        )
    }

    pub unsafe fn is_dragging_tab(&self, pos: *mut Vec2T) -> *mut TabI {
        self.is_dragging_tab.unwrap()(pos)
    }

    pub unsafe fn should_create_new_ui_for_tab(
        &self,
        create_context: *mut TabCreateContextT,
        rect: *mut RectT,
    ) -> *mut TabI {
        self.should_create_new_ui_for_tab.unwrap()(create_context, rect)
    }

    pub unsafe fn focused_tabwell(&self, ui: *mut UiO) -> *mut DockingTabwellO {
        self.focused_tabwell.unwrap()(ui)
    }

    pub unsafe fn focused_tab(&self, ui: *mut UiO) -> *mut TabI {
        self.focused_tab.unwrap()(ui)
    }

    pub unsafe fn tab_has_focus(&self, tab: *mut TabI) -> bool {
        self.tab_has_focus.unwrap()(tab)
    }

    pub unsafe fn tabwell_info(
        &self,
        tw: *mut DockingTabwellO,
        ta: *mut TempAllocatorI,
    ) -> DockingTabwellInfoT {
        self.tabwell_info.unwrap()(tw, ta)
    }

    pub unsafe fn tab_info(
        &self,
        tabs: *mut DockingTabInfoT,
        n: u32,
        filter_ui: *mut UiO,
        filter_visible: bool,
    ) -> u32 {
        self.tab_info.unwrap()(tabs, n, filter_ui, filter_visible)
    }

    pub unsafe fn root_history(&self, tab: *mut TabI, n: *mut u32) -> *mut TabVtRootT {
        self.root_history.unwrap()(tab, n)
    }

    pub unsafe fn clear_root_history_with_greater_counter(&self, tab: *mut TabI, counter: u64) {
        self.clear_root_history_with_greater_counter.unwrap()(tab, counter)
    }

    pub unsafe fn tab_ui(&self, tab: *mut TabI) -> *mut UiO {
        self.tab_ui.unwrap()(tab)
    }

    pub unsafe fn can_remove_ui(&self, ui: *mut UiO) -> bool {
        self.can_remove_ui.unwrap()(ui)
    }

    pub unsafe fn set_focus_tab(&self, ui: *mut UiO, tab: *mut TabI) {
        self.set_focus_tab.unwrap()(ui, tab)
    }

    pub unsafe fn close_focused_tab(&self, ui: *mut UiO) -> bool {
        self.close_focused_tab.unwrap()(ui)
    }

    pub unsafe fn close_all_tabs_and_workspaces(&self, ui: *mut UiO) {
        self.close_all_tabs_and_workspaces.unwrap()(ui)
    }

    pub unsafe fn close_all_tabs_in_workspace(&self, ui: *mut UiO) {
        self.close_all_tabs_in_workspace.unwrap()(ui)
    }

    pub unsafe fn send_focus_event(
        &self,
        from: *mut TabI,
        event: TabFocusEvent,
        tt: *mut TheTruthO,
        object: TtIdT,
        selection: *const TtIdT,
        selection_n: u32,
    ) {
        self.send_focus_event.unwrap()(from, event, tt, object, selection, selection_n)
    }

    pub unsafe fn destroy_truth(&self, tt: *mut TheTruthO) {
        self.destroy_truth.unwrap()(tt)
    }

    pub unsafe fn find_tab(
        &self,
        type_name_hash: StrhashT,
        opt: *const DockingFindTabOptT,
    ) -> DockingFindTabT {
        self.find_tab.unwrap()(type_name_hash, opt)
    }

    pub unsafe fn find_tabs(
        &self,
        type_name_hash: StrhashT,
        ta: *mut TempAllocatorI,
    ) -> *mut DockingFindTabT {
        self.find_tabs.unwrap()(type_name_hash, ta)
    }

    pub unsafe fn pin_type(&self, tab: *mut TabI) -> u32 {
        self.pin_type.unwrap()(tab)
    }

    pub unsafe fn pin_object(&self, tab: *mut TabI, tt: *mut TheTruthO, root: TtIdT) {
        self.pin_object.unwrap()(tab, tt, root)
    }

    pub unsafe fn toolbars_state(&self, tab: *mut TabI) -> *mut ToolbarsStateO {
        self.toolbars_state.unwrap()(tab)
    }

    pub unsafe fn clear_cached_ui(&self, tab: *mut TabI) {
        self.clear_cached_ui.unwrap()(tab)
    }

    pub unsafe fn tab_vt(&self, name_hash: StrhashT) -> *mut TabVt {
        self.tab_vt.unwrap()(name_hash)
    }

    pub unsafe fn set_workspace_context_menu(&self, menu: DockingWorkspaceContextMenuF) {
        self.set_workspace_context_menu.unwrap()(menu)
    }

    pub unsafe fn set_current_workspace(&self, ui: *mut UiO, workspace_idx: u32) {
        self.set_current_workspace.unwrap()(ui, workspace_idx)
    }
}

impl crate::Api for DockingApi {
    const NAME: ConstCStr = const_cstr!("tm_docking_api");
    const VERSION: VersionT = VersionT {
        major: 2u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl UiDragApi {
    pub unsafe fn start_dragging(&self, tt: *mut TheTruthO, object: TtIdT) {
        self.start_dragging.unwrap()(tt, object)
    }

    pub unsafe fn start_dragging_multiple_objects(
        &self,
        tt: *mut TheTruthO,
        objects: *const TtIdT,
        object_count: u64,
    ) {
        self.start_dragging_multiple_objects.unwrap()(tt, objects, object_count)
    }

    pub unsafe fn single_dragged_object(&self, tt: *const TheTruthO) -> TtIdT {
        self.single_dragged_object.unwrap()(tt)
    }

    pub unsafe fn first_dragged_object(&self, tt: *const TheTruthO) -> TtIdT {
        self.first_dragged_object.unwrap()(tt)
    }

    pub unsafe fn all_dragged_objects(
        &self,
        tt: *const TheTruthO,
        ta: *mut TempAllocatorI,
    ) -> *mut TtIdT {
        self.all_dragged_objects.unwrap()(tt, ta)
    }

    pub unsafe fn dragged_truth(&self) -> *const TheTruthO {
        self.dragged_truth.unwrap()()
    }

    pub unsafe fn stop_dragging(&self) {
        self.stop_dragging.unwrap()()
    }
}

impl crate::Api for UiDragApi {
    const NAME: ConstCStr = const_cstr!("tm_ui_drag_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl Draw2dApi {
    pub unsafe fn add_clip_rect(&self, vbuffer: *mut Draw2dVbufferT, clip: RectT) -> u32 {
        self.add_clip_rect.unwrap()(vbuffer, clip)
    }

    pub unsafe fn add_sub_clip_rect(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        parent: u32,
        clip: RectT,
    ) -> u32 {
        self.add_sub_clip_rect.unwrap()(vbuffer, parent, clip)
    }

    pub unsafe fn clip_rect(&self, vbuffer: *mut Draw2dVbufferT, clip: u32) -> RectT {
        self.clip_rect.unwrap()(vbuffer, clip)
    }

    pub unsafe fn font_memory(&self, font: *const FontT) -> u32 {
        self.font_memory.unwrap()(font)
    }

    pub unsafe fn add_font(&self, vbuffer: *mut Draw2dVbufferT, font: *const FontT) -> Draw2dFontT {
        self.add_font.unwrap()(vbuffer, font)
    }

    pub unsafe fn fill_rect(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        r: RectT,
    ) {
        self.fill_rect.unwrap()(vbuffer, ibuffer, style, r)
    }

    pub unsafe fn fill_rect_feathered(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        r: RectT,
    ) {
        self.fill_rect_feathered.unwrap()(vbuffer, ibuffer, style, r)
    }

    pub unsafe fn stroke_rect(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        r: RectT,
    ) {
        self.stroke_rect.unwrap()(vbuffer, ibuffer, style, r)
    }

    pub unsafe fn textured_rect(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        r: RectT,
        texture: u32,
        uv: RectT,
        native_color_space: bool,
    ) {
        self.textured_rect.unwrap()(vbuffer, ibuffer, style, r, texture, uv, native_color_space)
    }

    pub unsafe fn aux_rect(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        r: RectT,
        aux_data_type: u32,
        aux_data: *const ::std::os::raw::c_void,
        aux_data_size: u32,
    ) {
        self.aux_rect.unwrap()(
            vbuffer,
            ibuffer,
            style,
            r,
            aux_data_type,
            aux_data,
            aux_data_size,
        )
    }

    pub unsafe fn gradient_rect(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        r: RectT,
        colors: *mut ColorSrgbT,
    ) {
        self.gradient_rect.unwrap()(vbuffer, ibuffer, style, r, colors)
    }

    pub unsafe fn fill_rounded_rect(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        r: RectT,
        radius: f32,
    ) {
        self.fill_rounded_rect.unwrap()(vbuffer, ibuffer, style, r, radius)
    }

    pub unsafe fn fill_rounded_rect_per_corner(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        r: RectT,
        radiuses: Vec4T,
    ) {
        self.fill_rounded_rect_per_corner.unwrap()(vbuffer, ibuffer, style, r, radiuses)
    }

    pub unsafe fn stroke_rounded_rect(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        r: RectT,
        radius: f32,
    ) {
        self.stroke_rounded_rect.unwrap()(vbuffer, ibuffer, style, r, radius)
    }

    pub unsafe fn stroke_rounded_rect_per_corner(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        r: RectT,
        radiuses: Vec4T,
    ) {
        self.stroke_rounded_rect_per_corner.unwrap()(vbuffer, ibuffer, style, r, radiuses)
    }

    pub unsafe fn fill_circle(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        pos: Vec2T,
        radius: f32,
    ) {
        self.fill_circle.unwrap()(vbuffer, ibuffer, style, pos, radius)
    }

    pub unsafe fn stroke_circle(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        pos: Vec2T,
        radius: f32,
    ) {
        self.stroke_circle.unwrap()(vbuffer, ibuffer, style, pos, radius)
    }

    pub unsafe fn fill_triangles(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        points: *const Vec2T,
        num_points: u32,
        indices: *const u32,
        num_indices: u32,
    ) {
        self.fill_triangles.unwrap()(
            vbuffer,
            ibuffer,
            style,
            points,
            num_points,
            indices,
            num_indices,
        )
    }

    pub unsafe fn fill_convex_polyline(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        points: *const Vec2T,
        num_points: u32,
    ) {
        self.fill_convex_polyline.unwrap()(vbuffer, ibuffer, style, points, num_points)
    }

    pub unsafe fn stroke_polyline(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        points: *const Vec2T,
        num_points: u32,
        closed: bool,
    ) {
        self.stroke_polyline.unwrap()(vbuffer, ibuffer, style, points, num_points, closed)
    }

    pub unsafe fn stroke_polyline_widths(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        points: *const Vec2T,
        widths: *const f32,
        num_points: u32,
        closed: bool,
    ) {
        self.stroke_polyline_widths.unwrap()(
            vbuffer, ibuffer, style, points, widths, num_points, closed,
        )
    }

    pub unsafe fn bezier_path(
        &self,
        curve: *const Vec2T,
        num_curve_points: u32,
        tolerance: f32,
        ta: *mut TempAllocatorI,
        num_points: *mut u32,
        allocated_points: *mut u32,
    ) -> *mut Vec2T {
        self.bezier_path.unwrap()(
            curve,
            num_curve_points,
            tolerance,
            ta,
            num_points,
            allocated_points,
        )
    }

    pub unsafe fn fill_convex_bezier(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        curve: *const Vec2T,
        num_curve_points: u32,
        tolerance: f32,
        ta: *mut TempAllocatorI,
    ) {
        self.fill_convex_bezier.unwrap()(
            vbuffer,
            ibuffer,
            style,
            curve,
            num_curve_points,
            tolerance,
            ta,
        )
    }

    pub unsafe fn stroke_bezier(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        curve: *const Vec2T,
        num_curve_points: u32,
        closed: bool,
        tolerance: f32,
        ta: *mut TempAllocatorI,
    ) {
        self.stroke_bezier.unwrap()(
            vbuffer,
            ibuffer,
            style,
            curve,
            num_curve_points,
            closed,
            tolerance,
            ta,
        )
    }

    pub unsafe fn draw_glyphs(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        pos: Vec2T,
        glyphs: *const u16,
        num_glyphs: u32,
    ) -> RectT {
        self.draw_glyphs.unwrap()(vbuffer, ibuffer, style, pos, glyphs, num_glyphs)
    }

    pub unsafe fn draw_glyphs_rotated(
        &self,
        vbuffer: *mut Draw2dVbufferT,
        ibuffer: *mut Draw2dIbufferT,
        style: *const Draw2dStyleT,
        pos: Vec2T,
        glyphs: *const u16,
        num_glyphs: u32,
        x: Vec2T,
        y: Vec2T,
    ) -> f32 {
        self.draw_glyphs_rotated.unwrap()(vbuffer, ibuffer, style, pos, glyphs, num_glyphs, x, y)
    }

    pub unsafe fn merge_buffers(
        &self,
        to_vbuffer: *mut Draw2dVbufferT,
        to_ibuffers: *mut *mut Draw2dIbufferT,
        from_vbuffer: *const Draw2dVbufferT,
        from_ibuffers: *const *mut Draw2dIbufferT,
        num_ibuffers: u32,
    ) {
        self.merge_buffers.unwrap()(
            to_vbuffer,
            to_ibuffers,
            from_vbuffer,
            from_ibuffers,
            num_ibuffers,
        )
    }
}

impl crate::Api for Draw2dApi {
    const NAME: ConstCStr = const_cstr!("tm_draw2d_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl FontApi {
    pub unsafe fn glyphs(
        &self,
        font: *const FontT,
        glyphs: *mut u16,
        codepoints: *const u32,
        size: u32,
    ) {
        self.glyphs.unwrap()(font, glyphs, codepoints, size)
    }

    pub unsafe fn metrics(
        &self,
        font: *const FontT,
        font_scale: f32,
        glyphs: *const u16,
        num_glyphs: u32,
    ) -> RectT {
        self.metrics.unwrap()(font, font_scale, glyphs, num_glyphs)
    }

    pub unsafe fn metrics_array(
        &self,
        font: *const FontT,
        font_scale: f32,
        pos_x: *mut f32,
        rects: *mut RectT,
        glyphs: *const u16,
        num_glyphs: u32,
    ) -> RectT {
        self.metrics_array.unwrap()(font, font_scale, pos_x, rects, glyphs, num_glyphs)
    }

    pub unsafe fn line(
        &self,
        font: *const FontT,
        font_scale: f32,
        line_width: *mut f32,
        glyphs: *const u16,
        num_glyphs: u32,
    ) -> u32 {
        self.line.unwrap()(font, font_scale, line_width, glyphs, num_glyphs)
    }

    pub unsafe fn rescale_font(&self, font: *mut FontT, scale_factor: f32) {
        self.rescale_font.unwrap()(font, scale_factor)
    }

    pub unsafe fn glyph_set_from_scale(
        &self,
        font: *const FontT,
        font_scale: f32,
        new_font_scale: *mut f32,
    ) -> u32 {
        self.glyph_set_from_scale.unwrap()(font, font_scale, new_font_scale)
    }

    pub unsafe fn first_glyph_of_set(&self, font: *const FontT, set: u32) -> *const FontGlyphT {
        self.first_glyph_of_set.unwrap()(font, set)
    }
}

impl crate::Api for FontApi {
    const NAME: ConstCStr = const_cstr!("tm_font_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl FontLibraryApi {
    pub unsafe fn create(&self, allocator: *mut AllocatorI) -> *mut FontLibraryT {
        self.create.unwrap()(allocator)
    }

    pub unsafe fn destroy(&self, lib: *mut FontLibraryT) {
        self.destroy.unwrap()(lib)
    }

    pub unsafe fn add_ttf(
        &self,
        lib: *mut FontLibraryT,
        ttf: *const FontDescriptorT,
    ) -> TtfHandleT {
        self.add_ttf.unwrap()(lib, ttf)
    }

    pub unsafe fn lookup(
        &self,
        lib: *const FontLibraryT,
        ttf: TtfHandleT,
        size: u32,
        display_dpi: f32,
    ) -> *const FontT {
        self.lookup.unwrap()(lib, ttf, size, display_dpi)
    }

    pub unsafe fn generate(
        &self,
        lib: *mut FontLibraryT,
        tm_res_buf_api: *mut RendererResourceCommandBufferApi,
        res_buf: *mut RendererResourceCommandBufferO,
        device_affinity: u32,
        font_texture_id: u32,
        ttf: TtfHandleT,
        size: u32,
        display_dpi: f32,
        scales: *const f32,
        n_scales: u32,
        texture_handle: *mut RendererHandleT,
    ) -> *const FontT {
        self.generate.unwrap()(
            lib,
            tm_res_buf_api,
            res_buf,
            device_affinity,
            font_texture_id,
            ttf,
            size,
            display_dpi,
            scales,
            n_scales,
            texture_handle,
        )
    }
}

impl crate::Api for FontLibraryApi {
    const NAME: ConstCStr = const_cstr!("tm_font_library_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 1u32,
        patch: 0u32,
    };
}

impl GizmoApi {
    pub unsafe fn move_(
        &self,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        pbuf: *mut PrimitiveDrawerBufferT,
        vbuf: *mut PrimitiveDrawerBufferT,
        camera: *const CameraT,
        viewport: RectT,
        tm: *const TransformT,
        local: *mut TransformT,
        settings: *const GizmoMoveSettingsT,
        grid_settings: *mut GizmoGridSettingsT,
    ) -> GizmoMoveResult {
        self.move_.unwrap()(
            ui,
            uistyle,
            pbuf,
            vbuf,
            camera,
            viewport,
            tm,
            local,
            settings,
            grid_settings,
        )
    }

    pub unsafe fn rotate(
        &self,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        pbuf: *mut PrimitiveDrawerBufferT,
        vbuf: *mut PrimitiveDrawerBufferT,
        camera: *const CameraT,
        viewport: RectT,
        tm: *const TransformT,
        local: *mut TransformT,
        settings: *const GizmoRotateSettingsT,
    ) -> GizmoMoveResult {
        self.rotate.unwrap()(
            ui, uistyle, pbuf, vbuf, camera, viewport, tm, local, settings,
        )
    }

    pub unsafe fn scale(
        &self,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        pbuf: *mut PrimitiveDrawerBufferT,
        vbuf: *mut PrimitiveDrawerBufferT,
        camera: *const CameraT,
        viewport: RectT,
        tm: *const TransformT,
        local: *mut TransformT,
        settings: *const GizmoScaleSettingsT,
    ) -> GizmoMoveResult {
        self.scale.unwrap()(
            ui, uistyle, pbuf, vbuf, camera, viewport, tm, local, settings,
        )
    }
}

impl crate::Api for GizmoApi {
    const NAME: ConstCStr = const_cstr!("tm_gizmo_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl TabLayoutApi {
    pub unsafe fn instantiate_layout(
        &self,
        ui: *mut UiO,
        tabwell: *mut DockingTabwellO,
        layout: *mut TabLayoutT,
        context: *mut TabCreateContextT,
        reuse_old_tabs: bool,
    ) {
        self.instantiate_layout.unwrap()(ui, tabwell, layout, context, reuse_old_tabs)
    }

    pub unsafe fn load_and_instantiate_layout(
        &self,
        settings_tt: *mut TheTruthO,
        app_tt: *mut TheTruthO,
        tabwell_id: TtIdT,
        ui: *mut UiO,
        tabwell: *mut DockingTabwellO,
        context: *mut TabCreateContextT,
        restore_roots: bool,
        reuse_old_tabs: bool,
    ) {
        self.load_and_instantiate_layout.unwrap()(
            settings_tt,
            app_tt,
            tabwell_id,
            ui,
            tabwell,
            context,
            restore_roots,
            reuse_old_tabs,
        )
    }

    pub unsafe fn save_instantiated_layout(
        &self,
        tt: *mut TheTruthO,
        root_tabwell: *mut DockingTabwellO,
        only_save_restorable_tabs: bool,
        undo_scope: TtUndoScopeT,
    ) -> TtIdT {
        self.save_instantiated_layout.unwrap()(
            tt,
            root_tabwell,
            only_save_restorable_tabs,
            undo_scope,
        )
    }

    pub unsafe fn save_layout(
        &self,
        tt: *mut TheTruthO,
        layout: *const TabLayoutT,
        only_save_restorable_tabs: bool,
        undo_scope: TtUndoScopeT,
    ) -> TtIdT {
        self.save_layout.unwrap()(tt, layout, only_save_restorable_tabs, undo_scope)
    }
}

impl crate::Api for TabLayoutApi {
    const NAME: ConstCStr = const_cstr!("tm_tab_layout_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl UiModalApi {
    pub unsafe fn message_box(
        &self,
        title: *const ::std::os::raw::c_char,
        text: *const ::std::os::raw::c_char,
        buttons: *mut *const ::std::os::raw::c_char,
        num_buttons: u32,
    ) -> u32 {
        self.message_box.unwrap()(title, text, buttons, num_buttons)
    }

    pub unsafe fn message_box_with_checkboxes(
        &self,
        title: *const ::std::os::raw::c_char,
        text: *const ::std::os::raw::c_char,
        checkboxes: *mut *const ::std::os::raw::c_char,
        checkbox_values: *mut bool,
        num_checkboxes: u32,
        buttons: *mut *const ::std::os::raw::c_char,
        num_buttons: u32,
    ) -> u32 {
        self.message_box_with_checkboxes.unwrap()(
            title,
            text,
            checkboxes,
            checkbox_values,
            num_checkboxes,
            buttons,
            num_buttons,
        )
    }

    pub unsafe fn progress_box(
        &self,
        title: *const ::std::os::raw::c_char,
        text: *const ::std::os::raw::c_char,
        buttons: *mut *const ::std::os::raw::c_char,
        num_buttons: u32,
        callback: ::std::option::Option<
            unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> f32,
        >,
        callback_data: *mut ::std::os::raw::c_void,
    ) -> u32 {
        self.progress_box.unwrap()(title, text, buttons, num_buttons, callback, callback_data)
    }

    pub unsafe fn infinite_progress_box(
        &self,
        title: *const ::std::os::raw::c_char,
        text: *const ::std::os::raw::c_char,
        callback: ::std::option::Option<
            unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> f32,
        >,
        callback_data: *mut ::std::os::raw::c_void,
    ) {
        self.infinite_progress_box.unwrap()(title, text, callback, callback_data)
    }

    pub unsafe fn textinput_box(
        &self,
        title: *const ::std::os::raw::c_char,
        text: *const ::std::os::raw::c_char,
        buttons: *mut *const ::std::os::raw::c_char,
        num_buttons: u32,
        output_text: *mut ::std::os::raw::c_char,
        output_text_bytes: u32,
    ) -> u32 {
        self.textinput_box.unwrap()(
            title,
            text,
            buttons,
            num_buttons,
            output_text,
            output_text_bytes,
        )
    }

    pub unsafe fn message_box_with_custom_ui_callback(
        &self,
        title: *const ::std::os::raw::c_char,
        text: *const ::std::os::raw::c_char,
        callback: UiModalCustomCallback,
        callback_data: *mut ::std::os::raw::c_void,
        custom_ui_height: f32,
        buttons: *mut *const ::std::os::raw::c_char,
        num_buttons: u32,
    ) -> u32 {
        self.message_box_with_custom_ui_callback.unwrap()(
            title,
            text,
            callback,
            callback_data,
            custom_ui_height,
            buttons,
            num_buttons,
        )
    }
}

impl crate::Api for UiModalApi {
    const NAME: ConstCStr = const_cstr!("tm_ui_modal_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 1u32,
        patch: 0u32,
    };
}

impl Shape3dApi {
    pub unsafe fn circle(&self, w: *mut Vec3T, c: Vec3T, n: Vec3T, r: f32, num: u32) {
        self.circle.unwrap()(w, c, n, r, num)
    }

    pub unsafe fn arc(
        &self,
        w: *mut Vec3T,
        c: Vec3T,
        n: Vec3T,
        r: f32,
        x: Vec3T,
        start_angle: f32,
        stop_angle: f32,
        num: u32,
    ) {
        self.arc.unwrap()(w, c, n, r, x, start_angle, stop_angle, num)
    }

    pub unsafe fn box_(&self, w: *mut Vec3T, c: Vec3T, x: Vec3T, y: Vec3T, z: Vec3T) {
        self.box_.unwrap()(w, c, x, y, z)
    }
}

impl crate::Api for Shape3dApi {
    const NAME: ConstCStr = const_cstr!("tm_shape3d_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl ShortcutManagerApi {
    pub unsafe fn load_settings(&self, tt: *mut TheTruthO, index: TtIdT) {
        self.load_settings.unwrap()(tt, index)
    }

    pub unsafe fn accelerator_text(
        &self,
        shortcut: *const ShortcutI,
    ) -> *const ::std::os::raw::c_char {
        self.accelerator_text.unwrap()(shortcut)
    }

    pub unsafe fn is_shortcut_triggered(&self, ui: *mut UiO, shortcut: *const ShortcutI) -> bool {
        self.is_shortcut_triggered.unwrap()(ui, shortcut)
    }

    pub unsafe fn is_shortcut_triggered_in_input(
        &self,
        input_state: *const UiInputStateT,
        shortcut: *const ShortcutI,
    ) -> bool {
        self.is_shortcut_triggered_in_input.unwrap()(input_state, shortcut)
    }

    pub unsafe fn disable_shortcut_processing(&self) {
        self.disable_shortcut_processing.unwrap()()
    }
}

impl crate::Api for ShortcutManagerApi {
    const NAME: ConstCStr = const_cstr!("tm_shortcut_manager_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl ToolbarApi {
    pub unsafe fn create_state(&self, allocator: *mut AllocatorI) -> *mut ToolbarsStateO {
        self.create_state.unwrap()(allocator)
    }

    pub unsafe fn destroy_state(&self, state: *mut ToolbarsStateO) {
        self.destroy_state.unwrap()(state)
    }

    pub unsafe fn calculate_rect(
        &self,
        state: *const ToolbarsStateO,
        ui: *mut UiO,
        full_rect: RectT,
    ) -> RectT {
        self.calculate_rect.unwrap()(state, ui, full_rect)
    }

    pub unsafe fn ui(
        &self,
        toolbars: *mut ToolbarI,
        num_toolbars: u32,
        state: *mut ToolbarsStateO,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        full_rect: RectT,
        settings_tt: *mut TheTruthO,
        settings_obj: TtIdT,
    ) {
        self.ui.unwrap()(
            toolbars,
            num_toolbars,
            state,
            ui,
            uistyle,
            full_rect,
            settings_tt,
            settings_obj,
        )
    }
}

impl crate::Api for ToolbarApi {
    const NAME: ConstCStr = const_cstr!("tm_toolbar_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl TtfBakerApi {
    pub unsafe fn bake(
        &self,
        ttf: *const u8,
        font_index: u32,
        font_size: f32,
        font_scales: *const f32,
        num_scales: u32,
        texture_identifier: u32,
        pixels: *mut u8,
        width: u32,
        height: u32,
        ranges: *const TtfRangeT,
        num_ranges: u32,
        extract_glyph_segments: bool,
        allocator: *mut AllocatorI,
        font_bytes: *mut u64,
    ) -> *mut FontT {
        self.bake.unwrap()(
            ttf,
            font_index,
            font_size,
            font_scales,
            num_scales,
            texture_identifier,
            pixels,
            width,
            height,
            ranges,
            num_ranges,
            extract_glyph_segments,
            allocator,
            font_bytes,
        )
    }
}

impl crate::Api for TtfBakerApi {
    const NAME: ConstCStr = const_cstr!("tm_ttf_baker_api");
    const VERSION: VersionT = VersionT {
        major: 2u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl UiApi {
    pub unsafe fn create(&self, a: *mut AllocatorI) -> *mut UiO {
        self.create.unwrap()(a)
    }

    pub unsafe fn destroy(&self, i: *mut UiO) {
        self.destroy.unwrap()(i)
    }

    pub unsafe fn clear(&self, ui: *mut UiO) {
        self.clear.unwrap()(ui)
    }

    pub unsafe fn release_held_state(&self, ui: *mut UiO) {
        self.release_held_state.unwrap()(ui)
    }

    pub unsafe fn set_window_status(
        &self,
        ui: *mut UiO,
        rect: RectT,
        has_focus: bool,
        is_under_cursor: bool,
    ) {
        self.set_window_status.unwrap()(ui, rect, has_focus, is_under_cursor)
    }

    pub unsafe fn window_rect(&self, ui: *mut UiO) -> RectT {
        self.window_rect.unwrap()(ui)
    }

    pub unsafe fn window_has_focus(&self, ui: *mut UiO) -> bool {
        self.window_has_focus.unwrap()(ui)
    }

    pub unsafe fn window_is_under_cursor(&self, ui: *mut UiO) -> bool {
        self.window_is_under_cursor.unwrap()(ui)
    }

    pub unsafe fn set_feather_width(&self, ui: *mut UiO, feather_width: f32) {
        self.set_feather_width.unwrap()(ui, feather_width)
    }

    pub unsafe fn feather_width(&self, ui: *const UiO) -> f32 {
        self.feather_width.unwrap()(ui)
    }

    pub unsafe fn set_scroll_wheel_lines(&self, ui: *mut UiO, scroll_speed: f32) {
        self.set_scroll_wheel_lines.unwrap()(ui, scroll_speed)
    }

    pub unsafe fn feed_events(
        &self,
        ui: *mut UiO,
        events: *const InputEventT,
        count: u32,
        offset: Vec2T,
        scale: Vec2T,
    ) {
        self.feed_events.unwrap()(ui, events, count, offset, scale)
    }

    pub unsafe fn feed_external_edit_key(&self, ui: *mut UiO, key: u32) {
        self.feed_external_edit_key.unwrap()(ui, key)
    }

    pub unsafe fn merge_overlay(&self, ui: *mut UiO) {
        self.merge_overlay.unwrap()(ui)
    }

    pub unsafe fn begin_overlay_draw_scope(&self, ui: *mut UiO, id: u64) {
        self.begin_overlay_draw_scope.unwrap()(ui, id)
    }

    pub unsafe fn end_overlay_draw_scope(&self, ui: *mut UiO, id: u64) {
        self.end_overlay_draw_scope.unwrap()(ui, id)
    }

    pub unsafe fn drawing_in_overlay(&self, ui: *mut UiO) -> bool {
        self.drawing_in_overlay.unwrap()(ui)
    }

    pub unsafe fn cursor(&self, ui: *mut UiO) -> UiCursor {
        self.cursor.unwrap()(ui)
    }

    pub unsafe fn pane(&self, ui: *mut UiO, style: *const UiStyleT, rect: RectT) {
        self.pane.unwrap()(ui, style, rect)
    }

    pub unsafe fn scrollbar_x(
        &self,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        c: *const UiScrollbarT,
        scroll: *mut f32,
    ) -> bool {
        self.scrollbar_x.unwrap()(ui, uistyle, c, scroll)
    }

    pub unsafe fn scrollbar_y(
        &self,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        c: *const UiScrollbarT,
        scroll: *mut f32,
    ) -> bool {
        self.scrollbar_y.unwrap()(ui, uistyle, c, scroll)
    }

    pub unsafe fn begin_scrollview(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const UiScrollviewT,
        scroll_x: *mut f32,
        scroll_y: *mut f32,
        content_rect: *mut RectT,
    ) -> bool {
        self.begin_scrollview.unwrap()(ui, style, c, scroll_x, scroll_y, content_rect)
    }

    pub unsafe fn end_scrollview(
        &self,
        ui: *mut UiO,
        scroll_x: *mut f32,
        scroll_y: *mut f32,
        can_respond_to_keyboard: bool,
    ) -> bool {
        self.end_scrollview.unwrap()(ui, scroll_x, scroll_y, can_respond_to_keyboard)
    }

    pub unsafe fn label(&self, ui: *mut UiO, style: *const UiStyleT, c: *const UiLabelT) -> RectT {
        self.label.unwrap()(ui, style, c)
    }

    pub unsafe fn text(&self, ui: *mut UiO, style: *const UiStyleT, c: *const UiTextT) -> RectT {
        self.text.unwrap()(ui, style, c)
    }

    pub unsafe fn text_metrics(
        &self,
        style: *const UiStyleT,
        text: *const ::std::os::raw::c_char,
    ) -> RectT {
        self.text_metrics.unwrap()(style, text)
    }

    pub unsafe fn wrapped_text(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const UiTextT,
    ) -> RectT {
        self.wrapped_text.unwrap()(ui, style, c)
    }

    pub unsafe fn link(&self, ui: *mut UiO, style: *const UiStyleT, c: *const UiLinkT) -> bool {
        self.link.unwrap()(ui, style, c)
    }

    pub unsafe fn tooltip(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        text: *const ::std::os::raw::c_char,
    ) {
        self.tooltip.unwrap()(ui, style, text)
    }

    pub unsafe fn button(&self, ui: *mut UiO, style: *const UiStyleT, c: *const UiButtonT) -> bool {
        self.button.unwrap()(ui, style, c)
    }

    pub unsafe fn pushbutton(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const UiButtonT,
        pressed: *mut bool,
    ) -> bool {
        self.pushbutton.unwrap()(ui, style, c, pressed)
    }

    pub unsafe fn checkbox(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const UiCheckboxT,
        checked: *mut bool,
    ) -> bool {
        self.checkbox.unwrap()(ui, style, c, checked)
    }

    pub unsafe fn radio(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const UiRadioT,
        checked: bool,
    ) -> bool {
        self.radio.unwrap()(ui, style, c, checked)
    }

    pub unsafe fn progress(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const UiProgressT,
        fraction: f32,
    ) {
        self.progress.unwrap()(ui, style, c, fraction)
    }

    pub unsafe fn slider(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const UiSliderT,
        val: *mut f32,
        initial: *mut f32,
    ) -> UiInteractionResultT {
        self.slider.unwrap()(ui, style, c, val, initial)
    }

    pub unsafe fn slider_2d(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const Ui2dSliderT,
        val: *mut Vec2T,
        initial: *mut Vec2T,
    ) -> UiInteractionResultT {
        self.slider_2d.unwrap()(ui, style, c, val, initial)
    }

    pub unsafe fn spinner(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const UiSpinnerT,
        val: *mut f64,
        initial: *mut f64,
    ) -> UiInteractionResultT {
        self.spinner.unwrap()(ui, style, c, val, initial)
    }

    pub unsafe fn dropdown(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const UiDropdownT,
        selected: *mut u32,
    ) -> bool {
        self.dropdown.unwrap()(ui, style, c, selected)
    }

    pub unsafe fn textedit(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const UiTexteditT,
        buffer: *mut ::std::os::raw::c_char,
        buffer_bytes: u32,
    ) -> bool {
        self.textedit.unwrap()(ui, style, c, buffer, buffer_bytes)
    }

    pub unsafe fn multiline_textedit(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const UiTexteditT,
        buffer: *mut *mut ::std::os::raw::c_char,
        a: *mut AllocatorI,
        caret_rect: *mut RectT,
    ) -> bool {
        self.multiline_textedit.unwrap()(ui, style, c, buffer, a, caret_rect)
    }

    pub unsafe fn menubar(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const UiMenubarT,
    ) -> UiMenuResultT {
        self.menubar.unwrap()(ui, style, c)
    }

    pub unsafe fn menu(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const UiMenuT,
    ) -> UiMenuResultT {
        self.menu.unwrap()(ui, style, c)
    }

    pub unsafe fn sort_menu_items(&self, items: *mut UiMenuItemT, count: u32) {
        self.sort_menu_items.unwrap()(items, count)
    }

    pub unsafe fn tabbar(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const UiTabbarT,
        selected: *mut u32,
    ) -> UiTabbarResultT {
        self.tabbar.unwrap()(ui, style, c, selected)
    }

    pub unsafe fn draggedtab(&self, ui: *mut UiO, style: *const UiStyleT, c: *const UiDraggedtabT) {
        self.draggedtab.unwrap()(ui, style, c)
    }

    pub unsafe fn splitter_x(
        &self,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        c: *const UiSplitterT,
        bias: *mut f32,
        content_left: *mut RectT,
        content_right: *mut RectT,
    ) -> bool {
        self.splitter_x.unwrap()(ui, uistyle, c, bias, content_left, content_right)
    }

    pub unsafe fn splitter_x_rects(
        &self,
        c: *const UiSplitterT,
        bias: f32,
        content_left: *mut RectT,
        content_right: *mut RectT,
    ) {
        self.splitter_x_rects.unwrap()(c, bias, content_left, content_right)
    }

    pub unsafe fn splitter_y(
        &self,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        c: *const UiSplitterT,
        bias: *mut f32,
        content_top: *mut RectT,
        content_bottom: *mut RectT,
    ) -> bool {
        self.splitter_y.unwrap()(ui, uistyle, c, bias, content_top, content_bottom)
    }

    pub unsafe fn splitter_y_rects(
        &self,
        c: *const UiSplitterT,
        bias: f32,
        content_top: *mut RectT,
        content_bottom: *mut RectT,
    ) {
        self.splitter_y_rects.unwrap()(c, bias, content_top, content_bottom)
    }

    pub unsafe fn titlebar(
        &self,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        c: *const UiTitlebarT,
    ) -> UiTitlebarResultT {
        self.titlebar.unwrap()(ui, uistyle, c)
    }

    pub unsafe fn buffers(&self, ui: *mut UiO) -> UiBuffersT {
        self.buffers.unwrap()(ui)
    }

    pub unsafe fn shortcuts(&self) -> *mut *mut ShortcutI {
        self.shortcuts.unwrap()()
    }

    pub unsafe fn reserve_draw_memory(&self, ui: *mut UiO) {
        self.reserve_draw_memory.unwrap()(ui)
    }

    pub unsafe fn reserve_draw_memory_detailed(
        &self,
        ui: *mut UiO,
        primitive_bytes: u32,
        main_index_bytes: u32,
        overlay_index_bytes: u32,
    ) {
        self.reserve_draw_memory_detailed.unwrap()(
            ui,
            primitive_bytes,
            main_index_bytes,
            overlay_index_bytes,
        )
    }

    pub unsafe fn make_id(&self, ui: *mut UiO) -> u64 {
        self.make_id.unwrap()(ui)
    }

    pub unsafe fn last_id(&self, ui: *mut UiO) -> u64 {
        self.last_id.unwrap()(ui)
    }

    pub unsafe fn create_fixed_id_range(&self, ui: *mut UiO, size: u64) -> u64 {
        self.create_fixed_id_range.unwrap()(ui, size)
    }

    pub unsafe fn set_id(&self, ui: *mut UiO, id: u64) -> u64 {
        self.set_id.unwrap()(ui, id)
    }

    pub unsafe fn set_cursor(&self, ui: *mut UiO, cursor: UiCursor) {
        self.set_cursor.unwrap()(ui, cursor)
    }

    pub unsafe fn is_hovering(&self, ui: *mut UiO, r: RectT, clip: u32) -> bool {
        self.is_hovering.unwrap()(ui, r, clip)
    }

    pub unsafe fn set_responder_chain_root(&self, ui: *mut UiO, id: u64) {
        self.set_responder_chain_root.unwrap()(ui, id)
    }

    pub unsafe fn begin_responder_scope(&self, ui: *mut UiO, id: u64) {
        self.begin_responder_scope.unwrap()(ui, id)
    }

    pub unsafe fn end_responder_scope(&self, ui: *mut UiO, id: u64) {
        self.end_responder_scope.unwrap()(ui, id)
    }

    pub unsafe fn in_responder_chain(&self, ui: *mut UiO, id: u64) -> bool {
        self.in_responder_chain.unwrap()(ui, id)
    }

    pub unsafe fn is_first_responder(&self, ui: *mut UiO, id: u64) -> bool {
        self.is_first_responder.unwrap()(ui, id)
    }

    pub unsafe fn set_responder_chain(&self, ui: *mut UiO, id: u64) {
        self.set_responder_chain.unwrap()(ui, id)
    }

    pub unsafe fn pop_responder_chain(&self, ui: *mut UiO, id: u64) {
        self.pop_responder_chain.unwrap()(ui, id)
    }

    pub unsafe fn responder_chain(&self, ui: *mut UiO, count: *mut u32) -> *mut u64 {
        self.responder_chain.unwrap()(ui, count)
    }

    pub unsafe fn is_responder_chain_empty(&self, ui: *mut UiO) -> bool {
        self.is_responder_chain_empty.unwrap()(ui)
    }

    pub unsafe fn focus_on_mouse_press(&self, ui: *mut UiO, r: RectT, id: u64) -> bool {
        self.focus_on_mouse_press.unwrap()(ui, r, id)
    }

    pub unsafe fn consume_key(&self, ui: *mut UiO, keyboard_item: u32) {
        self.consume_key.unwrap()(ui, keyboard_item)
    }

    pub unsafe fn begin_tab_scope(&self, ui: *mut UiO, id: u64) -> bool {
        self.begin_tab_scope.unwrap()(ui, id)
    }

    pub unsafe fn end_tab_scope(&self, ui: *mut UiO) {
        self.end_tab_scope.unwrap()(ui)
    }

    pub unsafe fn focus_on_tab(&self, ui: *mut UiO, r: RectT, id: u64) -> bool {
        self.focus_on_tab.unwrap()(ui, r, id)
    }

    pub unsafe fn suppress_next_tab_focus(&self, ui: *mut UiO) {
        self.suppress_next_tab_focus.unwrap()(ui)
    }

    pub unsafe fn clear_active(&self, ui: *mut UiO) {
        self.clear_active.unwrap()(ui)
    }

    pub unsafe fn set_active(
        &self,
        ui: *mut UiO,
        id: u64,
        active_data_format: StrhashT,
    ) -> *mut ::std::os::raw::c_void {
        self.set_active.unwrap()(ui, id, active_data_format)
    }

    pub unsafe fn is_active(
        &self,
        ui: *mut UiO,
        id: u64,
        active_data_format: StrhashT,
    ) -> *mut ::std::os::raw::c_void {
        self.is_active.unwrap()(ui, id, active_data_format)
    }

    pub unsafe fn lost_active(
        &self,
        ui: *mut UiO,
        id: u64,
        active_data_format: StrhashT,
    ) -> *mut ::std::os::raw::c_void {
        self.lost_active.unwrap()(ui, id, active_data_format)
    }

    pub unsafe fn clear_lost_active(&self, ui: *mut UiO) {
        self.clear_lost_active.unwrap()(ui)
    }

    pub unsafe fn save_active_state(
        &self,
        ui: *mut UiO,
        ta: *mut TempAllocatorI,
    ) -> *mut ::std::os::raw::c_void {
        self.save_active_state.unwrap()(ui, ta)
    }

    pub unsafe fn restore_active_state(&self, ui: *mut UiO, state: *const ::std::os::raw::c_void) {
        self.restore_active_state.unwrap()(ui, state)
    }

    pub unsafe fn to_draw_style(
        &self,
        ui: *mut UiO,
        style: *mut Draw2dStyleT,
        uistyle: *const UiStyleT,
    ) -> *mut Draw2dStyleT {
        self.to_draw_style.unwrap()(ui, style, uistyle)
    }

    pub unsafe fn set_cache(&self, ui: *mut UiO, key: u64, carray: *mut ::std::os::raw::c_char) {
        self.set_cache.unwrap()(ui, key, carray)
    }

    pub unsafe fn lookup_cache(&self, ui: *mut UiO, key: u64) -> *mut ::std::os::raw::c_char {
        self.lookup_cache.unwrap()(ui, key)
    }

    pub unsafe fn get_cache(
        &self,
        ui: *mut UiO,
        key: u64,
        size: u32,
    ) -> *mut ::std::os::raw::c_void {
        self.get_cache.unwrap()(ui, key, size)
    }

    pub unsafe fn left_mouse_pressed(
        &self,
        ui: *mut UiO,
        help_text: *const ::std::os::raw::c_char,
    ) -> bool {
        self.left_mouse_pressed.unwrap()(ui, help_text)
    }

    pub unsafe fn middle_mouse_pressed(
        &self,
        ui: *mut UiO,
        help_text: *const ::std::os::raw::c_char,
    ) -> bool {
        self.middle_mouse_pressed.unwrap()(ui, help_text)
    }

    pub unsafe fn right_mouse_pressed(
        &self,
        ui: *mut UiO,
        help_text: *const ::std::os::raw::c_char,
    ) -> bool {
        self.right_mouse_pressed.unwrap()(ui, help_text)
    }

    pub unsafe fn left_mouse_released(
        &self,
        ui: *mut UiO,
        help_text: *const ::std::os::raw::c_char,
    ) -> bool {
        self.left_mouse_released.unwrap()(ui, help_text)
    }

    pub unsafe fn middle_mouse_released(
        &self,
        ui: *mut UiO,
        help_text: *const ::std::os::raw::c_char,
    ) -> bool {
        self.middle_mouse_released.unwrap()(ui, help_text)
    }

    pub unsafe fn right_mouse_released(
        &self,
        ui: *mut UiO,
        help_text: *const ::std::os::raw::c_char,
    ) -> bool {
        self.right_mouse_released.unwrap()(ui, help_text)
    }

    pub unsafe fn double_click(
        &self,
        ui: *mut UiO,
        help_text: *const ::std::os::raw::c_char,
    ) -> bool {
        self.double_click.unwrap()(ui, help_text)
    }

    pub unsafe fn triple_click(
        &self,
        ui: *mut UiO,
        help_text: *const ::std::os::raw::c_char,
    ) -> bool {
        self.triple_click.unwrap()(ui, help_text)
    }

    pub unsafe fn get_mouse_help_texts(&self, ui: *mut UiO) -> UiMouseHelpTextsT {
        self.get_mouse_help_texts.unwrap()(ui)
    }

    pub unsafe fn theme(&self, ui: *mut UiO) -> UiThemeT {
        self.theme.unwrap()(ui)
    }

    pub unsafe fn get_theme_colors(
        &self,
        theme: UiThemeT,
        theme_colors: *mut *mut ColorSrgbT,
        a: *mut AllocatorI,
    ) {
        self.get_theme_colors.unwrap()(theme, theme_colors, a)
    }

    pub unsafe fn set_theme(&self, ui: *mut UiO, theme: UiThemeT) {
        self.set_theme.unwrap()(ui, theme)
    }

    pub unsafe fn set_theme_colors(&self, ui: *mut UiO, colors: *const ColorSrgbT) {
        self.set_theme_colors.unwrap()(ui, colors)
    }

    pub unsafe fn create_custom_theme(&self, ui: *mut UiO, tt: *mut TheTruthO) -> UiThemeT {
        self.create_custom_theme.unwrap()(ui, tt)
    }

    pub unsafe fn set_parent_ui(&self, ui: *mut UiO, parent_ui: *mut UiO) {
        self.set_parent_ui.unwrap()(ui, parent_ui)
    }

    pub unsafe fn fork(&self, main: *mut UiO) -> *mut UiO {
        self.fork.unwrap()(main)
    }

    pub unsafe fn join(&self, main: *mut UiO, fork: *mut UiO) {
        self.join.unwrap()(main, fork)
    }

    pub unsafe fn merge_render_buffers(&self, main: *mut UiO, fork: *mut UiO) {
        self.merge_render_buffers.unwrap()(main, fork)
    }

    pub unsafe fn main_ui(&self, ui: *const UiO) -> *mut UiO {
        self.main_ui.unwrap()(ui)
    }

    pub unsafe fn add_font(
        &self,
        ui: *mut UiO,
        font_id: StrhashT,
        size: u32,
        font: *const FontT,
    ) -> *const Draw2dFontT {
        self.add_font.unwrap()(ui, font_id, size, font)
    }

    pub unsafe fn font(&self, ui: *mut UiO, font_id: StrhashT, size: u32) -> UiFontT {
        self.font.unwrap()(ui, font_id, size)
    }

    pub unsafe fn default_style(&self, ui: *const UiO) -> UiStyleT {
        self.default_style.unwrap()(ui)
    }

    pub unsafe fn create_truth_types(&self, tt: *mut TheTruthO) {
        self.create_truth_types.unwrap()(tt)
    }

    pub unsafe fn register_control(
        &self,
        ui: *mut UiO,
        role: StrhashT,
        title: *const ::std::os::raw::c_char,
        rect: RectT,
    ) {
        self.register_control.unwrap()(ui, role, title, rect)
    }

    pub unsafe fn find_control(
        &self,
        ui: *mut UiO,
        role: StrhashT,
        title: *const ::std::os::raw::c_char,
    ) -> RectT {
        self.find_control.unwrap()(ui, role, title)
    }

    pub unsafe fn automation_controls(
        &self,
        ui: *mut UiO,
        ta: *mut TempAllocatorI,
    ) -> *mut UiAutomationControlT {
        self.automation_controls.unwrap()(ui, ta)
    }

    pub unsafe fn mouse_move(&self, ui: *mut UiO, pos: Vec2T) {
        self.mouse_move.unwrap()(ui, pos)
    }

    pub unsafe fn mouse_button_state(&self, ui: *mut UiO, mouse_item: u32, down: bool) {
        self.mouse_button_state.unwrap()(ui, mouse_item, down)
    }

    pub unsafe fn keyboard_key_state(&self, ui: *mut UiO, keyboard_item: u32, down: bool) {
        self.keyboard_key_state.unwrap()(ui, keyboard_item, down)
    }

    pub unsafe fn text_input(&self, ui: *mut UiO, text: *const ::std::os::raw::c_char) {
        self.text_input.unwrap()(ui, text)
    }

    pub unsafe fn visualize_flag(&self, ui: *mut UiO, flag: UiVisualizeFlag) -> bool {
        self.visualize_flag.unwrap()(ui, flag)
    }

    pub unsafe fn set_visualize_flag(&self, ui: *mut UiO, flag: UiVisualizeFlag, on: bool) {
        self.set_visualize_flag.unwrap()(ui, flag, on)
    }

    pub unsafe fn visualize(&self, ui: *mut UiO) {
        self.visualize.unwrap()(ui)
    }
}

impl crate::Api for UiApi {
    const NAME: ConstCStr = const_cstr!("tm_ui_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl UiIconApi {
    pub unsafe fn label(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const UiIconLabelT,
    ) -> RectT {
        self.label.unwrap()(ui, style, c)
    }

    pub unsafe fn text(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        c: *const UiIconTextT,
    ) -> RectT {
        self.text.unwrap()(ui, style, c)
    }

    pub unsafe fn metrics(&self, ui: *mut UiO, font_scale: f32, icon: UiIcon) -> RectT {
        self.metrics.unwrap()(ui, font_scale, icon)
    }

    pub unsafe fn codepoint(&self, icon: UiIcon) -> u32 {
        self.codepoint.unwrap()(icon)
    }

    pub unsafe fn glyph(&self, ui: *mut UiO, icon: UiIcon) -> u16 {
        self.glyph.unwrap()(ui, icon)
    }

    pub unsafe fn draw(
        &self,
        ui: *mut UiO,
        style: *const UiStyleT,
        color: *mut ColorSrgbT,
        pos: Vec2T,
        icon: UiIcon,
    ) -> f32 {
        self.draw.unwrap()(ui, style, color, pos, icon)
    }

    pub unsafe fn title(&self, icon: UiIcon) -> *const ::std::os::raw::c_char {
        self.title.unwrap()(icon)
    }
}

impl crate::Api for UiIconApi {
    const NAME: ConstCStr = const_cstr!("tm_ui_icon_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl UiRendererApi {
    pub unsafe fn create(
        &self,
        res_buf: *mut RendererResourceCommandBufferO,
        shader_repository: *mut ShaderRepositoryO,
        allocator: *mut AllocatorI,
        device_affinity: u32,
    ) -> *mut UiRendererO {
        self.create.unwrap()(res_buf, shader_repository, allocator, device_affinity)
    }

    pub unsafe fn destroy(
        &self,
        ui_renderer: *mut UiRendererO,
        res_buf: *mut RendererResourceCommandBufferO,
    ) {
        self.destroy.unwrap()(ui_renderer, res_buf)
    }

    pub unsafe fn render(
        &self,
        ui_renderer: *mut UiRendererO,
        ui: *mut UiO,
        canvas: *const UiCanvasT,
        sort_key: u64,
        res_buf: *mut RendererResourceCommandBufferO,
        cmd_buf: *mut RendererCommandBufferO,
        color_space: *const ColorSpaceDescT,
    ) {
        self.render.unwrap()(
            ui_renderer,
            ui,
            canvas,
            sort_key,
            res_buf,
            cmd_buf,
            color_space,
        )
    }

    pub unsafe fn default_font(
        &self,
        ui_renderer: *mut UiRendererO,
        font_size: u32,
        window_dpi: f32,
        res_buf: *mut RendererResourceCommandBufferO,
    ) -> *const FontT {
        self.default_font.unwrap()(ui_renderer, font_size, window_dpi, res_buf)
    }

    pub unsafe fn custom_font(
        &self,
        ui_renderer: *mut UiRendererO,
        font_id: StrhashT,
        desc: *const FontDescriptorT,
        font_size: u32,
        window_dpi: f32,
        res_buf: *mut RendererResourceCommandBufferO,
    ) -> *const FontT {
        self.custom_font.unwrap()(ui_renderer, font_id, desc, font_size, window_dpi, res_buf)
    }

    pub unsafe fn add_all_font_providers(
        &self,
        ui_renderer: *mut UiRendererO,
        window_dpi: f32,
        res_buf: *mut RendererResourceCommandBufferO,
        ta: *mut TempAllocatorI,
    ) -> *mut AddedFontT {
        self.add_all_font_providers.unwrap()(ui_renderer, window_dpi, res_buf, ta)
    }

    pub unsafe fn allocate_image_slot(&self, ui_renderer: *mut UiRendererO) -> u32 {
        self.allocate_image_slot.unwrap()(ui_renderer)
    }

    pub unsafe fn set_image(
        &self,
        ui_renderer: *mut UiRendererO,
        slot: u32,
        image_handle: RendererHandleT,
    ) {
        self.set_image.unwrap()(ui_renderer, slot, image_handle)
    }
}

impl crate::Api for UiRendererApi {
    const NAME: ConstCStr = const_cstr!("tm_ui_renderer_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl UiTreeApi {
    pub unsafe fn tree(
        &self,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        tree: *const UiTreeT,
        state: *mut UiTreeStateT,
    ) {
        self.tree.unwrap()(ui, uistyle, tree, state)
    }

    pub unsafe fn tree_item(
        &self,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        rect: RectT,
        state: UiTreeItemStateT,
        tree_has_focus: bool,
        metrics: *const UiTreeItemMetricsT,
        ui_id: u64,
    ) -> UiTreeItemResT {
        self.tree_item.unwrap()(ui, uistyle, rect, state, tree_has_focus, metrics, ui_id)
    }

    pub unsafe fn tree_item_with_sets(
        &self,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        rect: RectT,
        key: u64,
        expanded_set: *mut SetT,
        selected_set: *mut SetT,
        tree_has_focus: bool,
        ui_id: u64,
    ) -> UiTreeItemResT {
        self.tree_item_with_sets.unwrap()(
            ui,
            uistyle,
            rect,
            key,
            expanded_set,
            selected_set,
            tree_has_focus,
            ui_id,
        )
    }

    pub unsafe fn tree_item_with_metrics(
        &self,
        ui: *mut UiO,
        uistyle: *const UiStyleT,
        rect: RectT,
        key: u64,
        expanded_set: *mut SetT,
        selected_set: *mut SetT,
        tree_has_focus: bool,
        metrics: *const UiTreeItemMetricsT,
        ui_id: u64,
    ) -> UiTreeItemResT {
        self.tree_item_with_metrics.unwrap()(
            ui,
            uistyle,
            rect,
            key,
            expanded_set,
            selected_set,
            tree_has_focus,
            metrics,
            ui_id,
        )
    }
}

impl crate::Api for UiTreeApi {
    const NAME: ConstCStr = const_cstr!("tm_ui_tree_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

pub const TM_TT_TYPE_HASH__CLIPBOARD: StrhashT = StrhashT {
    u64_: 11683249507946386256u64,
};
pub const TM_TT_TYPE_HASH__DOCKING_SETTINGS: StrhashT = StrhashT {
    u64_: 13677458105631742768u64,
};
pub const TM_TT_TYPE_HASH__TABWELL: StrhashT = StrhashT {
    u64_: 11643281543487517665u64,
};
pub const TM_TT_TYPE_HASH__TAB_VIEW: StrhashT = StrhashT {
    u64_: 7890642175358502121u64,
};
pub const TM_TT_TYPE_HASH__SHORTCUTS_ENTRY: StrhashT = StrhashT {
    u64_: 14951423256457982727u64,
};
pub const TM_TT_TYPE_HASH__SHORTCUTS_INDEX: StrhashT = StrhashT {
    u64_: 9028192998949932424u64,
};
pub const TM_TT_TYPE_HASH__TOOLBAR_SETTINGS: StrhashT = StrhashT {
    u64_: 16466166788834301889u64,
};
pub const TM_TT_TYPE_HASH__UI_THEME: StrhashT = StrhashT {
    u64_: 2335316840976647572u64,
};
pub const TM_TT_TYPE_HASH__UI_THEME_COLOR: StrhashT = StrhashT {
    u64_: 17779950405567492213u64,
};
pub const TM_UI_ACTIVE_DATA__TEXTEDIT: StrhashT = StrhashT {
    u64_: 5258182182097423975u64,
};
pub const TM_UI_ACTIVE_DATA__MULTILINE_TEXTEDIT: StrhashT = StrhashT {
    u64_: 17663214918397563568u64,
};
pub const TM_UI_ACTIVE_DATA__MENU: StrhashT = StrhashT {
    u64_: 5520113369133148529u64,
};
pub const TM_UI_ROLE__NONE: StrhashT = StrhashT {
    u64_: 8991135776538268156u64,
};
pub const TM_UI_ROLE__ANY: StrhashT = StrhashT {
    u64_: 17914253431124380503u64,
};
pub const TM_UI_ROLE__BUTTON: StrhashT = StrhashT {
    u64_: 2171618341661158550u64,
};
pub const TM_UI_ROLE__CHECKBOX: StrhashT = StrhashT {
    u64_: 9741561056555145438u64,
};
pub const TM_UI_ROLE__CUSTOM: StrhashT = StrhashT {
    u64_: 1210229691100134843u64,
};
pub const TM_UI_ROLE__DROPDOWN: StrhashT = StrhashT {
    u64_: 1646078252726028985u64,
};
pub const TM_UI_ROLE__DROPDOWN_ITEM: StrhashT = StrhashT {
    u64_: 12593592446744835453u64,
};
pub const TM_UI_ROLE__LINK: StrhashT = StrhashT {
    u64_: 2506692502147259529u64,
};
pub const TM_UI_ROLE__MENU_ITEM: StrhashT = StrhashT {
    u64_: 17943940539299394538u64,
};
pub const TM_UI_ROLE__MENUBAR_ITEM: StrhashT = StrhashT {
    u64_: 1888976680707396860u64,
};
pub const TM_UI_ROLE__PROGRESS_BAR: StrhashT = StrhashT {
    u64_: 16229023752359338034u64,
};
pub const TM_UI_ROLE__PUSHBUTTON: StrhashT = StrhashT {
    u64_: 3933447742410737640u64,
};
pub const TM_UI_ROLE__RADIO: StrhashT = StrhashT {
    u64_: 17747375840734793216u64,
};
pub const TM_UI_ROLE__STATIC_TEXT: StrhashT = StrhashT {
    u64_: 4018892029096512244u64,
};
pub const TM_UI_ROLE__TEXTEDIT: StrhashT = StrhashT {
    u64_: 12047848852090713774u64,
};
pub const TM_FONT__IONICONS: StrhashT = StrhashT {
    u64_: 11300213934132052854u64,
};
pub const TM_FONT_PROVIDER_T_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_FONT_LIBRARY_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 1u32,
    patch: 0u32,
};
pub const TM_DOCKING_API_VERSION: VersionT = VersionT {
    major: 2u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_UI_CLIPBOARD_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_TAB_LAYOUT_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_TAB_VT_VERSION: VersionT = VersionT {
    major: 3u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_UI_MODAL_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 1u32,
    patch: 0u32,
};
pub const TM_FONT_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_UI_DRAG_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_SHAPE3D_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_SHORTCUT_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_TOOLBAR_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_FONT_PROVIDER_F_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_DRAW2D_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_FONT_REQUEST_GLYPHS_T_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_UI_RENDERER_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_UI_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_UI_TREE_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_SHORTCUT_MANAGER_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_TTF_BAKER_API_VERSION: VersionT = VersionT {
    major: 2u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_UI_ICON_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_GIZMO_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
