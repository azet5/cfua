use crate::{cfua::CfuaType, Number};

type CfuaArrayTy = Vec<CfuaType>;

pub struct CfuaNumberArray {
    elements: CfuaArrayTy,
}

pub struct CfuaStringArray {
    elements: CfuaArrayTy,
}

pub struct CfuaBoolArray {
    elements: CfuaArrayTy,
}

impl CfuaNumberArray {
    pub fn push<V>(&mut self, value: i64) -> &mut Self {
        self.elements.push(CfuaType::Integer(value));
        self
    }

    pub fn finish(self) -> CfuaType {
        CfuaType::Array(self.elements)
    }
}

impl CfuaStringArray {
    pub fn push<V>(&mut self, value: V) -> &mut Self
    where V: ToString {
        self.elements.push(CfuaType::String(value.to_string()));
        self
    }

    pub fn finish(self) -> CfuaType {
        CfuaType::Array(self.elements)
    }
}

impl CfuaBoolArray {
    pub fn push<V>(&mut self, value: bool) -> &mut Self {
        self.elements.push(CfuaType::Boolean(value));
        self
    }

    pub fn finish(self) -> CfuaType {
        CfuaType::Array(self.elements)
    }
}

pub trait ToCfuaArray {
    fn finish(self) -> CfuaType;
}

macro_rules! impl_cfua_array {
    ($tt: ty) => {
        impl ToCfuaArray for $tt {
            fn finish(self) -> CfuaType {
                CfuaType::Array(self.elements)
            }
        }
    };
}

impl_cfua_array!(CfuaNumberArray);
impl_cfua_array!(CfuaStringArray);
impl_cfua_array!(CfuaBoolArray);