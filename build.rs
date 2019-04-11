extern crate cc;

fn main() {
    cc::Build::new().
        .file("./boot/vector_table.S")
        .flag("-c")
        .flag("-O0")
        .flag("-mcpu=cortex-m4")
        .flag("-mthumb")
        .compile("vector-table");
    cc::Build::new()
        .file("./boot/core.S")
        .flag("-c")
        .flag("-O0")
        .flag("-mcpu=cortex-m4")
        .flag("-mthumb")
        .compile("core");
}