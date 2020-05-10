use std::env;
use std::path::Path;
use std::process;

use rpassword;
use env_logger;
use getopts::Options;
use cinderella::ExecutionConfig;

fn print_usage(program: &str) {
    println!("Usage: {} (run | encrypt | decrypt)", program);
}

fn print_usage_command(program: &str, argline: &str, opts: Options) {
    let brief = format!("Usage: {} {}", program, argline);
    print!("{}", opts.usage(&brief));
}

fn main() {
    env_logger::init();

    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("{} v{}", NAME, VERSION);

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    match args.get(1) {
        Some(command) => {
            match command.as_ref() {
                "run" => run(args),
                "encrypt" => encrypt(args),
                "decrypt" => decrypt(args),
                "--help" | "-h" => print_usage(&program),
                _ => {
                    println!("Unknown command!");
                    print_usage(&program);
                },
            }
        },
        None => {
            println!("Please provide a command");
            print_usage(&program);
        },
    }
}

fn parse_password_arg(opts: &Options, args: Vec<String>)
    -> Result<Option<String>, String>
{
    match opts.parse(&args[2..]) {
        Ok(m) => Ok(m.opt_str("p")),
        Err(f) => Err(f.to_string()),
    }
}

fn encrypt(args: Vec<String>) {
    let mut opts = Options::new();
    opts.optopt("p", "password", "set the password for encryption/decryption", "PASSWORD");

    let pass_or_none = parse_password_arg(&opts, args)
        .unwrap_or_else(|msg| {
            println!("{}", msg);
            print_usage_command("cinderella", "encrypt [options]", opts);
            process::exit(1);
        });

    let pass = match pass_or_none {
        Some(pass) => pass,
        None => rpassword::read_password_from_tty(Some("Password: ")).unwrap(),
    };

    cinderella::encrypt(Path::new(".cinderella/secrets.toml"),
        Path::new(".cinderella/secrets"), &pass);
}

fn decrypt(args: Vec<String>) {
    let mut opts = Options::new();
    opts.optopt("p", "password", "set the password for encryption/decryption", "PASSWORD");

    let pass_or_none = parse_password_arg(&opts, args)
        .unwrap_or_else(|msg| {
            println!("{}", msg);
            print_usage_command("cinderella", "decrypt [options]", opts);
            process::exit(1);
        });

    let pass = match pass_or_none {
        Some(pass) => pass,
        None => rpassword::read_password_from_tty(Some("Password: ")).unwrap(),
    };

    cinderella::decrypt(Path::new(".cinderella/secrets"),
        Path::new(".cinderella/secrets.toml"), &pass);
}

fn run(args: Vec<String>) {
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("b", "branch", "set the branch to checkout", "BRANCH");
    opts.optopt("t", "tag", "set the tag to checkout", "TAG");
    opts.optopt("f", "file", "set a file to the cinderella CI configuration", "FILEPATH");

    let matches = match opts.parse(&args[2..]) {
        Ok(m) => { m },
        Err(f) => {
            println!("{}", f.to_string());
            print_usage_command(&program, "run [options] REPO", opts);
            process::exit(1);
        },
    };

    let repository_url = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program);
        return;
    };

    let repo = ExecutionConfig {
        repo_url: repository_url,
        branch: matches.opt_str("b"),
        tag: matches.opt_str("t"),
        cinderella_filepath: matches.opt_str("f"),
    };

    // TODO: Handle error from cinderella:run and display error message + usage
    cinderella::run(&repo)
}
