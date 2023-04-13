use exitcode;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("No valid command supplied");
        std::process::exit(exitcode::USAGE);
    }

    if args[1].eq("--is-darwin") {
        if cfg!(target_os = "macos") {
            println!("This is Darwin");
            std::process::exit(exitcode::OK);
        } else {
            println!("This is NOT Darwin");
            std::process::exit(1);
        }
    }
    println!("No valid command supplied");
    std::process::exit(1);
}
