use crate::common::color_println;
mod help;
mod init;
mod nose;

pub fn parse_command(args: Vec<String>){
    let args_len = args.len();
    if args_len <= 1 {
        no_sub_command();
    } else if args_len >= 2 {
        match &args[1].to_lowercase()[..] {
            "help" => help::index(),
            "init" => init::index(args, args_len),
            "nose" => nose::index(),
            _ => incorrect_command(args[1].to_lowercase())
        }
    }
}

fn no_sub_command(){
    color_println("[*] No subcommand supplied...", 31);
    color_println(" └── SubCommands", 33);
    color_println("     ├─── Help", 33);
    color_println("     ├─── Init", 33);
    color_println("     └─── Nose 🐶", 33);
}

fn incorrect_command(command: String){
    color_println(&*format!("[*] Unknown Command: `{}`", command), 31)
}
