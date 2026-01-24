use mlua::StdLib;

fn main() {
    // We'll try to guess and see if they compile
    // Let's try common bitflags names
    let _ = StdLib::Base;
    let _ = StdLib::Table;
    let _ = StdLib::String;
    let _ = StdLib::Math;
    let _ = StdLib::Utf8;
}
