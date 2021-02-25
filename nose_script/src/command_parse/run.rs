use crate::common::{color_println, gen_project_name};

pub fn index(args: Vec<String>, args_len: usize) {
    let mut project_name = "".to_string();
    if args_len <= 2 {
        color_println("[*] No folder supplied...", 31);
    } else if args_len > 2 {
        project_name = gen_project_name(args, "run ");
    }

    color_println(&*format!("[*] Folder Name: `{}`", project_name), 36);
}
