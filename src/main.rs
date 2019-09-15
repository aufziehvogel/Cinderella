use std::env;

use getopts::Options;
use cinderella::RepoPointer;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} REPO_URL [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("b", "branch", "set the branch to checkout", "BRANCH");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) },
    };

    let repository_url = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let repo = RepoPointer {
        repo_url: repository_url,
        branch: matches.opt_str("b"),
    };

    cinderella::run(&repo)
}
