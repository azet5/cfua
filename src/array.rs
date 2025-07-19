use crate::cfua::CfuaType;

type CfuaArrayTy = Vec<CfuaType>;

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
        pub struct $name {
            elements: CfuaArrayTy,
        }

        impl $name {
            /// Creates new instance of `$name`.
            pub fn new() -> Self {
                Self {
                    elements: Vec::new(),
                }
            }

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

array_type!(CfuaIntegerArray<i64> {CfuaType::Integer});
array_type!(CfuaFloatArray<f64> {CfuaType::Float});
array_type!(CfuaBooleanArray<bool> {CfuaType::Boolean});
array_type!(CfuaStringArray<String> {CfuaType::String});
