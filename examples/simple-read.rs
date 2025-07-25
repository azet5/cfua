use cfua::{Cfua, CfuaError};

fn main() {
    let data = Cfua::from_file_path("examples/example.cfua").unwrap();

    let number = data.read_integer("number-value").unwrap();
    let string = data.read_string("string-value").unwrap();

    eprintln!("the number is {} and the string says: '{}'", number, string);
}