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
pub const TM_TT_TYPE__ASSET_BROWSER: &'static [u8; 17usize] = b"tm_asset_browser\0";
pub const TM_ASSET_BROWSER_CREATE_ASSET_INTERFACE_NAME: &'static [u8; 32usize] =
    b"tm_asset_browser_create_asset_i\0";
pub const TM_ASSET_BROWSER_OPEN_ASSET_API_NAME: &'static [u8; 32usize] =
    b"tm_asset_browser_open_asset_api\0";
pub const TM_ASSET_BROWSER_ADD_ASSET_API_NAME: &'static [u8; 31usize] =
    b"tm_asset_browser_add_asset_api\0";
pub const TM_ASSET_BROWSER_SELECT_ASSET_API_NAME: &'static [u8; 34usize] =
    b"tm_asset_browser_select_asset_api\0";
pub const TM_ASSET_BROWSER_API_NAME: &'static [u8; 21usize] = b"tm_asset_browser_api\0";
pub const TM_ASSET_LABEL_MAX: u32 = 256;
pub const TM_ASSET_LABEL_BITFLAG_UINT64_COUNT: u32 = 4;
pub const TM_ASSET_LABEL_API_NAME: &'static [u8; 19usize] = b"tm_asset_label_api\0";
pub const TM_TT_TYPE__GRAPH: &'static [u8; 9usize] = b"tm_graph\0";
pub const TM_TT_TYPE__GRAPH_NODE: &'static [u8; 14usize] = b"tm_graph_node\0";
pub const TM_TT_TYPE__GRAPH_CONNECTION: &'static [u8; 20usize] = b"tm_graph_connection\0";
pub const TM_TT_TYPE__GRAPH_DATA: &'static [u8; 14usize] = b"tm_graph_data\0";
pub const TM_TT_TYPE__GRAPH_COMMENT: &'static [u8; 17usize] = b"tm_graph_comment\0";
pub const TM_TT_TYPE__GRAPH_INTERFACE: &'static [u8; 19usize] = b"tm_graph_interface\0";
pub const TM_TT_TYPE__GRAPH_INPUT: &'static [u8; 15usize] = b"tm_graph_input\0";
pub const TM_TT_TYPE__GRAPH_INPUT_REFERENCE: &'static [u8; 25usize] = b"tm_graph_input_reference\0";
pub const TM_TT_TYPE__GRAPH_OUTPUT: &'static [u8; 16usize] = b"tm_graph_output\0";
pub const TM_TT_TYPE__GRAPH_OUTPUT_REFERENCE: &'static [u8; 26usize] =
    b"tm_graph_output_reference\0";
pub const TM_TT_TYPE__GRAPH_CACHED_NODE_RESULT: &'static [u8; 28usize] =
    b"tm_graph_cached_node_result\0";
pub const TM_TT_TYPE__GRAPH_EVENT: &'static [u8; 17usize] = b"tm_graph_event_t\0";
pub const TM_TT_TYPE__GRAPH_SETTINGS: &'static [u8; 18usize] = b"tm_graph_settings\0";
pub const TM_TT_TYPE__GRAPH_DEBUGGER_PERSISTENT_SETTINGS: &'static [u8; 38usize] =
    b"tm_graph_debugger_persistent_settings\0";
pub const TM_TT_TYPE__GRAPH_SETTINGS_BREAKPOINT: &'static [u8; 29usize] =
    b"tm_graph_settings_breakpoint\0";
pub const TM_GRAPH_DRAG_AND_DROP_INTERFACE_NAME: &'static [u8; 25usize] =
    b"tm_graph_drag_and_drop_i\0";
pub const TM_GRAPH_API_NAME: &'static [u8; 13usize] = b"tm_graph_api\0";
pub const TM_PROFILER_VIEW_API_NAME: &'static [u8; 21usize] = b"tm_profiler_view_api\0";
pub const TM_TT_TYPE__PROPERTIES_SETTINGS: &'static [u8; 23usize] = b"tm_properties_settings\0";
pub const TM_PROPERTIES_VIEW_API_NAME: &'static [u8; 23usize] = b"tm_properties_view_api\0";
pub const TM_TREE_VIEW_API_NAME: &'static [u8; 17usize] = b"tm_tree_view_api\0";
pub const TM_UI_POPUP_ITEM_PICKER_API_NAME: &'static [u8; 28usize] =
    b"tm_ui_popup_item_picker_api\0";
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
pub const TM_TT_PROP__ASSET_BROWSER__SELECTION: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__ASSET_BROWSER__CURRENT_DIRECTORY: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__ASSET_BROWSER__FOCUS: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__ASSET_BROWSER__SORT_ASCENDING: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__ASSET_BROWSER__SORT_BY: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__ASSET_BROWSER__VIEW_MODE: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__ASSET_BROWSER__FILTER_BY_FILE_EXT: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__ASSET_BROWSER__FILTER_BY_LABEL_BITFLAG_1: ::std::os::raw::c_int = 7;
pub const TM_TT_PROP__ASSET_BROWSER__FILTER_BY_LABEL_BITFLAG_2: ::std::os::raw::c_int = 8;
pub const TM_TT_PROP__ASSET_BROWSER__FILTER_BY_LABEL_BITFLAG_3: ::std::os::raw::c_int = 9;
pub const TM_TT_PROP__ASSET_BROWSER__FILTER_BY_LABEL_BITFLAG_4: ::std::os::raw::c_int = 10;
pub const TM_TT_PROP__ASSET_BROWSER__CURRENT_DIRECTORY_UUID_A: ::std::os::raw::c_int = 11;
pub const TM_TT_PROP__ASSET_BROWSER__CURRENT_DIRECTORY_UUID_B: ::std::os::raw::c_int = 12;
pub type _bindgen_ty_1 = ::std::os::raw::c_int;
pub const TM_ASSET_SAVE_STATUS__SAVED: AssetSaveStatus = 0;
pub const TM_ASSET_SAVE_STATUS__MODIFIED: AssetSaveStatus = 1;
pub const TM_ASSET_SAVE_STATUS__CREATED: AssetSaveStatus = 2;
pub type AssetSaveStatus = ::std::os::raw::c_int;
pub const TM_ASSET_SAVE_FILE_TREE_MODIFICATION_TYPE__CREATED: AssetSaveFileTreeModificationType = 0;
pub const TM_ASSET_SAVE_FILE_TREE_MODIFICATION_TYPE__DELETED: AssetSaveFileTreeModificationType = 1;
pub const TM_ASSET_SAVE_FILE_TREE_MODIFICATION_TYPE__MOVED: AssetSaveFileTreeModificationType = 2;
pub const TM_ASSET_SAVE_FILE_TREE_MODIFICATION_TYPE__RENAMED: AssetSaveFileTreeModificationType = 3;
pub type AssetSaveFileTreeModificationType = ::std::os::raw::c_int;
#[repr(C)]
pub struct AssetSaveFileTreeModificationT {
    pub item: TtIdT,
    pub type_: AssetSaveFileTreeModificationType,
    pub _padding_85: [::std::os::raw::c_char; 4usize],
    pub original_name: *const ::std::os::raw::c_char,
    pub original_directory: TtIdT,
}
impl Default for AssetSaveFileTreeModificationT {
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
pub struct AssetSaveO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AssetSaveI {
    pub inst: *mut AssetSaveO,
    pub can_save_individual_assets:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut AssetSaveO) -> bool>,
    pub file_tree_modifications: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetSaveO,
            ta: *mut TempAllocatorI,
            n: *mut u32,
        ) -> *mut AssetSaveFileTreeModificationT,
    >,
    pub status: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut AssetSaveO, asset: TtIdT) -> AssetSaveStatus,
    >,
    pub save_asset:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut AssetSaveO, asset: TtIdT)>,
    pub revert_asset: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut AssetSaveO, asset: TtIdT, undo_scope: TtUndoScopeT),
    >,
    pub save_all: ::std::option::Option<unsafe extern "C" fn(inst: *mut AssetSaveO)>,
    pub save_all_except: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut AssetSaveO, ignore: *mut TtIdT, num_ignore: u32),
    >,
    pub asset_root_path: ::std::option::Option<
        unsafe extern "C" fn(inst: *const AssetSaveO) -> *const ::std::os::raw::c_char,
    >,
}
impl Default for AssetSaveI {
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
pub struct AssetBrowserCreateAssetI {
    pub inst: *mut AssetBrowserCreateAssetO,
    pub menu_name: *const ::std::os::raw::c_char,
    pub asset_name: *const ::std::os::raw::c_char,
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetBrowserCreateAssetO,
            tt: *mut TheTruthO,
            undo_scope: TtUndoScopeT,
        ) -> TtIdT,
    >,
}
impl Default for AssetBrowserCreateAssetI {
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
pub struct AssetBrowserOpenAssetApi {
    pub inst: *mut AssetBrowserOpenAssetO,
    pub open: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetBrowserOpenAssetO,
            ui: *mut UiO,
            from_tab: *mut TabI,
            object: TtIdT,
        ),
    >,
    pub can_open: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetBrowserOpenAssetO,
            ui: *mut UiO,
            object: TtIdT,
        ) -> bool,
    >,
}
impl Default for AssetBrowserOpenAssetApi {
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
pub struct AssetBrowserAddAssetApi {
    pub inst: *mut AssetBrowserAddAssetO,
    pub current_directory: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut AssetBrowserAddAssetO, ui: *mut UiO) -> TtIdT,
    >,
    pub exists: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetBrowserAddAssetO,
            directory: TtIdT,
            name: *const ::std::os::raw::c_char,
        ) -> TtIdT,
    >,
    pub add: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetBrowserAddAssetO,
            directory: TtIdT,
            object: TtIdT,
            name: *const ::std::os::raw::c_char,
            undo_scope: TtUndoScopeT,
            select: bool,
            ui: *mut UiO,
            asset_labels: *mut u64,
            num_asset_labels: u32,
        ),
    >,
    pub add_directory: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetBrowserAddAssetO,
            parent_directory: TtIdT,
            name: *const ::std::os::raw::c_char,
            undo_scope: TtUndoScopeT,
            select: bool,
            ui: *mut UiO,
        ) -> TtIdT,
    >,
}
impl Default for AssetBrowserAddAssetApi {
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
pub struct AssetBrowserSelectAssetApi {
    pub inst: *mut AssetBrowserSelectAssetO,
    pub select_asset: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut AssetBrowserSelectAssetO, ui: *mut UiO, asset: TtIdT),
    >,
    pub select_assets: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetBrowserSelectAssetO,
            ui: *mut UiO,
            assets: *mut TtIdT,
            count: u32,
        ),
    >,
}
impl Default for AssetBrowserSelectAssetApi {
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
pub struct AssetBrowserO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AssetBrowserCustomMenuItemI {
    pub ud: *mut ::std::os::raw::c_void,
    pub menu_item: ::std::option::Option<
        unsafe extern "C" fn(
            item: *mut UiMenuItemT,
            ud: *mut ::std::os::raw::c_void,
            ab: *mut AssetBrowserO,
            tt: *mut TheTruthO,
            asset_browser_object: TtIdT,
        ),
    >,
    pub menu_select: ::std::option::Option<
        unsafe extern "C" fn(
            ud: *mut ::std::os::raw::c_void,
            ab: *mut AssetBrowserO,
            ui: *mut UiO,
            tt: *mut TheTruthO,
            asset_browser_object: TtIdT,
            undo_stack: *mut UndoStackI,
        ),
    >,
}
impl Default for AssetBrowserCustomMenuItemI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct AssetBrowserConfigT {
    pub tt: *mut TheTruthO,
    pub asset_browser: TtIdT,
    pub undo_stack: *mut UndoStackI,
    pub tab: *mut TabI,
    pub open_ud: *mut ::std::os::raw::c_void,
    pub open: ::std::option::Option<
        unsafe extern "C" fn(
            open_ud: *mut ::std::os::raw::c_void,
            ui: *mut UiO,
            tab: *mut TabI,
            asset: TtIdT,
        ),
    >,
    pub num_custom_menu_items: u32,
    pub _padding_281: [::std::os::raw::c_char; 4usize],
    pub custom_menu_items: *const AssetBrowserCustomMenuItemI,
    pub save_interface: *mut AssetSaveI,
    pub ui_renderer: *mut UiRendererO,
}
impl Default for AssetBrowserConfigT {
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
pub struct AssetBrowserUiResT {
    pub focus_changed: bool,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct AssetBrowserApi {
    pub create_asset_browser: ::std::option::Option<
        unsafe extern "C" fn(
            allocator: *mut AllocatorI,
            config: *const AssetBrowserConfigT,
        ) -> *mut AssetBrowserO,
    >,
    pub destroy_asset_browser:
        ::std::option::Option<unsafe extern "C" fn(asset_browser: *mut AssetBrowserO)>,
    pub ui: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetBrowserO,
            asset_root: TtIdT,
            ui: *mut UiO,
            style: *const UiStyleT,
            rect: RectT,
            tab_id: u64,
        ) -> AssetBrowserUiResT,
    >,
    pub set_new_truth: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut AssetBrowserO, tt: *mut TheTruthO, asset_browser: TtIdT),
    >,
    pub menu: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetBrowserO,
            asset_root: TtIdT,
            ui: *mut UiO,
            style: *const UiStyleT,
            pos: Vec2T,
        ),
    >,
    pub process_dropped_os_files: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut AssetBrowserO,
            ui: *mut UiO,
            asset_root: TtIdT,
            files: *mut *mut ::std::os::raw::c_char,
            num_files: u32,
        ),
    >,
    pub focused_object:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut AssetBrowserO) -> TtIdT>,
    pub selected_objects: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut AssetBrowserO, ta: *mut TempAllocatorI) -> *const TtIdT,
    >,
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
pub type _bindgen_ty_2 = ::std::os::raw::c_int;
pub type _bindgen_ty_3 = ::std::os::raw::c_int;
pub type _bindgen_ty_4 = ::std::os::raw::c_int;
pub type _bindgen_ty_5 = ::std::os::raw::c_int;
pub type _bindgen_ty_6 = ::std::os::raw::c_int;
#[repr(C)]
pub struct AssetLabelsT {
    pub uuids: [AssetLabelUuidT; 256usize],
    pub names: [*const ::std::os::raw::c_char; 256usize],
    pub icons: [u32; 256usize],
    pub icon_colors: [u32; 256usize],
    pub num_labels: u32,
    pub num_system_labels: u32,
}
impl Default for AssetLabelsT {
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
pub struct AssetLabelBitflagT {
    pub bitflag: [u64; 4usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct AssetLabelApi {
    pub all_labels: ::std::option::Option<
        unsafe extern "C" fn(labels: *mut AssetLabelsT, tt: *const TheTruthO, asset_root: TtIdT),
    >,
    pub labels_of_asset: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            asset: TtIdT,
            asset_labels: *const AssetLabelsT,
            out_labels: *mut AssetLabelsT,
        ),
    >,
    pub label_uuids_of_asset: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            asset: TtIdT,
            ta: *mut TempAllocatorI,
        ) -> *mut u64,
    >,
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            asset_root: TtIdT,
            directory: TtIdT,
            label: *const ::std::os::raw::c_char,
            undo_scope: TtUndoScopeT,
        ) -> AssetLabelUuidT,
    >,
    pub add: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            asset: TtIdT,
            label: AssetLabelUuidT,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub remove: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            asset: TtIdT,
            label: AssetLabelUuidT,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub rename: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            asset_root: TtIdT,
            label: AssetLabelUuidT,
            new_label_name: *const ::std::os::raw::c_char,
            undo_scope: TtUndoScopeT,
        ),
    >,
    pub assets_with_label: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            asset_root: TtIdT,
            label: AssetLabelUuidT,
            ta: *mut TempAllocatorI,
        ) -> *mut TtIdT,
    >,
    pub assets_with_all_labels: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            asset_root: TtIdT,
            labels: *const AssetLabelBitflagT,
            ta: *mut TempAllocatorI,
        ) -> *mut TtIdT,
    >,
    pub assets_with_any_label: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            asset_root: TtIdT,
            labels: *const AssetLabelBitflagT,
            ta: *mut TempAllocatorI,
        ) -> *mut TtIdT,
    >,
    pub toggle_bitflags: ::std::option::Option<
        unsafe extern "C" fn(
            labels: *const AssetLabelsT,
            bitflag: *mut AssetLabelBitflagT,
            labels_uuid: *const AssetLabelUuidT,
            num_labels: u32,
        ),
    >,
    pub matches_all: ::std::option::Option<
        unsafe extern "C" fn(
            asset_bitflag: *const AssetLabelBitflagT,
            filter_bitflag: *const AssetLabelBitflagT,
        ) -> bool,
    >,
    pub matches_any: ::std::option::Option<
        unsafe extern "C" fn(
            asset_bitflag: *const AssetLabelBitflagT,
            filter_bitflag: *const AssetLabelBitflagT,
        ) -> bool,
    >,
}
#[repr(C)]
pub struct GraphAspectI {
    pub node_interface_name: *const ::std::os::raw::c_char,
    pub io_type_interface_name: *const ::std::os::raw::c_char,
    pub graph_object: ::std::option::Option<
        unsafe extern "C" fn(tt: *const TheTruthO, root_object: TtIdT) -> TtIdT,
    >,
    pub subgraph_type_hash: StrhashT,
}
impl Default for GraphAspectI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct GraphIoTypeT {
    pub type_hash: StrhashT,
    pub display_name: *const ::std::os::raw::c_char,
    pub size: u32,
    pub _padding_84: [::std::os::raw::c_char; 4usize],
}
impl Default for GraphIoTypeT {
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
pub type _bindgen_ty_7 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_NODE__TYPE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_NODE__LABEL: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_NODE__POSITION_X: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_NODE__POSITION_Y: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH_NODE__WIDTH: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__GRAPH_NODE__SETTINGS: ::std::os::raw::c_int = 5;
pub type _bindgen_ty_8 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_CONNECTION__FROM_NODE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_CONNECTION__TO_NODE: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_CONNECTION__FROM_CONNECTOR_HASH: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_CONNECTION__TO_CONNECTOR_HASH: ::std::os::raw::c_int = 3;
pub type _bindgen_ty_9 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_DATA__TO_NODE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_DATA__TO_CONNECTOR_HASH: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_DATA__DATA: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_10 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_COMMENT__TEXT: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_COMMENT__POSITION_X: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_COMMENT__POSITION_Y: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_COMMENT__FONT_SCALE: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH_COMMENT__COLOR: ::std::os::raw::c_int = 4;
pub type _bindgen_ty_11 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_INTERFACE__INPUTS: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_INTERFACE__OUTPUTS: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_INTERFACE__LAST_ID: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_INTERFACE__CACHED_NODE_RESULT: ::std::os::raw::c_int = 3;
pub type _bindgen_ty_12 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_INPUT__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_INPUT__DISPLAY_NAME: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_INPUT__ID: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_INPUT__TYPE_HASH: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH_INPUT__VALUE_SET_BY_USER: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__GRAPH_INPUT__VALUE: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__GRAPH_INPUT__TOOLTIP: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__GRAPH_INPUT__PUBLIC: ::std::os::raw::c_int = 7;
pub const TM_TT_PROP__GRAPH_INPUT__ORDER: ::std::os::raw::c_int = 8;
pub type _bindgen_ty_13 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_INPUT_REFERENCE__NAME: TtPropGraphInputReference = 0;
pub const TM_TT_PROP__GRAPH_INPUT_REFERENCE__TYPE_HASH: TtPropGraphInputReference = 1;
pub type TtPropGraphInputReference = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_OUTPUT__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_OUTPUT__DISPLAY_NAME: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_OUTPUT__ID: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_OUTPUT__TYPE_HASH: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH_OUTPUT__ORDER: ::std::os::raw::c_int = 4;
pub type _bindgen_ty_14 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_OUTPUT_REFERENCE__NAME: TtPropGraphOutputReference = 0;
pub const TM_TT_PROP__GRAPH_OUTPUT_REFERENCE__TYPE_HASH: TtPropGraphOutputReference = 1;
pub type TtPropGraphOutputReference = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_CACHED_NODE_RESULT__NODE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_CACHED_NODE_RESULT__VALIDITY_HASH: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_CACHED_NODE_RESULT__OBJECT: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_15 = ::std::os::raw::c_int;
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
pub type _bindgen_ty_16 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_SETTINGS__BREAKPOINTS: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_SETTINGS__OVERLAY_OPENED: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_17 = ::std::os::raw::c_int;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__NODE_UNIQUE_ID: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__NODE_UUID_A: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__NODE_UUID_B: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__SUBGRAPH_NODE_UNIQUE_ID: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__SUBGRAPH_NODE_UUID_A: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__SUBGRAPH_NODE_UUID_B: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__DISABLED: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__GRAPH_UUID_A: ::std::os::raw::c_int = 7;
pub const TM_TT_PROP__GRAPH_SETTINGS_BREAKPOINT__GRAPH_UUID_B: ::std::os::raw::c_int = 8;
pub type _bindgen_ty_18 = ::std::os::raw::c_int;
pub const TM_GRAPH_MAX_CONNECTORS: ::std::os::raw::c_int = 16;
pub type _bindgen_ty_19 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GraphGenericValueT {
    pub __bindgen_anon_1: GraphGenericValueTBindgenTy1,
}
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
impl Default for GraphGenericValueT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct GraphConnectorT {
    pub name: *const ::std::os::raw::c_char,
    pub type_hash: StrhashT,
    pub edit_type_hash: StrhashT,
    pub display_name: *const ::std::os::raw::c_char,
    pub optional: bool,
    pub _padding_336: [::std::os::raw::c_char; 7usize],
    pub tooltip: *mut ::std::os::raw::c_char,
    pub hidden: bool,
    pub _padding_345: [::std::os::raw::c_char; 7usize],
    pub default_value: *const GraphGenericValueT,
}
impl Default for GraphConnectorT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct GraphNodeConnectorsT {
    pub in_: [GraphConnectorT; 16usize],
    pub out: [GraphConnectorT; 16usize],
    pub num_in: u32,
    pub num_out: u32,
}
impl Default for GraphNodeConnectorsT {
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
pub struct GraphDrawConnectorsResultT {
    pub y: f32,
    pub in_connector_positions: [f32; 16usize],
    pub in_connector_disabled: [bool; 16usize],
    pub out_connector_positions: [f32; 16usize],
}
#[repr(C)]
pub struct GraphDrawConnectorsArgsT {
    pub prop_args: *mut PropertiesUiArgsT,
    pub graph_id: TtIdT,
    pub connectors: *mut GraphNodeConnectorsT,
    pub in_connected: u32,
    pub out_connected: u32,
    pub item_rect: RectT,
    pub graph_scale: f32,
    pub property_panel_mode: bool,
    pub show_removed_inherited: bool,
    pub _padding_439: [::std::os::raw::c_char; 2usize],
    pub expanded: *mut SetT,
    pub graph: *mut GraphO,
    pub node_id: TtIdT,
    pub node_prototype_relation: u32,
    pub node_idx: u32,
    pub draw_in_connector_label_and_property: ::std::option::Option<
        unsafe extern "C" fn(
            graph: *mut GraphO,
            node_idx: u32,
            connector: *mut GraphConnectorT,
            editable: bool,
            r: RectT,
            args: *mut PropertiesUiArgsT,
        ) -> RectT,
    >,
    pub draw_out_connector_label: ::std::option::Option<
        unsafe extern "C" fn(
            graph: *mut GraphO,
            connector: *mut GraphConnectorT,
            editable: bool,
            r: RectT,
            args: *mut PropertiesUiArgsT,
            alignment: u32,
        ) -> RectT,
    >,
}
impl Default for GraphDrawConnectorsArgsT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct GraphNodeTypeI {
    pub name: *const ::std::os::raw::c_char,
    pub display_name: *const ::std::os::raw::c_char,
    pub definition_path: *const ::std::os::raw::c_char,
    pub category: *const ::std::os::raw::c_char,
    pub static_connectors: GraphNodeConnectorsT,
    pub settings_type_hash: StrhashT,
    pub cache_tag: StrhashT,
    pub dynamic_connectors: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            graph_id: TtIdT,
            node_data: *const TtIdT,
            node_settings: TtIdT,
            connectors: *mut GraphNodeConnectorsT,
        ),
    >,
    pub render_background_callback: ::std::option::Option<
        unsafe extern "C" fn(
            nt: *const GraphNodeTypeI,
            bg_rect: RectT,
            title_rect: RectT,
            title_bg_color: ColorSrgbT,
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            graph_scale: f32,
            vbuffer: *mut Draw2dVbufferT,
            ibuffer: *mut Draw2dIbufferT,
        ),
    >,
    pub draw_connectors: ::std::option::Option<
        unsafe extern "C" fn(args: *mut GraphDrawConnectorsArgsT) -> GraphDrawConnectorsResultT,
    >,
    pub handle_custom_default_values: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            psudo_node_data: TtIdT,
            edit_type_hash: StrhashT,
            default_data: *const GraphGenericValueT,
        ),
    >,
    pub disabled: ::std::option::Option<unsafe extern "C" fn(nt: *const GraphNodeTypeI) -> bool>,
}
impl Default for GraphNodeTypeI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct GraphNodeCacheBuildContextT {
    pub graph_id: TtIdT,
    pub node_id: TtIdT,
}
impl Default for GraphNodeCacheBuildContextT {
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
pub struct GraphDragAndDropI {
    pub type_name: *const ::std::os::raw::c_char,
    pub can_create_from: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, graph_id: TtIdT, dragged_object: TtIdT) -> bool,
    >,
    pub setup_graph_node_from_dragged_object: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            graph_id: TtIdT,
            node_id: TtIdT,
            dragged_object: TtIdT,
            undo_scope: TtUndoScopeT,
        ),
    >,
}
impl Default for GraphDragAndDropI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct GraphUiResT {
    pub focus_changed: bool,
    pub selection_changed: bool,
    pub _padding_589: [::std::os::raw::c_char; 6usize],
    pub focus: TtIdT,
}
impl Default for GraphUiResT {
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
pub struct GraphApi {
    pub create_truth_types: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
    pub create_graph: ::std::option::Option<
        unsafe extern "C" fn(
            allocator: *mut AllocatorI,
            tt: *mut TheTruthO,
            graph_id: TtIdT,
            node_interface_name: *const ::std::os::raw::c_char,
            undo_stack: *mut UndoStackI,
            asset_root: TtIdT,
            settings_tt: *mut TheTruthO,
            settings_object: TtIdT,
        ) -> *mut GraphO,
    >,
    pub destroy_graph: ::std::option::Option<
        unsafe extern "C" fn(graph: *mut GraphO, truth_already_destroyed: bool),
    >,
    pub set_settings_object: ::std::option::Option<
        unsafe extern "C" fn(
            graph: *mut GraphO,
            settings_tt: *mut TheTruthO,
            settings_object: TtIdT,
        ),
    >,
    pub id: ::std::option::Option<unsafe extern "C" fn(graph: *mut GraphO) -> TtIdT>,
    pub ui: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut GraphO,
            ui: *mut UiO,
            style: *const UiStyleT,
            rect: RectT,
            tab: *mut TabI,
        ) -> GraphUiResT,
    >,
    pub menu: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut GraphO, ui: *mut UiO, style: *const UiStyleT, pos: Vec2T),
    >,
    pub selected_objects: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut GraphO, ta: *mut TempAllocatorI) -> *const TtIdT,
    >,
    pub refresh_node_types: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut GraphO, node_interface_name: *const ::std::os::raw::c_char),
    >,
    pub toolbars: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut GraphO, ta: *mut TempAllocatorI) -> *mut ToolbarI,
    >,
    pub migrate_to_connector_hash: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            graphs: *mut TtIdT,
            num_graphs: u32,
            node_types: *const *const GraphNodeTypeI,
            num_node_types: u32,
        ) -> bool,
    >,
    pub needs_migrate_to_connector_hash:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO) -> bool>,
    pub migrate_to_instantiated_connections: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, graphs: *mut TtIdT, num_graphs: u32) -> bool,
    >,
}
#[repr(C)]
pub struct ProfilerViewUiResOpenStatisticsT {
    pub name: *const ::std::os::raw::c_char,
    pub source: *const ::std::os::raw::c_char,
    pub color: ColorSrgbT,
    pub _padding_22: [::std::os::raw::c_char; 4usize],
}
impl Default for ProfilerViewUiResOpenStatisticsT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct ProfilerViewUiResT {
    pub open_statistics_view: ProfilerViewUiResOpenStatisticsT,
}
impl Default for ProfilerViewUiResT {
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
pub struct ProfilerViewO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ProfilerViewApi {
    pub create_profiler_view: ::std::option::Option<
        unsafe extern "C" fn(allocator: *mut AllocatorI) -> *mut ProfilerViewO,
    >,
    pub destroy_profiler_view:
        ::std::option::Option<unsafe extern "C" fn(profiler_view: *mut ProfilerViewO)>,
    pub ui: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ProfilerViewO,
            ui: *mut UiO,
            style: *const UiStyleT,
            rect: RectT,
            tab_id: u64,
        ) -> ProfilerViewUiResT,
    >,
    pub start_recording:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut ProfilerViewO, with_history: bool)>,
    pub stop_recording: ::std::option::Option<unsafe extern "C" fn(inst: *mut ProfilerViewO)>,
    pub is_recording: ::std::option::Option<unsafe extern "C" fn(inst: *mut ProfilerViewO) -> bool>,
    pub menu: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ProfilerViewO,
            ui: *mut UiO,
            style: *const UiStyleT,
            pos: Vec2T,
        ),
    >,
}
pub const TM_TT_PROP__PROPERTIES_SETTINGS__MASK: TM_TT_PROP__PROPERTIES_SETTINGS = 0;
pub type TM_TT_PROP__PROPERTIES_SETTINGS = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PropertiesViewO {
    _unused: [u8; 0],
}
pub const TM_PROPERTIES_METRIC_LABEL_WIDTH: PropertiesMetric = 0;
pub const TM_PROPERTIES_METRIC_MARGIN: PropertiesMetric = 1;
pub const TM_PROPERTIES_METRIC_ITEM_HEIGHT: PropertiesMetric = 2;
pub const TM_PROPERTIES_METRIC_INDENT: PropertiesMetric = 3;
pub const TM_PROPERTIES_METRIC_EDIT_WIDTH: PropertiesMetric = 4;
pub const TM_PROPERTIES_METRIC_SUBOBJECT_LABEL_MARGIN: PropertiesMetric = 5;
pub const TM_PROPERTIES_METRIC_CHECKBOX_CONTROL_WIDTH: PropertiesMetric = 6;
pub const TM_PROPERTIES_METRIC_GROUP_RECT_PADDING: PropertiesMetric = 7;
pub const TM_PROPERTIES_METRIC_GROUP_LABEL_LEFT_MARGIN: PropertiesMetric = 8;
pub const TM_PROPERTIES_METRIC_MENU_WIDTH: PropertiesMetric = 9;
pub const TM_PROPERTIES_METRIC_COLOR_PICKER_RGB_LABEL_SIZE: PropertiesMetric = 10;
pub const TM_PROPERTIES_METRIC_COLOR_PICKER_INPUT_SIZE: PropertiesMetric = 11;
pub const TM_PROPERTIES_METRIC_COLOR_PICKER_HSV_LABEL_SIZE: PropertiesMetric = 12;
pub const TM_PROPERTIES_METRIC_COLOR_PICKER_SLIDER_MARGIN: PropertiesMetric = 13;
pub const TM_PROPERTIES_METRIC_COLOR_PICKER_SLIDER_KNOB_SIZE: PropertiesMetric = 14;
pub const TM_PROPERTIES_METRIC_NUM: PropertiesMetric = 15;
pub type PropertiesMetric = ::std::os::raw::c_int;
#[repr(C)]
pub struct PropertiesUiArgsT {
    pub pv: *mut PropertiesViewO,
    pub ui: *mut UiO,
    pub uistyle: *mut UiStyleT,
    pub tab: *mut TabI,
    pub tt: *mut TheTruthO,
    pub asset_root: TtIdT,
    pub undo_stack: *mut UndoStackI,
    pub metrics: *mut f32,
    pub last_undo_scope: TtUndoScopeT,
    pub context: TtIdT,
}
impl Default for PropertiesUiArgsT {
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
pub struct PropertiesAspectI {
    pub custom_ui: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            object: TtIdT,
            indent: u32,
        ) -> f32,
    >,
    pub prototype_asset_picker_type_name:
        ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO, o: TtIdT) -> StrhashT>,
    pub custom_subobject_ui: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            subobject: TtIdT,
            indent: u32,
        ) -> f32,
    >,
    pub custom_child_ui: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            parent: TtIdT,
            child: TtIdT,
            indent: u32,
        ) -> f32,
    >,
    pub get_type_display_name:
        ::std::option::Option<unsafe extern "C" fn() -> *const ::std::os::raw::c_char>,
    pub get_display_name: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object: TtIdT,
            buffer: *mut ::std::os::raw::c_char,
            size: u32,
        ),
    >,
}
pub type TtPropAspectPropertiesAssetPicker = *mut ::std::os::raw::c_char;
pub type TtPropAspectPropertiesCustomUi = ::std::option::Option<
    unsafe extern "C" fn(
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        indent: u32,
        property: u32,
    ) -> f32,
>;
pub type TtPropAspectPropertiesReprototypeCallback = ::std::option::Option<
    unsafe extern "C" fn(
        tt: *mut TheTruthO,
        new_object: TtIdT,
        old_object: TtIdT,
        undo_scope: TtUndoScopeT,
    ),
>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PropertiesFloatDisplayConverterI {
    pub inst: *mut ::std::os::raw::c_void,
    pub model_to_display: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, v: f32) -> f32,
    >,
    pub display_to_model: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, v: f32) -> f32,
    >,
    pub get_spinner_settings: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, spinner: *mut UiSpinnerT),
    >,
}
impl Default for PropertiesFloatDisplayConverterI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type ValidateAspectI = ::std::option::Option<
    unsafe extern "C" fn(
        tt: *mut TheTruthO,
        o: TtIdT,
        property: u32,
        value: *const ::std::os::raw::c_void,
    ) -> bool,
>;
#[repr(C)]
pub struct PropertiesUiInfoT {
    pub last_hovered_focusable_control: u64,
    pub last_hovered_context_menu_rect: RectT,
}
impl Default for PropertiesUiInfoT {
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
pub struct PropertiesUint32DisplayConverterI {
    pub inst: *mut ::std::os::raw::c_void,
    pub model_to_display: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, v: u32) -> u32,
    >,
    pub display_to_model: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut ::std::os::raw::c_void, v: u32) -> u32,
    >,
}
impl Default for PropertiesUint32DisplayConverterI {
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
pub struct PropertiesReferencePickerArgsT {
    pub labels: *const u64,
    pub excludes: *const TtIdT,
    pub num_labels: u32,
    pub num_excludes: u32,
}
impl Default for PropertiesReferencePickerArgsT {
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
pub struct PropertiesViewConfigT {
    pub tt: *mut TheTruthO,
    pub undo_stack: *mut UndoStackI,
}
impl Default for PropertiesViewConfigT {
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
pub struct PropertiesFloatSliderT {
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub show_edit_box: bool,
    pub _padding_378: [::std::os::raw::c_char; 3usize],
    pub edit_min: f32,
    pub edit_max: f32,
    pub converter: *mut PropertiesFloatDisplayConverterI,
}
impl Default for PropertiesFloatSliderT {
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
pub struct PropertiesViewApi {
    pub create_properties_view: ::std::option::Option<
        unsafe extern "C" fn(
            allocator: *mut AllocatorI,
            config: *const PropertiesViewConfigT,
        ) -> *mut PropertiesViewO,
    >,
    pub destroy_properties_view:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut PropertiesViewO)>,
    pub set_object: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut PropertiesViewO, object: TtIdT) -> bool,
    >,
    pub set_object_with_mask: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut PropertiesViewO, object: TtIdT, property_mask: u64) -> bool,
    >,
    pub set_mask: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut PropertiesViewO, property_mask: u64) -> bool,
    >,
    pub set_objects: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut PropertiesViewO, objects: *const TtIdT, n: u32) -> bool,
    >,
    pub objects:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut PropertiesViewO) -> *const TtIdT>,
    pub ui: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut PropertiesViewO,
            asset_root: TtIdT,
            ui: *mut UiO,
            style: *const UiStyleT,
            rect: RectT,
            tab: *mut TabI,
        ),
    >,
    pub update:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut PropertiesViewO, ui: *mut UiO)>,
    pub ui_info: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut PropertiesViewO) -> *mut PropertiesUiInfoT,
    >,
    pub metrics:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut PropertiesViewO) -> *mut f32>,
    pub get_display_name: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object: TtIdT,
            buffer: *mut ::std::os::raw::c_char,
            size: u32,
        ),
    >,
    pub get_type_display_name: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *const TheTruthO,
            object_type: TtTypeT,
            buffer: *mut ::std::os::raw::c_char,
            size: u32,
        ),
    >,
    pub ui_object: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            object: TtIdT,
            indent: u32,
        ) -> f32,
    >,
    pub ui_object_default: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            object: TtIdT,
            indent: u32,
        ) -> f32,
    >,
    pub ui_property: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            object: TtIdT,
            indent: u32,
            property: u32,
        ) -> f32,
    >,
    pub ui_property_with_name: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            indent: u32,
            property: u32,
        ) -> f32,
    >,
    pub ui_property_default: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            object: TtIdT,
            indent: u32,
            property: u32,
        ) -> f32,
    >,
    pub ui_property_default_with_name: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            indent: u32,
            property: u32,
        ) -> f32,
    >,
    pub ui_tree_item: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            key: u64,
            rect: RectT,
            indent: u32,
            expanded_default: bool,
            tooltip: *const ::std::os::raw::c_char,
            ui_id: u64,
        ) -> bool,
    >,
    pub ui_group: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            indent: u32,
            expanded_default: bool,
            is_expanded: *mut bool,
        ) -> f32,
    >,
    pub show_subobject_menu: ::std::option::Option<
        unsafe extern "C" fn(
            tt: *mut TheTruthO,
            undo_stack: *mut UndoStackI,
            ui: *mut UiO,
            uistyle: *mut UiStyleT,
            object: TtIdT,
            property: u32,
            sub: TtIdT,
            pos: Vec2T,
        ) -> TtUndoScopeT,
    >,
    pub begin_subobject_menu_scope: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            rect: RectT,
            object: TtIdT,
            property: u32,
            sub: TtIdT,
            ui_id: u64,
        ),
    >,
    pub end_subobject_menu_scope: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            object: TtIdT,
            property: u32,
            sub: TtIdT,
            ui_id: u64,
        ),
    >,
    pub begin_context_menu_scope:
        ::std::option::Option<unsafe extern "C" fn(pv: *mut PropertiesViewO, rect: RectT)>,
    pub end_context_menu_scope: ::std::option::Option<
        unsafe extern "C" fn(
            pv: *mut PropertiesViewO,
            out_id: *mut u64,
            out_text: *mut *mut ::std::os::raw::c_char,
        ),
    >,
    pub selected_context_menu_item: ::std::option::Option<
        unsafe extern "C" fn(
            pv: *mut PropertiesViewO,
            out_id: *mut u64,
            out_text: *mut *mut ::std::os::raw::c_char,
        ),
    >,
    pub add_context_menu_items: ::std::option::Option<
        unsafe extern "C" fn(pv: *mut PropertiesViewO, items: *const UiMenuItemT, num_items: u32),
    >,
    pub context_menu: ::std::option::Option<unsafe extern "C" fn(args: *mut PropertiesUiArgsT)>,
    pub ui_bool: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
        ) -> f32,
    >,
    pub ui_uint32: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            converter: *mut PropertiesUint32DisplayConverterI,
        ) -> f32,
    >,
    pub ui_uint64: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
        ) -> f32,
    >,
    pub ui_float: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            converter: *const PropertiesFloatDisplayConverterI,
        ) -> f32,
    >,
    pub ui_double: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
        ) -> f32,
    >,
    pub ui_string: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
        ) -> f32,
    >,
    pub ui_reference: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
        ) -> f32,
    >,
    pub ui_reference_args: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            picker_args: *const PropertiesReferencePickerArgsT,
        ) -> f32,
    >,
    pub ui_reference_entity_picker: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            entity_root: TtIdT,
        ) -> f32,
    >,
    pub ui_subobject: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            indent: u32,
            expanded_by_default: bool,
        ) -> f32,
    >,
    pub ui_subobject_direct: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            parent: TtIdT,
            subobject: TtIdT,
            indent: u32,
            expanded_by_default: bool,
        ) -> f32,
    >,
    pub ui_reference_set: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            indent: u32,
        ) -> f32,
    >,
    pub ui_subobject_set: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            indent: u32,
        ) -> f32,
    >,
    pub ui_subobject_set_reorderable: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            indent: u32,
            item_order_property: u32,
            draw_header: bool,
        ) -> f32,
    >,
    pub ui_subobject_set_item_header: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            object: TtIdT,
            property: u32,
            sub: TtIdT,
            indent: u32,
            custom_name: *const ::std::os::raw::c_char,
            expanded_by_default: bool,
        ) -> bool,
    >,
    pub ui_subobject_set_item: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            object: TtIdT,
            property: u32,
            sub: TtIdT,
            indent: u32,
            expanded_by_default: bool,
        ) -> f32,
    >,
    pub ui_uint32_popup_picker: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            items: *mut *const ::std::os::raw::c_char,
            num_items: u32,
        ) -> f32,
    >,
    pub ui_uint64_popup_picker: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            items: *mut *const ::std::os::raw::c_char,
            items_values: *const u64,
            num_items: u32,
        ) -> f32,
    >,
    pub ui_reference_popup_picker: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            items: *mut *const ::std::os::raw::c_char,
            items_references: *const TtIdT,
            num_items: u32,
        ) -> f32,
    >,
    pub ui_string_popup_picker: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            items: *mut *const ::std::os::raw::c_char,
            num_items: u32,
        ) -> f32,
    >,
    pub popup_pick_with_categories: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            ui_id: u64,
            pos: Vec2T,
            strings: *const *const ::std::os::raw::c_char,
            categories: *const *const ::std::os::raw::c_char,
            num_strings: u32,
            pick_index: *mut u32,
        ) -> bool,
    >,
    pub ui_uint32_dropdown: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            items: *mut *const ::std::os::raw::c_char,
            item_tooltips: *mut *const ::std::os::raw::c_char,
            items_uint32: *const u32,
            num_items: u32,
        ) -> f32,
    >,
    pub ui_uint64_dropdown: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            items: *mut *const ::std::os::raw::c_char,
            item_tooltips: *mut *const ::std::os::raw::c_char,
            items_uint64: *const u64,
            num_items: u32,
        ) -> f32,
    >,
    pub ui_float_slider: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            slider: *const PropertiesFloatSliderT,
        ) -> f32,
    >,
    pub ui_vec2: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            vec: TtIdT,
        ) -> f32,
    >,
    pub ui_vec3: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            vec: TtIdT,
        ) -> f32,
    >,
    pub ui_vec4: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            vec: TtIdT,
        ) -> f32,
    >,
    pub ui_color_picker: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            color: TtIdT,
        ) -> f32,
    >,
    pub ui_color_temperature: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            tooltip: *const ::std::os::raw::c_char,
            color: TtIdT,
        ) -> f32,
    >,
    pub ui_color_button: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            color_button: *const UiButtonT,
            color: TtIdT,
        ) -> bool,
    >,
    pub ui_expanded_color_picker: ::std::option::Option<
        unsafe extern "C" fn(args: *mut PropertiesUiArgsT, item_rect: RectT, color: TtIdT) -> f32,
    >,
    pub ui_rotation: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            rotation: TtIdT,
        ) -> f32,
    >,
    pub ui_visibility_flags: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            indent: u32,
        ) -> f32,
    >,
    pub ui_label: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
        ) -> f32,
    >,
    pub ui_horizontal_line: ::std::option::Option<
        unsafe extern "C" fn(args: *mut PropertiesUiArgsT, item_rect: RectT) -> f32,
    >,
    pub ui_static_text: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            text: *const ::std::os::raw::c_char,
        ) -> f32,
    >,
    pub ui_prototype: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            parent: TtIdT,
            property: u32,
            subobject: TtIdT,
            pick_type_hash: StrhashT,
        ) -> f32,
    >,
    pub ui_open_path: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            extensions: *const ::std::os::raw::c_char,
            description: *const ::std::os::raw::c_char,
            picked: *mut bool,
        ) -> f32,
    >,
    pub ui_save_path: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            name: *const ::std::os::raw::c_char,
            tooltip: *const ::std::os::raw::c_char,
            object: TtIdT,
            property: u32,
            extensions: *const ::std::os::raw::c_char,
            description: *const ::std::os::raw::c_char,
            picked: *mut bool,
        ) -> f32,
    >,
    pub ui_multi_object: ::std::option::Option<
        unsafe extern "C" fn(
            args: *mut PropertiesUiArgsT,
            item_rect: RectT,
            parent_proxy: TtIdT,
            property: u32,
            objects: *const TtIdT,
            n: u32,
            indent: u32,
        ) -> f32,
    >,
    pub proxy_to_objects: ::std::option::Option<
        unsafe extern "C" fn(pv: *mut PropertiesViewO, proxy: TtIdT) -> *const TtIdT,
    >,
    pub multi_proxy: ::std::option::Option<
        unsafe extern "C" fn(
            pv: *mut PropertiesViewO,
            parent_proxy: TtIdT,
            property: u32,
            objects: *const TtIdT,
            n: u32,
        ) -> TtIdT,
    >,
    pub get_property_value: ::std::option::Option<
        unsafe extern "C" fn(
            pv: *mut PropertiesViewO,
            object: TtIdT,
            property: u32,
            ta: *mut TempAllocatorI,
        ) -> *const TtPropValueT,
    >,
    pub was_changed: ::std::option::Option<
        unsafe extern "C" fn(args: *mut PropertiesUiArgsT, object: TtIdT, property: u32) -> bool,
    >,
    pub internal__copy_expanded_state: ::std::option::Option<
        unsafe extern "C" fn(pv: *mut PropertiesViewO, from_key: TtIdT, to_key: TtIdT),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TreeViewO {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct TreeViewChildrenT {
    pub children: *const TtIdT,
    pub child_count: u32,
    pub property: u32,
    pub aspect_reference_type: TtTypeT,
}
impl Default for TreeViewChildrenT {
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
pub struct TreeViewAspectI {
    pub setup: ::std::option::Option<
        unsafe extern "C" fn(
            state: *mut ::std::os::raw::c_void,
            tree: *mut TreeViewO,
            tt: *const TheTruthO,
            root: TtIdT,
        ),
    >,
    pub context_menu: ::std::option::Option<
        unsafe extern "C" fn(
            state: *mut ::std::os::raw::c_void,
            tree: *mut TreeViewO,
            tt: *mut TheTruthO,
            undo_stack: *mut UndoStackI,
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            pos: Vec2T,
            parent: *mut TreeViewParentT,
            object: TtIdT,
            object_property: u32,
        ),
    >,
    pub compute_node_properties: ::std::option::Option<
        unsafe extern "C" fn(
            state: *mut ::std::os::raw::c_void,
            tree: *mut TreeViewO,
            tt: *mut TheTruthO,
            parent: *const TreeViewParentT,
            object: TtIdT,
            can_expand: *mut bool,
            can_select: *mut bool,
            can_drop: *mut bool,
            can_drag: *mut bool,
            expanded_key: *mut u64,
            ta: *mut TempAllocatorI,
        ),
    >,
    pub ui: ::std::option::Option<
        unsafe extern "C" fn(
            state: *mut ::std::os::raw::c_void,
            tree: *mut TreeViewO,
            tt: *mut TheTruthO,
            ui: *mut UiO,
            uistyle: *mut UiStyleT,
            object: TtIdT,
            parent: *mut TreeViewParentT,
            tree_has_focus: bool,
            res: *mut UiTreeItemResT,
            item_rect: RectT,
            can_drop: bool,
        ),
    >,
    pub icon: ::std::option::Option<
        unsafe extern "C" fn(
            state: *mut ::std::os::raw::c_void,
            tree: *mut TreeViewO,
            tt: *mut TheTruthO,
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            object: TtIdT,
            rect: RectT,
            res: *mut UiTreeItemResT,
        ) -> RectT,
    >,
    pub additional_elements: ::std::option::Option<
        unsafe extern "C" fn(
            state: *mut ::std::os::raw::c_void,
            tree: *mut TreeViewO,
            tt: *mut TheTruthO,
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            object: TtIdT,
            parent: *mut TreeViewParentT,
            rect: RectT,
            res: *mut UiTreeItemResT,
            metrics: *mut UiTreeItemMetricsT,
        ) -> RectT,
    >,
    pub display_name: ::std::option::Option<
        unsafe extern "C" fn(
            state: *mut ::std::os::raw::c_void,
            tree: *mut TreeViewO,
            tt: *mut TheTruthO,
            object: TtIdT,
            buffer: *mut ::std::os::raw::c_char,
            buffer_size: u32,
        ),
    >,
    pub tooltip: ::std::option::Option<
        unsafe extern "C" fn(
            state: *mut ::std::os::raw::c_void,
            tree: *mut TreeViewO,
            tt: *mut TheTruthO,
            ui: *mut UiO,
            uistyle: *const UiStyleT,
            object: TtIdT,
        ),
    >,
    pub get_node_children: ::std::option::Option<
        unsafe extern "C" fn(
            state: *mut ::std::os::raw::c_void,
            tree: *mut TreeViewO,
            tt: *mut TheTruthO,
            parent: *mut TreeViewParentT,
            object: TtIdT,
            ta: *mut TempAllocatorI,
        ) -> *mut TreeViewChildrenT,
    >,
    pub can_drop_above: ::std::option::Option<
        unsafe extern "C" fn(
            state: *mut ::std::os::raw::c_void,
            ui: *mut UiO,
            parent: *mut TreeViewParentT,
            above_object: TtIdT,
            dragged_objects: *mut TtIdT,
        ) -> bool,
    >,
    pub drop_above: ::std::option::Option<
        unsafe extern "C" fn(
            state: *mut ::std::os::raw::c_void,
            ui: *mut UiO,
            above_object: TtIdT,
            dragged_objects: *mut TtIdT,
            undo_scope: TtUndoScopeT,
        ),
    >,
}
#[repr(C)]
pub struct TreeViewSetupT {
    pub allocator: *mut AllocatorI,
    pub undo_stack: *mut UndoStackI,
    pub tt: *mut TheTruthO,
    pub keyboard_interaction: ::std::option::Option<
        unsafe extern "C" fn(
            state: *mut ::std::os::raw::c_void,
            tree: *mut TreeViewO,
            ui: *mut UiO,
            is_first_responder: bool,
        ),
    >,
    pub root: TtIdT,
    pub cache_tree_heights: bool,
    pub _padding_165: [::std::os::raw::c_char; 7usize],
    pub state: *mut ::std::os::raw::c_void,
}
impl Default for TreeViewSetupT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct TreeViewUiResT {
    pub focus_changed: bool,
    pub _padding_176: [::std::os::raw::c_char; 7usize],
    pub focus: TtIdT,
    pub selection_changed: bool,
    pub focus_on_tree_background: bool,
    pub _padding_186: [::std::os::raw::c_char; 6usize],
}
impl Default for TreeViewUiResT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct TreeViewParentT {
    pub id: TtIdT,
    pub property: u32,
    pub _padding_197: [::std::os::raw::c_char; 4usize],
    pub aspect_reference_type: TtTypeT,
    pub parent: *mut TreeViewParentT,
    pub pr: u32,
    pub _padding_216: [::std::os::raw::c_char; 4usize],
    pub expanded_key: u64,
}
impl Default for TreeViewParentT {
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
pub struct TreeViewApi {
    pub create_tree_view:
        ::std::option::Option<unsafe extern "C" fn(setup: *const TreeViewSetupT) -> *mut TreeViewO>,
    pub destroy_tree_view: ::std::option::Option<unsafe extern "C" fn(tree: *mut TreeViewO)>,
    pub ui: ::std::option::Option<
        unsafe extern "C" fn(
            tree: *mut TreeViewO,
            ui: *mut UiO,
            style: *const UiStyleT,
            rect: RectT,
        ) -> TreeViewUiResT,
    >,
    pub property_group_object:
        ::std::option::Option<unsafe extern "C" fn(object: TtIdT, property: u32) -> TtIdT>,
    pub set_expanded: ::std::option::Option<
        unsafe extern "C" fn(tree: *mut TreeViewO, object: TtIdT, expanded: bool),
    >,
    pub is_expanded:
        ::std::option::Option<unsafe extern "C" fn(tree: *mut TreeViewO, object: TtIdT) -> bool>,
    pub select: ::std::option::Option<unsafe extern "C" fn(tree: *mut TreeViewO, object: TtIdT)>,
    pub add_to_selection:
        ::std::option::Option<unsafe extern "C" fn(tree: *mut TreeViewO, object: TtIdT)>,
    pub remove_from_selection:
        ::std::option::Option<unsafe extern "C" fn(tree: *mut TreeViewO, object: TtIdT)>,
    pub scroll_to_object:
        ::std::option::Option<unsafe extern "C" fn(tree: *mut TreeViewO, object: TtIdT)>,
    pub deselect_all: ::std::option::Option<unsafe extern "C" fn(tree: *mut TreeViewO)>,
    pub rename: ::std::option::Option<unsafe extern "C" fn(tree: *mut TreeViewO, object: TtIdT)>,
    pub get_sorted_subobjects: ::std::option::Option<
        unsafe extern "C" fn(
            tree: *const TreeViewO,
            object: TtIdT,
            property: u32,
            ta: *mut TempAllocatorI,
        ) -> *const TtIdT,
    >,
    pub selected_objects: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut TreeViewO, ta: *mut TempAllocatorI) -> *const TtIdT,
    >,
    pub get_root_object: ::std::option::Option<unsafe extern "C" fn(inst: *mut TreeViewO) -> TtIdT>,
    pub set_root_object:
        ::std::option::Option<unsafe extern "C" fn(inst: *mut TreeViewO, object: TtIdT)>,
    pub add_default_menu_items: ::std::option::Option<
        unsafe extern "C" fn(
            tree: *mut TreeViewO,
            parent: *mut TreeViewParentT,
            object: TtIdT,
            object_property: u32,
            id_base: u64,
            items: *mut *mut UiMenuItemT,
            ta: *mut TempAllocatorI,
        ),
    >,
    pub execute_menu_item: ::std::option::Option<
        unsafe extern "C" fn(
            tree: *mut TreeViewO,
            parent: *mut TreeViewParentT,
            object: TtIdT,
            object_property: u32,
            id_base: u64,
            res: *const UiMenuResultT,
        ),
    >,
    pub set_empty_text: ::std::option::Option<
        unsafe extern "C" fn(tree: *mut TreeViewO, text: *const ::std::os::raw::c_char),
    >,
    pub filter_nodes: ::std::option::Option<
        unsafe extern "C" fn(
            tree: *mut TreeViewO,
            node_type: u64,
            text: *const ::std::os::raw::c_char,
            case_unsensitive: bool,
        ),
    >,
    pub renaming_object: ::std::option::Option<unsafe extern "C" fn(tree: *mut TreeViewO) -> TtIdT>,
}
#[repr(C)]
pub struct UiPopupItemPickerT {
    pub id: u64,
    pub pos: Vec2T,
    pub search_text: *mut ::std::os::raw::c_char,
    pub search_text_bytes: u32,
    pub _padding_23: [::std::os::raw::c_char; 4usize],
    pub strings: *const *const ::std::os::raw::c_char,
    pub num_strings: u32,
    pub _padding_28: [::std::os::raw::c_char; 4usize],
}
impl Default for UiPopupItemPickerT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiCategoryPopupItemPickerT {
    pub id: u64,
    pub pos: Vec2T,
    pub search_text: *mut ::std::os::raw::c_char,
    pub search_text_bytes: u32,
    pub _padding_38: [::std::os::raw::c_char; 4usize],
    pub strings: *const *const ::std::os::raw::c_char,
    pub categories: *const *const ::std::os::raw::c_char,
    pub num_strings: u32,
    pub _padding_44: [::std::os::raw::c_char; 4usize],
    pub expanded: *mut SetT,
}
impl Default for UiCategoryPopupItemPickerT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiPopupAssetPickerT {
    pub id: u64,
    pub pos: Vec2T,
    pub search_text: *mut ::std::os::raw::c_char,
    pub search_text_bytes: u32,
    pub _padding_56: [::std::os::raw::c_char; 4usize],
    pub tt: *mut TheTruthO,
    pub asset_root: TtIdT,
    pub type_hash: StrhashT,
    pub required_labels: *mut AssetLabelBitflagT,
}
impl Default for UiPopupAssetPickerT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct UiTexteditPickerT {
    pub id: u64,
    pub rect: RectT,
    pub search_text: *mut ::std::os::raw::c_char,
    pub not_found_text: *const ::std::os::raw::c_char,
    pub default_text: *const ::std::os::raw::c_char,
    pub strings: *const *const ::std::os::raw::c_char,
    pub num_of_strings: u32,
    pub search_text_bytes: u32,
    pub no_suggestions: bool,
    pub _padding_86: [::std::os::raw::c_char; 7usize],
}
impl Default for UiTexteditPickerT {
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
pub struct UiPopupItemPickerApi {
    pub pick: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle_in: *const UiStyleT,
            c: *const UiPopupItemPickerT,
            picked: *mut u32,
        ) -> bool,
    >,
    pub pick_with_categories: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle_in: *const UiStyleT,
            c: *const UiCategoryPopupItemPickerT,
            picked: *mut u32,
        ) -> bool,
    >,
    pub pick_asset: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle_in: *const UiStyleT,
            c: *const UiPopupAssetPickerT,
            asset: *mut TtIdT,
        ) -> bool,
    >,
    pub pick_textedit: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            uistyle_in: *const UiStyleT,
            config: *const UiTexteditPickerT,
            picked: *mut u32,
            not_in_list: *mut bool,
        ) -> bool,
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct AssetBrowserCreateAssetO {
    pub _address: u8,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct AssetBrowserOpenAssetO {
    pub _address: u8,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct AssetBrowserAddAssetO {
    pub _address: u8,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct AssetBrowserSelectAssetO {
    pub _address: u8,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct GraphO {
    pub _address: u8,
}

// Extensions generated by machinery-generator

use const_cstr::{const_cstr, ConstCStr};

use crate::tm::foundation::*;
use crate::tm::plugins::ui::*;

impl AssetBrowserOpenAssetApi {
    pub unsafe fn open(
        &self,
        inst: *mut AssetBrowserOpenAssetO,
        ui: *mut UiO,
        from_tab: *mut TabI,
        object: TtIdT,
    ) {
        self.open.unwrap()(inst, ui, from_tab, object)
    }

    pub unsafe fn can_open(
        &self,
        inst: *mut AssetBrowserOpenAssetO,
        ui: *mut UiO,
        object: TtIdT,
    ) -> bool {
        self.can_open.unwrap()(inst, ui, object)
    }
}

impl crate::Api for AssetBrowserOpenAssetApi {
    const NAME: ConstCStr = const_cstr!("tm_asset_browser_open_asset_api");
}

impl AssetBrowserAddAssetApi {
    pub unsafe fn current_directory(
        &self,
        inst: *mut AssetBrowserAddAssetO,
        ui: *mut UiO,
    ) -> TtIdT {
        self.current_directory.unwrap()(inst, ui)
    }

    pub unsafe fn exists(
        &self,
        inst: *mut AssetBrowserAddAssetO,
        directory: TtIdT,
        name: *const ::std::os::raw::c_char,
    ) -> TtIdT {
        self.exists.unwrap()(inst, directory, name)
    }

    pub unsafe fn add(
        &self,
        inst: *mut AssetBrowserAddAssetO,
        directory: TtIdT,
        object: TtIdT,
        name: *const ::std::os::raw::c_char,
        undo_scope: TtUndoScopeT,
        select: bool,
        ui: *mut UiO,
        asset_labels: *mut u64,
        num_asset_labels: u32,
    ) {
        self.add.unwrap()(
            inst,
            directory,
            object,
            name,
            undo_scope,
            select,
            ui,
            asset_labels,
            num_asset_labels,
        )
    }

    pub unsafe fn add_directory(
        &self,
        inst: *mut AssetBrowserAddAssetO,
        parent_directory: TtIdT,
        name: *const ::std::os::raw::c_char,
        undo_scope: TtUndoScopeT,
        select: bool,
        ui: *mut UiO,
    ) -> TtIdT {
        self.add_directory.unwrap()(inst, parent_directory, name, undo_scope, select, ui)
    }
}

impl crate::Api for AssetBrowserAddAssetApi {
    const NAME: ConstCStr = const_cstr!("tm_asset_browser_add_asset_api");
}

impl AssetBrowserSelectAssetApi {
    pub unsafe fn select_asset(
        &self,
        inst: *mut AssetBrowserSelectAssetO,
        ui: *mut UiO,
        asset: TtIdT,
    ) {
        self.select_asset.unwrap()(inst, ui, asset)
    }

    pub unsafe fn select_assets(
        &self,
        inst: *mut AssetBrowserSelectAssetO,
        ui: *mut UiO,
        assets: *mut TtIdT,
        count: u32,
    ) {
        self.select_assets.unwrap()(inst, ui, assets, count)
    }
}

impl crate::Api for AssetBrowserSelectAssetApi {
    const NAME: ConstCStr = const_cstr!("tm_asset_browser_select_asset_api");
}

impl AssetBrowserApi {
    pub unsafe fn create_asset_browser(
        &self,
        allocator: *mut AllocatorI,
        config: *const AssetBrowserConfigT,
    ) -> *mut AssetBrowserO {
        self.create_asset_browser.unwrap()(allocator, config)
    }

    pub unsafe fn destroy_asset_browser(&self, asset_browser: *mut AssetBrowserO) {
        self.destroy_asset_browser.unwrap()(asset_browser)
    }

    pub unsafe fn ui(
        &self,
        inst: *mut AssetBrowserO,
        asset_root: TtIdT,
        ui: *mut UiO,
        style: *const UiStyleT,
        rect: RectT,
        tab_id: u64,
    ) -> AssetBrowserUiResT {
        self.ui.unwrap()(inst, asset_root, ui, style, rect, tab_id)
    }

    pub unsafe fn set_new_truth(
        &self,
        inst: *mut AssetBrowserO,
        tt: *mut TheTruthO,
        asset_browser: TtIdT,
    ) {
        self.set_new_truth.unwrap()(inst, tt, asset_browser)
    }

    pub unsafe fn menu(
        &self,
        inst: *mut AssetBrowserO,
        asset_root: TtIdT,
        ui: *mut UiO,
        style: *const UiStyleT,
        pos: Vec2T,
    ) {
        self.menu.unwrap()(inst, asset_root, ui, style, pos)
    }

    pub unsafe fn process_dropped_os_files(
        &self,
        inst: *mut AssetBrowserO,
        ui: *mut UiO,
        asset_root: TtIdT,
        files: *mut *mut ::std::os::raw::c_char,
        num_files: u32,
    ) {
        self.process_dropped_os_files.unwrap()(inst, ui, asset_root, files, num_files)
    }

    pub unsafe fn focused_object(&self, inst: *mut AssetBrowserO) -> TtIdT {
        self.focused_object.unwrap()(inst)
    }

    pub unsafe fn selected_objects(
        &self,
        inst: *mut AssetBrowserO,
        ta: *mut TempAllocatorI,
    ) -> *const TtIdT {
        self.selected_objects.unwrap()(inst, ta)
    }
}

impl crate::Api for AssetBrowserApi {
    const NAME: ConstCStr = const_cstr!("tm_asset_browser_api");
}

impl AssetLabelApi {
    pub unsafe fn all_labels(
        &self,
        labels: *mut AssetLabelsT,
        tt: *const TheTruthO,
        asset_root: TtIdT,
    ) {
        self.all_labels.unwrap()(labels, tt, asset_root)
    }

    pub unsafe fn labels_of_asset(
        &self,
        tt: *mut TheTruthO,
        asset: TtIdT,
        asset_labels: *const AssetLabelsT,
        out_labels: *mut AssetLabelsT,
    ) {
        self.labels_of_asset.unwrap()(tt, asset, asset_labels, out_labels)
    }

    pub unsafe fn label_uuids_of_asset(
        &self,
        tt: *const TheTruthO,
        asset: TtIdT,
        ta: *mut TempAllocatorI,
    ) -> *mut u64 {
        self.label_uuids_of_asset.unwrap()(tt, asset, ta)
    }

    pub unsafe fn create(
        &self,
        tt: *mut TheTruthO,
        asset_root: TtIdT,
        directory: TtIdT,
        label: *const ::std::os::raw::c_char,
        undo_scope: TtUndoScopeT,
    ) -> AssetLabelUuidT {
        self.create.unwrap()(tt, asset_root, directory, label, undo_scope)
    }

    pub unsafe fn add(
        &self,
        tt: *mut TheTruthO,
        asset: TtIdT,
        label: AssetLabelUuidT,
        undo_scope: TtUndoScopeT,
    ) {
        self.add.unwrap()(tt, asset, label, undo_scope)
    }

    pub unsafe fn remove(
        &self,
        tt: *mut TheTruthO,
        asset: TtIdT,
        label: AssetLabelUuidT,
        undo_scope: TtUndoScopeT,
    ) {
        self.remove.unwrap()(tt, asset, label, undo_scope)
    }

    pub unsafe fn rename(
        &self,
        tt: *mut TheTruthO,
        asset_root: TtIdT,
        label: AssetLabelUuidT,
        new_label_name: *const ::std::os::raw::c_char,
        undo_scope: TtUndoScopeT,
    ) {
        self.rename.unwrap()(tt, asset_root, label, new_label_name, undo_scope)
    }

    pub unsafe fn assets_with_label(
        &self,
        tt: *mut TheTruthO,
        asset_root: TtIdT,
        label: AssetLabelUuidT,
        ta: *mut TempAllocatorI,
    ) -> *mut TtIdT {
        self.assets_with_label.unwrap()(tt, asset_root, label, ta)
    }

    pub unsafe fn assets_with_all_labels(
        &self,
        tt: *mut TheTruthO,
        asset_root: TtIdT,
        labels: *const AssetLabelBitflagT,
        ta: *mut TempAllocatorI,
    ) -> *mut TtIdT {
        self.assets_with_all_labels.unwrap()(tt, asset_root, labels, ta)
    }

    pub unsafe fn assets_with_any_label(
        &self,
        tt: *mut TheTruthO,
        asset_root: TtIdT,
        labels: *const AssetLabelBitflagT,
        ta: *mut TempAllocatorI,
    ) -> *mut TtIdT {
        self.assets_with_any_label.unwrap()(tt, asset_root, labels, ta)
    }

    pub unsafe fn toggle_bitflags(
        &self,
        labels: *const AssetLabelsT,
        bitflag: *mut AssetLabelBitflagT,
        labels_uuid: *const AssetLabelUuidT,
        num_labels: u32,
    ) {
        self.toggle_bitflags.unwrap()(labels, bitflag, labels_uuid, num_labels)
    }

    pub unsafe fn matches_all(
        &self,
        asset_bitflag: *const AssetLabelBitflagT,
        filter_bitflag: *const AssetLabelBitflagT,
    ) -> bool {
        self.matches_all.unwrap()(asset_bitflag, filter_bitflag)
    }

    pub unsafe fn matches_any(
        &self,
        asset_bitflag: *const AssetLabelBitflagT,
        filter_bitflag: *const AssetLabelBitflagT,
    ) -> bool {
        self.matches_any.unwrap()(asset_bitflag, filter_bitflag)
    }
}

impl crate::Api for AssetLabelApi {
    const NAME: ConstCStr = const_cstr!("tm_asset_label_api");
}

impl GraphApi {
    pub unsafe fn create_truth_types(&self, tt: *mut TheTruthO) {
        self.create_truth_types.unwrap()(tt)
    }

    pub unsafe fn create_graph(
        &self,
        allocator: *mut AllocatorI,
        tt: *mut TheTruthO,
        graph_id: TtIdT,
        node_interface_name: *const ::std::os::raw::c_char,
        undo_stack: *mut UndoStackI,
        asset_root: TtIdT,
        settings_tt: *mut TheTruthO,
        settings_object: TtIdT,
    ) -> *mut GraphO {
        self.create_graph.unwrap()(
            allocator,
            tt,
            graph_id,
            node_interface_name,
            undo_stack,
            asset_root,
            settings_tt,
            settings_object,
        )
    }

    pub unsafe fn destroy_graph(&self, graph: *mut GraphO, truth_already_destroyed: bool) {
        self.destroy_graph.unwrap()(graph, truth_already_destroyed)
    }

    pub unsafe fn set_settings_object(
        &self,
        graph: *mut GraphO,
        settings_tt: *mut TheTruthO,
        settings_object: TtIdT,
    ) {
        self.set_settings_object.unwrap()(graph, settings_tt, settings_object)
    }

    pub unsafe fn id(&self, graph: *mut GraphO) -> TtIdT {
        self.id.unwrap()(graph)
    }

    pub unsafe fn ui(
        &self,
        inst: *mut GraphO,
        ui: *mut UiO,
        style: *const UiStyleT,
        rect: RectT,
        tab: *mut TabI,
    ) -> GraphUiResT {
        self.ui.unwrap()(inst, ui, style, rect, tab)
    }

    pub unsafe fn menu(&self, inst: *mut GraphO, ui: *mut UiO, style: *const UiStyleT, pos: Vec2T) {
        self.menu.unwrap()(inst, ui, style, pos)
    }

    pub unsafe fn selected_objects(
        &self,
        inst: *mut GraphO,
        ta: *mut TempAllocatorI,
    ) -> *const TtIdT {
        self.selected_objects.unwrap()(inst, ta)
    }

    pub unsafe fn refresh_node_types(
        &self,
        inst: *mut GraphO,
        node_interface_name: *const ::std::os::raw::c_char,
    ) {
        self.refresh_node_types.unwrap()(inst, node_interface_name)
    }

    pub unsafe fn toolbars(&self, inst: *mut GraphO, ta: *mut TempAllocatorI) -> *mut ToolbarI {
        self.toolbars.unwrap()(inst, ta)
    }

    pub unsafe fn migrate_to_connector_hash(
        &self,
        tt: *mut TheTruthO,
        graphs: *mut TtIdT,
        num_graphs: u32,
        node_types: *const *const GraphNodeTypeI,
        num_node_types: u32,
    ) -> bool {
        self.migrate_to_connector_hash.unwrap()(tt, graphs, num_graphs, node_types, num_node_types)
    }

    pub unsafe fn needs_migrate_to_connector_hash(&self, tt: *mut TheTruthO) -> bool {
        self.needs_migrate_to_connector_hash.unwrap()(tt)
    }

    pub unsafe fn migrate_to_instantiated_connections(
        &self,
        tt: *mut TheTruthO,
        graphs: *mut TtIdT,
        num_graphs: u32,
    ) -> bool {
        self.migrate_to_instantiated_connections.unwrap()(tt, graphs, num_graphs)
    }
}

impl crate::Api for GraphApi {
    const NAME: ConstCStr = const_cstr!("tm_graph_api");
}

impl ProfilerViewApi {
    pub unsafe fn create_profiler_view(&self, allocator: *mut AllocatorI) -> *mut ProfilerViewO {
        self.create_profiler_view.unwrap()(allocator)
    }

    pub unsafe fn destroy_profiler_view(&self, profiler_view: *mut ProfilerViewO) {
        self.destroy_profiler_view.unwrap()(profiler_view)
    }

    pub unsafe fn ui(
        &self,
        inst: *mut ProfilerViewO,
        ui: *mut UiO,
        style: *const UiStyleT,
        rect: RectT,
        tab_id: u64,
    ) -> ProfilerViewUiResT {
        self.ui.unwrap()(inst, ui, style, rect, tab_id)
    }

    pub unsafe fn start_recording(&self, inst: *mut ProfilerViewO, with_history: bool) {
        self.start_recording.unwrap()(inst, with_history)
    }

    pub unsafe fn stop_recording(&self, inst: *mut ProfilerViewO) {
        self.stop_recording.unwrap()(inst)
    }

    pub unsafe fn is_recording(&self, inst: *mut ProfilerViewO) -> bool {
        self.is_recording.unwrap()(inst)
    }

    pub unsafe fn menu(
        &self,
        inst: *mut ProfilerViewO,
        ui: *mut UiO,
        style: *const UiStyleT,
        pos: Vec2T,
    ) {
        self.menu.unwrap()(inst, ui, style, pos)
    }
}

impl crate::Api for ProfilerViewApi {
    const NAME: ConstCStr = const_cstr!("tm_profiler_view_api");
}

impl PropertiesViewApi {
    pub unsafe fn create_properties_view(
        &self,
        allocator: *mut AllocatorI,
        config: *const PropertiesViewConfigT,
    ) -> *mut PropertiesViewO {
        self.create_properties_view.unwrap()(allocator, config)
    }

    pub unsafe fn destroy_properties_view(&self, inst: *mut PropertiesViewO) {
        self.destroy_properties_view.unwrap()(inst)
    }

    pub unsafe fn set_object(&self, inst: *mut PropertiesViewO, object: TtIdT) -> bool {
        self.set_object.unwrap()(inst, object)
    }

    pub unsafe fn set_object_with_mask(
        &self,
        inst: *mut PropertiesViewO,
        object: TtIdT,
        property_mask: u64,
    ) -> bool {
        self.set_object_with_mask.unwrap()(inst, object, property_mask)
    }

    pub unsafe fn set_mask(&self, inst: *mut PropertiesViewO, property_mask: u64) -> bool {
        self.set_mask.unwrap()(inst, property_mask)
    }

    pub unsafe fn set_objects(
        &self,
        inst: *mut PropertiesViewO,
        objects: *const TtIdT,
        n: u32,
    ) -> bool {
        self.set_objects.unwrap()(inst, objects, n)
    }

    pub unsafe fn objects(&self, inst: *mut PropertiesViewO) -> *const TtIdT {
        self.objects.unwrap()(inst)
    }

    pub unsafe fn ui(
        &self,
        inst: *mut PropertiesViewO,
        asset_root: TtIdT,
        ui: *mut UiO,
        style: *const UiStyleT,
        rect: RectT,
        tab: *mut TabI,
    ) {
        self.ui.unwrap()(inst, asset_root, ui, style, rect, tab)
    }

    pub unsafe fn update(&self, inst: *mut PropertiesViewO, ui: *mut UiO) {
        self.update.unwrap()(inst, ui)
    }

    pub unsafe fn ui_info(&self, inst: *mut PropertiesViewO) -> *mut PropertiesUiInfoT {
        self.ui_info.unwrap()(inst)
    }

    pub unsafe fn metrics(&self, inst: *mut PropertiesViewO) -> *mut f32 {
        self.metrics.unwrap()(inst)
    }

    pub unsafe fn get_display_name(
        &self,
        tt: *const TheTruthO,
        object: TtIdT,
        buffer: *mut ::std::os::raw::c_char,
        size: u32,
    ) {
        self.get_display_name.unwrap()(tt, object, buffer, size)
    }

    pub unsafe fn get_type_display_name(
        &self,
        tt: *const TheTruthO,
        object_type: TtTypeT,
        buffer: *mut ::std::os::raw::c_char,
        size: u32,
    ) {
        self.get_type_display_name.unwrap()(tt, object_type, buffer, size)
    }

    pub unsafe fn ui_object(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        object: TtIdT,
        indent: u32,
    ) -> f32 {
        self.ui_object.unwrap()(args, item_rect, object, indent)
    }

    pub unsafe fn ui_object_default(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        object: TtIdT,
        indent: u32,
    ) -> f32 {
        self.ui_object_default.unwrap()(args, item_rect, object, indent)
    }

    pub unsafe fn ui_property(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        object: TtIdT,
        indent: u32,
        property: u32,
    ) -> f32 {
        self.ui_property.unwrap()(args, item_rect, object, indent, property)
    }

    pub unsafe fn ui_property_with_name(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        indent: u32,
        property: u32,
    ) -> f32 {
        self.ui_property_with_name.unwrap()(
            args, item_rect, name, tooltip, object, indent, property,
        )
    }

    pub unsafe fn ui_property_default(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        object: TtIdT,
        indent: u32,
        property: u32,
    ) -> f32 {
        self.ui_property_default.unwrap()(args, item_rect, object, indent, property)
    }

    pub unsafe fn ui_property_default_with_name(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        indent: u32,
        property: u32,
    ) -> f32 {
        self.ui_property_default_with_name.unwrap()(
            args, item_rect, name, tooltip, object, indent, property,
        )
    }

    pub unsafe fn ui_tree_item(
        &self,
        args: *mut PropertiesUiArgsT,
        key: u64,
        rect: RectT,
        indent: u32,
        expanded_default: bool,
        tooltip: *const ::std::os::raw::c_char,
        ui_id: u64,
    ) -> bool {
        self.ui_tree_item.unwrap()(args, key, rect, indent, expanded_default, tooltip, ui_id)
    }

    pub unsafe fn ui_group(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        indent: u32,
        expanded_default: bool,
        is_expanded: *mut bool,
    ) -> f32 {
        self.ui_group.unwrap()(
            args,
            item_rect,
            name,
            tooltip,
            object,
            indent,
            expanded_default,
            is_expanded,
        )
    }

    pub unsafe fn show_subobject_menu(
        &self,
        tt: *mut TheTruthO,
        undo_stack: *mut UndoStackI,
        ui: *mut UiO,
        uistyle: *mut UiStyleT,
        object: TtIdT,
        property: u32,
        sub: TtIdT,
        pos: Vec2T,
    ) -> TtUndoScopeT {
        self.show_subobject_menu.unwrap()(tt, undo_stack, ui, uistyle, object, property, sub, pos)
    }

    pub unsafe fn begin_subobject_menu_scope(
        &self,
        args: *mut PropertiesUiArgsT,
        rect: RectT,
        object: TtIdT,
        property: u32,
        sub: TtIdT,
        ui_id: u64,
    ) {
        self.begin_subobject_menu_scope.unwrap()(args, rect, object, property, sub, ui_id)
    }

    pub unsafe fn end_subobject_menu_scope(
        &self,
        args: *mut PropertiesUiArgsT,
        object: TtIdT,
        property: u32,
        sub: TtIdT,
        ui_id: u64,
    ) {
        self.end_subobject_menu_scope.unwrap()(args, object, property, sub, ui_id)
    }

    pub unsafe fn begin_context_menu_scope(&self, pv: *mut PropertiesViewO, rect: RectT) {
        self.begin_context_menu_scope.unwrap()(pv, rect)
    }

    pub unsafe fn end_context_menu_scope(
        &self,
        pv: *mut PropertiesViewO,
        out_id: *mut u64,
        out_text: *mut *mut ::std::os::raw::c_char,
    ) {
        self.end_context_menu_scope.unwrap()(pv, out_id, out_text)
    }

    pub unsafe fn selected_context_menu_item(
        &self,
        pv: *mut PropertiesViewO,
        out_id: *mut u64,
        out_text: *mut *mut ::std::os::raw::c_char,
    ) {
        self.selected_context_menu_item.unwrap()(pv, out_id, out_text)
    }

    pub unsafe fn add_context_menu_items(
        &self,
        pv: *mut PropertiesViewO,
        items: *const UiMenuItemT,
        num_items: u32,
    ) {
        self.add_context_menu_items.unwrap()(pv, items, num_items)
    }

    pub unsafe fn context_menu(&self, args: *mut PropertiesUiArgsT) {
        self.context_menu.unwrap()(args)
    }

    pub unsafe fn ui_bool(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
    ) -> f32 {
        self.ui_bool.unwrap()(args, item_rect, name, tooltip, object, property)
    }

    pub unsafe fn ui_uint32(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        converter: *mut PropertiesUint32DisplayConverterI,
    ) -> f32 {
        self.ui_uint32.unwrap()(args, item_rect, name, tooltip, object, property, converter)
    }

    pub unsafe fn ui_uint64(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
    ) -> f32 {
        self.ui_uint64.unwrap()(args, item_rect, name, tooltip, object, property)
    }

    pub unsafe fn ui_float(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        converter: *const PropertiesFloatDisplayConverterI,
    ) -> f32 {
        self.ui_float.unwrap()(args, item_rect, name, tooltip, object, property, converter)
    }

    pub unsafe fn ui_double(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
    ) -> f32 {
        self.ui_double.unwrap()(args, item_rect, name, tooltip, object, property)
    }

    pub unsafe fn ui_string(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
    ) -> f32 {
        self.ui_string.unwrap()(args, item_rect, name, tooltip, object, property)
    }

    pub unsafe fn ui_reference(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
    ) -> f32 {
        self.ui_reference.unwrap()(args, item_rect, name, tooltip, object, property)
    }

    pub unsafe fn ui_reference_args(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        picker_args: *const PropertiesReferencePickerArgsT,
    ) -> f32 {
        self.ui_reference_args.unwrap()(
            args,
            item_rect,
            name,
            tooltip,
            object,
            property,
            picker_args,
        )
    }

    pub unsafe fn ui_reference_entity_picker(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        entity_root: TtIdT,
    ) -> f32 {
        self.ui_reference_entity_picker.unwrap()(
            args,
            item_rect,
            name,
            tooltip,
            object,
            property,
            entity_root,
        )
    }

    pub unsafe fn ui_subobject(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        indent: u32,
        expanded_by_default: bool,
    ) -> f32 {
        self.ui_subobject.unwrap()(
            args,
            item_rect,
            name,
            tooltip,
            object,
            property,
            indent,
            expanded_by_default,
        )
    }

    pub unsafe fn ui_subobject_direct(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        parent: TtIdT,
        subobject: TtIdT,
        indent: u32,
        expanded_by_default: bool,
    ) -> f32 {
        self.ui_subobject_direct.unwrap()(
            args,
            item_rect,
            name,
            tooltip,
            parent,
            subobject,
            indent,
            expanded_by_default,
        )
    }

    pub unsafe fn ui_reference_set(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        indent: u32,
    ) -> f32 {
        self.ui_reference_set.unwrap()(args, item_rect, name, tooltip, object, property, indent)
    }

    pub unsafe fn ui_subobject_set(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        indent: u32,
    ) -> f32 {
        self.ui_subobject_set.unwrap()(args, item_rect, name, tooltip, object, property, indent)
    }

    pub unsafe fn ui_subobject_set_reorderable(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        indent: u32,
        item_order_property: u32,
        draw_header: bool,
    ) -> f32 {
        self.ui_subobject_set_reorderable.unwrap()(
            args,
            item_rect,
            name,
            tooltip,
            object,
            property,
            indent,
            item_order_property,
            draw_header,
        )
    }

    pub unsafe fn ui_subobject_set_item_header(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        object: TtIdT,
        property: u32,
        sub: TtIdT,
        indent: u32,
        custom_name: *const ::std::os::raw::c_char,
        expanded_by_default: bool,
    ) -> bool {
        self.ui_subobject_set_item_header.unwrap()(
            args,
            item_rect,
            object,
            property,
            sub,
            indent,
            custom_name,
            expanded_by_default,
        )
    }

    pub unsafe fn ui_subobject_set_item(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        object: TtIdT,
        property: u32,
        sub: TtIdT,
        indent: u32,
        expanded_by_default: bool,
    ) -> f32 {
        self.ui_subobject_set_item.unwrap()(
            args,
            item_rect,
            object,
            property,
            sub,
            indent,
            expanded_by_default,
        )
    }

    pub unsafe fn ui_uint32_popup_picker(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        items: *mut *const ::std::os::raw::c_char,
        num_items: u32,
    ) -> f32 {
        self.ui_uint32_popup_picker.unwrap()(
            args, item_rect, name, tooltip, object, property, items, num_items,
        )
    }

    pub unsafe fn ui_uint64_popup_picker(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        items: *mut *const ::std::os::raw::c_char,
        items_values: *const u64,
        num_items: u32,
    ) -> f32 {
        self.ui_uint64_popup_picker.unwrap()(
            args,
            item_rect,
            name,
            tooltip,
            object,
            property,
            items,
            items_values,
            num_items,
        )
    }

    pub unsafe fn ui_reference_popup_picker(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        items: *mut *const ::std::os::raw::c_char,
        items_references: *const TtIdT,
        num_items: u32,
    ) -> f32 {
        self.ui_reference_popup_picker.unwrap()(
            args,
            item_rect,
            name,
            tooltip,
            object,
            property,
            items,
            items_references,
            num_items,
        )
    }

    pub unsafe fn ui_string_popup_picker(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        items: *mut *const ::std::os::raw::c_char,
        num_items: u32,
    ) -> f32 {
        self.ui_string_popup_picker.unwrap()(
            args, item_rect, name, tooltip, object, property, items, num_items,
        )
    }

    pub unsafe fn popup_pick_with_categories(
        &self,
        args: *mut PropertiesUiArgsT,
        ui_id: u64,
        pos: Vec2T,
        strings: *const *const ::std::os::raw::c_char,
        categories: *const *const ::std::os::raw::c_char,
        num_strings: u32,
        pick_index: *mut u32,
    ) -> bool {
        self.popup_pick_with_categories.unwrap()(
            args,
            ui_id,
            pos,
            strings,
            categories,
            num_strings,
            pick_index,
        )
    }

    pub unsafe fn ui_uint32_dropdown(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        items: *mut *const ::std::os::raw::c_char,
        item_tooltips: *mut *const ::std::os::raw::c_char,
        items_uint32: *const u32,
        num_items: u32,
    ) -> f32 {
        self.ui_uint32_dropdown.unwrap()(
            args,
            item_rect,
            name,
            tooltip,
            object,
            property,
            items,
            item_tooltips,
            items_uint32,
            num_items,
        )
    }

    pub unsafe fn ui_uint64_dropdown(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        items: *mut *const ::std::os::raw::c_char,
        item_tooltips: *mut *const ::std::os::raw::c_char,
        items_uint64: *const u64,
        num_items: u32,
    ) -> f32 {
        self.ui_uint64_dropdown.unwrap()(
            args,
            item_rect,
            name,
            tooltip,
            object,
            property,
            items,
            item_tooltips,
            items_uint64,
            num_items,
        )
    }

    pub unsafe fn ui_float_slider(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        slider: *const PropertiesFloatSliderT,
    ) -> f32 {
        self.ui_float_slider.unwrap()(args, item_rect, name, tooltip, object, property, slider)
    }

    pub unsafe fn ui_vec2(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        vec: TtIdT,
    ) -> f32 {
        self.ui_vec2.unwrap()(args, item_rect, name, tooltip, vec)
    }

    pub unsafe fn ui_vec3(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        vec: TtIdT,
    ) -> f32 {
        self.ui_vec3.unwrap()(args, item_rect, name, tooltip, vec)
    }

    pub unsafe fn ui_vec4(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        vec: TtIdT,
    ) -> f32 {
        self.ui_vec4.unwrap()(args, item_rect, name, tooltip, vec)
    }

    pub unsafe fn ui_color_picker(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        color: TtIdT,
    ) -> f32 {
        self.ui_color_picker.unwrap()(args, item_rect, name, tooltip, color)
    }

    pub unsafe fn ui_color_temperature(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        tooltip: *const ::std::os::raw::c_char,
        color: TtIdT,
    ) -> f32 {
        self.ui_color_temperature.unwrap()(args, item_rect, tooltip, color)
    }

    pub unsafe fn ui_color_button(
        &self,
        args: *mut PropertiesUiArgsT,
        color_button: *const UiButtonT,
        color: TtIdT,
    ) -> bool {
        self.ui_color_button.unwrap()(args, color_button, color)
    }

    pub unsafe fn ui_expanded_color_picker(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        color: TtIdT,
    ) -> f32 {
        self.ui_expanded_color_picker.unwrap()(args, item_rect, color)
    }

    pub unsafe fn ui_rotation(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        rotation: TtIdT,
    ) -> f32 {
        self.ui_rotation.unwrap()(args, item_rect, name, tooltip, rotation)
    }

    pub unsafe fn ui_visibility_flags(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        indent: u32,
    ) -> f32 {
        self.ui_visibility_flags.unwrap()(args, item_rect, name, tooltip, object, property, indent)
    }

    pub unsafe fn ui_label(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
    ) -> f32 {
        self.ui_label.unwrap()(args, item_rect, name, tooltip)
    }

    pub unsafe fn ui_horizontal_line(&self, args: *mut PropertiesUiArgsT, item_rect: RectT) -> f32 {
        self.ui_horizontal_line.unwrap()(args, item_rect)
    }

    pub unsafe fn ui_static_text(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        text: *const ::std::os::raw::c_char,
    ) -> f32 {
        self.ui_static_text.unwrap()(args, item_rect, name, tooltip, text)
    }

    pub unsafe fn ui_prototype(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        parent: TtIdT,
        property: u32,
        subobject: TtIdT,
        pick_type_hash: StrhashT,
    ) -> f32 {
        self.ui_prototype.unwrap()(
            args,
            item_rect,
            name,
            tooltip,
            parent,
            property,
            subobject,
            pick_type_hash,
        )
    }

    pub unsafe fn ui_open_path(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        extensions: *const ::std::os::raw::c_char,
        description: *const ::std::os::raw::c_char,
        picked: *mut bool,
    ) -> f32 {
        self.ui_open_path.unwrap()(
            args,
            item_rect,
            name,
            tooltip,
            object,
            property,
            extensions,
            description,
            picked,
        )
    }

    pub unsafe fn ui_save_path(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        name: *const ::std::os::raw::c_char,
        tooltip: *const ::std::os::raw::c_char,
        object: TtIdT,
        property: u32,
        extensions: *const ::std::os::raw::c_char,
        description: *const ::std::os::raw::c_char,
        picked: *mut bool,
    ) -> f32 {
        self.ui_save_path.unwrap()(
            args,
            item_rect,
            name,
            tooltip,
            object,
            property,
            extensions,
            description,
            picked,
        )
    }

    pub unsafe fn ui_multi_object(
        &self,
        args: *mut PropertiesUiArgsT,
        item_rect: RectT,
        parent_proxy: TtIdT,
        property: u32,
        objects: *const TtIdT,
        n: u32,
        indent: u32,
    ) -> f32 {
        self.ui_multi_object.unwrap()(args, item_rect, parent_proxy, property, objects, n, indent)
    }

    pub unsafe fn proxy_to_objects(&self, pv: *mut PropertiesViewO, proxy: TtIdT) -> *const TtIdT {
        self.proxy_to_objects.unwrap()(pv, proxy)
    }

    pub unsafe fn multi_proxy(
        &self,
        pv: *mut PropertiesViewO,
        parent_proxy: TtIdT,
        property: u32,
        objects: *const TtIdT,
        n: u32,
    ) -> TtIdT {
        self.multi_proxy.unwrap()(pv, parent_proxy, property, objects, n)
    }

    pub unsafe fn get_property_value(
        &self,
        pv: *mut PropertiesViewO,
        object: TtIdT,
        property: u32,
        ta: *mut TempAllocatorI,
    ) -> *const TtPropValueT {
        self.get_property_value.unwrap()(pv, object, property, ta)
    }

    pub unsafe fn was_changed(
        &self,
        args: *mut PropertiesUiArgsT,
        object: TtIdT,
        property: u32,
    ) -> bool {
        self.was_changed.unwrap()(args, object, property)
    }
}

impl crate::Api for PropertiesViewApi {
    const NAME: ConstCStr = const_cstr!("tm_properties_view_api");
}

impl TreeViewApi {
    pub unsafe fn create_tree_view(&self, setup: *const TreeViewSetupT) -> *mut TreeViewO {
        self.create_tree_view.unwrap()(setup)
    }

    pub unsafe fn destroy_tree_view(&self, tree: *mut TreeViewO) {
        self.destroy_tree_view.unwrap()(tree)
    }

    pub unsafe fn ui(
        &self,
        tree: *mut TreeViewO,
        ui: *mut UiO,
        style: *const UiStyleT,
        rect: RectT,
    ) -> TreeViewUiResT {
        self.ui.unwrap()(tree, ui, style, rect)
    }

    pub unsafe fn property_group_object(&self, object: TtIdT, property: u32) -> TtIdT {
        self.property_group_object.unwrap()(object, property)
    }

    pub unsafe fn set_expanded(&self, tree: *mut TreeViewO, object: TtIdT, expanded: bool) {
        self.set_expanded.unwrap()(tree, object, expanded)
    }

    pub unsafe fn is_expanded(&self, tree: *mut TreeViewO, object: TtIdT) -> bool {
        self.is_expanded.unwrap()(tree, object)
    }

    pub unsafe fn select(&self, tree: *mut TreeViewO, object: TtIdT) {
        self.select.unwrap()(tree, object)
    }

    pub unsafe fn add_to_selection(&self, tree: *mut TreeViewO, object: TtIdT) {
        self.add_to_selection.unwrap()(tree, object)
    }

    pub unsafe fn remove_from_selection(&self, tree: *mut TreeViewO, object: TtIdT) {
        self.remove_from_selection.unwrap()(tree, object)
    }

    pub unsafe fn scroll_to_object(&self, tree: *mut TreeViewO, object: TtIdT) {
        self.scroll_to_object.unwrap()(tree, object)
    }

    pub unsafe fn deselect_all(&self, tree: *mut TreeViewO) {
        self.deselect_all.unwrap()(tree)
    }

    pub unsafe fn rename(&self, tree: *mut TreeViewO, object: TtIdT) {
        self.rename.unwrap()(tree, object)
    }

    pub unsafe fn get_sorted_subobjects(
        &self,
        tree: *const TreeViewO,
        object: TtIdT,
        property: u32,
        ta: *mut TempAllocatorI,
    ) -> *const TtIdT {
        self.get_sorted_subobjects.unwrap()(tree, object, property, ta)
    }

    pub unsafe fn selected_objects(
        &self,
        inst: *mut TreeViewO,
        ta: *mut TempAllocatorI,
    ) -> *const TtIdT {
        self.selected_objects.unwrap()(inst, ta)
    }

    pub unsafe fn get_root_object(&self, inst: *mut TreeViewO) -> TtIdT {
        self.get_root_object.unwrap()(inst)
    }

    pub unsafe fn set_root_object(&self, inst: *mut TreeViewO, object: TtIdT) {
        self.set_root_object.unwrap()(inst, object)
    }

    pub unsafe fn add_default_menu_items(
        &self,
        tree: *mut TreeViewO,
        parent: *mut TreeViewParentT,
        object: TtIdT,
        object_property: u32,
        id_base: u64,
        items: *mut *mut UiMenuItemT,
        ta: *mut TempAllocatorI,
    ) {
        self.add_default_menu_items.unwrap()(
            tree,
            parent,
            object,
            object_property,
            id_base,
            items,
            ta,
        )
    }

    pub unsafe fn execute_menu_item(
        &self,
        tree: *mut TreeViewO,
        parent: *mut TreeViewParentT,
        object: TtIdT,
        object_property: u32,
        id_base: u64,
        res: *const UiMenuResultT,
    ) {
        self.execute_menu_item.unwrap()(tree, parent, object, object_property, id_base, res)
    }

    pub unsafe fn set_empty_text(&self, tree: *mut TreeViewO, text: *const ::std::os::raw::c_char) {
        self.set_empty_text.unwrap()(tree, text)
    }

    pub unsafe fn filter_nodes(
        &self,
        tree: *mut TreeViewO,
        node_type: u64,
        text: *const ::std::os::raw::c_char,
        case_unsensitive: bool,
    ) {
        self.filter_nodes.unwrap()(tree, node_type, text, case_unsensitive)
    }

    pub unsafe fn renaming_object(&self, tree: *mut TreeViewO) -> TtIdT {
        self.renaming_object.unwrap()(tree)
    }
}

impl crate::Api for TreeViewApi {
    const NAME: ConstCStr = const_cstr!("tm_tree_view_api");
}

impl UiPopupItemPickerApi {
    pub unsafe fn pick(
        &self,
        ui: *mut UiO,
        uistyle_in: *const UiStyleT,
        c: *const UiPopupItemPickerT,
        picked: *mut u32,
    ) -> bool {
        self.pick.unwrap()(ui, uistyle_in, c, picked)
    }

    pub unsafe fn pick_with_categories(
        &self,
        ui: *mut UiO,
        uistyle_in: *const UiStyleT,
        c: *const UiCategoryPopupItemPickerT,
        picked: *mut u32,
    ) -> bool {
        self.pick_with_categories.unwrap()(ui, uistyle_in, c, picked)
    }

    pub unsafe fn pick_asset(
        &self,
        ui: *mut UiO,
        uistyle_in: *const UiStyleT,
        c: *const UiPopupAssetPickerT,
        asset: *mut TtIdT,
    ) -> bool {
        self.pick_asset.unwrap()(ui, uistyle_in, c, asset)
    }

    pub unsafe fn pick_textedit(
        &self,
        ui: *mut UiO,
        uistyle_in: *const UiStyleT,
        config: *const UiTexteditPickerT,
        picked: *mut u32,
        not_in_list: *mut bool,
    ) -> bool {
        self.pick_textedit.unwrap()(ui, uistyle_in, config, picked, not_in_list)
    }
}

impl crate::Api for UiPopupItemPickerApi {
    const NAME: ConstCStr = const_cstr!("tm_ui_popup_item_picker_api");
}

pub const TM_TT_TYPE_HASH__ASSET_BROWSER: StrhashT = StrhashT {
    u64_: 12068649633182618843u64,
};
pub const TM_TT_ASPECT__GRAPH: StrhashT = StrhashT {
    u64_: 10895267865589807857u64,
};
pub const TM_TT_TYPE_HASH__GRAPH: StrhashT = StrhashT {
    u64_: 95269731083356399u64,
};
pub const TM_TT_TYPE_HASH__GRAPH_NODE: StrhashT = StrhashT {
    u64_: 12786904892607600620u64,
};
pub const TM_TT_TYPE_HASH__GRAPH_CONNECTION: StrhashT = StrhashT {
    u64_: 8664952747042998757u64,
};
pub const TM_TT_TYPE_HASH__GRAPH_DATA: StrhashT = StrhashT {
    u64_: 18370962117825208298u64,
};
pub const TM_TT_TYPE_HASH__GRAPH_COMMENT: StrhashT = StrhashT {
    u64_: 4781960635264659497u64,
};
pub const TM_TT_TYPE_HASH__GRAPH_INTERFACE: StrhashT = StrhashT {
    u64_: 15623741324713339326u64,
};
pub const TM_TT_TYPE_HASH__GRAPH_INPUT: StrhashT = StrhashT {
    u64_: 11246844010691861202u64,
};
pub const TM_TT_TYPE_HASH__GRAPH_INPUT_REFERENCE: StrhashT = StrhashT {
    u64_: 14033884072105142967u64,
};
pub const TM_TT_TYPE_HASH__GRAPH_OUTPUT: StrhashT = StrhashT {
    u64_: 4991950285085300571u64,
};
pub const TM_TT_TYPE_HASH__GRAPH_OUTPUT_REFERENCE: StrhashT = StrhashT {
    u64_: 11785675865252752677u64,
};
pub const TM_TT_TYPE_HASH__GRAPH_CACHED_NODE_RESULT: StrhashT = StrhashT {
    u64_: 17618373603327765363u64,
};
pub const TM_TT_TYPE_HASH__GRAPH_EVENT: StrhashT = StrhashT {
    u64_: 10468363316996196499u64,
};
pub const TM_TT_TYPE_HASH__GRAPH_SETTINGS: StrhashT = StrhashT {
    u64_: 11145703035822157950u64,
};
pub const TM_TT_TYPE_HASH__GRAPH_DEBUGGER_PERSISTENT_SETTINGS: StrhashT = StrhashT {
    u64_: 8360824573156045655u64,
};
pub const TM_TT_TYPE_HASH__GRAPH_SETTINGS_BREAKPOINT: StrhashT = StrhashT {
    u64_: 13010082666089549637u64,
};
pub const TM_TT_TYPE_HASH__PROPERTIES_SETTINGS: StrhashT = StrhashT {
    u64_: 3666695013651476581u64,
};
pub const TM_TT_ASPECT__PROPERTIES: StrhashT = StrhashT {
    u64_: 14319599508033735186u64,
};
pub const TM_TT_PROP_ASPECT__PROPERTIES__ASSET_PICKER: StrhashT = StrhashT {
    u64_: 11150166027877100563u64,
};
pub const TM_TT_PROP_ASPECT__PROPERTIES__USE_LOCAL_ENTITY_PICKER: StrhashT = StrhashT {
    u64_: 8688867799886802492u64,
};
pub const TM_TT_PROP_ASPECT__PROPERTIES__CUSTOM_UI: StrhashT = StrhashT {
    u64_: 12485648413046928702u64,
};
pub const TM_TT_PROP_ASPECT__PROPERTIES__REPROTOTYPE_CALLBACK: StrhashT = StrhashT {
    u64_: 14075951065603772573u64,
};
pub const TM_PROPERTY_ASPECT__PROPERTIES__FLOAT_DISPLAY_CONVERTER: StrhashT = StrhashT {
    u64_: 10064183571310805966u64,
};
pub const TM_TT_ASPECT__VALIDATE: StrhashT = StrhashT {
    u64_: 4078988735204785477u64,
};
pub const TM_TT_ASPECT__TREE_VIEW: StrhashT = StrhashT {
    u64_: 15540045876964282411u64,
};
pub const TM_UI_ACTIVE_DATA__PICKER: StrhashT = StrhashT {
    u64_: 5258182182097423975u64,
};
