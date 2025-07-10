#![deny(unsafe_code)]

use crate::array::ToCfuaArray;

// type CfuaKV = HashMap<String, CfuaType>;
type CfuaKV = Vec<(String, CfuaType)>;
// pub(crate) struct CfuaKV {
//     data: Vec<(String, CfuaType)>,
// }

/// Main library type representing cfua data. To begin working with cfua data,
/// see `Cfua::from_file()` for reading existing data, or `Cfua::create()`
/// for writing data.
#[derive(Debug, Clone, PartialEq)]
pub struct Cfua {
    data: CfuaKV,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CfuaType {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<CfuaType>),
    /// A section, as defined by `@` sign. Note that section's name
    /// is stored as a value's key.
    Section(()),
}

impl Cfua {
    /// Creates empty cfua structure, which may be later saved.
    pub fn create() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    /// Appends integer `value` with `key` into the end of structure.
    pub fn write_integer<K>(&mut self, key: K, value: i64)
    where K: ToString {
        self.data.push((key.to_string(), CfuaType::Integer(value)));
    }

    /// Appends float `value` with `key` into the end of structure.
    pub fn write_float<K>(&mut self, key: K, value: f64)
    where K: ToString {
        self.data.push((key.to_string(), CfuaType::Float(value)));
    }

    /// Appends string `value` with `key` into the end of structure.
    pub fn write_string<S>(&mut self, key: S, value: S)
    where S: ToString {
        self.data.push((key.to_string(), CfuaType::String(value.to_string())));
    }

    /// Appends logic `value` with `key` into the end of structure.
    pub fn write_bool<K>(&mut self, key: K, value: bool)
    where K: ToString {
        self.data.push((key.to_string(), CfuaType::Boolean(value)));
    }

    /// Appends section (`@key`) containing key-value
    /// pairs into the end of structure.
    pub fn write_section<K>(&mut self, key: K)
    where K: ToString {
        self.data.push((key.to_string(), CfuaType::Section(())));
    }

    /// Appends array into the end of structure. The `value` is constructed
    /// from `CfuaNumberArray`, `CfuaStringArray` or `CfuaBoolArray`.
    pub fn write_array<K, F>(&mut self, key: K, value: F)
    where K: ToString,
          F: ToCfuaArray {
        self.data.push((key.to_string(), value.finish()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_structure() {
        let from_builder = Cfua::create()
            .write_bool("is-cfua", true)
            .write_string("purpose", "Testing builder functions");

        let mut map: CfuaKV = Vec::with_capacity(2);
        map.push(("is-cfua".to_string(), CfuaType::Boolean(true)));
        map.push(("purpose".to_string(), CfuaType::String("Testing builder functions".to_string())));

        assert_eq!(from_builder, Cfua { data: map });
    }
}