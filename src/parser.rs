use std::{fmt::Display, io};

use crate::Cfua;

enum State {
    Reading,
    Key,
    Separator,
    Value,
    SectionName,
    Comment,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ValueType {
    Number,
    String,
    Other,
    Array(Box<ValueType>),
}

pub struct ParserData {
    input: String,
    key_buffer: String,
    value_buffer: String,
    value_type: ValueType,
    state: State,
    data: Cfua,
}

#[derive(Debug)]
pub enum CfuaError {
    EmptyValue,
    NonGraphicChar,
    InvalidChar,
    InvalidKeyChar(char),
    InvalidHyphenInKey,
    InvalidSectionChar(char),
    InvalidHyphenInSection,
    UnknownKeyword(String),
    IoError(io::Error),
}

impl Display for CfuaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CfuaError::EmptyValue => write!(f, "value must not be empty"),
            CfuaError::NonGraphicChar => write!(f, "non-garphic char"),
            CfuaError::InvalidChar => write!(f, "invalid syntax"),
            CfuaError::InvalidKeyChar(ch) => write!(f, "invalid char: '{ch}' in key name"),
            CfuaError::InvalidHyphenInKey => write!(f, "hyphen not allowed at the beginning of key name"),
            CfuaError::InvalidSectionChar(ch) => write!(f, "invalid char: '{ch}' in section name"),
            CfuaError::InvalidHyphenInSection => write!(f, "hyphen not allowed at the beginning of section name"),
            CfuaError::UnknownKeyword(kw) => write!(f, "unknown keyword: '{kw}'"),
            CfuaError::IoError(err) => write!(f, "io error: {err}"),
        }
    }
}

impl ParserData {
    pub fn new(input: String) -> Self {
        Self {
            input,
            key_buffer: String::with_capacity(256),
            value_buffer: String::with_capacity(256),
            value_type: ValueType::Number,
            state: State::Reading,
            data: Cfua::create(),
        }
    }

    fn push_value(&mut self) -> Result<(), CfuaError> {
        if self.value_type == ValueType::String {
            self.data.write_string(self.key_buffer.clone(), self.value_buffer.strip_prefix('\'').unwrap().to_string());
        } else if self.value_type == ValueType::Number {
            if self.value_buffer.contains(".") {
                self.data.write_float(self.key_buffer.clone(), self.value_buffer.clone().parse().unwrap());
            } else {
                self.data.write_integer(self.key_buffer.clone(), self.value_buffer.clone().parse().unwrap());
            }
        } else {
            if self.value_buffer == "true" {
                self.data.write_bool(self.key_buffer.clone(), true);
            } else if self.value_buffer == "false" {
                self.data.write_bool(self.key_buffer.clone(), false);
            } else if self.value_buffer == "nan" {
                self.data.write_float(self.key_buffer.clone(), f64::NAN);
            } else if self.value_buffer == "inf" {
                self.data.write_float(self.key_buffer.clone(), f64::INFINITY);
            } else if self.value_buffer == "-inf" {
                self.data.write_float(self.key_buffer.clone(), f64::NEG_INFINITY);
            } else {
                return Err(CfuaError::UnknownKeyword(self.value_buffer.clone()));
            }
        }

        self.key_buffer.clear();
        self.value_buffer.clear();
        self.state = State::Reading;
        Ok(())
    }

    fn section_char(&mut self, char: char) -> Result<(), CfuaError> {
        match char {
            '\n' => {
                self.state = State::Reading;
                self.data.write_section(self.key_buffer.clone());
            },
            'a'..'z' => self.key_buffer.push(char),
            '-' => {
                if self.key_buffer.len() > 1 {
                    self.key_buffer.push(char);
                } else {
                    return Err(CfuaError::InvalidHyphenInSection);
                }
            },
            _ => return Err(CfuaError::InvalidSectionChar(char)),
        }

        Ok(())
    }

    fn key_char(&mut self, char: char) -> Result<(), CfuaError> {
        match char {
            ':' => self.state = State::Separator,
            'a'..'z' => self.key_buffer.push(char),
            '-' => if self.key_buffer.len() > 1 {
                self.key_buffer.push(char);
            } else {
                return Err(CfuaError::InvalidHyphenInKey);
            },
            ' ' => if !self.key_buffer.is_empty() {
                return Err(CfuaError::InvalidKeyChar(char));
            },
            _ => return Err(CfuaError::InvalidKeyChar(char)),
        }

        Ok(())
    }
    
    fn separator_char(&mut self, char: char) -> Result<(), CfuaError> {
        if char == ' ' {
            Ok(())
        } else if char.is_ascii_graphic() {
            self.state = State::Value;
            return self.value_char(char);
        } else {
            Err(CfuaError::NonGraphicChar)
        }
    }

    fn value_char(&mut self, char: char) -> Result<(), CfuaError> {
        if self.value_buffer.len() == 0 {
            match char {
                '\'' => self.value_type = ValueType::String,
                '-' |
                'b' |
                'h' |
                'o' |
                '0'..'9' => self.value_type = ValueType::Number,
                '\n' => return Err(CfuaError::EmptyValue),
                _ => self.value_type = ValueType::Other,
            }
        } else {
            if char == '\n' {
                return self.push_value();
            }
        }

        self.value_buffer.push(char);
        Ok(())
    }

    fn comment_char(&mut self, char: char) -> Result<(), CfuaError> {
        if char == '\n' {
            self.state = State::Reading;
        }

        Ok(())
    }

    fn basic_char(&mut self, char: char) -> Result<(), CfuaError> {
        match char {
            '%' => self.state = State::Comment,
            '@' => self.state = State::SectionName,
            'a'..'z' => {
                self.key_buffer.push(char);
                self.state = State::Key;
            },
            '\n' => {},
            _ => return Err(CfuaError::InvalidChar),
        }

        Ok(())
    }

    fn read_char(&mut self, char: char) -> Result<(), CfuaError> {
        match self.state {
            State::Reading => self.basic_char(char),
            State::Key => self.key_char(char),
            State::Separator => self.separator_char(char),
            State::Value => self.value_char(char),
            State::SectionName => self.section_char(char),
            State::Comment => self.comment_char(char),
        }
    }

    pub fn parse(&mut self) -> Result<Cfua, CfuaError> {
        let input = self.input.clone();
        let mut chars = input.chars();

        while let Some(char) = chars.next() {
            self.read_char(char)?;
        }

        Ok(self.data.clone())
    }
}