use std::ops::Index;

use crate::{cfua::CfuaType, Cfua};

impl ToString for Cfua {
    /// Converts constructed data to string.
    fn to_string(&self) -> String {
        let mut output = String::new();

        for (key, value) in self.get_all() {
            if let CfuaType::Section(_) = value {
                output.push('@');
                output.push_str(key.as_str());
            } else {
                output.push_str(key.as_str());
                output.push_str(": ");
                match value {
                    CfuaType::Integer(value) => output.push_str(value.to_string().as_str()),
                    CfuaType::Float(value) => output.push_str(value.to_string().as_str()),
                    CfuaType::String(value) => {
                        let split: Vec<_> = value.split('\n').collect();
                        for i in 0..split.len() {
                            output.push('\'');
                            output.push_str(split.index(i));
                            if i + 1 != split.len() {
                                output.push('\n');
                            }
                        }
                    },
                    CfuaType::Boolean(value) => output.push_str(value.to_string().as_str()),
                    CfuaType::Array(value) => {
                        output.push('[');
                        for i in 0..value.len() {
                            match value.index(i) {
                                CfuaType::Integer(el) => output.push_str(el.to_string().as_str()),
                                CfuaType::Float(el) => output.push_str(el.to_string().as_str()),
                                CfuaType::String(el) => {
                                    output.push('#');
                                    let split: Vec<_> = el.split('\n').collect();
                                    for i in 0..split.len() {
                                        output.push('\'');
                                        output.push_str(split.index(i));
                                        if i + 1 != split.len() {
                                            output.push('\n');
                                        }
                                    }
                                },
                                CfuaType::Boolean(el) => output.push_str(el.to_string().as_str()),
                                _ => unreachable!(),
                            }
                            if i + 1 != value.len() {
                                if let CfuaType::String(_) = value.index(i) {
                                    output.push('\n');
                                } else {
                                    output.push_str(", ");
                                }
                            }
                        }
                        output.push(']');
                    },
                    CfuaType::Section(_) => unreachable!(),
                }
            }
            output.push_str("\n");
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use crate::Cfua;

    #[test]
    fn to_string_basic() {
        let mut structure = Cfua::create();
        structure.write_integer("number-value", 1);
        structure.write_string("string-value", "Testing to_string() method");
        structure.write_float("another-number", -0.123);

        let example =
r"number-value: 1
string-value: 'Testing to_string() method
another-number: -0.123
".to_string();
        assert_eq!(structure.to_string(), example)
    }
}