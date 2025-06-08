use crate::Cfua;

enum Part {
    Key,
    Separator,
    ValueString,
    ValueNumber,
    ValueBool,
    ValueArray,
    SectionName,
    Comment,
}

pub(crate) struct ParserData {
    input: String,
    buffer: String,
    data: Cfua,
}

impl ParserData {
    pub fn new(input: String) -> Self {
        Self {
            input,
            buffer: String::with_capacity(256),
            data: Cfua::create(),
        }
    }

    pub fn parse(self) -> Result<Cfua, ()> {
        todo!("implement this")
    }
}