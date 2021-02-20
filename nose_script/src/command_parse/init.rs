use crate::common::{color_println, mk_dir, mk_file};

pub fn index(args: Vec<String>, args_len: usize) {
    if args_len <= 2 {
        color_println("[*] No Project name defined.", 31)
    } else if args_len > 2 {
        let project_name = gen_project_name(args);
        init_project_dir(project_name);
    }
}

fn init_project_dir(dir: String) {
    color_println(&*format!("[*] Creating Directory: `{}`", dir), 36);
    let _result = mk_dir(&*dir);
    color_println(&*"[*] Created Directory Successfully!\n", 32);

    color_println(&*"[*] Creating Subdirectories: `src` and `build`", 36);
    let _result = mk_dir(&*format!("{}/src", dir));
    let _result = mk_dir(&*format!("{}/build", dir));
    color_println(&*"[*] Created Subdirectories Successfully!\n", 32);

    color_println(&*"[*] Creating `index.ns`", 36);
    let _result = mk_file(&*format!("{}/src/index.ns", dir), "cout(\"Hello World\");");
    color_println(&*"[*] Created File Successfully!\n", 32);
}

fn gen_project_name(args: Vec<String>) -> String {
    //TODO: Make this less Bad...
    let args_string: String = args.join(" ").to_string();
    let split = args_string.split("init ");
    let mut working: String = "".to_string();
    for s in split {
        working = s.parse().unwrap();
    }
    let string_underscore = str::replace(&mut &*working, " ", "_");
    return string_underscore.to_ascii_lowercase();
}
