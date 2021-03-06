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
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
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
pub const TM_TT_TYPE__CAMERA_COMPONENT: &'static [u8; 20usize] = b"tm_camera_component\0";
pub const TM_TT_TYPE__CONSTRAINT_COMPONENT: &'static [u8; 24usize] = b"tm_constraint_component\0";
pub const TM_TT_TYPE__AIM_CONSTRAINT: &'static [u8; 18usize] = b"tm_aim_constraint\0";
pub const TM_TT_TYPE__FABRIK_CONSTRAINT: &'static [u8; 21usize] = b"tm_fabrik_constraint\0";
pub const TM_TT_TYPE__VERLET_CONSTRAINT__COLLISION_SHAPE: &'static [u8; 37usize] =
    b"tm_verlet_constraint_collision_shape\0";
pub const TM_TT_TYPE__VERLET_CONSTRAINT: &'static [u8; 21usize] = b"tm_verlet_constraint\0";
pub const TM_TT_TYPE__ENTITY_SORT_VALUE: &'static [u8; 21usize] = b"tm_entity_sort_value\0";
pub const TM_TT_TYPE__ENTITY: &'static [u8; 10usize] = b"tm_entity\0";
pub const TM_MAX_COMPONENTS_IN_CONTEXT: u32 = 1024;
pub const TM_MAX_COMPONENTS_FOR_ENGINE: u32 = 16;
pub const TM_TT_TYPE__OWNER_COMPONENT: &'static [u8; 19usize] = b"tm_owner_component\0";
pub const TM_TT_TYPE__SCENE_TREE_COMPONENT: &'static [u8; 24usize] = b"tm_scene_tree_component\0";
pub const TM_TT_TYPE__TAG_COMPONENT: &'static [u8; 17usize] = b"tm_tag_component\0";
pub const TM_TT_TYPE__TAG: &'static [u8; 7usize] = b"tm_tag\0";
pub const TM_TT_TYPE__TRANSFORM_COMPONENT: &'static [u8; 23usize] = b"tm_transform_component\0";
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
pub const TM_TT_PROP__CAMERA_COMPONENT__PROJECTION_MODE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__CAMERA_COMPONENT__NEAR_PLANE: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__CAMERA_COMPONENT__FAR_PLANE: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__CAMERA_COMPONENT__VERTICAL_FOV_OR_BOX_HEIGHT: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__CAMERA_COMPONENT__SHUTTER_SPEED: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__CAMERA_COMPONENT__APERTURE: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__CAMERA_COMPONENT__ISO: ::std::os::raw::c_int = 6;
pub type _bindgen_ty_1 = ::std::os::raw::c_int;
pub type CameraComponentT = CameraSettingsT;
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
#[derive(Default, Copy, Clone)]
pub struct ComponentTypeT {
    pub index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetEntityT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EntityContextO {
    _unused: [u8; 0],
}
pub const TM_CONSTRAINT_TYPE__NONE: ConstraintType = 0;
pub const TM_CONSTRAINT_TYPE__LOCK: ConstraintType = 1;
pub const TM_CONSTRAINT_TYPE__AIM: ConstraintType = 2;
pub const TM_CONSTRAINT_TYPE__FABRIK: ConstraintType = 3;
pub const TM_CONSTRAINT_TYPE__VERLET: ConstraintType = 4;
pub const TM_CONSTRAINT_TYPE__COUNT: ConstraintType = 5;
pub type ConstraintType = ::std::os::raw::c_int;
pub const TM_CONSTRAINT_COMPONENT_NOT_LINKED_TO_SCENE_TREE_NODE: ::std::os::raw::c_int = -1;
pub type _bindgen_ty_2 = ::std::os::raw::c_int;
pub const TM_TT_PROP__CONSTRAINT_COMPONENT__ENABLED: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__CONSTRAINT_COMPONENT__BLEND_TIME: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__CONSTRAINT_COMPONENT__CONSTRAINED_ENTITY: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__CONSTRAINT_COMPONENT__CONSTRAINED_NODE: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__CONSTRAINT_COMPONENT__GOAL_ENTITY: ::std::os::raw::c_int = 4;
pub const TM_TT_PROP__CONSTRAINT_COMPONENT__GOAL_NODE: ::std::os::raw::c_int = 5;
pub const TM_TT_PROP__CONSTRAINT_COMPONENT__GOAL_POSITION: ::std::os::raw::c_int = 6;
pub const TM_TT_PROP__CONSTRAINT_COMPONENT__MATCH_ROTATION: ::std::os::raw::c_int = 7;
pub const TM_TT_PROP__CONSTRAINT_COMPONENT__GOAL_ROTATION: ::std::os::raw::c_int = 8;
pub const TM_TT_PROP__CONSTRAINT_COMPONENT__TYPE: ::std::os::raw::c_int = 9;
pub const TM_TT_PROP__CONSTRAINT_COMPONENT__CONSTRAINT: ::std::os::raw::c_int = 10;
pub type _bindgen_ty_3 = ::std::os::raw::c_int;
pub const TM_TT_PROP__AIM_CONSTRAINT__AXIS: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__AIM_CONSTRAINT__MAX_ANGLE: ::std::os::raw::c_int = 1;
pub type _bindgen_ty_4 = ::std::os::raw::c_int;
pub const TM_TT_PROP__FABRIK_CONSTRAINT__IK_CHAIN_LENGTH: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_5 = ::std::os::raw::c_int;
pub const TM_VERLET_CONSTRAINT_COLLISION_SHAPE_TYPE__PLANE: VerletConstraintCollisionShapeType = 0;
pub const TM_VERLET_CONSTRAINT_COLLISION_SHAPE_TYPE__SPHERE: VerletConstraintCollisionShapeType = 1;
pub const TM_VERLET_CONSTRAINT_COLLISION_SHAPE_TYPE__COUNT: VerletConstraintCollisionShapeType = 2;
pub type VerletConstraintCollisionShapeType = ::std::os::raw::c_int;
pub const TM_TT_PROP__VERLET_CONSTRAINT__COLLISION_SHAPE__TYPE: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__VERLET_CONSTRAINT__COLLISION_SHAPE__NODE: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__VERLET_CONSTRAINT__COLLISION_SHAPE__POSITION: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__VERLET_CONSTRAINT__COLLISION_SHAPE__ROTATION: ::std::os::raw::c_int = 3;
pub const TM_TT_PROP__VERLET_CONSTRAINT__COLLISION_SHAPE__RADIUS: ::std::os::raw::c_int = 4;
pub type _bindgen_ty_6 = ::std::os::raw::c_int;
pub const TM_TT_PROP__VERLET_CONSTRAINT__CHAIN_LENGTH: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__VERLET_CONSTRAINT__DAMPING: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__VERLET_CONSTRAINT__COLLISION_SHAPES: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_7 = ::std::os::raw::c_int;
#[repr(C)]
pub struct AimConstraintT {
    pub axis: Vec3T,
    pub max_angle: f32,
}
impl Default for AimConstraintT {
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
pub struct FabrikConstraintT {
    pub ik_chain_length: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VerletConstraintDataT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VerletConstraintT {
    pub chain_length: u32,
    pub damping: f32,
    pub data: *mut VerletConstraintDataT,
}
impl Default for VerletConstraintT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct ConstraintComponentT {
    pub type_: ConstraintType,
    pub enabled: bool,
    pub was_enabled: bool,
    pub _padding_226: [::std::os::raw::c_char; 2usize],
    pub blend_time: f32,
    pub blend: f32,
    pub constrained_entity: EntityT,
    pub constrained_node_idx: u32,
    pub _padding_239: [::std::os::raw::c_char; 4usize],
    pub goal_entity: EntityT,
    pub goal_node_idx: u32,
    pub _padding_246: [::std::os::raw::c_char; 4usize],
    pub goal_position: Vec3T,
    pub match_rotation: bool,
    pub _padding_254: [::std::os::raw::c_char; 3usize],
    pub goal_rotation: Vec4T,
    pub __bindgen_anon_1: ConstraintComponentTBindgenTy1,
}
#[repr(C)]
pub struct ConstraintComponentTBindgenTy1 {
    pub aim: __BindgenUnionField<AimConstraintT>,
    pub fabrik: __BindgenUnionField<FabrikConstraintT>,
    pub verlet: __BindgenUnionField<VerletConstraintT>,
    pub bindgen_union_field: [u64; 2usize],
}
impl Default for ConstraintComponentTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for ConstraintComponentT {
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
pub struct ConstraintComponentManagerO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PrimitiveDrawerBufferT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GamestateO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GamestateObjectIdT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GamestateStructIdT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GamestateMemberT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GamestatePersistentBufferT {
    _unused: [u8; 0],
}
pub const TM_TT_PROP__ENTITY__SORT_VALUE_CHILD: TM_TT_PROP__ENTITY_SORT_VALUE = 0;
pub const TM_TT_PROP__ENTITY__SORT_VALUE_VALUE: TM_TT_PROP__ENTITY_SORT_VALUE = 1;
pub type TM_TT_PROP__ENTITY_SORT_VALUE = ::std::os::raw::c_int;
pub const TM_TT_PROP__ENTITY__NAME: TM_TT_PROP__ENTITY = 0;
pub const TM_TT_PROP__ENTITY__COMPONENTS: TM_TT_PROP__ENTITY = 1;
pub const TM_TT_PROP__ENTITY__CHILDREN: TM_TT_PROP__ENTITY = 2;
pub const TM_TT_PROP__ENTITY__CHILD_SORT_VALUES: TM_TT_PROP__ENTITY = 3;
pub const TM_TT_PROP__ENTITY__PERSISTENCE: TM_TT_PROP__ENTITY = 4;
pub type TM_TT_PROP__ENTITY = ::std::os::raw::c_int;
pub const TM_ENTITY_PERSISTENCE__INHERIT: EntityPersistence = 0;
pub const TM_ENTITY_PERSISTENCE__PERSISTENT: EntityPersistence = 1;
pub const TM_ENTITY_PERSISTENCE__PERSISTENT_REPLICATED: EntityPersistence = 2;
pub const TM_ENTITY_PERSISTENCE__NON_PERSISTENT: EntityPersistence = 3;
pub type EntityPersistence = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EntityCommandsO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ComponentMaskT {
    pub bits: [u64; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComponentManagerO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComponentGamestateRepresentationI {
    pub size: u32,
    pub compute_initial_hash_immediately: bool,
    pub lazy_component_serialization: bool,
    pub _padding_158: [::std::os::raw::c_char; 2usize],
    pub restore_sort_order: f32,
    pub _padding_169: [::std::os::raw::c_char; 4usize],
    pub user_data: *mut ::std::os::raw::c_void,
    pub serialize: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            e: EntityT,
            component: ComponentTypeT,
            buffer: *mut ::std::os::raw::c_void,
            buffer_size: u32,
        ),
    >,
    pub deserialize: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            e: EntityT,
            component: ComponentTypeT,
            buffer: *const ::std::os::raw::c_void,
            buffer_size: u32,
        ),
    >,
    pub compute_hash: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            e: EntityT,
            component: ComponentTypeT,
            data: *const ::std::os::raw::c_void,
            size: u32,
        ) -> u64,
    >,
    pub compute_asset_hash: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            tt: *mut TheTruthO,
            component: ComponentTypeT,
            asset: TtIdT,
        ) -> u64,
    >,
    pub loaded: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ComponentManagerO,
            e: EntityT,
            data: *mut ::std::os::raw::c_void,
        ),
    >,
    pub num_shared_members: u32,
    pub _padding_217: [::std::os::raw::c_char; 4usize],
    pub shared_members: *mut GamestateMemberT,
}
impl Default for ComponentGamestateRepresentationI {
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
pub struct ComponentPersistenceI {
    pub manual_tracking: bool,
    pub _padding_235: [::std::os::raw::c_char; 3usize],
    pub num_members: u32,
    pub members: *mut GamestateMemberT,
}
impl Default for ComponentPersistenceI {
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
pub struct MemberNetworkReplicationT {
    pub watch_timer: f64,
    pub raw_component_offset: u32,
    pub _padding_259: [::std::os::raw::c_char; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComponentNetworkReplicationI {
    pub watch_timer: f64,
    pub num_members: u32,
    pub _padding_271: [::std::os::raw::c_char; 4usize],
    pub members: *mut GamestateMemberT,
    pub member_replication: *mut MemberNetworkReplicationT,
}
impl Default for ComponentNetworkReplicationI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct ComponentI {
    pub name: *const ::std::os::raw::c_char,
    pub bytes: u32,
    pub _padding_285: [::std::os::raw::c_char; 4usize],
    pub default_data: *const ::std::os::raw::c_void,
    pub manager: *mut ComponentManagerO,
    pub components_created:
        ::std::option::Option<unsafe extern "C" fn(manager: *mut ComponentManagerO)>,
    pub load_asset: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ComponentManagerO,
            commands: *mut EntityCommandsO,
            e: EntityT,
            data: *mut ::std::os::raw::c_void,
            tt: *const TheTruthO,
            asset: TtIdT,
        ) -> bool,
    >,
    pub asset_loaded: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ComponentManagerO,
            commands: *mut EntityCommandsO,
            e: EntityT,
            data: *mut ::std::os::raw::c_void,
        ),
    >,
    pub asset_loaded_sort_order: f64,
    pub asset_reloaded: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ComponentManagerO,
            commands: *mut EntityCommandsO,
            e: EntityT,
            data: *mut ::std::os::raw::c_void,
        ),
    >,
    pub add: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ComponentManagerO,
            commands: *mut EntityCommandsO,
            e: EntityT,
            data: *mut ::std::os::raw::c_void,
        ),
    >,
    pub remove: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ComponentManagerO,
            commands: *mut EntityCommandsO,
            e: EntityT,
            data: *mut ::std::os::raw::c_void,
        ),
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(manager: *mut ComponentManagerO)>,
    pub debug_draw: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut ComponentManagerO,
            e: *mut EntityT,
            data: *mut *const ::std::os::raw::c_void,
            n: u32,
            pbuf: *mut PrimitiveDrawerBufferT,
            vbuf: *mut PrimitiveDrawerBufferT,
            allocator: *mut AllocatorI,
            camera: *const CameraT,
            viewport: RectT,
        ),
    >,
    pub debug_draw_settings: TtIdT,
    pub gamestate_representation: *mut ComponentGamestateRepresentationI,
    pub persistence: *mut ComponentPersistenceI,
    pub network_replication: *mut ComponentNetworkReplicationI,
}
impl Default for ComponentI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type EntityCreateComponentI =
    ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO)>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EngineUpdateArrayT {
    pub entities: *mut EntityT,
    pub components: [*mut ::std::os::raw::c_void; 16usize],
    pub component_bytes: [u32; 16usize],
    pub n: u32,
    pub _padding_373: [::std::os::raw::c_char; 4usize],
}
impl Default for EngineUpdateArrayT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct EntityBlackboardValueT {
    pub id: StrhashT,
    pub __bindgen_anon_1: EntityBlackboardValueTBindgenTy1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union EntityBlackboardValueTBindgenTy1 {
    pub double_value: f64,
    pub ptr_value: *mut ::std::os::raw::c_void,
}
impl Default for EntityBlackboardValueTBindgenTy1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for EntityBlackboardValueT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct EngineUpdateSetT {
    pub engine: *const EngineI,
    pub total_entities: u32,
    pub _padding_396: [::std::os::raw::c_char; 4usize],
    pub blackboard_start: *const EntityBlackboardValueT,
    pub blackboard_end: *const EntityBlackboardValueT,
    pub num_arrays: u32,
    pub _padding_401: [::std::os::raw::c_char; 4usize],
    pub arrays: __IncompleteArrayField<EngineUpdateArrayT>,
}
impl Default for EngineUpdateSetT {
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
pub struct EngineO {
    _unused: [u8; 0],
}
pub const TM_MAX_DEPENDENCIES_FOR_ENGINE: ::std::os::raw::c_int = 16;
pub type _bindgen_ty_8 = ::std::os::raw::c_int;
#[repr(C)]
pub struct EngineSystemCommonI {
    pub ui_name: *const ::std::os::raw::c_char,
    pub hash: StrhashT,
    pub disabled: bool,
    pub exclusive: bool,
    pub _padding_448: [::std::os::raw::c_char; 2usize],
    pub num_components: u32,
    pub components: [ComponentTypeT; 16usize],
    pub writes: [bool; 16usize],
    pub before_me: [StrhashT; 16usize],
    pub after_me: [StrhashT; 16usize],
    pub phase: StrhashT,
}
impl Default for EngineSystemCommonI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct EngineI {
    pub super_: EngineSystemCommonI,
    pub inst: *mut EngineO,
    pub update: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut EngineO,
            data: *mut EngineUpdateSetT,
            commands: *mut EntityCommandsO,
        ),
    >,
    pub num_excluded: u32,
    pub _padding_504: [::std::os::raw::c_char; 4usize],
    pub excluded: [ComponentTypeT; 16usize],
    pub filter: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut EngineO,
            components: *const ComponentTypeT,
            num_components: u32,
            mask: *const ComponentMaskT,
        ) -> bool,
    >,
}
impl Default for EngineI {
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
pub struct EntitySystemO {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct EntitySystemI {
    pub super_: EngineSystemCommonI,
    pub inst: *mut EntitySystemO,
    pub inited: bool,
    pub _padding_553: [::std::os::raw::c_char; 7usize],
    pub init: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            inst: *mut EntitySystemO,
            commands: *mut EntityCommandsO,
        ),
    >,
    pub update: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            inst: *mut EntitySystemO,
            commands: *mut EntityCommandsO,
        ),
    >,
    pub shutdown: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            inst: *mut EntitySystemO,
            commands: *mut EntityCommandsO,
        ),
    >,
    pub hot_reload: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            inst: *mut EntitySystemO,
            commands: *mut EntityCommandsO,
        ),
    >,
}
impl Default for EntitySystemI {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type EntityRegisterEnginesI =
    ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO)>;
pub type EntityRegisterEnginesSimulationI = EntityRegisterEnginesI;
pub type EntityRegisterEnginesEditorI = EntityRegisterEnginesI;
pub const TM_ENTITY_CREATE_COMPONENTS_NONE: EntityCreateComponents = 0;
pub const TM_ENTITY_CREATE_COMPONENTS_ALL: EntityCreateComponents = 1;
pub const TM_ENTITY_CREATE_COMPONENTS_EDITOR: EntityCreateComponents = 2;
pub type EntityCreateComponents = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EntityListenerI {
    pub man: *mut ComponentManagerO,
    pub notify_e: EntityT,
    pub notify_c: ComponentTypeT,
    pub _padding_666: [::std::os::raw::c_char; 4usize],
    pub notify: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            man: *mut ComponentManagerO,
            e: EntityT,
            c: *mut ::std::os::raw::c_void,
            notify_e: EntityT,
            notify_c: *mut ::std::os::raw::c_void,
        ),
    >,
}
impl Default for EntityListenerI {
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
pub struct EntityEventListenerI {
    pub inst: *mut ::std::os::raw::c_void,
    pub user_data: *mut ::std::os::raw::c_void,
    pub notify: ::std::option::Option<
        unsafe extern "C" fn(
            inst: *mut ::std::os::raw::c_void,
            ctx: *mut EntityContextO,
            event: StrhashT,
            e: EntityT,
            event_data: *const ::std::os::raw::c_void,
            event_data_bytes: u32,
            user_data: *mut ::std::os::raw::c_void,
        ),
    >,
}
impl Default for EntityEventListenerI {
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
pub struct EntityArrayT {
    pub entities: *mut EntityT,
    pub n: u32,
    pub _padding_694: [::std::os::raw::c_char; 4usize],
}
impl Default for EntityArrayT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct EntitySetT {
    pub total_entities: u32,
    pub num_arrays: u32,
    pub arrays: __IncompleteArrayField<EntityArrayT>,
}
impl Default for EntitySetT {
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
pub struct EntityApi {
    pub create_truth_types: ::std::option::Option<unsafe extern "C" fn(tt: *mut TheTruthO)>,
    pub create_context: ::std::option::Option<
        unsafe extern "C" fn(
            a: *mut AllocatorI,
            tt: *mut TheTruthO,
            create_components: EntityCreateComponents,
        ) -> *mut EntityContextO,
    >,
    pub create_components: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, create_components: EntityCreateComponents),
    >,
    pub destroy_context: ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO)>,
    pub register_component: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, com: *const ComponentI) -> ComponentTypeT,
    >,
    pub disable_component: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, component_hash: StrhashT),
    >,
    pub num_components:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO) -> u32>,
    pub component: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            component_type: ComponentTypeT,
        ) -> *const ComponentI,
    >,
    pub register_engine: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, engine: *const EngineI),
    >,
    pub remove_engine: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, engine_hash: StrhashT),
    >,
    pub registered_engines: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, count: *mut u32) -> *mut EngineI,
    >,
    pub register_system: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, system: *const EntitySystemI),
    >,
    pub remove_system: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, system_hash: StrhashT),
    >,
    pub registered_systems: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, count: *mut u32) -> *mut EntitySystemI,
    >,
    pub create_child_allocator: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            name: *const ::std::os::raw::c_char,
            a: *mut AllocatorI,
        ),
    >,
    pub destroy_child_allocator:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO, a: *mut AllocatorI)>,
    pub the_truth:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO) -> *mut TheTruthO>,
    pub create_entity:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO) -> EntityT>,
    pub batch_create_entity: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, es: *mut EntityT, n: u32),
    >,
    pub create_entity_from_mask: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, mask: *const ComponentMaskT) -> EntityT,
    >,
    pub batch_create_entity_from_mask: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            mask: *const ComponentMaskT,
            es: *mut EntityT,
            n: u32,
        ),
    >,
    pub create_entity_from_asset: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, asset: TtIdT) -> EntityT,
    >,
    pub batch_create_entity_from_asset: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, asset: TtIdT, es: *mut EntityT, n: u32),
    >,
    pub destroy_entity:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO, e: EntityT)>,
    pub batch_destroy_entity: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, es: *const EntityT, n: u32),
    >,
    pub clear_world: ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO)>,
    pub queue_destroy_entities: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, es: *const EntityT, n: u32),
    >,
    pub is_alive:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO, e: EntityT) -> bool>,
    pub num_entities: ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO) -> u32>,
    pub entities_matching: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            required: *const ComponentMaskT,
            ta: *mut TempAllocatorI,
        ) -> *mut EntitySetT,
    >,
    pub entities_matching_with_forbidden: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            required: *const ComponentMaskT,
            forbidden: *const ComponentMaskT,
            ta: *mut TempAllocatorI,
        ) -> *mut EntitySetT,
    >,
    pub flatten_set:
        ::std::option::Option<unsafe extern "C" fn(entities: *mut EntityT, set: *const EntitySetT)>,
    pub lookup_component_type: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, name_hash: StrhashT) -> ComponentTypeT,
    >,
    pub component_manager: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            component_type: ComponentTypeT,
        ) -> *mut ComponentManagerO,
    >,
    pub component_manager_by_hash: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            name_hash: StrhashT,
        ) -> *mut ComponentManagerO,
    >,
    pub create_component_mask: ::std::option::Option<
        unsafe extern "C" fn(components: *const ComponentTypeT, n: u32) -> ComponentMaskT,
    >,
    pub component_mask: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, e: EntityT) -> *const ComponentMaskT,
    >,
    pub add_component: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            e: EntityT,
            component: ComponentTypeT,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_component: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            e: EntityT,
            component: ComponentTypeT,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub get_component_by_hash: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            e: EntityT,
            name_hash: StrhashT,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub remove_component: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, e: EntityT, component: ComponentTypeT),
    >,
    pub call_remove_on_all_entities: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, component: ComponentTypeT),
    >,
    pub asset_parent: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, e: EntityT) -> EntityT,
    >,
    pub children: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            e: EntityT,
            ta: *mut TempAllocatorI,
        ) -> *mut EntityT,
    >,
    pub asset:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO, e: EntityT) -> TtIdT>,
    pub find_entity_from_asset: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, asset: TtIdT) -> EntityT,
    >,
    pub resolve_asset_reference: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, e: EntityT, asset: TtIdT) -> EntityT,
    >,
    pub resolve_path: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            e: EntityT,
            path: *const ::std::os::raw::c_char,
        ) -> EntityT,
    >,
    pub propagate_asset_changes:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO)>,
    pub propagate_listen_to: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, e: EntityT, object: TtIdT),
    >,
    pub set_blackboard_double: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, id: StrhashT, value: f64),
    >,
    pub set_blackboard_ptr: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            id: StrhashT,
            value: *mut ::std::os::raw::c_void,
        ),
    >,
    pub has_blackboard:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO, id: StrhashT) -> bool>,
    pub get_blackboard_double: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, id: StrhashT, def: f64) -> f64,
    >,
    pub get_blackboard_ptr: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, id: StrhashT) -> *mut ::std::os::raw::c_void,
    >,
    pub run_engine: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, engine: *const EngineI),
    >,
    pub run_engine_with_commands: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            engine: *const EngineI,
            commands: *mut EntityCommandsO,
        ),
    >,
    pub update: ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO)>,
    pub listen: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            e: EntityT,
            c: ComponentTypeT,
            listener: *const EntityListenerI,
        ),
    >,
    pub unlisten: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            e: EntityT,
            c: ComponentTypeT,
            listener: *const EntityListenerI,
        ),
    >,
    pub notify: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            component_type: ComponentTypeT,
            entities: *const EntityT,
            num_entities: u32,
        ),
    >,
    pub listen_event: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            e: EntityT,
            event: StrhashT,
            listener: *const EntityEventListenerI,
        ),
    >,
    pub unlisten_event: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            e: EntityT,
            event: StrhashT,
            listener: *const EntityEventListenerI,
        ),
    >,
    pub unlisten_all: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, instance: *mut ::std::os::raw::c_void),
    >,
    pub notify_event: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            event: StrhashT,
            e: *mut EntityT,
            entity_stride: u32,
            data: *mut ::std::os::raw::c_void,
            data_stride: u32,
            n: u32,
        ),
    >,
    pub hot_reload: ::std::option::Option<unsafe extern "C" fn()>,
    pub set_debug_draw: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            component_type: ComponentTypeT,
            tag: StrhashT,
            debug_draw: bool,
        ),
    >,
    pub get_all_debug_draws: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            component_types: *mut *mut ComponentTypeT,
            tags: *mut *mut StrhashT,
            ta: *mut TempAllocatorI,
        ),
    >,
    pub has_debug_draw: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            component_type: ComponentTypeT,
            tag: StrhashT,
        ) -> bool,
    >,
    pub clear_debug_draw: ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO)>,
    pub debug_draw: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            pbuf: *mut PrimitiveDrawerBufferT,
            vbuf: *mut PrimitiveDrawerBufferT,
            allocator: *mut AllocatorI,
            camera: *const CameraT,
            viewport: RectT,
        ),
    >,
    pub gamestate:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO) -> *mut GamestateO>,
    pub override_component_persistence: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            c: ComponentTypeT,
            persistence: *mut ComponentPersistenceI,
        ),
    >,
    pub override_component_network_replication: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            c: ComponentTypeT,
            replication: *mut ComponentNetworkReplicationI,
        ),
    >,
    pub propagate_persistence_changes_to_gamestate:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO)>,
    pub propagate_network_replication_changes_to_gamestate:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO)>,
    pub ensure_entity_is_persistent:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO, e: EntityT)>,
    pub ensure_entity_is_replicated:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut EntityContextO, e: EntityT)>,
    pub get_entity_persistent_id: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            e: EntityT,
            output: *mut GamestateObjectIdT,
        ) -> bool,
    >,
    pub get_entity_network_id: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            e: EntityT,
            output: *mut GamestateObjectIdT,
        ) -> bool,
    >,
    pub get_component_persistent_id: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            e: EntityT,
            c: ComponentTypeT,
            output: *mut GamestateStructIdT,
        ) -> bool,
    >,
    pub get_component_network_id: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut EntityContextO,
            e: EntityT,
            c: ComponentTypeT,
            output: *mut GamestateStructIdT,
        ) -> bool,
    >,
    pub lookup_entity_from_gamestate_id: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut EntityContextO, id: *const GamestateObjectIdT) -> EntityT,
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union EntityCommandEntityHandleT {
    pub __bindgen_anon_1: EntityCommandEntityHandleTBindgenTy1,
    pub u64_: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct EntityCommandEntityHandleTBindgenTy1 {
    pub entity_index: u32,
    pub asset_index: u32,
}
impl Default for EntityCommandEntityHandleT {
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
pub struct EntityCommandsApi {
    pub create_entity_from_mask: ::std::option::Option<
        unsafe extern "C" fn(
            commands: *mut EntityCommandsO,
            mask: *const ComponentMaskT,
        ) -> EntityCommandEntityHandleT,
    >,
    pub batch_create_entity_from_mask: ::std::option::Option<
        unsafe extern "C" fn(
            commands: *mut EntityCommandsO,
            mask: *const ComponentMaskT,
            n: u32,
            ta: *mut TempAllocatorI,
        ) -> *mut EntityCommandEntityHandleT,
    >,
    pub create_entity_from_asset: ::std::option::Option<
        unsafe extern "C" fn(
            commands: *mut EntityCommandsO,
            asset: TtIdT,
        ) -> EntityCommandEntityHandleT,
    >,
    pub batch_create_entity_from_asset: ::std::option::Option<
        unsafe extern "C" fn(
            commands: *mut EntityCommandsO,
            asset: *mut TtIdT,
            n: u32,
            ta: *mut TempAllocatorI,
        ) -> *mut EntityCommandEntityHandleT,
    >,
    pub destroy_entity:
        ::std::option::Option<unsafe extern "C" fn(commands: *mut EntityCommandsO, e: EntityT)>,
    pub batch_destroy_entity: ::std::option::Option<
        unsafe extern "C" fn(commands: *mut EntityCommandsO, es: *const EntityT, n: u32),
    >,
    pub clear_world: ::std::option::Option<unsafe extern "C" fn(commands: *mut EntityCommandsO)>,
    pub add_component: ::std::option::Option<
        unsafe extern "C" fn(
            commands: *mut EntityCommandsO,
            e: EntityT,
            component: ComponentTypeT,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub remove_component: ::std::option::Option<
        unsafe extern "C" fn(commands: *mut EntityCommandsO, e: EntityT, component: ComponentTypeT),
    >,
    pub add_component_by_handle: ::std::option::Option<
        unsafe extern "C" fn(
            commands: *mut EntityCommandsO,
            e: EntityCommandEntityHandleT,
            component: ComponentTypeT,
        ) -> *mut ::std::os::raw::c_void,
    >,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct OwnerComponentT {
    pub num_children: u32,
    pub inst: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OwnerComponentManagerO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct OwnerComponentApi {
    pub children: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *const OwnerComponentManagerO,
            c: *const OwnerComponentT,
            children: *mut EntityT,
        ),
    >,
    pub add_children: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut OwnerComponentManagerO,
            c: *mut OwnerComponentT,
            e: *const EntityT,
            n: u32,
        ),
    >,
    pub remove_children: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut OwnerComponentManagerO,
            c: *mut OwnerComponentT,
            e: *const EntityT,
            n: u32,
        ),
    >,
    pub remove_all_children: ::std::option::Option<
        unsafe extern "C" fn(manager: *mut OwnerComponentManagerO, c: *mut OwnerComponentT),
    >,
    pub descendants: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *const OwnerComponentManagerO,
            c: *const OwnerComponentT,
            ta: *mut TempAllocatorI,
        ) -> *mut EntityT,
    >,
}
pub const TM_TT_PROP__SCENE_TREE_COMPONENT__NODES: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__SCENE_TREE_COMPONENT__NODE_NAMES: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__SCENE_TREE_COMPONENT__ASSET: ::std::os::raw::c_int = 2;
pub type _bindgen_ty_9 = ::std::os::raw::c_int;
pub const TM_SCENE_TREE_COMPONENT_ROOT_NODE_PARENT: ::std::os::raw::c_int = -1;
pub type _bindgen_ty_10 = ::std::os::raw::c_int;
#[repr(C)]
pub struct SceneTreeNodeT {
    pub name: StrhashT,
    pub parent_idx: u32,
    pub local_transform: TransformT,
    pub _padding_53: [::std::os::raw::c_char; 4usize],
}
impl Default for SceneTreeNodeT {
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
pub struct SceneTreeNodeNamesT {
    pub hash: u64,
    pub n: u32,
    pub _padding_71: [::std::os::raw::c_char; 4usize],
    pub node_names: *const StrhashT,
}
impl Default for SceneTreeNodeNamesT {
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
pub struct SceneTreeComponentT {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceneTreeComponentManagerO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct SceneTreeComponentApi {
    pub setup_nodes: ::std::option::Option<
        unsafe extern "C" fn(
            manager: *mut SceneTreeComponentManagerO,
            st: *mut SceneTreeComponentT,
            nodes: *const SceneTreeNodeT,
            num_nodes: u32,
            debug_names: *const ::std::os::raw::c_char,
        ),
    >,
    pub node_names: ::std::option::Option<
        unsafe extern "C" fn(st: *const SceneTreeComponentT) -> SceneTreeNodeNamesT,
    >,
    pub node_index_from_name: ::std::option::Option<
        unsafe extern "C" fn(
            component: *const SceneTreeComponentT,
            node_name: StrhashT,
            not_found: u32,
        ) -> u32,
    >,
    pub node_debug_name_from_index: ::std::option::Option<
        unsafe extern "C" fn(
            component: *const SceneTreeComponentT,
            node_idx: u32,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub world_matrices: ::std::option::Option<
        unsafe extern "C" fn(component: *const SceneTreeComponentT) -> *const Mat44T,
    >,
    pub local_matrices: ::std::option::Option<
        unsafe extern "C" fn(component: *const SceneTreeComponentT) -> *const Mat44T,
    >,
    pub local_transform: ::std::option::Option<
        unsafe extern "C" fn(component: *const SceneTreeComponentT, node_idx: u32) -> TransformT,
    >,
    pub world_transform: ::std::option::Option<
        unsafe extern "C" fn(component: *const SceneTreeComponentT, node_idx: u32) -> TransformT,
    >,
    pub set_local_transform: ::std::option::Option<
        unsafe extern "C" fn(
            component: *mut SceneTreeComponentT,
            node_idx: u32,
            t: *const TransformT,
        ),
    >,
    pub set_local_matrix: ::std::option::Option<
        unsafe extern "C" fn(component: *mut SceneTreeComponentT, node_idx: u32, tm: *const Mat44T),
    >,
    pub set_world_transform: ::std::option::Option<
        unsafe extern "C" fn(
            component: *mut SceneTreeComponentT,
            trans: *const TransformT,
            node_idx: u32,
            t: *const TransformT,
        ),
    >,
    pub transform: ::std::option::Option<
        unsafe extern "C" fn(st: *mut SceneTreeComponentT, trans: *const TransformT, version: u64),
    >,
    pub transform_subtree:
        ::std::option::Option<unsafe extern "C" fn(st: *mut SceneTreeComponentT, node_idx: u32)>,
    pub set_debug_color: ::std::option::Option<
        unsafe extern "C" fn(st: *mut SceneTreeComponentT, color: ColorSrgbT),
    >,
    pub node_parent_index: ::std::option::Option<
        unsafe extern "C" fn(st: *mut SceneTreeComponentT, node_idx: u32) -> u32,
    >,
}
pub const TM_TT_PROP__TAG_COMPONENT__TAGS: ::std::os::raw::c_int = 0;
pub type _bindgen_ty_11 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct TagComponentT {
    pub first_ti: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TagComponentManagerO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct TagComponentApi {
    pub tag_assets: ::std::option::Option<
        unsafe extern "C" fn(tt: *mut TheTruthO, ta: *mut TempAllocatorI) -> *mut TtIdT,
    >,
    pub add_tag: ::std::option::Option<
        unsafe extern "C" fn(tcm: *mut TagComponentManagerO, e: EntityT, tag: StrhashT),
    >,
    pub add_tag_optional: ::std::option::Option<
        unsafe extern "C" fn(tcm: *mut TagComponentManagerO, e: EntityT, tag: StrhashT),
    >,
    pub remove_tag: ::std::option::Option<
        unsafe extern "C" fn(tcm: *mut TagComponentManagerO, e: EntityT, tag: StrhashT),
    >,
    pub has_tag: ::std::option::Option<
        unsafe extern "C" fn(tcm: *const TagComponentManagerO, e: EntityT, tag: StrhashT) -> bool,
    >,
    pub find_all: ::std::option::Option<
        unsafe extern "C" fn(
            tcm: *const TagComponentManagerO,
            tag: StrhashT,
            ta: *mut TempAllocatorI,
        ) -> *mut EntityT,
    >,
    pub find_first: ::std::option::Option<
        unsafe extern "C" fn(tcm: *const TagComponentManagerO, tag: StrhashT) -> EntityT,
    >,
}
pub const TM_TT_PROP__TRANSFORM_COMPONENT__LOCAL_POSITION: ::std::os::raw::c_int = 0;
pub const TM_TT_PROP__TRANSFORM_COMPONENT__LOCAL_ROTATION: ::std::os::raw::c_int = 1;
pub const TM_TT_PROP__TRANSFORM_COMPONENT__LOCAL_SCALE: ::std::os::raw::c_int = 2;
pub const TM_TT_PROP__TRANSFORM_COMPONENT__SCENE_TREE_NODE: ::std::os::raw::c_int = 3;
pub type _bindgen_ty_12 = ::std::os::raw::c_int;
pub const TM_TRANSFORM_COMPONENT_NOT_LINKED_TO_SCENE_TREE_NODE: ::std::os::raw::c_int = -1;
pub type _bindgen_ty_13 = ::std::os::raw::c_int;
#[repr(C)]
pub struct TransformComponentT {
    pub world: TransformT,
    pub local: TransformT,
    pub parent: EntityT,
    pub version: u32,
    pub scene_tree_node_idx: u32,
    pub scene_tree_node_name: StrhashT,
}
impl Default for TransformComponentT {
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
pub struct TransformComponentManagerO {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct TransformComponentApi {
    pub get_position: ::std::option::Option<
        unsafe extern "C" fn(man: *mut TransformComponentManagerO, e: EntityT) -> Vec3T,
    >,
    pub get_rotation: ::std::option::Option<
        unsafe extern "C" fn(man: *mut TransformComponentManagerO, e: EntityT) -> Vec4T,
    >,
    pub get_scale: ::std::option::Option<
        unsafe extern "C" fn(man: *mut TransformComponentManagerO, e: EntityT) -> Vec3T,
    >,
    pub get_transform: ::std::option::Option<
        unsafe extern "C" fn(man: *mut TransformComponentManagerO, e: EntityT) -> *const TransformT,
    >,
    pub set_position: ::std::option::Option<
        unsafe extern "C" fn(man: *mut TransformComponentManagerO, e: EntityT, pos: Vec3T),
    >,
    pub set_rotation: ::std::option::Option<
        unsafe extern "C" fn(man: *mut TransformComponentManagerO, e: EntityT, rot: Vec4T),
    >,
    pub set_scale: ::std::option::Option<
        unsafe extern "C" fn(man: *mut TransformComponentManagerO, e: EntityT, scl: Vec3T),
    >,
    pub set_transform: ::std::option::Option<
        unsafe extern "C" fn(
            man: *mut TransformComponentManagerO,
            e: EntityT,
            transform: *const TransformT,
        ),
    >,
    pub get_local_position: ::std::option::Option<
        unsafe extern "C" fn(man: *mut TransformComponentManagerO, e: EntityT) -> Vec3T,
    >,
    pub get_local_rotation: ::std::option::Option<
        unsafe extern "C" fn(man: *mut TransformComponentManagerO, e: EntityT) -> Vec4T,
    >,
    pub get_local_scale: ::std::option::Option<
        unsafe extern "C" fn(man: *mut TransformComponentManagerO, e: EntityT) -> Vec3T,
    >,
    pub get_local_transform: ::std::option::Option<
        unsafe extern "C" fn(man: *mut TransformComponentManagerO, e: EntityT) -> *const TransformT,
    >,
    pub set_local_position: ::std::option::Option<
        unsafe extern "C" fn(man: *mut TransformComponentManagerO, e: EntityT, local_pos: Vec3T),
    >,
    pub set_local_rotation: ::std::option::Option<
        unsafe extern "C" fn(man: *mut TransformComponentManagerO, e: EntityT, local_rot: Vec4T),
    >,
    pub set_local_scale: ::std::option::Option<
        unsafe extern "C" fn(man: *mut TransformComponentManagerO, e: EntityT, local_scl: Vec3T),
    >,
    pub set_local_transform: ::std::option::Option<
        unsafe extern "C" fn(
            man: *mut TransformComponentManagerO,
            e: EntityT,
            local_transform: *const TransformT,
        ),
    >,
    pub update_world_transform: ::std::option::Option<
        unsafe extern "C" fn(man: *mut TransformComponentManagerO, e: EntityT),
    >,
    pub update_local_transform: ::std::option::Option<
        unsafe extern "C" fn(man: *mut TransformComponentManagerO, e: EntityT),
    >,
    pub compute_world_transform: ::std::option::Option<
        unsafe extern "C" fn(
            world: *mut TransformT,
            local: *const TransformT,
            parent_world: *const TransformT,
        ) -> *mut TransformT,
    >,
    pub compute_local_transform: ::std::option::Option<
        unsafe extern "C" fn(
            local: *mut TransformT,
            world: *const TransformT,
            parent_world: *const TransformT,
        ) -> *mut TransformT,
    >,
    pub compute_parent_transform: ::std::option::Option<
        unsafe extern "C" fn(
            parent_world: *mut TransformT,
            world: *const TransformT,
            local: *const TransformT,
        ) -> *mut TransformT,
    >,
    pub set_parent: ::std::option::Option<
        unsafe extern "C" fn(man: *mut TransformComponentManagerO, e: EntityT, parent: EntityT),
    >,
}

// Extensions generated by machinery-generator

use const_cstr::{const_cstr, ConstCStr};

use crate::foundation::VersionT;

use crate::foundation::*;

impl EntityApi {
    pub unsafe fn create_truth_types(&self, tt: *mut TheTruthO) {
        self.create_truth_types.unwrap()(tt)
    }

    pub unsafe fn create_context(
        &self,
        a: *mut AllocatorI,
        tt: *mut TheTruthO,
        create_components: EntityCreateComponents,
    ) -> *mut EntityContextO {
        self.create_context.unwrap()(a, tt, create_components)
    }

    pub unsafe fn create_components(
        &self,
        ctx: *mut EntityContextO,
        create_components: EntityCreateComponents,
    ) {
        self.create_components.unwrap()(ctx, create_components)
    }

    pub unsafe fn destroy_context(&self, ctx: *mut EntityContextO) {
        self.destroy_context.unwrap()(ctx)
    }

    pub unsafe fn register_component(
        &self,
        ctx: *mut EntityContextO,
        com: *const ComponentI,
    ) -> ComponentTypeT {
        self.register_component.unwrap()(ctx, com)
    }

    pub unsafe fn disable_component(&self, ctx: *mut EntityContextO, component_hash: StrhashT) {
        self.disable_component.unwrap()(ctx, component_hash)
    }

    pub unsafe fn num_components(&self, ctx: *mut EntityContextO) -> u32 {
        self.num_components.unwrap()(ctx)
    }

    pub unsafe fn component(
        &self,
        ctx: *mut EntityContextO,
        component_type: ComponentTypeT,
    ) -> *const ComponentI {
        self.component.unwrap()(ctx, component_type)
    }

    pub unsafe fn register_engine(&self, ctx: *mut EntityContextO, engine: *const EngineI) {
        self.register_engine.unwrap()(ctx, engine)
    }

    pub unsafe fn remove_engine(&self, ctx: *mut EntityContextO, engine_hash: StrhashT) {
        self.remove_engine.unwrap()(ctx, engine_hash)
    }

    pub unsafe fn registered_engines(
        &self,
        ctx: *mut EntityContextO,
        count: *mut u32,
    ) -> *mut EngineI {
        self.registered_engines.unwrap()(ctx, count)
    }

    pub unsafe fn register_system(&self, ctx: *mut EntityContextO, system: *const EntitySystemI) {
        self.register_system.unwrap()(ctx, system)
    }

    pub unsafe fn remove_system(&self, ctx: *mut EntityContextO, system_hash: StrhashT) {
        self.remove_system.unwrap()(ctx, system_hash)
    }

    pub unsafe fn registered_systems(
        &self,
        ctx: *mut EntityContextO,
        count: *mut u32,
    ) -> *mut EntitySystemI {
        self.registered_systems.unwrap()(ctx, count)
    }

    pub unsafe fn create_child_allocator(
        &self,
        ctx: *mut EntityContextO,
        name: *const ::std::os::raw::c_char,
        a: *mut AllocatorI,
    ) {
        self.create_child_allocator.unwrap()(ctx, name, a)
    }

    pub unsafe fn destroy_child_allocator(&self, ctx: *mut EntityContextO, a: *mut AllocatorI) {
        self.destroy_child_allocator.unwrap()(ctx, a)
    }

    pub unsafe fn the_truth(&self, ctx: *mut EntityContextO) -> *mut TheTruthO {
        self.the_truth.unwrap()(ctx)
    }

    pub unsafe fn create_entity(&self, ctx: *mut EntityContextO) -> EntityT {
        self.create_entity.unwrap()(ctx)
    }

    pub unsafe fn batch_create_entity(&self, ctx: *mut EntityContextO, es: *mut EntityT, n: u32) {
        self.batch_create_entity.unwrap()(ctx, es, n)
    }

    pub unsafe fn create_entity_from_mask(
        &self,
        ctx: *mut EntityContextO,
        mask: *const ComponentMaskT,
    ) -> EntityT {
        self.create_entity_from_mask.unwrap()(ctx, mask)
    }

    pub unsafe fn batch_create_entity_from_mask(
        &self,
        ctx: *mut EntityContextO,
        mask: *const ComponentMaskT,
        es: *mut EntityT,
        n: u32,
    ) {
        self.batch_create_entity_from_mask.unwrap()(ctx, mask, es, n)
    }

    pub unsafe fn create_entity_from_asset(
        &self,
        ctx: *mut EntityContextO,
        asset: TtIdT,
    ) -> EntityT {
        self.create_entity_from_asset.unwrap()(ctx, asset)
    }

    pub unsafe fn batch_create_entity_from_asset(
        &self,
        ctx: *mut EntityContextO,
        asset: TtIdT,
        es: *mut EntityT,
        n: u32,
    ) {
        self.batch_create_entity_from_asset.unwrap()(ctx, asset, es, n)
    }

    pub unsafe fn destroy_entity(&self, ctx: *mut EntityContextO, e: EntityT) {
        self.destroy_entity.unwrap()(ctx, e)
    }

    pub unsafe fn batch_destroy_entity(
        &self,
        ctx: *mut EntityContextO,
        es: *const EntityT,
        n: u32,
    ) {
        self.batch_destroy_entity.unwrap()(ctx, es, n)
    }

    pub unsafe fn clear_world(&self, ctx: *mut EntityContextO) {
        self.clear_world.unwrap()(ctx)
    }

    pub unsafe fn queue_destroy_entities(
        &self,
        ctx: *mut EntityContextO,
        es: *const EntityT,
        n: u32,
    ) {
        self.queue_destroy_entities.unwrap()(ctx, es, n)
    }

    pub unsafe fn is_alive(&self, ctx: *mut EntityContextO, e: EntityT) -> bool {
        self.is_alive.unwrap()(ctx, e)
    }

    pub unsafe fn num_entities(&self, ctx: *mut EntityContextO) -> u32 {
        self.num_entities.unwrap()(ctx)
    }

    pub unsafe fn entities_matching(
        &self,
        ctx: *mut EntityContextO,
        required: *const ComponentMaskT,
        ta: *mut TempAllocatorI,
    ) -> *mut EntitySetT {
        self.entities_matching.unwrap()(ctx, required, ta)
    }

    pub unsafe fn entities_matching_with_forbidden(
        &self,
        ctx: *mut EntityContextO,
        required: *const ComponentMaskT,
        forbidden: *const ComponentMaskT,
        ta: *mut TempAllocatorI,
    ) -> *mut EntitySetT {
        self.entities_matching_with_forbidden.unwrap()(ctx, required, forbidden, ta)
    }

    pub unsafe fn flatten_set(&self, entities: *mut EntityT, set: *const EntitySetT) {
        self.flatten_set.unwrap()(entities, set)
    }

    pub unsafe fn lookup_component_type(
        &self,
        ctx: *mut EntityContextO,
        name_hash: StrhashT,
    ) -> ComponentTypeT {
        self.lookup_component_type.unwrap()(ctx, name_hash)
    }

    pub unsafe fn component_manager(
        &self,
        ctx: *mut EntityContextO,
        component_type: ComponentTypeT,
    ) -> *mut ComponentManagerO {
        self.component_manager.unwrap()(ctx, component_type)
    }

    pub unsafe fn component_manager_by_hash(
        &self,
        ctx: *mut EntityContextO,
        name_hash: StrhashT,
    ) -> *mut ComponentManagerO {
        self.component_manager_by_hash.unwrap()(ctx, name_hash)
    }

    pub unsafe fn create_component_mask(
        &self,
        components: *const ComponentTypeT,
        n: u32,
    ) -> ComponentMaskT {
        self.create_component_mask.unwrap()(components, n)
    }

    pub unsafe fn component_mask(
        &self,
        ctx: *mut EntityContextO,
        e: EntityT,
    ) -> *const ComponentMaskT {
        self.component_mask.unwrap()(ctx, e)
    }

    pub unsafe fn add_component(
        &self,
        ctx: *mut EntityContextO,
        e: EntityT,
        component: ComponentTypeT,
    ) -> *mut ::std::os::raw::c_void {
        self.add_component.unwrap()(ctx, e, component)
    }

    pub unsafe fn get_component(
        &self,
        ctx: *mut EntityContextO,
        e: EntityT,
        component: ComponentTypeT,
    ) -> *mut ::std::os::raw::c_void {
        self.get_component.unwrap()(ctx, e, component)
    }

    pub unsafe fn get_component_by_hash(
        &self,
        ctx: *mut EntityContextO,
        e: EntityT,
        name_hash: StrhashT,
    ) -> *mut ::std::os::raw::c_void {
        self.get_component_by_hash.unwrap()(ctx, e, name_hash)
    }

    pub unsafe fn remove_component(
        &self,
        ctx: *mut EntityContextO,
        e: EntityT,
        component: ComponentTypeT,
    ) {
        self.remove_component.unwrap()(ctx, e, component)
    }

    pub unsafe fn call_remove_on_all_entities(
        &self,
        ctx: *mut EntityContextO,
        component: ComponentTypeT,
    ) {
        self.call_remove_on_all_entities.unwrap()(ctx, component)
    }

    pub unsafe fn asset_parent(&self, ctx: *mut EntityContextO, e: EntityT) -> EntityT {
        self.asset_parent.unwrap()(ctx, e)
    }

    pub unsafe fn children(
        &self,
        ctx: *mut EntityContextO,
        e: EntityT,
        ta: *mut TempAllocatorI,
    ) -> *mut EntityT {
        self.children.unwrap()(ctx, e, ta)
    }

    pub unsafe fn asset(&self, ctx: *mut EntityContextO, e: EntityT) -> TtIdT {
        self.asset.unwrap()(ctx, e)
    }

    pub unsafe fn find_entity_from_asset(&self, ctx: *mut EntityContextO, asset: TtIdT) -> EntityT {
        self.find_entity_from_asset.unwrap()(ctx, asset)
    }

    pub unsafe fn resolve_asset_reference(
        &self,
        ctx: *mut EntityContextO,
        e: EntityT,
        asset: TtIdT,
    ) -> EntityT {
        self.resolve_asset_reference.unwrap()(ctx, e, asset)
    }

    pub unsafe fn resolve_path(
        &self,
        ctx: *mut EntityContextO,
        e: EntityT,
        path: *const ::std::os::raw::c_char,
    ) -> EntityT {
        self.resolve_path.unwrap()(ctx, e, path)
    }

    pub unsafe fn propagate_asset_changes(&self, ctx: *mut EntityContextO) {
        self.propagate_asset_changes.unwrap()(ctx)
    }

    pub unsafe fn propagate_listen_to(&self, ctx: *mut EntityContextO, e: EntityT, object: TtIdT) {
        self.propagate_listen_to.unwrap()(ctx, e, object)
    }

    pub unsafe fn set_blackboard_double(&self, ctx: *mut EntityContextO, id: StrhashT, value: f64) {
        self.set_blackboard_double.unwrap()(ctx, id, value)
    }

    pub unsafe fn set_blackboard_ptr(
        &self,
        ctx: *mut EntityContextO,
        id: StrhashT,
        value: *mut ::std::os::raw::c_void,
    ) {
        self.set_blackboard_ptr.unwrap()(ctx, id, value)
    }

    pub unsafe fn has_blackboard(&self, ctx: *mut EntityContextO, id: StrhashT) -> bool {
        self.has_blackboard.unwrap()(ctx, id)
    }

    pub unsafe fn get_blackboard_double(
        &self,
        ctx: *mut EntityContextO,
        id: StrhashT,
        def: f64,
    ) -> f64 {
        self.get_blackboard_double.unwrap()(ctx, id, def)
    }

    pub unsafe fn get_blackboard_ptr(
        &self,
        ctx: *mut EntityContextO,
        id: StrhashT,
    ) -> *mut ::std::os::raw::c_void {
        self.get_blackboard_ptr.unwrap()(ctx, id)
    }

    pub unsafe fn run_engine(&self, ctx: *mut EntityContextO, engine: *const EngineI) {
        self.run_engine.unwrap()(ctx, engine)
    }

    pub unsafe fn run_engine_with_commands(
        &self,
        ctx: *mut EntityContextO,
        engine: *const EngineI,
        commands: *mut EntityCommandsO,
    ) {
        self.run_engine_with_commands.unwrap()(ctx, engine, commands)
    }

    pub unsafe fn update(&self, ctx: *mut EntityContextO) {
        self.update.unwrap()(ctx)
    }

    pub unsafe fn listen(
        &self,
        ctx: *mut EntityContextO,
        e: EntityT,
        c: ComponentTypeT,
        listener: *const EntityListenerI,
    ) {
        self.listen.unwrap()(ctx, e, c, listener)
    }

    pub unsafe fn unlisten(
        &self,
        ctx: *mut EntityContextO,
        e: EntityT,
        c: ComponentTypeT,
        listener: *const EntityListenerI,
    ) {
        self.unlisten.unwrap()(ctx, e, c, listener)
    }

    pub unsafe fn notify(
        &self,
        ctx: *mut EntityContextO,
        component_type: ComponentTypeT,
        entities: *const EntityT,
        num_entities: u32,
    ) {
        self.notify.unwrap()(ctx, component_type, entities, num_entities)
    }

    pub unsafe fn listen_event(
        &self,
        ctx: *mut EntityContextO,
        e: EntityT,
        event: StrhashT,
        listener: *const EntityEventListenerI,
    ) {
        self.listen_event.unwrap()(ctx, e, event, listener)
    }

    pub unsafe fn unlisten_event(
        &self,
        ctx: *mut EntityContextO,
        e: EntityT,
        event: StrhashT,
        listener: *const EntityEventListenerI,
    ) {
        self.unlisten_event.unwrap()(ctx, e, event, listener)
    }

    pub unsafe fn unlisten_all(
        &self,
        ctx: *mut EntityContextO,
        instance: *mut ::std::os::raw::c_void,
    ) {
        self.unlisten_all.unwrap()(ctx, instance)
    }

    pub unsafe fn notify_event(
        &self,
        ctx: *mut EntityContextO,
        event: StrhashT,
        e: *mut EntityT,
        entity_stride: u32,
        data: *mut ::std::os::raw::c_void,
        data_stride: u32,
        n: u32,
    ) {
        self.notify_event.unwrap()(ctx, event, e, entity_stride, data, data_stride, n)
    }

    pub unsafe fn hot_reload(&self) {
        self.hot_reload.unwrap()()
    }

    pub unsafe fn set_debug_draw(
        &self,
        ctx: *mut EntityContextO,
        component_type: ComponentTypeT,
        tag: StrhashT,
        debug_draw: bool,
    ) {
        self.set_debug_draw.unwrap()(ctx, component_type, tag, debug_draw)
    }

    pub unsafe fn get_all_debug_draws(
        &self,
        ctx: *mut EntityContextO,
        component_types: *mut *mut ComponentTypeT,
        tags: *mut *mut StrhashT,
        ta: *mut TempAllocatorI,
    ) {
        self.get_all_debug_draws.unwrap()(ctx, component_types, tags, ta)
    }

    pub unsafe fn has_debug_draw(
        &self,
        ctx: *mut EntityContextO,
        component_type: ComponentTypeT,
        tag: StrhashT,
    ) -> bool {
        self.has_debug_draw.unwrap()(ctx, component_type, tag)
    }

    pub unsafe fn clear_debug_draw(&self, ctx: *mut EntityContextO) {
        self.clear_debug_draw.unwrap()(ctx)
    }

    pub unsafe fn debug_draw(
        &self,
        ctx: *mut EntityContextO,
        pbuf: *mut PrimitiveDrawerBufferT,
        vbuf: *mut PrimitiveDrawerBufferT,
        allocator: *mut AllocatorI,
        camera: *const CameraT,
        viewport: RectT,
    ) {
        self.debug_draw.unwrap()(ctx, pbuf, vbuf, allocator, camera, viewport)
    }

    pub unsafe fn gamestate(&self, ctx: *mut EntityContextO) -> *mut GamestateO {
        self.gamestate.unwrap()(ctx)
    }

    pub unsafe fn override_component_persistence(
        &self,
        ctx: *mut EntityContextO,
        c: ComponentTypeT,
        persistence: *mut ComponentPersistenceI,
    ) {
        self.override_component_persistence.unwrap()(ctx, c, persistence)
    }

    pub unsafe fn override_component_network_replication(
        &self,
        ctx: *mut EntityContextO,
        c: ComponentTypeT,
        replication: *mut ComponentNetworkReplicationI,
    ) {
        self.override_component_network_replication.unwrap()(ctx, c, replication)
    }

    pub unsafe fn propagate_persistence_changes_to_gamestate(&self, ctx: *mut EntityContextO) {
        self.propagate_persistence_changes_to_gamestate.unwrap()(ctx)
    }

    pub unsafe fn propagate_network_replication_changes_to_gamestate(
        &self,
        ctx: *mut EntityContextO,
    ) {
        self.propagate_network_replication_changes_to_gamestate
            .unwrap()(ctx)
    }

    pub unsafe fn ensure_entity_is_persistent(&self, ctx: *mut EntityContextO, e: EntityT) {
        self.ensure_entity_is_persistent.unwrap()(ctx, e)
    }

    pub unsafe fn ensure_entity_is_replicated(&self, ctx: *mut EntityContextO, e: EntityT) {
        self.ensure_entity_is_replicated.unwrap()(ctx, e)
    }

    pub unsafe fn get_entity_persistent_id(
        &self,
        ctx: *mut EntityContextO,
        e: EntityT,
        output: *mut GamestateObjectIdT,
    ) -> bool {
        self.get_entity_persistent_id.unwrap()(ctx, e, output)
    }

    pub unsafe fn get_entity_network_id(
        &self,
        ctx: *mut EntityContextO,
        e: EntityT,
        output: *mut GamestateObjectIdT,
    ) -> bool {
        self.get_entity_network_id.unwrap()(ctx, e, output)
    }

    pub unsafe fn get_component_persistent_id(
        &self,
        ctx: *mut EntityContextO,
        e: EntityT,
        c: ComponentTypeT,
        output: *mut GamestateStructIdT,
    ) -> bool {
        self.get_component_persistent_id.unwrap()(ctx, e, c, output)
    }

    pub unsafe fn get_component_network_id(
        &self,
        ctx: *mut EntityContextO,
        e: EntityT,
        c: ComponentTypeT,
        output: *mut GamestateStructIdT,
    ) -> bool {
        self.get_component_network_id.unwrap()(ctx, e, c, output)
    }

    pub unsafe fn lookup_entity_from_gamestate_id(
        &self,
        ctx: *mut EntityContextO,
        id: *const GamestateObjectIdT,
    ) -> EntityT {
        self.lookup_entity_from_gamestate_id.unwrap()(ctx, id)
    }
}

impl crate::Api for EntityApi {
    const NAME: ConstCStr = const_cstr!("tm_entity_api");
    const VERSION: VersionT = VersionT {
        major: 2u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl EntityCommandsApi {
    pub unsafe fn create_entity_from_mask(
        &self,
        commands: *mut EntityCommandsO,
        mask: *const ComponentMaskT,
    ) -> EntityCommandEntityHandleT {
        self.create_entity_from_mask.unwrap()(commands, mask)
    }

    pub unsafe fn batch_create_entity_from_mask(
        &self,
        commands: *mut EntityCommandsO,
        mask: *const ComponentMaskT,
        n: u32,
        ta: *mut TempAllocatorI,
    ) -> *mut EntityCommandEntityHandleT {
        self.batch_create_entity_from_mask.unwrap()(commands, mask, n, ta)
    }

    pub unsafe fn create_entity_from_asset(
        &self,
        commands: *mut EntityCommandsO,
        asset: TtIdT,
    ) -> EntityCommandEntityHandleT {
        self.create_entity_from_asset.unwrap()(commands, asset)
    }

    pub unsafe fn batch_create_entity_from_asset(
        &self,
        commands: *mut EntityCommandsO,
        asset: *mut TtIdT,
        n: u32,
        ta: *mut TempAllocatorI,
    ) -> *mut EntityCommandEntityHandleT {
        self.batch_create_entity_from_asset.unwrap()(commands, asset, n, ta)
    }

    pub unsafe fn destroy_entity(&self, commands: *mut EntityCommandsO, e: EntityT) {
        self.destroy_entity.unwrap()(commands, e)
    }

    pub unsafe fn batch_destroy_entity(
        &self,
        commands: *mut EntityCommandsO,
        es: *const EntityT,
        n: u32,
    ) {
        self.batch_destroy_entity.unwrap()(commands, es, n)
    }

    pub unsafe fn clear_world(&self, commands: *mut EntityCommandsO) {
        self.clear_world.unwrap()(commands)
    }

    pub unsafe fn add_component(
        &self,
        commands: *mut EntityCommandsO,
        e: EntityT,
        component: ComponentTypeT,
    ) -> *mut ::std::os::raw::c_void {
        self.add_component.unwrap()(commands, e, component)
    }

    pub unsafe fn remove_component(
        &self,
        commands: *mut EntityCommandsO,
        e: EntityT,
        component: ComponentTypeT,
    ) {
        self.remove_component.unwrap()(commands, e, component)
    }

    pub unsafe fn add_component_by_handle(
        &self,
        commands: *mut EntityCommandsO,
        e: EntityCommandEntityHandleT,
        component: ComponentTypeT,
    ) -> *mut ::std::os::raw::c_void {
        self.add_component_by_handle.unwrap()(commands, e, component)
    }
}

impl crate::Api for EntityCommandsApi {
    const NAME: ConstCStr = const_cstr!("tm_entity_commands_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl OwnerComponentApi {
    pub unsafe fn children(
        &self,
        manager: *const OwnerComponentManagerO,
        c: *const OwnerComponentT,
        children: *mut EntityT,
    ) {
        self.children.unwrap()(manager, c, children)
    }

    pub unsafe fn add_children(
        &self,
        manager: *mut OwnerComponentManagerO,
        c: *mut OwnerComponentT,
        e: *const EntityT,
        n: u32,
    ) {
        self.add_children.unwrap()(manager, c, e, n)
    }

    pub unsafe fn remove_children(
        &self,
        manager: *mut OwnerComponentManagerO,
        c: *mut OwnerComponentT,
        e: *const EntityT,
        n: u32,
    ) {
        self.remove_children.unwrap()(manager, c, e, n)
    }

    pub unsafe fn remove_all_children(
        &self,
        manager: *mut OwnerComponentManagerO,
        c: *mut OwnerComponentT,
    ) {
        self.remove_all_children.unwrap()(manager, c)
    }

    pub unsafe fn descendants(
        &self,
        manager: *const OwnerComponentManagerO,
        c: *const OwnerComponentT,
        ta: *mut TempAllocatorI,
    ) -> *mut EntityT {
        self.descendants.unwrap()(manager, c, ta)
    }
}

impl crate::Api for OwnerComponentApi {
    const NAME: ConstCStr = const_cstr!("tm_owner_component_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl SceneTreeComponentApi {
    pub unsafe fn setup_nodes(
        &self,
        manager: *mut SceneTreeComponentManagerO,
        st: *mut SceneTreeComponentT,
        nodes: *const SceneTreeNodeT,
        num_nodes: u32,
        debug_names: *const ::std::os::raw::c_char,
    ) {
        self.setup_nodes.unwrap()(manager, st, nodes, num_nodes, debug_names)
    }

    pub unsafe fn node_names(&self, st: *const SceneTreeComponentT) -> SceneTreeNodeNamesT {
        self.node_names.unwrap()(st)
    }

    pub unsafe fn node_index_from_name(
        &self,
        component: *const SceneTreeComponentT,
        node_name: StrhashT,
        not_found: u32,
    ) -> u32 {
        self.node_index_from_name.unwrap()(component, node_name, not_found)
    }

    pub unsafe fn node_debug_name_from_index(
        &self,
        component: *const SceneTreeComponentT,
        node_idx: u32,
    ) -> *const ::std::os::raw::c_char {
        self.node_debug_name_from_index.unwrap()(component, node_idx)
    }

    pub unsafe fn world_matrices(&self, component: *const SceneTreeComponentT) -> *const Mat44T {
        self.world_matrices.unwrap()(component)
    }

    pub unsafe fn local_matrices(&self, component: *const SceneTreeComponentT) -> *const Mat44T {
        self.local_matrices.unwrap()(component)
    }

    pub unsafe fn local_transform(
        &self,
        component: *const SceneTreeComponentT,
        node_idx: u32,
    ) -> TransformT {
        self.local_transform.unwrap()(component, node_idx)
    }

    pub unsafe fn world_transform(
        &self,
        component: *const SceneTreeComponentT,
        node_idx: u32,
    ) -> TransformT {
        self.world_transform.unwrap()(component, node_idx)
    }

    pub unsafe fn set_local_transform(
        &self,
        component: *mut SceneTreeComponentT,
        node_idx: u32,
        t: *const TransformT,
    ) {
        self.set_local_transform.unwrap()(component, node_idx, t)
    }

    pub unsafe fn set_local_matrix(
        &self,
        component: *mut SceneTreeComponentT,
        node_idx: u32,
        tm: *const Mat44T,
    ) {
        self.set_local_matrix.unwrap()(component, node_idx, tm)
    }

    pub unsafe fn set_world_transform(
        &self,
        component: *mut SceneTreeComponentT,
        trans: *const TransformT,
        node_idx: u32,
        t: *const TransformT,
    ) {
        self.set_world_transform.unwrap()(component, trans, node_idx, t)
    }

    pub unsafe fn transform(
        &self,
        st: *mut SceneTreeComponentT,
        trans: *const TransformT,
        version: u64,
    ) {
        self.transform.unwrap()(st, trans, version)
    }

    pub unsafe fn transform_subtree(&self, st: *mut SceneTreeComponentT, node_idx: u32) {
        self.transform_subtree.unwrap()(st, node_idx)
    }

    pub unsafe fn set_debug_color(&self, st: *mut SceneTreeComponentT, color: ColorSrgbT) {
        self.set_debug_color.unwrap()(st, color)
    }

    pub unsafe fn node_parent_index(&self, st: *mut SceneTreeComponentT, node_idx: u32) -> u32 {
        self.node_parent_index.unwrap()(st, node_idx)
    }
}

impl crate::Api for SceneTreeComponentApi {
    const NAME: ConstCStr = const_cstr!("tm_scene_tree_component_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl TagComponentApi {
    pub unsafe fn tag_assets(&self, tt: *mut TheTruthO, ta: *mut TempAllocatorI) -> *mut TtIdT {
        self.tag_assets.unwrap()(tt, ta)
    }

    pub unsafe fn add_tag(&self, tcm: *mut TagComponentManagerO, e: EntityT, tag: StrhashT) {
        self.add_tag.unwrap()(tcm, e, tag)
    }

    pub unsafe fn add_tag_optional(
        &self,
        tcm: *mut TagComponentManagerO,
        e: EntityT,
        tag: StrhashT,
    ) {
        self.add_tag_optional.unwrap()(tcm, e, tag)
    }

    pub unsafe fn remove_tag(&self, tcm: *mut TagComponentManagerO, e: EntityT, tag: StrhashT) {
        self.remove_tag.unwrap()(tcm, e, tag)
    }

    pub unsafe fn has_tag(
        &self,
        tcm: *const TagComponentManagerO,
        e: EntityT,
        tag: StrhashT,
    ) -> bool {
        self.has_tag.unwrap()(tcm, e, tag)
    }

    pub unsafe fn find_all(
        &self,
        tcm: *const TagComponentManagerO,
        tag: StrhashT,
        ta: *mut TempAllocatorI,
    ) -> *mut EntityT {
        self.find_all.unwrap()(tcm, tag, ta)
    }

    pub unsafe fn find_first(&self, tcm: *const TagComponentManagerO, tag: StrhashT) -> EntityT {
        self.find_first.unwrap()(tcm, tag)
    }
}

impl crate::Api for TagComponentApi {
    const NAME: ConstCStr = const_cstr!("tm_tag_component_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

impl TransformComponentApi {
    pub unsafe fn get_position(&self, man: *mut TransformComponentManagerO, e: EntityT) -> Vec3T {
        self.get_position.unwrap()(man, e)
    }

    pub unsafe fn get_rotation(&self, man: *mut TransformComponentManagerO, e: EntityT) -> Vec4T {
        self.get_rotation.unwrap()(man, e)
    }

    pub unsafe fn get_scale(&self, man: *mut TransformComponentManagerO, e: EntityT) -> Vec3T {
        self.get_scale.unwrap()(man, e)
    }

    pub unsafe fn get_transform(
        &self,
        man: *mut TransformComponentManagerO,
        e: EntityT,
    ) -> *const TransformT {
        self.get_transform.unwrap()(man, e)
    }

    pub unsafe fn set_position(
        &self,
        man: *mut TransformComponentManagerO,
        e: EntityT,
        pos: Vec3T,
    ) {
        self.set_position.unwrap()(man, e, pos)
    }

    pub unsafe fn set_rotation(
        &self,
        man: *mut TransformComponentManagerO,
        e: EntityT,
        rot: Vec4T,
    ) {
        self.set_rotation.unwrap()(man, e, rot)
    }

    pub unsafe fn set_scale(&self, man: *mut TransformComponentManagerO, e: EntityT, scl: Vec3T) {
        self.set_scale.unwrap()(man, e, scl)
    }

    pub unsafe fn set_transform(
        &self,
        man: *mut TransformComponentManagerO,
        e: EntityT,
        transform: *const TransformT,
    ) {
        self.set_transform.unwrap()(man, e, transform)
    }

    pub unsafe fn get_local_position(
        &self,
        man: *mut TransformComponentManagerO,
        e: EntityT,
    ) -> Vec3T {
        self.get_local_position.unwrap()(man, e)
    }

    pub unsafe fn get_local_rotation(
        &self,
        man: *mut TransformComponentManagerO,
        e: EntityT,
    ) -> Vec4T {
        self.get_local_rotation.unwrap()(man, e)
    }

    pub unsafe fn get_local_scale(
        &self,
        man: *mut TransformComponentManagerO,
        e: EntityT,
    ) -> Vec3T {
        self.get_local_scale.unwrap()(man, e)
    }

    pub unsafe fn get_local_transform(
        &self,
        man: *mut TransformComponentManagerO,
        e: EntityT,
    ) -> *const TransformT {
        self.get_local_transform.unwrap()(man, e)
    }

    pub unsafe fn set_local_position(
        &self,
        man: *mut TransformComponentManagerO,
        e: EntityT,
        local_pos: Vec3T,
    ) {
        self.set_local_position.unwrap()(man, e, local_pos)
    }

    pub unsafe fn set_local_rotation(
        &self,
        man: *mut TransformComponentManagerO,
        e: EntityT,
        local_rot: Vec4T,
    ) {
        self.set_local_rotation.unwrap()(man, e, local_rot)
    }

    pub unsafe fn set_local_scale(
        &self,
        man: *mut TransformComponentManagerO,
        e: EntityT,
        local_scl: Vec3T,
    ) {
        self.set_local_scale.unwrap()(man, e, local_scl)
    }

    pub unsafe fn set_local_transform(
        &self,
        man: *mut TransformComponentManagerO,
        e: EntityT,
        local_transform: *const TransformT,
    ) {
        self.set_local_transform.unwrap()(man, e, local_transform)
    }

    pub unsafe fn update_world_transform(&self, man: *mut TransformComponentManagerO, e: EntityT) {
        self.update_world_transform.unwrap()(man, e)
    }

    pub unsafe fn update_local_transform(&self, man: *mut TransformComponentManagerO, e: EntityT) {
        self.update_local_transform.unwrap()(man, e)
    }

    pub unsafe fn compute_world_transform(
        &self,
        world: *mut TransformT,
        local: *const TransformT,
        parent_world: *const TransformT,
    ) -> *mut TransformT {
        self.compute_world_transform.unwrap()(world, local, parent_world)
    }

    pub unsafe fn compute_local_transform(
        &self,
        local: *mut TransformT,
        world: *const TransformT,
        parent_world: *const TransformT,
    ) -> *mut TransformT {
        self.compute_local_transform.unwrap()(local, world, parent_world)
    }

    pub unsafe fn compute_parent_transform(
        &self,
        parent_world: *mut TransformT,
        world: *const TransformT,
        local: *const TransformT,
    ) -> *mut TransformT {
        self.compute_parent_transform.unwrap()(parent_world, world, local)
    }

    pub unsafe fn set_parent(
        &self,
        man: *mut TransformComponentManagerO,
        e: EntityT,
        parent: EntityT,
    ) {
        self.set_parent.unwrap()(man, e, parent)
    }
}

impl crate::Api for TransformComponentApi {
    const NAME: ConstCStr = const_cstr!("tm_transform_component_api");
    const VERSION: VersionT = VersionT {
        major: 1u32,
        minor: 0u32,
        patch: 0u32,
    };
}

pub const TM_TT_TYPE_HASH__CAMERA_COMPONENT: StrhashT = StrhashT {
    u64_: 4702191454645448961u64,
};
pub const TM_TT_TYPE_HASH__CONSTRAINT_COMPONENT: StrhashT = StrhashT {
    u64_: 8328882474048285115u64,
};
pub const TM_TT_TYPE_HASH__AIM_CONSTRAINT: StrhashT = StrhashT {
    u64_: 1952626934336071325u64,
};
pub const TM_TT_TYPE_HASH__FABRIK_CONSTRAINT: StrhashT = StrhashT {
    u64_: 8908113391808412804u64,
};
pub const TM_TT_TYPE_HASH__VERLET_CONSTRAINT__COLLISION_SHAPE: StrhashT = StrhashT {
    u64_: 8004910458215113213u64,
};
pub const TM_TT_TYPE_HASH__VERLET_CONSTRAINT: StrhashT = StrhashT {
    u64_: 8425923701408414813u64,
};
pub const TM_ENGINE__CONSTRAINT: StrhashT = StrhashT {
    u64_: 11031446622748962226u64,
};
pub const TM_TT_TYPE_HASH__ENTITY_SORT_VALUE: StrhashT = StrhashT {
    u64_: 16910798103005116181u64,
};
pub const TM_TT_TYPE_HASH__ENTITY: StrhashT = StrhashT {
    u64_: 18380718000494627389u64,
};
pub const TM_PHASE__ANIMATION: StrhashT = StrhashT {
    u64_: 7021251402767592222u64,
};
pub const TM_PHASE__PHYSICS: StrhashT = StrhashT {
    u64_: 9931461870687654838u64,
};
pub const TM_PHASE__GRAPH: StrhashT = StrhashT {
    u64_: 7939849938661415817u64,
};
pub const TM_PHASE__CAMERA: StrhashT = StrhashT {
    u64_: 5035910868299140997u64,
};
pub const TM_PHASE__RENDER: StrhashT = StrhashT {
    u64_: 9520953946449223354u64,
};
pub const TM_ENTITY_BB__SIMULATION_SPEED: StrhashT = StrhashT {
    u64_: 410455368974121996u64,
};
pub const TM_ENTITY_BB__DELTA_TIME: StrhashT = StrhashT {
    u64_: 6798144013069773515u64,
};
pub const TM_ENTITY_BB__TIME: StrhashT = StrhashT {
    u64_: 7651809770448464541u64,
};
pub const TM_ENTITY_BB__WALL_DELTA_TIME: StrhashT = StrhashT {
    u64_: 7346627808032986698u64,
};
pub const TM_ENTITY_BB__WALL_TIME: StrhashT = StrhashT {
    u64_: 11026812457965896411u64,
};
pub const TM_ENTITY_BB__UI: StrhashT = StrhashT {
    u64_: 10083941705517751800u64,
};
pub const TM_ENTITY_BB__UI_STYLE: StrhashT = StrhashT {
    u64_: 7962502529529531972u64,
};
pub const TM_ENTITY_BB__UI_RECT: StrhashT = StrhashT {
    u64_: 9398225916257219576u64,
};
pub const TM_ENTITY_BB__UI_VIEWPORT_RECT: StrhashT = StrhashT {
    u64_: 17260522197470571648u64,
};
pub const TM_ENTITY_BB__WINDOW: StrhashT = StrhashT {
    u64_: 7502574202760729531u64,
};
pub const TM_ENTITY_BB__CAMERA: StrhashT = StrhashT {
    u64_: 173440160933014808u64,
};
pub const TM_ENTITY_BB__CAMERA_TRANSFORM: StrhashT = StrhashT {
    u64_: 12188817770794419482u64,
};
pub const TM_ENTITY_BB__DISABLED_INPUT: StrhashT = StrhashT {
    u64_: 17722481245418423046u64,
};
pub const TM_ENTITY_BB__EDITOR: StrhashT = StrhashT {
    u64_: 8066620458189861297u64,
};
pub const TM_ENTITY_BB__ASSET_ROOT: StrhashT = StrhashT {
    u64_: 12427581353952698963u64,
};
pub const ENTITY_GAMESTATE_NETWORK_REPLICATION_CONFIG: StrhashT = StrhashT {
    u64_: 14073158813658986614u64,
};
pub const ENTITY_GAMESTATE_PERSISTENCE_CONFIG: StrhashT = StrhashT {
    u64_: 436401338584922652u64,
};
pub const TM_TT_TYPE_HASH__OWNER_COMPONENT: StrhashT = StrhashT {
    u64_: 13997781925460329975u64,
};
pub const TM_TT_TYPE_HASH__SCENE_TREE_COMPONENT: StrhashT = StrhashT {
    u64_: 4467473609223121736u64,
};
pub const TM_ENGINE__SCENE_TREE: StrhashT = StrhashT {
    u64_: 8537231655615130692u64,
};
pub const TM_TT_TYPE_HASH__TAG_COMPONENT: StrhashT = StrhashT {
    u64_: 11268327857097111547u64,
};
pub const TM_TT_TYPE_HASH__TAG: StrhashT = StrhashT {
    u64_: 13878226445497189962u64,
};
pub const TM_TT_TYPE_HASH__TRANSFORM_COMPONENT: StrhashT = StrhashT {
    u64_: 10126216049058934656u64,
};
pub const TM_ENTITY_REGISTER_ENGINES_SIMULATION_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_TAG_COMPONENT_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_TRANSFORM_COMPONENT_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_ENTITY_API_VERSION: VersionT = VersionT {
    major: 2u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_SCENE_TREE_COMPONENT_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_ENTITY_CREATE_COMPONENT_I_VERSION: VersionT = VersionT {
    major: 2u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_ENTITY_REGISTER_ENGINES_EDITOR_I_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_OWNER_COMPONENT_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
pub const TM_ENTITY_COMMANDS_API_VERSION: VersionT = VersionT {
    major: 1u32,
    minor: 0u32,
    patch: 0u32,
};
