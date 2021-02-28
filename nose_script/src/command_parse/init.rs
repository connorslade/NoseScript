use crate::common::{color_println, gen_project_name, mk_dir, mk_file};

pub fn index(args: Vec<String>, args_len: usize) {
    if args_len <= 2 {
        color_println("[*] No Project name defined.", 31)
    } else if args_len > 2 {
        let project_name = gen_project_name(args, "init ");
        init_project_dir(project_name);
    }
}

fn init_project_dir(dir: String) {
    color_println(&*format!("[*] Creating Directory: `{}`", dir), 36);
    check_is_err(mk_dir(&*dir), "Directory");

    color_println(&*"[*] Creating Subdirectories: `src` and `build`", 36);
    check_is_err(mk_dir(&*format!("{}/src", dir)), "Subdirectory `src`");
    check_is_err(mk_dir(&*format!("{}/build", dir)), "Subdirectory `build`");

    color_println(&*"[*] Creating File: `index.ns`", 36);
    check_is_err(
        mk_file(&*format!("{}/src/index.ns", dir), "cout(\"Hello World\");"),
        "File `index.ns`",
    );
}

fn check_is_err(res: std::io::Result<()>, process: &str) {
    if res.is_ok() {
        color_println(&*format!("[*] Created {} Successfully!\n", process), 32);
    } else {
        color_println(&*format!("[*] Error creating {}...\n", process), 31);
        std::process::exit(0);
    }
}
