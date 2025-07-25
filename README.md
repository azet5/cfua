# cfua

**cfua** (may be pronounced /siːθuːɑː/ [see-foo-ah]) stands for *configuration file with unpronounceable acronym* and is a data exchange format.

> [!NOTE]
> This is beta version of library. While it does work, it may have some minor bugs.
> If you find one, please create an issue.

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
