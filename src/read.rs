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