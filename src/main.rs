use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    //rust_java::exposed_to_java(config.file_path);
    rust_java::exposed_to_java(config.file_path);
}

struct Config {
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 2 {
            panic!("A file_path was not given");
        }
        let file_path = args[1].clone();
        Config { file_path }
    }
}