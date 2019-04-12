use std::process::Command;

fn main() {
    Command::new("arm-none-eabi-gcc")
        .args(&["./boot/vector_table.S",
            "-C",
            "-O0",
            "-mcpu=cortex-m4",
            "-mthumb",
            "-o ./boot/vector_table.o"]
        );
    Command::new("arm-none-eabi-gcc")
        .args(&["./boot/core.S",
            "-C",
            "-O0",
            "-mcpu=cortex-m4",
            "-mthumb",
            "-o ./boot/core.o"]
        );
}