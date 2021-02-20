use crate::common::{color_println, color_text};

pub fn index() {
    color_println("NoseScript Help", 33);
    println!();
    color_println("NAME", 34);
    color_println("         ns - Create Run and Build Nose Script Projects!", 34);
    println!();
    color_println("SYNOPSIS", 34);
    color_println("         ns [SUBCOMMAND] ...", 34);
    println!();
    color_println("SUBCOMMAND", 34);
    color_println("        Help", 34);
    color_println("        Init", 34);
    color_println("        Nose", 34);
}
