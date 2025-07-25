# cfua

**cfua** (may be pronounced /siːθuːɑː/ [see-foo-ah]) stands for *configuration file with unpronounceable acronym* and is a data exchange format.

This implementation is written in Rust.

> [!NOTE]
> This is beta version of library. While it does work, it may have some minor bugs.
> If you find one, please create an issue.

## A few words about language

Cfua is a text data exchange format. Its syntax is pretty straightforward and looks like this:
```text
example-number: 42
example-text: 'Hello, world!
few-numbers: [1, -2, 4, -8, 16]
```

Currently supported data types are:
- integers
- floats
- strings
- booleans
- arrays of above

## Specification

The latest version of language is 0.1, which was released in 2025-07-25.

Language specification is available [here].

[here]: https://azet.dev/projects/cfua/0.1/

## Example code

To begin working with library, use:
```rs
// If you're creating new struct to be saved
let mut data = Cfua::create();

// If you're reading existing data, use:
let data = Cfua::from_fiile_name("example.cfua").unwrap();
```

You can find examples in `examples/` directory.
