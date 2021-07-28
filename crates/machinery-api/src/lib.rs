#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(clippy::all)]

pub mod foundation;
pub mod plugins;
pub mod the_machinery;

use const_cstr::ConstCStr;

pub trait Api {
    const NAME: ConstCStr;
}
