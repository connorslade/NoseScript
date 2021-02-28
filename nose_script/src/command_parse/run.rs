use crate::common::{color_println, gen_project_name, read_file};

pub fn index(args: Vec<String>, args_len: usize) {
    let mut project_name = "".to_string();
    if args_len <= 2 {
        color_println("[*] No folder supplied...", 31);
    } else if args_len > 2 {
        project_name = gen_project_name(args, "run ");
    }
    color_println(&*format!("[*] Folder Name: `{}`", project_name), 36);

    let file_data = read_file(&*format!("{}/src/index.ns", project_name));

    if file_data.is_err() {
        color_println(
            &*format!("File not found :/ `{}/src/index.ns`", project_name),
            31,
        );
        std::process::exit(0);
    }

    color_println(&*format!("{}", file_data.unwrap()), 36);
}
