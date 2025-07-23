#![deny(unsafe_code)]

use crate::array::ToCfuaArray;

// type CfuaKV = HashMap<String, CfuaType>;
type CfuaKV = Vec<(String, CfuaType)>;
// pub(crate) struct CfuaKV {
//     data: Vec<(String, CfuaType)>,
// }

/// Main library type representing cfua data.
/// 
/// To begin working with cfua data, you can either read from string data
/// (see either [`from_file_path`] or [`from_string`]), or write data
/// with code, by creating an empty struct with [`create`].
/// 
/// ## Examples
/// 
/// To further read data, use appropriate reading functions.
/// 
/// ```
/// let data: Cfua = Cfua::from_file_path("example.cfua");
/// let example_string: String = data.read_string("example_string").unwrap();
/// let example_number: i64 = data.read_int("example_number").unwrap();
/// ```
/// 
/// To further write data, use appropriate writing functions.
/// After finishing, use [`to_string`] function to convert data into string,
/// which can be, for example, written to file.
/// 
/// ```
/// let mut data: Cfua = Cfua::create();
/// data.write_string("example_string", "Hello, world!");
/// data.write_int("example_number", 42);
/// let str_data: String = data.to_string();
/// // do something with string
/// ```
/// 
/// [`from_file_path`]: self::Cfua::from_file_path
/// [`from_string`]: self::Cfua::from_string
/// [`create`]: self::Cfua::create
/// [`to_string`]: self::Cfua::to_string
#[derive(Debug, Clone, PartialEq)]
pub struct Cfua {
    data: CfuaKV,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CfuaType {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
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

    /// Appends boolean `value` with `key` into the end of structure.
    pub fn write_bool<K>(&mut self, key: K, value: bool)
    where K: ToString {
        self.data.push((key.to_string(), CfuaType::Bool(value)));
    }

    /// Appends section (`@key`) into the end of structure.
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

    /// Searches for integer stored within `key`
    /// and returns its value if found.
    pub fn read_integer<K>(&self, key: K) -> Option<i64>
    where K: ToString {
        if let Some((_, CfuaType::Integer(i))) = self.data.iter().find(|p| p.0 == key.to_string()) {
            Some(*i)
        } else {
            None
        }
    }

    /// Searches for float stored within `key`
    /// and returns its value if found.
    pub fn read_float<K>(&self, key: K) -> Option<f64>
    where K: ToString {
        if let Some((_, CfuaType::Float(f))) = self.data.iter().find(|p| p.0 == key.to_string()) {
            Some(*f)
        } else {
            None
        }
    }

    /// Searches for string stored within `key`
    /// and returns its value if found.
    pub fn read_string<K>(&self, key: K) -> Option<String>
    where K: ToString {
        if let Some((_, CfuaType::String(s))) = self.data.iter().find(|p| p.0 == key.to_string()) {
            Some(s.clone())
        } else {
            None
        }
    }

    /// Searches for boolean stored within `key`
    /// and returns its value if found.
    pub fn read_bool<K>(&self, key: K) -> Option<bool>
    where K: ToString {
        if let Some((_, CfuaType::Bool(b))) = self.data.iter().find(|p| p.0 == key.to_string()) {
            Some(*b)
        } else {
            None
        }
    }

    /// Searches for array stored within `key`
    /// and returns its copy as `Vec` if found.
    pub fn read_array<K>(&self, key: K) -> Option<Vec<CfuaType>>
    where K: ToString {
        if let Some((_, CfuaType::Array(v))) = self.data.iter().find(|p| p.0 == key.to_string()) {
            Some(v.clone())
        } else {
            None
        }
    }

    /// Returns a copy of all data stored in key-value pairs.
    pub fn get_all(&self) -> Vec<(String, CfuaType)> {
        self.data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_structure() {
        let mut data = Cfua::create();
        data.write_bool("is-cfua", true);
        data.write_string("purpose", "Testing builder functions");

        let mut map: CfuaKV = Vec::with_capacity(2);
        map.push(("is-cfua".to_string(), CfuaType::Bool(true)));
        map.push(("purpose".to_string(), CfuaType::String("Testing builder functions".to_string())));

        assert_eq!(data, Cfua { data: map });
    }
}