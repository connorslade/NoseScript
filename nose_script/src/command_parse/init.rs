use crate::common::color_println;

pub fn index(args: Vec<String>, args_len: usize) {
    if args_len <= 2 {
        color_println("[*] No Project name defined.", 31)
    } else if args_len > 2 {
        let project_name = gen_project_name(args);
        color_println(&*format!("[*] Creating Folder: `{}`", project_name), 36);
    }
}

fn gen_project_name(args: Vec<String>) -> String { //TODO: Make this less Bad...
    let args_string:String = args.join(" ").to_string();
    let mut split = args_string.split("init ");
    let mut working: String = "".to_string();

    for s in split {
        working = s.parse().unwrap();
    }

    return working.to_string();
}