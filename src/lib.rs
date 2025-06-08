mod cfua;
pub use cfua::Cfua;
pub use cfua::Number;

mod array;
pub use array::CfuaBoolArray;
pub use array::CfuaNumberArray;
pub use array::CfuaStringArray;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
