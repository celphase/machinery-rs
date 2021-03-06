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
pub const TM_ASSET_BROWSER_TAB_VT_NAME: &'static [u8; 21usize] = b"tm_asset_browser_tab\0";
pub const TM_CACHE_MANAGER_TAB_VT_NAME: &'static [u8; 21usize] = b"tm_cache_manager_tab\0";
pub const TM_CANVAS_TAB_VT_NAME: &'static [u8; 14usize] = b"tm_canvas_tab\0";
pub const TM_COLLABORATION_TAB_VT_NAME: &'static [u8; 21usize] = b"tm_collaboration_tab\0";
pub const TM_CONSOLE_TAB_VT_NAME: &'static [u8; 15usize] = b"tm_console_tab\0";
pub const TM_DELAUNAY_TAB_VT_NAME: &'static [u8; 16usize] = b"tm_delaunay_tab\0";
pub const TM_DOWNLOAD_VT_NAME: &'static [u8; 16usize] = b"tm_download_tab\0";
pub const TM_WELCOME_VT_NAME: &'static [u8; 15usize] = b"tm_welcome_tab\0";
pub const TM_TT_TYPE__APPLICATION_SETTINGS__UPDATES: &'static [u8; 33usize] =
    b"tm_application_settings__updates\0";
pub const TM_TT_TYPE__APPLICATION_SETTINGS__DOWNLOAD: &'static [u8; 34usize] =
    b"tm_application_settings__download\0";
pub const TM_TREE_TAB_VT_NAME: &'static [u8; 12usize] = b"tm_tree_tab\0";
pub const TM_GENERIC_TREE_VIEW_TAB_VT_NAME: &'static [u8; 17usize] = b"tm_tree_view_tab\0";
pub const TM_GRAPH_TAB_VT_NAME: &'static [u8; 13usize] = b"tm_graph_tab\0";
pub const TM_IMPORT_PROJECT_TAB_VT_NAME: &'static [u8; 22usize] = b"tm_import_project_tab\0";
pub const TM_INPUT_LOG_TAB_VT_NAME: &'static [u8; 17usize] = b"tm_input_log_tab\0";
pub const TM_JSON_TAB_VT_NAME: &'static [u8; 12usize] = b"tm_json_tab\0";
pub const TM_LOGIN_TAB_VT_NAME: &'static [u8; 13usize] = b"tm_login_tab\0";
pub const TM_MODIFIED_ASSETS_TAB_VT_NAME: &'static [u8; 23usize] = b"tm_modified_assets_tab\0";
pub const TM_NETWORK_TAB_VT_NAME: &'static [u8; 15usize] = b"tm_network_tab\0";
pub const TM_PREVIEW_TAB_VT_NAME: &'static [u8; 15usize] = b"tm_preview_tab\0";
pub const TM_PROFILER_TAB_VT_NAME: &'static [u8; 16usize] = b"tm_profiler_tab\0";
pub const TM_PROPERTIES_TAB_VT_NAME: &'static [u8; 18usize] = b"tm_properties_tab\0";
pub const TM_SCENE_TAB_VT_NAME: &'static [u8; 13usize] = b"tm_scene_tab\0";
pub const TM_TT_TYPE__SCENE_TAB: &'static [u8; 13usize] = b"tm_scene_tab\0";
pub const TM_TT_TYPE__SCENE_TAB_SETTINGS: &'static [u8; 22usize] = b"tm_scene_tab_settings\0";
pub const TM_TT_TYPE__SCENE_SETTINGS: &'static [u8; 18usize] = b"tm_scene_settings\0";
pub const TM_SIMULATE_TAB_VT_NAME: &'static [u8; 16usize] = b"tm_simulate_tab\0";
pub const TM_TT_TYPE__SIMULATE_TAB_SETTINGS: &'static [u8; 25usize] = b"tm_simulate_tab_settings\0";
pub const TM_STATE_GRAPH_TAB_VT_NAME: &'static [u8; 19usize] = b"tm_state_graph_tab\0";
pub const TM_STATISTICS_TAB_VT_NAME: &'static [u8; 18usize] = b"tm_statistics_tab\0";
pub const TM_TASK_MANAGER_TAB_VT_NAME: &'static [u8; 20usize] = b"tm_task_manager_tab\0";
pub const THE_MACHINERY_VERSION: &'static [u8; 7usize] = b"2021.9\0";
pub const TM_TT_TYPE__APPLICATION_SETTINGS: &'static [u8; 24usize] = b"tm_application_settings\0";
pub const TM_TT_TYPE__PROJECT_MANAGEMENT_SETTINGS: &'static [u8; 31usize] =
    b"tm_project_management_settings\0";
pub const TM_TT_TYPE__INTERFACE_SETTINGS: &'static [u8; 22usize] = b"tm_interface_settings\0";
pub const TM_TT_TYPE__WINDOW_LAYOUTS: &'static [u8; 18usize] = b"tm_window_layouts\0";
pub const TM_TT_TYPE__WINDOW_LAYOUT: &'static [u8; 17usize] = b"tm_window_layout\0";
pub const TM_TT_TYPE__WORKSPACE: &'static [u8; 13usize] = b"tm_workspace\0";
pub const TM_TT_TYPE__RECENT_FILE: &'static [u8; 15usize] = b"tm_recent_file\0";
pub const TM_TT_TYPE__PUBLISH_SETTINGS: &'static [u8; 20usize] = b"tm_publish_settings\0";
pub const TM_TT_TYPE__APPLICATION_SECURITY: &'static [u8; 24usize] = b"tm_application_security\0";
pub const TM_TT_TYPE__PROJECT_SETTINGS: &'static [u8; 20usize] = b"tm_project_settings\0";
pub const TM_TT_TYPE__NETWORK_SETTINGS: &'static [u8; 20usize] = b"tm_network_settings\0";
pub const TM_TT_TYPE__LOCALIZATION_SETTINGS: &'static [u8; 25usize] = b"tm_localization_settings\0";
pub const TM_TT_TYPE__TRANSLATION: &'static [u8; 15usize] = b"tm_translation\0";
pub const TM_TT_TYPE__TRANSLATION_CACHE: &'static [u8; 21usize] = b"tm_translation_cache\0";
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
pub const TM_TT_PROP__DOCKING_SETTINGS__TOOLBARS_SETTINGS: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_1 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct AssetBrowserTabApi {
    pub find_asset_browser: ::std::option::Option<unsafe extern "C" fn(ui: *mut UiO) -> TtIdT>,
    pub selected_object: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, asset_browser: TtIdT) -> TtIdT,
    >,
    pub set_selection: ::std::option::Option<
        unsafe extern "C" fn(
            ui: *mut UiO,
            asset_or_dirs: *const TtIdT,
            n: u32,
            undo_scope: TtUndoScopeT,
        ) -> bool,
    >,
}
extern "C" {
    pub static mut console_logger: *mut LoggerI;
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct UpdateTrackerApi {
    pub init: ::std::option::Option<unsafe extern "C" fn(allocator: *mut AllocatorI)>,
    pub should_show_update_notification:
        ::std::option::Option<unsafe extern "C" fn(app: *mut ApplicationO) -> bool>,
    pub shutdown: ::std::option::Option<unsafe extern "C" fn()>,
}
pub const TM_TT_PROP__APPLICATION_SETTINGS__UPDATES__LATEST_VERSION: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__APPLICATION_SETTINGS__UPDATES__SKIP_THIS_VERSION: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_2 = ::std::os::raw::c_int;
pub const TM_TT_PROP__APPLICATION_SETTINGS__DOWNLOAD__URL: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__APPLICAITON_SETTINGS__DOWNLOAD__FILE: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__APPLICAITON_SETTINGS__DOWNLOAD__UNZIP_DIRECTORY: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_3 = ::std::os::raw::c_int;
extern "C" {
    pub static mut tm_update_tracker_api: *mut UpdateTrackerApi;
}
extern "C" {
    pub fn import_gltf_asset(
        gltf_file: *const ::std::os::raw::c_char,
        target_dir: TtIdT,
        a: *mut AllocatorI,
        tt: *mut TheTruthO,
        reimport_into: TtIdT,
        ui: *mut UiO,
        undo_scope: TtUndoScopeT,
    ) -> u64;
}
extern "C" {
    pub static mut gltf_asset_io: *mut AssetIoI;
}
extern "C" {
    pub fn import_tab__import_project(tab: *mut TabO, file: *const ::std::os::raw::c_char);
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct IntegrationTestRunnerApi {
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            app: *mut ApplicationO,
            allocator: *mut AllocatorI,
            context: StrhashT,
        ) -> *mut IntegrationTestRunnerO,
    >,
    pub add_test: ::std::option::Option<
        unsafe extern "C" fn(
            runner: *mut IntegrationTestRunnerO,
            name: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub tick: ::std::option::Option<unsafe extern "C" fn(runner: *mut IntegrationTestRunnerO)>,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(runner: *mut IntegrationTestRunnerO)>,
}
extern "C" {
    pub static mut tm_integration_test_runner_api: *mut IntegrationTestRunnerApi;
}
pub const TM_LOGIN__TAB_ACTION__NONE: LoginTabAction = 0;
pub const TM_LOGIN__TAB_ACTION__SHOW: LoginTabAction = 1;
pub const TM_LOGIN__TAB_ACTION__FULLSCREEN: LoginTabAction = 2;
pub const TM_LOGIN__TAB_ACTION__HIDE: LoginTabAction = 3;
pub const TM_LOGIN__TAB_ACTION__QUIT: LoginTabAction = 4;
pub type LoginTabAction = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct LoginUpdateT {
    pub show_login_tab: bool,
    pub force_focus_login_tab: bool,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct LoginApi {
    pub init: ::std::option::Option<unsafe extern "C" fn(allocator: *mut AllocatorI)>,
    pub update: ::std::option::Option<unsafe extern "C" fn() -> LoginTabAction>,
    pub shutdown: ::std::option::Option<unsafe extern "C" fn()>,
}
extern "C" {
    pub static mut tm_login_api: *mut LoginApi;
}
extern "C" {
    pub fn menu__new_plugin__editor_tab();
}
extern "C" {
    pub fn menu__new_plugin__entity_component();
}
extern "C" {
    pub fn menu__new_plugin__simulation_entry();
}
extern "C" {
    pub fn menu__new_plugin__minimal();
}
extern "C" {
    pub fn load_plugin_templates(reg: *mut ApiRegistryApi, load: bool);
}
extern "C" {
    pub fn profiler_view(tab: *mut TabO) -> *mut ProfilerViewO;
}
extern "C" {
    pub fn properties_tab_set_root_with_mask(
        tab: *mut TabO,
        tt: *mut TheTruthO,
        root: TtIdT,
        mask: u64,
    );
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
pub const TM_TT_PROP__SCENE_TAB__ASSET: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__SCENE_TAB__PIN_ASSSET: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__SCENE_TAB__SELECTION: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__SCENE_TAB__LAST_SELECTED: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__SCENE_TAB__PICK_COUNTER: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__SCENE_TAB__FRAME_OBJECT: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__SCENE_TAB__HIDDEN: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__SCENE_TAB__TO_SHOW: ::std::os::raw::c_int = 7;
pub const TM_TT_PROP__SCENE_TAB__LOCKED: ::std::os::raw::c_int = 8;
pub const TM_TT_PROP__SCENE_TAB__UNLOCKED: ::std::os::raw::c_int = 9;
pub type _bindgen_ty_4 = ::std::os::raw::c_int;
pub const TM_TT_PROP__SCENE_TAB_SETTINGS__NATIVE_RESOLUTION: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_5 = ::std::os::raw::c_int;
pub const TM_TT_PROP__SCENE_SETTINGS__ASSET_UUID_A: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__SCENE_SETTINGS__USE_WORLD_AXES: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__SCENE_SETTINGS__MULTI_SELECT_PIVOT: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__SCENE_SETTINGS__MOVE_SNAP: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__SCENE_SETTINGS__MOVE_SNAP_SIZE: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__SCENE_SETTINGS__ROT_SNAP: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__SCENE_SETTINGS__ROT_SNAP_ANGLE: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__SCENE_SETTINGS__SCALE_ABSOLUTE_HANDLE: ::std::os::raw::c_int = 7;
pub const TM_TT_PROP__SCENE_SETTINGS__SCALE_SNAP: ::std::os::raw::c_int = 8;
pub const TM_TT_PROP__SCENE_SETTINGS__SCALE_SNAP_INCREMENT: ::std::os::raw::c_int = 9;
pub type _bindgen_ty_6 = ::std::os::raw::c_int;
#[repr(C)]
pub struct scene_tab_entity_data_t {
    pub entity_ctx: *mut EntityContextO,
    pub camera_controller_manager: *mut CameraControllerComponentManagerO,
    pub camera_controller_component: ComponentTypeT,
    pub dcc_asset_component: ComponentTypeT,
    pub camera: EntityT,
    pub entity: EntityT,
}
impl Default for scene_tab_entity_data_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub fn scene_tab_entity_data(tab: *mut TabO) -> scene_tab_entity_data_t;
}
extern "C" {
    pub fn scene_tab_truth_object(tab: *mut TabO) -> TtIdT;
}
pub const TM_TT_PROP__SIMULATE_TAB_SETTINGS__NATIVE_RESOLUTION: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_7 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct SimulateTabSettingsT {
    pub native_resolution: bool,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct SimulateTabApi {
    pub update_settings: ::std::option::Option<
        unsafe extern "C" fn(inst: *mut TabO, settings: *const SimulateTabSettingsT),
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct StatisticsTabApi {
    pub add_source: ::std::option::Option<
        unsafe extern "C" fn(
            tab: *mut TabI,
            name: *const ::std::os::raw::c_char,
            source: *const ::std::os::raw::c_char,
            color: ColorSrgbT,
        ),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaskManagerO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct TaskManagerApi {
    pub create:
        ::std::option::Option<unsafe extern "C" fn(alloc: *mut AllocatorI) -> *mut TaskManagerO>,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(inst: *mut TaskManagerO)>,
    pub ui: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut TaskManagerO,
            ui: *mut UiO,
            id: u64,
            uistyle: *const UiStyleT,
            r: RectT,
        ),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WindowO {
    _unused: [u8; 0],
}
pub const TM_TT_PROP__APPLICATION_SETTINGS__ANALYTICS_CLIENT_ID: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__APPLICATION_SETTINGS__ANALYTICS_OPT_OUT: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__APPLICATION_SETTINGS__CAMERA_SETTINGS: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__APPLICATION_SETTINGS__NETWORK_SETTINGS: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__APPLICATION_SETTINGS__GRID_SETTINGS: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__APPLICATION_SETTINGS__WINDOW_LAYOUTS: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__APPLICATION_SETTINGS__RECENT_FILES: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__APPLICATION_SETTINGS__INTERFACE_SETTINGS: ::std::os::raw::c_int = 7;
pub const TM_TT_PROP__APPLICATION_SETTINGS__LOCALIZATION_SETTINGS: ::std::os::raw::c_int = 8;
pub const TM_TT_PROP__APPLICATION_SETTINGS__KEYBOARD_SHORTCUTS: ::std::os::raw::c_int = 9;
pub const TM_TT_PROP__APPLICATION_SETTINGS__UI_THEMES: ::std::os::raw::c_int = 10;
pub const TM_TT_PROP__APPLICATION_SETTINGS__UI_THEME: ::std::os::raw::c_int = 11;
pub const TM_TT_PROP__APPLICATION_SETTINGS__BUILTIN_UI_THEME: ::std::os::raw::c_int = 12;
pub const TM_TT_PROP__APPLICATION_SETTINGS__PROJECTS_SETTINGS: ::std::os::raw::c_int = 13;
pub const TM_TT_PROP__APPLICATION_SETTINGS__UPDATES: ::std::os::raw::c_int = 14;
pub const TM_TT_PROP__APPLICATION_SETTINGS__DOWNLOADS: ::std::os::raw::c_int = 15;
pub const TM_TT_PROP__APPLICATION_SETTINGS__LAST_OPENED_PROJECT: ::std::os::raw::c_int = 16;
pub const TM_TT_PROP__APPLICATION_SETTINGS__DOCKING_SETTINGS: ::std::os::raw::c_int = 17;
pub const TM_TT_PROP__APPLICATION_SETTINGS__PROJECT_MANAGEMENT: ::std::os::raw::c_int = 18;
pub const TM_TT_PROP__APPLICATION_SETTINGS__SECURITY: ::std::os::raw::c_int = 19;
pub type _bindgen_ty_8 = ::std::os::raw::c_int;
pub const TM_TT_PROP__PROJECT_MANAGEMENT_SETTINGS__OPEN_EMPTY_PROJECT_ON_STARTUP:
    ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__PROJECT_MANAGEMENT_SETTINGS__ALWAYS_OPEN_DEFAULT_PROJECT_WINDOW:
    ::std::os::raw::c_int = 1;
pub type _bindgen_ty_9 = ::std::os::raw::c_int;
pub const TM_TT_PROP__INTERFACE_SETTINGS__RESOLUTION_SCALE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__INTERFACE_SETTINGS__WHEEL_SCROLL_LINES: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_10 = ::std::os::raw::c_int;
pub const TM_TT_PROP__WINDOW_LAYOUTS__LAYOUTS: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_11 = ::std::os::raw::c_int;
pub const TM_TT_PROP__WINDOW_LAYOUT__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__WINDOW_LAYOUT__ICON: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__WINDOW_LAYOUT__WINDOW_X: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__WINDOW_LAYOUT__WINDOW_Y: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__WINDOW_LAYOUT__WINDOW_WIDTH: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__WINDOW_LAYOUT__WINDOW_HEIGHT: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__WINDOW_LAYOUT__TABWELL: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__WINDOW_LAYOUT__WORKSPACES: ::std::os::raw::c_int = 7;
pub type _bindgen_ty_12 = ::std::os::raw::c_int;
pub const TM_TT_PROP__WORKSPACE__NAME: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__WORKSPACE__ICON: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__WORKSPACE__TABWELL: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_13 = ::std::os::raw::c_int;
pub const TM_TT_PROP__RECENT_FILE__PATH: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__RECENT_FILE__TIME_STAMP: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_14 = ::std::os::raw::c_int;
pub const TM_TT_PROP__PUBLISH_SETTINGS__DIRECTORY_PROJECT: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__PUBLISH_SETTINGS__EXECUTABLE_NAME: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__PUBLISH_SETTINGS__WINDOW_TITLE: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__PUBLISH_SETTINGS__WORLD_ENTITY: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__PUBLISH_SETTINGS__RESOLUTION: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__PUBLISH_SETTINGS__FULLSCREEN: ::std::os::raw::c_int = 5;
pub type _bindgen_ty_15 = ::std::os::raw::c_int;
pub const TM_TT_PROP__APPLICATION_SECURITY__ALWAYS_ALLOW_DLL_PROJECT_PATHS: ::std::os::raw::c_int =
    0;
pub const TM_TT_PROP__APPLICATION_SECURITY__ALWAYS_DENY_DLL_PROJECT_PATHS: ::std::os::raw::c_int =
    1;
pub type _bindgen_ty_16 = ::std::os::raw::c_int;
pub const TM_TT_PROP__PROJECT_SETTINGS__PATH: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__PROJECT_SETTINGS__GRAPHS_SETTINGS: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__PROJECT_SETTINGS__SCENES_SETTINGS: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__PROJECT_SETTINGS__PUBLISH_SETTINGS: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__PROJECT_SETTINGS__WINDOW_LAYOUT: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__PROJECT_SETTINGS__GRAPH_DEBUGGER_PERSISTENT_SETTINGS: ::std::os::raw::c_int =
    5;
pub type _bindgen_ty_17 = ::std::os::raw::c_int;
pub const TM_TT_PROP__NETWORK_SETTINGS__NODE_INSTANCES: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_18 = ::std::os::raw::c_int;
pub const TM_TT_PROP__LOCALIZATION_SETTINGS__MICROSOFT_CACHE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__LOCALIZATION_SETTINGS__GOOGLE_CACHE: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__LOCALIZATION_SETTINGS__LANGUAGE: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__LOCALIZATION_SETTINGS__USE_GOOGLE: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__LOCALIZATION_SETTINGS__USE_MICROSOFT: ::std::os::raw::c_int = 4;
pub type _bindgen_ty_19 = ::std::os::raw::c_int;
pub type TheMachineryProjectCallbackI = ::std::option::Option<
    unsafe extern "C" fn(app: *mut ApplicationO, truth: *mut TheTruthO, asset_root: TtIdT),
>;
pub type TheMachineryProjectLoadedI = TheMachineryProjectCallbackI;
pub type TheMachineryProjectUnloadedI = TheMachineryProjectCallbackI;
pub type TheMachineryProjectSavedI = TheMachineryProjectCallbackI;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TheMachineryApi {
    pub app: *mut ApplicationO,
    pub open_asset: ::std::option::Option<
        unsafe extern "C" fn(
            app: *mut ApplicationO,
            ui: *mut UiO,
            from_tab: *mut TabI,
            asset: TtIdT,
        ),
    >,
    pub get_asset_root:
        ::std::option::Option<unsafe extern "C" fn(app: *mut ApplicationO) -> TtIdT>,
    pub reimport_asset: ::std::option::Option<
        unsafe extern "C" fn(
            app: *mut ApplicationO,
            ui: *mut UiO,
            tt: *mut TheTruthO,
            asset: TtIdT,
        ),
    >,
    pub create_or_select_tab: ::std::option::Option<
        unsafe extern "C" fn(
            app: *mut ApplicationO,
            ui: *mut UiO,
            vt_name: *const ::std::os::raw::c_char,
            opt: *const DockingFindTabOptT,
        ) -> *mut TabI,
    >,
    pub window_for_ui: ::std::option::Option<
        unsafe extern "C" fn(app: *mut ApplicationO, ui: *const UiO) -> *mut WindowO,
    >,
    pub collaboration:
        ::std::option::Option<unsafe extern "C" fn(app: *mut ApplicationO) -> *mut CollaborationO>,
    pub collaboration_p2p: ::std::option::Option<
        unsafe extern "C" fn(app: *mut ApplicationO) -> *mut CollaborationP2pO,
    >,
    pub collaboration_discord: ::std::option::Option<
        unsafe extern "C" fn(app: *mut ApplicationO) -> *mut CollaborationDiscordO,
    >,
    pub settings: ::std::option::Option<
        unsafe extern "C" fn(app: *mut ApplicationO, id: *mut TtIdT) -> *mut TheTruthO,
    >,
    pub project_settings: ::std::option::Option<
        unsafe extern "C" fn(app: *mut ApplicationO, id: *mut TtIdT) -> *mut TheTruthO,
    >,
    pub open_project: ::std::option::Option<
        unsafe extern "C" fn(
            app: *mut ApplicationO,
            ui: *mut UiO,
            path: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub open_any_project: ::std::option::Option<
        unsafe extern "C" fn(app: *mut ApplicationO, file: *const ::std::os::raw::c_char),
    >,
    pub save_to_asset_database: ::std::option::Option<
        unsafe extern "C" fn(app: *mut ApplicationO, file: *const ::std::os::raw::c_char),
    >,
    pub import_asset: ::std::option::Option<
        unsafe extern "C" fn(
            app: *mut ApplicationO,
            file: *const ::std::os::raw::c_char,
            reimport_into: TtIdT,
        ),
    >,
    pub save_all: ::std::option::Option<unsafe extern "C" fn(app: *mut ApplicationO)>,
    pub get_truth:
        ::std::option::Option<unsafe extern "C" fn(app: *mut ApplicationO) -> *mut TheTruthO>,
    pub new_project: ::std::option::Option<unsafe extern "C" fn(app: *mut ApplicationO)>,
}
impl Default for TheMachineryApi {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type TheMachineryCreateLayoutI =
    ::std::option::Option<unsafe extern "C" fn(app: *mut ApplicationO)>;
extern "C" {
    pub static mut tm_global_api_registry: *mut ApiRegistryApi;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DccAssetApi {
    _unused: [u8; 0],
}
extern "C" {
    pub static mut tm_dcc_asset_api: *mut DccAssetApi;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DefaultRenderPipeApi {
    _unused: [u8; 0],
}
extern "C" {
    pub static mut tm_default_render_pipe_api: *mut DefaultRenderPipeApi;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GridRendererApi {
    _unused: [u8; 0],
}
extern "C" {
    pub static mut tm_grid_renderer_api: *mut GridRendererApi;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsDisplayApi {
    _unused: [u8; 0],
}
extern "C" {
    pub static mut tm_os_display_api: *mut OsDisplayApi;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsWindowApi {
    _unused: [u8; 0],
}
extern "C" {
    pub static mut tm_os_window_api: *mut OsWindowApi;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PrimitiveDrawerApi {
    _unused: [u8; 0],
}
extern "C" {
    pub static mut tm_primitive_drawer_api: *mut PrimitiveDrawerApi;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuPickingApi {
    _unused: [u8; 0],
}
extern "C" {
    pub static mut tm_gpu_picking_api: *mut GpuPickingApi;
}
extern "C" {
    pub static mut tm_task_manager_api: *mut TaskManagerApi;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GltfApi {
    _unused: [u8; 0],
}
extern "C" {
    pub static mut tm_gltf_api_opt: *mut GltfApi;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpenvrApi {
    _unused: [u8; 0],
}
extern "C" {
    pub static mut tm_openvr_api_opt: *mut OpenvrApi;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VulkanApi {
    _unused: [u8; 0],
}
extern "C" {
    pub static mut tm_vulkan_api_opt: *mut VulkanApi;
}
extern "C" {
    pub static mut tm_cmd_buf_api: *mut RendererCommandBufferApi;
}
extern "C" {
    pub static mut tm_res_buf_api: *mut RendererResourceCommandBufferApi;
}
extern "C" {
    pub static mut tm_the_machinery_api: *mut TheMachineryApi;
}
pub const TM_TT_PROP__TRANSLATION__ENGLISH: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__TRANSLATION__LANGUAGE: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__TRANSLATION__TRANSLATION: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_20 = ::std::os::raw::c_int;
pub const TM_TT_PROP__TRANSLATION_CACHE__TRANSLATIONS: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_21 = ::std::os::raw::c_int;
#[repr(C)]
pub struct LocalizerStateT {
    pub language: StrhashT,
    pub test_mode: bool,
    pub reveal_source_text: bool,
    pub microsoft: bool,
    pub google: bool,
    pub _padding_55: [::std::os::raw::c_char; 4usize],
}
impl Default for LocalizerStateT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub fn create_localizer(a: *mut AllocatorI) -> *mut LocalizerO;
}
extern "C" {
    pub fn destroy_localizer(loc: *mut LocalizerO);
}
extern "C" {
    pub fn localizer_set_language(loc: *mut LocalizerO, language: StrhashT);
}
extern "C" {
    pub fn localizer_set_test_mode(loc: *mut LocalizerO, test_mode: bool);
}
extern "C" {
    pub fn localizer_set_reveal_source_text(loc: *mut LocalizerO, reveal: bool);
}
extern "C" {
    pub fn localizer_set_microsoft_translate(
        loc: *mut LocalizerO,
        language: StrhashT,
        api_key: *const ::std::os::raw::c_char,
        tt: *mut TheTruthO,
        translation_cache_id: TtIdT,
    );
}
extern "C" {
    pub fn localizer_set_google_translate(
        loc: *mut LocalizerO,
        language: StrhashT,
        api_key: *const ::std::os::raw::c_char,
        tt: *mut TheTruthO,
        translation_cache_id: TtIdT,
    );
}
extern "C" {
    pub fn localizer_state(loc: *mut LocalizerO) -> LocalizerStateT;
}
extern "C" {
    pub fn localizer_interface(loc: *mut LocalizerO) -> *mut LocalizerI;
}
extern "C" {
    pub fn load_localizer(reg: *mut ApiRegistryApi, load: bool);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TabCreateContextT {
    pub allocator: *mut AllocatorI,
    pub id: *mut u64,
    pub application: *mut ApplicationO,
    pub tt: *mut TheTruthO,
    pub undo_stack: *mut UndoStackI,
    pub save_interface: *mut AssetSaveI,
    pub ui_renderer: *mut UiRendererO,
    pub network: *mut NetworkO,
}
impl Default for TabCreateContextT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct PreviewTabVt {
    pub super_: TabVt,
    pub preview_entity: ::std::option::Option<
        unsafe extern "C" fn(tab: *mut TabO, e: *mut EntityT) -> *mut EntityContextO,
    >,
}
impl Default for PreviewTabVt {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct GraphTabVt {
    pub super_: TabVt,
    pub refresh_node_types: ::std::option::Option<
        unsafe extern "C" fn(
            tab: *mut TabO,
            node_interface_name: *const ::std::os::raw::c_char,
            node_interface_version: VersionT,
        ),
    >,
}
impl Default for GraphTabVt {
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
pub struct CollaborationDiscordO {
    pub _address: u8,
}

// Extensions generated by machinery-generator

use const_cstr::{const_cstr, ConstCStr};

use crate::foundation::VersionT;

use crate::foundation::*;
use crate::plugins::editor_views::*;
use crate::plugins::entity::*;
use crate::plugins::renderer::*;
use crate::plugins::the_machinery_shared::*;
use crate::plugins::ui::*;

impl AssetBrowserTabApi {
    pub unsafe fn find_asset_browser(&self, ui: *mut UiO) -> TtIdT {
        self.find_asset_browser.unwrap()(ui)
    }

    pub unsafe fn selected_object(&self, tt: *mut TheTruthO, asset_browser: TtIdT) -> TtIdT {
        self.selected_object.unwrap()(tt, asset_browser)
    }

    pub unsafe fn set_selection(
        &self,
        ui: *mut UiO,
        asset_or_dirs: *const TtIdT,
        n: u32,
        undo_scope: TtUndoScopeT,
    ) -> bool {
        self.set_selection.unwrap()(ui, asset_or_dirs, n, undo_scope)
    }
}

impl crate::Api for AssetBrowserTabApi {
    const NAME: ConstCStr = const_cstr!("tm_asset_browser_tab_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl UpdateTrackerApi {
    pub unsafe fn init(&self, allocator: *mut AllocatorI) {
        self.init.unwrap()(allocator)
    }

    pub unsafe fn should_show_update_notification(&self, app: *mut ApplicationO) -> bool {
        self.should_show_update_notification.unwrap()(app)
    }

    pub unsafe fn shutdown(&self) {
        self.shutdown.unwrap()()
    }
}

impl crate::Api for UpdateTrackerApi {
    const NAME: ConstCStr = const_cstr!("tm_update_tracker_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl IntegrationTestRunnerApi {
    pub unsafe fn create(
        &self,
        app: *mut ApplicationO,
        allocator: *mut AllocatorI,
        context: StrhashT,
    ) -> *mut IntegrationTestRunnerO {
        self.create.unwrap()(app, allocator, context)
    }

    pub unsafe fn add_test(
        &self,
        runner: *mut IntegrationTestRunnerO,
        name: *const ::std::os::raw::c_char,
    ) -> bool {
        self.add_test.unwrap()(runner, name)
    }

    pub unsafe fn tick(&self, runner: *mut IntegrationTestRunnerO) {
        self.tick.unwrap()(runner)
    }

    pub unsafe fn destroy(&self, runner: *mut IntegrationTestRunnerO) {
        self.destroy.unwrap()(runner)
    }
}

impl crate::Api for IntegrationTestRunnerApi {
    const NAME: ConstCStr = const_cstr!("tm_integration_test_runner_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl LoginApi {
    pub unsafe fn init(&self, allocator: *mut AllocatorI) {
        self.init.unwrap()(allocator)
    }

    pub unsafe fn update(&self) -> LoginTabAction {
        self.update.unwrap()()
    }

    pub unsafe fn shutdown(&self) {
        self.shutdown.unwrap()()
    }
}

impl crate::Api for LoginApi {
    const NAME: ConstCStr = const_cstr!("tm_login_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl SimulateTabApi {
    pub unsafe fn update_settings(&self, inst: *mut TabO, settings: *const SimulateTabSettingsT) {
        self.update_settings.unwrap()(inst, settings)
    }
}

impl crate::Api for SimulateTabApi {
    const NAME: ConstCStr = const_cstr!("tm_simulate_tab_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl StatisticsTabApi {
    pub unsafe fn add_source(
        &self,
        tab: *mut TabI,
        name: *const ::std::os::raw::c_char,
        source: *const ::std::os::raw::c_char,
        color: ColorSrgbT,
    ) {
        self.add_source.unwrap()(tab, name, source, color)
    }
}

impl crate::Api for StatisticsTabApi {
    const NAME: ConstCStr = const_cstr!("tm_statistics_tab_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl TaskManagerApi {
    pub unsafe fn create(&self, alloc: *mut AllocatorI) -> *mut TaskManagerO {
        self.create.unwrap()(alloc)
    }

    pub unsafe fn destroy(&self, inst: *mut TaskManagerO) {
        self.destroy.unwrap()(inst)
    }

    pub unsafe fn ui(
        &self,
        inst: *mut TaskManagerO,
        ui: *mut UiO,
        id: u64,
        uistyle: *const UiStyleT,
        r: RectT,
    ) {
        self.ui.unwrap()(inst, ui, id, uistyle, r)
    }
}

impl crate::Api for TaskManagerApi {
    const NAME: ConstCStr = const_cstr!("tm_task_manager_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl TheMachineryApi {
    pub unsafe fn open_asset(
        &self,
        app: *mut ApplicationO,
        ui: *mut UiO,
        from_tab: *mut TabI,
        asset: TtIdT,
    ) {
        self.open_asset.unwrap()(app, ui, from_tab, asset)
    }

    pub unsafe fn get_asset_root(&self, app: *mut ApplicationO) -> TtIdT {
        self.get_asset_root.unwrap()(app)
    }

    pub unsafe fn reimport_asset(
        &self,
        app: *mut ApplicationO,
        ui: *mut UiO,
        tt: *mut TheTruthO,
        asset: TtIdT,
    ) {
        self.reimport_asset.unwrap()(app, ui, tt, asset)
    }

    pub unsafe fn create_or_select_tab(
        &self,
        app: *mut ApplicationO,
        ui: *mut UiO,
        vt_name: *const ::std::os::raw::c_char,
        opt: *const DockingFindTabOptT,
    ) -> *mut TabI {
        self.create_or_select_tab.unwrap()(app, ui, vt_name, opt)
    }

    pub unsafe fn window_for_ui(&self, app: *mut ApplicationO, ui: *const UiO) -> *mut WindowO {
        self.window_for_ui.unwrap()(app, ui)
    }

    pub unsafe fn collaboration(&self, app: *mut ApplicationO) -> *mut CollaborationO {
        self.collaboration.unwrap()(app)
    }

    pub unsafe fn collaboration_p2p(&self, app: *mut ApplicationO) -> *mut CollaborationP2pO {
        self.collaboration_p2p.unwrap()(app)
    }

    pub unsafe fn collaboration_discord(
        &self,
        app: *mut ApplicationO,
    ) -> *mut CollaborationDiscordO {
        self.collaboration_discord.unwrap()(app)
    }

    pub unsafe fn settings(&self, app: *mut ApplicationO, id: *mut TtIdT) -> *mut TheTruthO {
        self.settings.unwrap()(app, id)
    }

    pub unsafe fn project_settings(
        &self,
        app: *mut ApplicationO,
        id: *mut TtIdT,
    ) -> *mut TheTruthO {
        self.project_settings.unwrap()(app, id)
    }

    pub unsafe fn open_project(
        &self,
        app: *mut ApplicationO,
        ui: *mut UiO,
        path: *const ::std::os::raw::c_char,
    ) -> bool {
        self.open_project.unwrap()(app, ui, path)
    }

    pub unsafe fn open_any_project(
        &self,
        app: *mut ApplicationO,
        file: *const ::std::os::raw::c_char,
    ) {
        self.open_any_project.unwrap()(app, file)
    }

    pub unsafe fn save_to_asset_database(
        &self,
        app: *mut ApplicationO,
        file: *const ::std::os::raw::c_char,
    ) {
        self.save_to_asset_database.unwrap()(app, file)
    }

    pub unsafe fn import_asset(
        &self,
        app: *mut ApplicationO,
        file: *const ::std::os::raw::c_char,
        reimport_into: TtIdT,
    ) {
        self.import_asset.unwrap()(app, file, reimport_into)
    }

    pub unsafe fn save_all(&self, app: *mut ApplicationO) {
        self.save_all.unwrap()(app)
    }

    pub unsafe fn get_truth(&self, app: *mut ApplicationO) -> *mut TheTruthO {
        self.get_truth.unwrap()(app)
    }

    pub unsafe fn new_project(&self, app: *mut ApplicationO) {
        self.new_project.unwrap()(app)
    }
}

impl crate::Api for TheMachineryApi {
    const NAME: ConstCStr = const_cstr!("tm_the_machinery_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl DccAssetApi {}

impl DefaultRenderPipeApi {}

impl GridRendererApi {}

impl OsDisplayApi {}

impl OsWindowApi {}

impl PrimitiveDrawerApi {}

impl GpuPickingApi {}

impl GltfApi {}

impl OpenvrApi {}

impl VulkanApi {}

pub const TM_ASSET_BROWSER_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 13036556916491485142u64,
};
pub const TM_CACHE_MANAGER_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 17336090661109815624u64,
};
pub const TM_CANVAS_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 9991448700525851219u64,
};
pub const TM_COLLABORATION_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 13304429456151464228u64,
};
pub const TM_CONSOLE_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 12163199768431146244u64,
};
pub const TM_CORE_ID_DCC_IMAGE_CG: StrhashT = StrhashT {
    u64_: 4745056028275695429u64,
};
pub const TM_CORE_ID_DCC_MATERIAL_CG: StrhashT = StrhashT {
    u64_: 14644393322716058569u64,
};
pub const TM_CORE_ID_DCC_MATERIAL_OPACITY_CG: StrhashT = StrhashT {
    u64_: 15439250747231496698u64,
};
pub const TM_CORE_ID_DCC_MESH_CG: StrhashT = StrhashT {
    u64_: 3192826436324017360u64,
};
pub const TM_CORE_ID_DCC_MESH_LOD_CG: StrhashT = StrhashT {
    u64_: 14880516632037890971u64,
};
pub const TM_CORE_ID_DCC_MESH_LOD_STEP_CG: StrhashT = StrhashT {
    u64_: 12029032052130817361u64,
};
pub const TM_CORE_ID_IMPORT_IMAGE_CG: StrhashT = StrhashT {
    u64_: 8434100624486330861u64,
};
pub const TM_CORE_ID_DROP_IMAGE_CG: StrhashT = StrhashT {
    u64_: 4513188134555908800u64,
};
pub const TM_CORE_ID_EDITOR_ICON_CG: StrhashT = StrhashT {
    u64_: 7038704372328695573u64,
};
pub const TM_CORE_ID_GEOMETRY_MATERIAL_CG: StrhashT = StrhashT {
    u64_: 14108441262108032777u64,
};
pub const TM_CORE_ID_GEOMETRY_BOX_ENTITY: StrhashT = StrhashT {
    u64_: 7252367683291092979u64,
};
pub const TM_CORE_ID_GEOMETRY_SPHERE_ENTITY: StrhashT = StrhashT {
    u64_: 11582288377819735590u64,
};
pub const TM_CORE_ID_GEOMETRY_PLANE_ENTITY: StrhashT = StrhashT {
    u64_: 7311513872247150992u64,
};
pub const TM_CORE_ID_LIGHT_ENTITY: StrhashT = StrhashT {
    u64_: 7173462963839382841u64,
};
pub const TM_CORE_ID_CAMERA_ENTITY: StrhashT = StrhashT {
    u64_: 2187989508077250605u64,
};
pub const TM_CORE_ID_POST_PROCESSING_STACK: StrhashT = StrhashT {
    u64_: 17154254651014689721u64,
};
pub const TM_CORE_ID_POST_PROCESSING_VOLUME: StrhashT = StrhashT {
    u64_: 6525241469376208722u64,
};
pub const TM_CORE_ID_REFLECTION_PROBE_ENTITY: StrhashT = StrhashT {
    u64_: 4249566392992356494u64,
};
pub const TM_CORE_ID_DEFAULT_WORLD: StrhashT = StrhashT {
    u64_: 9823233946160333378u64,
};
pub const TM_CORE_ID_DEFAULT_LIGHTING_ENVIRONMENT: StrhashT = StrhashT {
    u64_: 9068331648142131762u64,
};
pub const TM_CORE_ID_SHORTCUT_LABEL_GEOMETRY: StrhashT = StrhashT {
    u64_: 10328401752681028853u64,
};
pub const TM_CORE_ID_DEFAULT_SKY: StrhashT = StrhashT {
    u64_: 7363654769829550863u64,
};
pub const TM_DELAUNAY_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 8171429600103526299u64,
};
pub const TM_DOWNLOAD_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 11801884756962955328u64,
};
pub const TM_WELCOME_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 10557901196531259385u64,
};
pub const TM_TT_TYPE_HASH__APPLICATION_SETTINGS__UPDATES: StrhashT = StrhashT {
    u64_: 9690335476561887142u64,
};
pub const TM_TT_TYPE_HASH__APPLICATION_SETTINGS__DOWNLOAD: StrhashT = StrhashT {
    u64_: 15292496360168744251u64,
};
pub const TM_TREE_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 15475822667703539061u64,
};
pub const TM_GENERIC_TREE_VIEW_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 7879992602091662578u64,
};
pub const TM_GRAPH_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 7469907488998227710u64,
};
pub const TM_IMPORT_PROJECT_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 5425513247114970647u64,
};
pub const TM_INPUT_LOG_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 3670387838326478915u64,
};
pub const TM_JSON_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 6402701518264315318u64,
};
pub const TM_LOGIN_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 13936248484350100215u64,
};
pub const TM_MODIFIED_ASSETS_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 10659480203678754394u64,
};
pub const TM_NETWORK_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 8766316902787013043u64,
};
pub const TM_PREVIEW_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 3090695599756087083u64,
};
pub const TM_PROFILER_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 6501996879298069992u64,
};
pub const TM_PROPERTIES_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 13146008834947116426u64,
};
pub const TM_SCENE_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 8924396880956854330u64,
};
pub const TM_TT_TYPE_HASH__SCENE_TAB: StrhashT = StrhashT {
    u64_: 8924396880956854330u64,
};
pub const TM_TT_TYPE_HASH__SCENE_TAB_SETTINGS: StrhashT = StrhashT {
    u64_: 11302401724387347019u64,
};
pub const TM_TT_TYPE_HASH__SCENE_SETTINGS: StrhashT = StrhashT {
    u64_: 1480825796711809733u64,
};
pub const TM_SIMULATE_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 15540707913236182876u64,
};
pub const TM_TT_TYPE_HASH__SIMULATE_TAB_SETTINGS: StrhashT = StrhashT {
    u64_: 9183098095369085156u64,
};
pub const TM_STATE_GRAPH_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 14410119148743899870u64,
};
pub const TM_STATISTICS_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 8580271604033701237u64,
};
pub const TM_TASK_MANAGER_TAB_VT_NAME_HASH: StrhashT = StrhashT {
    u64_: 3581857845760553842u64,
};
pub const TM_TT_TYPE_HASH__APPLICATION_SETTINGS: StrhashT = StrhashT {
    u64_: 4146725298831762617u64,
};
pub const TM_TT_TYPE_HASH__PROJECT_MANAGEMENT_SETTINGS: StrhashT = StrhashT {
    u64_: 10054850684661594383u64,
};
pub const TM_TT_TYPE_HASH__INTERFACE_SETTINGS: StrhashT = StrhashT {
    u64_: 15196734740682162288u64,
};
pub const TM_TT_TYPE_HASH__WINDOW_LAYOUTS: StrhashT = StrhashT {
    u64_: 12486477828238302005u64,
};
pub const TM_TT_TYPE_HASH__WINDOW_LAYOUT: StrhashT = StrhashT {
    u64_: 2242415212777679619u64,
};
pub const TM_TT_TYPE_HASH__WORKSPACE: StrhashT = StrhashT {
    u64_: 14829736330726518513u64,
};
pub const TM_TT_TYPE_HASH__RECENT_FILE: StrhashT = StrhashT {
    u64_: 13424770189634016662u64,
};
pub const TM_TT_TYPE_HASH__PUBLISH_SETTINGS: StrhashT = StrhashT {
    u64_: 10298575024838714316u64,
};
pub const TM_TT_TYPE_HASH__APPLICATION_SECURITY: StrhashT = StrhashT {
    u64_: 12727897416163656223u64,
};
pub const TM_TT_TYPE_HASH__PROJECT_SETTINGS: StrhashT = StrhashT {
    u64_: 16275721671749807405u64,
};
pub const TM_TT_TYPE_HASH__NETWORK_SETTINGS: StrhashT = StrhashT {
    u64_: 4133456848111380615u64,
};
pub const TM_TT_TYPE_HASH__LOCALIZATION_SETTINGS: StrhashT = StrhashT {
    u64_: 12227184065019180970u64,
};
pub const TM_TT_TYPE_HASH__TRANSLATION: StrhashT = StrhashT {
    u64_: 1601245301749355993u64,
};
pub const TM_TT_TYPE_HASH__TRANSLATION_CACHE: StrhashT = StrhashT {
    u64_: 15932710862828710769u64,
};
pub const TM_THE_MACHINERY_PROJECT_CALLBACK_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_THE_MACHINERY_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_THE_MACHINERY_CREATE_LAYOUT_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_SIMULATE_TAB_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_STATISTICS_TAB_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_ASSET_BROWSER_TAB_CUSTOM_MENU_ITEMS_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_UPDATE_TRACKER_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_TASK_MANAGER_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_INTEGRATION_TEST_RUNNER_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_LOGIN_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_ASSET_BROWSER_TAB_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
