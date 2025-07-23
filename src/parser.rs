use std::{fmt::Display, io};

use crate::{cfua::CfuaType, Cfua};

#[derive(Debug, PartialEq, Eq)]
enum State {
    Reading,
    Key,
    Separator,
    Value,
    /// comma-based syntax
    ArraySimple,
    /// hash-based (`#`) syntax
    ArrayNormal(Option<bool>),
    SectionName,
    Comment,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ValueType {
    Number,
    Integer,
    Float,
    String,
    Bool,
    Other,
}

/// Structure storing data needed for parser.
pub struct ParserData {
    input: String,
    key_buffer: String,
    value_buffer: String,
    value_type: ValueType,
    array_buffer: Vec<CfuaType>,
    state: State,
    data: Cfua,
}

/// Errors which may occur while parsing data.
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
    NestedArray,
    MixedArrayType,
    MixedArrayDecl,
    StringInSimpleArray,
    InvalidArrayValue(String),
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
            CfuaError::NestedArray => write!(f, "nested arrays are not allowed"),
            CfuaError::MixedArrayType => write!(f, "array type is ambiguous"),
            CfuaError::MixedArrayDecl => write!(f, "mixed comma-based and hash-based array declatation"),
            CfuaError::StringInSimpleArray => write!(f, "string value in simple array declaration"),
            CfuaError::InvalidArrayValue(kw) => write!(f, "invalid array element: '{kw}'"),
            CfuaError::IoError(err) => write!(f, "io error: {err}"),
        }
    }
}

impl ParserData {
    /// Creates empty `ParserData` structure from `input` string.
    pub fn new(input: String) -> Self {
        Self {
            input,
            key_buffer: String::with_capacity(256),
            value_buffer: String::with_capacity(256),
            value_type: ValueType::Number,
            array_buffer: Vec::with_capacity(64),
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
                // TODO: refactor repeating code
                if self.value_buffer.starts_with('-') {
                    if self.value_buffer == "-inf" {
                        self.data.write_float(self.key_buffer.clone(), f64::NEG_INFINITY);
                    } else if let Some(c) = self.value_buffer.chars().nth(1) {
                        match c {
                            'b' |
                            'h' |
                            'o' => self.data.write_integer(self.key_buffer.clone(), i64::from_str_radix(
                                &self.value_buffer.replace(c, ""),
                                if c == 'b' { 2 } else if c == 'h' { 16 } else { 8 }).unwrap()
                            ),
                            '0'..'9' => self.data.write_integer(self.key_buffer.clone(), self.value_buffer.clone().parse().unwrap()),
                            x => return Err(CfuaError::UnknownKeyword(x.to_string()))
                        }
                    }
                } else {
                    if let Some(c) = self.value_buffer.chars().nth(0) {
                        match c {
                            'b' |
                            'h' |
                            'o' => self.data.write_integer(self.key_buffer.clone(), i64::from_str_radix(
                                &self.value_buffer.replace(c, ""),
                                if c == 'b' { 2 } else if c == 'h' { 16 } else { 8 }).unwrap()
                            ),
                            '0'..'9' => self.data.write_integer(self.key_buffer.clone(), self.value_buffer.clone().parse().unwrap()),
                            _ => return Err(CfuaError::UnknownKeyword(self.value_buffer.clone()))
                        }
                    } else {
                        self.data.write_integer(self.key_buffer.clone(), self.value_buffer.clone().parse().unwrap());
                    }
                }
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
            ':' => {
                self.state = State::Separator;
            },
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
        // try to guess value type (when buffer is empty)
        if self.value_buffer.len() == 0 {
            match char {
                '\'' => self.value_type = ValueType::String,
                '-' |
                'b' |
                'h' |
                'o' |
                '0'..'9' => self.value_type = ValueType::Number,
                '[' => {
                    self.value_type = ValueType::Other;
                    self.state = State::ArraySimple;
                    return Ok(());
                },
                '\n' => return Err(CfuaError::EmptyValue),
                _ => self.value_type = ValueType::Other,
            }
        // else, push consecutive chars until newline is approached
        } else {
            if char == '\n' {
                if self.value_type == ValueType::String {
                    self.state = State::Reading;
                    return Ok(());
                } else {
                    return self.push_value();
                }
            }
        }

        self.value_buffer.push(char);
        Ok(())
    }

    fn array_push_value(&mut self) -> Result<(), CfuaError> {
        match self.value_type {
            ValueType::Number => if self.value_buffer.contains('.') {
                self.array_buffer.push(CfuaType::Float(self.value_buffer.parse().unwrap()));
                self.value_type = ValueType::Float;
            } else {
                self.array_buffer.push(CfuaType::Integer(self.value_buffer.parse().unwrap()));
                self.value_type = ValueType::Integer;
            },
            ValueType::Integer => self.array_buffer.push(CfuaType::Integer(self.value_buffer.parse().unwrap())),
            ValueType::Float => self.array_buffer.push(CfuaType::Float(self.value_buffer.parse().unwrap())),
            ValueType::String => self.array_buffer.push(CfuaType::String(self.value_buffer.clone())),
            ValueType::Bool => if self.value_buffer == "true" {
                self.array_buffer.push(CfuaType::Bool(true));
            } else if self.value_buffer == "false" {
                self.array_buffer.push(CfuaType::Bool(false));
            } else {
                return Err(CfuaError::UnknownKeyword(self.value_buffer.clone()));
            },
            ValueType::Other => return Err(CfuaError::InvalidArrayValue(self.value_buffer.clone())),
        }
        
        self.value_buffer.clear();
        if self.state == State::ArrayNormal(Some(true)) {
            self.state = State::ArrayNormal(None);
        }
        Ok(())
    }

    fn array_char(&mut self, char: char) -> Result<(), CfuaError> {
        if self.value_buffer.len() == 0 {
            // `State::ArrayNormal(Some(false)` is set when there is
            // string element being pushed
            if self.state != State::ArrayNormal(Some(false)) {
                match char {
                    ' ' |
                    '\n' => return Ok(()),
                    '#' => if self.array_buffer.len() == 0 {
                        self.state = State::ArrayNormal(None);
                    } else {
                        if self.state == State::ArraySimple {
                            return Err(CfuaError::MixedArrayDecl);
                        }
                    },
                    '\'' => {
                        self.state = State::ArrayNormal(Some(false));
                        self.value_type = ValueType::String;
                    },
                    '-' |
                    'b' |
                    'h' |
                    'o' |
                    '0'..'9' => {
                        self.value_type = ValueType::Number;
                        self.value_buffer.push(char);
                    },
                    '[' => {
                        return Err(CfuaError::NestedArray);
                    },
                    _ => return Err(CfuaError::InvalidChar),
                }
            // create string value
            } else {
                if self.value_type == ValueType::String {
                    self.value_buffer.push(char);
                }
            }
        } else {
            match self.state {
                // in "simple" array syntax an array element is pushed
                // when comma `,` sign is reached or when an array
                // is ended with `]`
                State::ArraySimple => if char == ',' {
                    return self.array_push_value();
                } else if char == ']' {
                    let result = self.array_push_value();
                    self.data.write_array(self.key_buffer.clone(), self.array_buffer.clone());
                    self.key_buffer.clear();
                    self.array_buffer.clear();
                    self.state = State::Reading;
                    return result;
                } else {
                    self.value_buffer.push(char);
                },
                // while parsing array element
                State::ArrayNormal(Some(false)) => if char == '\n' {
                    self.state = State::ArrayNormal(Some(true));
                } else {
                    self.value_buffer.push(char);
                },
                // reached after newline while parsing array element
                State::ArrayNormal(Some(true)) => match char {
                    '\'' => if self.value_type == ValueType::String {
                        self.value_buffer.push('\n');
                        self.state = State::ArrayNormal(Some(false));
                    },
                    ']' => {
                        let result = self.array_push_value();
                        self.data.write_array(self.key_buffer.clone(), self.array_buffer.clone());
                        self.key_buffer.clear();
                        self.array_buffer.clear();
                        self.state = State::Reading;
                        return result;
                    },
                    '#' => return self.array_push_value(),
                    ' ' => {},
                    _ => return Err(CfuaError::InvalidChar),
                },
                _ => unreachable!(),
            }            
        }

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
                // reached after newline -- it means that value is not
                // continued and new key begins
                let result = if self.value_type == ValueType::String && self.value_buffer.len() > 1 {
                    self.push_value()
                } else { Ok(()) };

                self.key_buffer.push(char);
                self.state = State::Key;
                return result;
            },
            // if there is an `'` at newline, then there is multiline string
            '\'' => if self.value_buffer.len() > 0 {
                self.value_buffer.push('\n');
                self.state = State::Value;
            },
            '\n' => {},
            _ => return Err(CfuaError::InvalidChar),
        }

        Ok(())
    }

    // reads char
    fn read_char(&mut self, char: char) -> Result<(), CfuaError> {
        match self.state {
            // generic state -- in the beginning, after pushing value
            State::Reading => self.basic_char(char),
            // reading key
            State::Key => self.key_char(char),
            // after reaching colon `:`
            State::Separator => self.separator_char(char),
            // reading / parsing value
            State::Value => self.value_char(char),
            State::ArraySimple |
            State::ArrayNormal(_) => self.array_char(char),
            // reading section name (after `@`)
            State::SectionName => self.section_char(char),
            // reading comment (after `%`)
            State::Comment => self.comment_char(char),
        }
    }

    /// Parses string given while creating structure.
    pub fn parse(&mut self) -> Result<Cfua, CfuaError> {
        let input = self.input.clone();
        let mut chars = input.chars();

        while let Some(char) = chars.next() {
            self.read_char(char)?;
        }

        if self.value_buffer.len() > 0 {
            self.push_value()?;
        }

        Ok(self.data.clone())
    }
}