mod cfua;
pub use cfua::Cfua;
pub use cfua::CfuaType;

pub mod array;

mod read;
mod write;

mod parser;
pub use parser::CfuaError;