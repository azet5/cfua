//! Module containing helpers for building arrays used in [`Cfua`] struct.
//! 
//! ## Example
//! 
//! A simple number set:
//! ```
//! let numbers: CfuaIntegerArray = CfuaIntegerArray::new()
//!     .push(1)
//!     .push(2)
//!     .push(4)
//!     .push(8)
//!     .push(16)
//!     .push(32)
//!     .push(64)
//!     .push(128)
//!     .push(256);
//! ```
//! 
//! [`Cfua`]: crate::Cfua

use crate::cfua::CfuaType;

type CfuaArrayTy = Vec<CfuaType>;

/// Internal trait required for type conversion.
/// Do not use or implement it directly.
pub trait ToCfuaArray {
    fn finish(self) -> CfuaType;
}

impl ToCfuaArray for Vec<CfuaType> {
    fn finish(self) -> CfuaType {
        CfuaType::Array(self)
    }
}

macro_rules! array_type {
    ($name: ident<$ty: ty> { $enumitem: expr }) => {
        #[doc = concat!("Helper type storing `", stringify!($ty), "` values.")]
        pub struct $name {
            elements: CfuaArrayTy,
        }

        impl $name {
            #[doc = concat!("Creates new instance of ", stringify!($name), ".")]
            pub fn new() -> Self {
                Self {
                    elements: Vec::new(),
                }
            }

            /// Pushes an element into the end of an array.
            pub fn push(mut self, value: $ty) -> Self {
                self.elements.push($enumitem(value));
                self
            }
        }

        impl ToCfuaArray for $name {
            fn finish(self) -> CfuaType {
                CfuaType::Array(self.elements)
            }
        }
    };
}

/// An array of integers.
array_type!(CfuaIntegerArray<i64> {CfuaType::Integer});
/// An array of floating-point numbers.
array_type!(CfuaFloatArray<f64> {CfuaType::Float});
/// An array of booleans.
array_type!(CfuaBooleanArray<bool> {CfuaType::Boolean});
/// An array of strings.
array_type!(CfuaStringArray<String> {CfuaType::String});
