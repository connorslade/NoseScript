use std::fs;
use std::fs::File;
use std::io::Write;

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

pub fn gen_project_name(args: Vec<String>, split: &str) -> String {
    //TODO: Make this less Bad...
    let args_string: String = args.join(" ").to_string();
    let split = args_string.split(split);
    let mut working: String = "".to_string();
    for s in split {
        working = s.parse().unwrap();
    }
    let string_underscore = str::replace(&mut &*working, " ", "_");
    return string_underscore.to_ascii_lowercase();
}
