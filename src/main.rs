use std::env;
use std::path::Path;

use rpassword;
use getopts::Options;
use cinderella::ExecutionConfig;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} REPO_URL [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(command) => {
            match command.as_ref() {
                "run" => run(args),
                "encrypt" => encrypt(),
                "decrypt" => decrypt(),
                _ => println!("Unknown command!"),
            }
        },
        None => println!("Please provide a command"),
    }
}

fn encrypt() {
    let pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();

    cinderella::encrypt(Path::new(".cinderella/secrets.toml"),
        Path::new(".cinderella/secrets"), &pass);
}

fn decrypt() {
    let pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();

    cinderella::decrypt(Path::new(".cinderella/secrets"),
        Path::new(".cinderella/secrets.toml"), &pass);
}

fn run(args: Vec<String>) {
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("b", "branch", "set the branch to checkout", "BRANCH");
    opts.optopt("f", "file", "set a file to the cinderella CI configuration", "FILEPATH");

    let matches = match opts.parse(&args[2..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) },
    };

    let repository_url = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let repo = ExecutionConfig {
        repo_url: repository_url,
        branch: matches.opt_str("b"),
        cinderella_filepath: matches.opt_str("f"),
    };

    cinderella::run(&repo)
}
