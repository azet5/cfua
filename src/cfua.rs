#![deny(unsafe_code)]

use std::collections::HashMap;

use crate::array::ToCfuaArray;

type CfuaKV = HashMap<String, CfuaType>;

/// Main library type representing cfua data. To begin working with cfua data,
/// see `Cfua::from_file()` for reading existing data, or `Cfua::create()`
/// for writing data.
#[derive(Clone)]
pub struct Cfua {
    data: CfuaKV,
}

/// Somewhat optimised wrapper type storing either of `i64` or `f64` number.
/// Generally should not be created directly; use
/// `Cfua::write_number()` instead.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Number {
    int: Option<i64>,
    float: Option<f64>,
}

impl Number {
    pub(self) fn from_int<I>(number: I) -> Self
    where I: Into<i64> {
        Self {
            int: Some(number.into()),
            float: None,
        }
    }

    pub(self) fn from_float<F>(number: F) -> Self
    where F: Into<f64> {
        Self {
            int: None,
            float: Some(number.into()),
        }
    }

    pub(self) fn read_as_int(&self) -> i64 {
        self.int.expect("attempted to read integer from float number")
    }

    pub(self) fn read_as_float(&self) -> f64 {
        self.float.expect("attempted to read float from integer number")
    }
}

macro_rules! into_number_int {
    ($tt: ty) => {
        impl Into<Number> for $tt {
            fn into(self) -> Number {
                Number::from_int(self)
            }
        }
    };
}

macro_rules! into_number_float {
    ($tt: ty) => {
        impl Into<Number> for $tt {
            fn into(self) -> Number {
                Number::from_float(self)
            }
        }
    };
}

into_number_int!(i8);
into_number_int!(i16);
into_number_int!(i32);
into_number_int!(i64);
into_number_int!(u8);
into_number_int!(u16);
into_number_int!(u32);
into_number_float!(f32);
into_number_float!(f64);

#[derive(Debug, Clone)]
pub(crate) enum CfuaType {
    Number(Number),
    String(String),
    Boolean(bool),
    Array(Vec<CfuaType>),
    /// A section, as defined by `@` sign. Note that section's name
    /// is stored as a value's key.
    Section(CfuaKV),
}

impl Cfua {
    /// Creates empty cfua structure, which may be later saved.
    pub fn create() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Appends number `value` with `key` into the end of structure.
    pub fn write_number<K, N>(&mut self, key: K, value: N) -> &mut Self
    where K: ToString,
          N: Into<Number> {
        self.data.insert(key.to_string(), CfuaType::Number(value.into()));
        self
    }

    /// Appends string `value` with `key` into the end of structure.
    pub fn write_string<S>(&mut self, key: S, value: S) -> &mut Self
    where S: ToString {
        self.data.insert(key.to_string(), CfuaType::String(value.to_string()));
        self
    }

    /// Appends logic `value` with `key` into the end of structure.
    pub fn write_bool<K>(&mut self, key: K, value: bool) -> &mut Self
    where K: ToString {
        self.data.insert(key.to_string(), CfuaType::Boolean(value));
        self
    }

    /// Appends section (`@key`) containing key-value
    /// pairs into the end of structure.
    pub fn write_section<K, F>(&mut self, key: K, content: F) -> &mut Self
    where K: ToString,
          F: Fn(&mut Cfua) -> Cfua {
        self.data.insert(key.to_string(), CfuaType::Section(content(&mut self.clone()).data));
        self
    }

    /// Appends array into the end of structure. The `value` is constructed
    /// from `CfuaNumberArray`, `CfuaStringArray` or `CfuaBoolArray`.
    pub fn write_array<K, F>(&mut self, key: K, value: F) -> &mut Self 
    where K: ToString,
          F: ToCfuaArray {
        self.data.insert(key.to_string(), value.finish());
        self
    }
}