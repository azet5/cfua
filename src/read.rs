use std::{fs, io, path::Path};

use crate::{parser::{CfuaError, ParserData}, Cfua};

impl Cfua {
    /// Reads cfua data from file with specified `path`.
    /// Will return error if there is a problem with reading file
    /// or if file content is not proper cfua data.
    pub fn from_file_path<P>(path: P) -> Result<Cfua, CfuaError>
    where P: AsRef<Path> {
        match fs::read_to_string(path) {
            Ok(content) => Cfua::from_string(content),
            // TODO: implement custom error type
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

    use super::*;

    #[test]
    fn from_string_test() {
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
}