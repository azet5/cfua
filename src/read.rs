use std::{fs, path::Path};

use crate::{parser::{CfuaError, ParserData}, Cfua};

impl Cfua {
    /// Reads cfua data from file with specified `path`.
    /// Will return error if there is a problem with reading file
    /// or if file content is not proper cfua data.
    pub fn from_file_path<P>(path: P) -> Result<Cfua, CfuaError>
    where P: AsRef<Path> {
        match fs::read_to_string(path) {
            Ok(content) => Cfua::from_string(content),
            Err(e) => Err(CfuaError::IoError(e)),
        }
    }
    
    /// Reads cfua data from string. Will return error if string
    /// content is not proper cfua data.
    pub fn from_string<S>(string: S) -> Result<Cfua, CfuaError>
    where S: ToString {
        eprintln!("{}", &string.to_string());
        ParserData::new(string.to_string()).parse()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{array::ToCfuaArray, CfuaIntegerArray, CfuaStringArray};

    use super::*;

    #[test]
    fn from_string_basic() {
        let example =
r"number-value: 1
string-value: 'Testing from_string() method
another-number: -0.123
";
        let mut structure = Cfua::create();
        structure.write_integer("number-value", 1);
        structure.write_string("string-value", "Testing from_string() method");
        structure.write_float("another-number", -0.123);
        
        assert_eq!(Cfua::from_string(example).unwrap(), structure);
    }

    #[test]
    fn from_string_advanced() {
        let example = 
r"fibonacci: [1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
multiline: 'First Line
'Second Line
'Third Line
strings-array: [
    #'String 1
    #'String 2.1
     'String 2.2
    #'String 3        
]
special-number: -inf
par1: head
par: 'head
x:-o7
apostrophes: '''''''
";
        let mut structure = Cfua::create();
        structure.write_array("fibonacci", CfuaIntegerArray::new()
            .push(1)
            .push(1)
            .push(2)
            .push(3)
            .push(5)
            .push(8)
            .push(13)
            .push(21)
            .push(34)
            .push(55)
        );
        structure.write_array("strings-array", CfuaStringArray::new()
            .push("String 1".to_string())
            .push("String 2.1\nString 2.2".to_string())
            .push("String 3".to_string())
        );
        structure.write_float("special-number", f64::NEG_INFINITY);
        structure.write_integer("par1", 0xead);
        structure.write_string("par", "head");
        structure.write_integer("x", -0o7);
        structure.write_string("apostrophes", "''''''");

        assert_eq!(Cfua::from_string(example).unwrap(), structure);
    }
}