use std::{fs::File, io::{self, Write}};

use cfua::Cfua;

fn main() -> io::Result<()> {
    let mut data = Cfua::create();

    data.write_integer("example-number", 42);
    data.write_string("greeting", "Hello, world!");

    let mut file = File::create_new("examples/output.cfua")?;
    write!(&mut file, "{}", data.to_string())?;
    file.flush()?;
    Ok(())
}