use std::fs::File;
use std::io::{Read, Write};
use std::{fs, io};

pub fn color_text(text: &str, color_index: i32) -> String {
    let output = ["[", &color_index.to_string()[..], "m", text, "[0m"].join("");
    return output;
}

pub fn color_println(text: &str, color_index: i32) {
    println!("{}", color_text(text, color_index));
}

pub fn mk_dir(path: &str) -> std::io::Result<()> {
    fs::create_dir(path)?;
    Ok(())
}

pub fn mk_file(path: &str, text: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(text.as_ref())?;
    Ok(())
}

pub fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn gen_project_name(args: Vec<String>, split: &str) -> String {
    let args_string: String = args.join(" ").to_string();
    let args_split = args_string.split(split).collect::<Vec<&str>>();
    let working = args_split[args_split.len() - 1]
        .to_string()
        .replace(" ", "_");
    return working;
}
