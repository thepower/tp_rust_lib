#![no_std]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]
#![feature(slice_concat_ext)]

pub use core::str::FromStr;
pub use core::fmt::{self, Debug, Display};
pub use core::ops::Index;

extern crate alloc;
pub use alloc::{format, vec};
//pub use alloc::slice::SliceConcatExt;
pub use alloc::borrow::{Cow, ToOwned};
pub use alloc::boxed::Box;
pub use alloc::collections::btree_map::BTreeMap;
pub use alloc::string::{String, ToString};
pub use alloc::vec::Vec;

extern crate fixed_hash;
pub use fixed_hash::construct_fixed_hash;

extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub extern crate corepack;
pub extern crate serde;
pub use serde::{Deserialize, Serialize};

pub extern crate crc;

extern crate power_env;

extern crate power_derive;
pub use power_derive::power_method;

pub mod types;
pub use crate::types::*;

pub mod value;
pub use crate::value::*;

pub mod env;
pub use crate::env::*;

pub mod io;
pub use crate::io::*;

pub mod ser;
pub use crate::ser::*;

pub mod de;
pub use crate::de::*;
