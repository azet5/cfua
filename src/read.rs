use std::{fs, io, path::Path};

use crate::Cfua;

impl Cfua {
    /// Reads cfua data from file with specified `path`.
    /// Will return error if there is a problem with reading file
    /// or if file content is not proper cfua data.
    pub fn from_file_path<P>(path: P) -> Result<Cfua, ()>
    where P: AsRef<Path> {
        match fs::read_to_string(path) {
            Ok(content) => Cfua::from_string(content),
            // TODO: implement custom error type
            Err(e) => Err(()),
        }
    }
    
    /// Reads cfua data from string. Will return error if string
    /// content is not proper cfua data.
    pub fn from_string<S>(string: S) -> Result<Cfua, ()>
    where S: ToString {
        eprintln!("{}", &string.to_string());
        // TODO: actually parse string
        Ok(Cfua::create())
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
        let structure = Cfua::create()
            .write_number("number-value", 1)
            .write_string("string-value", "Testing from_string() method")
            .write_number("another-number", -0.123);
        assert_eq!(Cfua::from_string(example).unwrap(), structure);
    }
}