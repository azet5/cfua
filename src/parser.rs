use std::str::Chars;

use crate::{cfua::CfuaType, Cfua};

enum State {
    Reading,
    Key,
    Separator,
    ValueString,
    ValueNumber,
    ValueBool,
    ValueArray,
    SectionName,
    Comment,
    EndOfData,
}

pub struct ParserData {
    input: String,
    key_buffer: String,
    value_buffer: CfuaType,
    state: State,
    data: Cfua,
}

impl ParserData {
    pub fn new(input: String) -> Self {
        Self {
            input,
            key_buffer: String::with_capacity(256),
            // its value doesn't matter as it will be replaced before using
            value_buffer: CfuaType::Integer(0),
            state: State::Reading,
            data: Cfua::create(),
        }
    }

    fn advance(&mut self, chars: &mut Chars<'_>) {
        
    }

    fn section_char(&mut self, char: char) {
        if char == '\n' {
            self.state = State::Reading;
            // self.data.write_section(self.key_buffer);
        } else {
            self.key_buffer.push(char);
        }
    }

    fn comment_char(&mut self, char: char) {
        if char == '\n' {
            self.state = State::Reading
        }
    }

    fn basic_char(&mut self, char: char) {
        match char {
            '%' => self.state = State::Comment,
            '@' => self.state = State::SectionName,
            _ => {},
        }
    }

    fn read_char(&mut self, char: char) {
        match self.state {
            State::Reading => self.basic_char(char),
            State::Key => todo!(),
            State::Separator => todo!(),
            State::ValueString => todo!(),
            State::ValueNumber => todo!(),
            State::ValueBool => todo!(),
            State::ValueArray => todo!(),
            State::SectionName => self.section_char(char),
            State::Comment => self.comment_char(char),
            State::EndOfData => todo!(),
        }
    }

    pub fn parse(&mut self) -> Result<Cfua, ()> {
        let input = self.input.clone();
        let mut chars = input.chars();

        while let Some(char) = chars.next() {
            self.read_char(char);
        }

        // loop {
        //     match self.state {
        //         State::Reading => self.advance(&mut chars),
        //         State::Key => todo!(),
        //         State::Separator => todo!(),
        //         State::ValueString => todo!(),
        //         State::ValueNumber => todo!(),
        //         State::ValueBool => todo!(),
        //         State::ValueArray => todo!(),
        //         State::SectionName => todo!(),
        //         State::Comment => todo!(),
        //         State::EndOfData => break,
        //     }
        // }

        Ok(self.data.clone())
    }
}