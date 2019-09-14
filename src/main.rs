use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        cinderella::run(&args[1])
    } else {
        println!("Please specify the URL to a git repository");
    }
}
