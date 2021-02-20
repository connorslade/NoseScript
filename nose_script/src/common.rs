pub fn color_text(text: &str, color_index: i32) -> String {
    let output = ["[", &color_index.to_string()[..], "m", text, "[0m"].join("");
    return output;
}

pub fn color_println(text: &str, color_index: i32) {
    println!("{}", color_text(text, color_index));
}