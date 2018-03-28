// TODO:
// prevent index out of bounds for arguments/params

mod tools;

use tools::Cmd;
use tools::rvm::RvmGenerator;

struct Config {
    kind: String,
    params: Vec<String>,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let kind = args[1].clone();
        let params: Vec<String> = args[2..].to_vec();

        Config { kind, params }
    }

    fn resolve(&self) -> Cmd {
        match self.kind.as_ref() {
            "help" => Cmd::Help,
            "rvm" => Cmd::Rvm(RvmGenerator::new(&self.params)),
            _ => Cmd::None,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args);
    let cmd = config.resolve();

    match cmd {
        Cmd::Help => {
            println!("[Usage]\n");
            println!("cli command [options]\n");
            println!("Command can be one of:");
            println!("- rvm");
            println!("  rvm 2.5.0@project // creates proper rvm files with version & gemset");
        },
        Cmd::Rvm(generator) => generator.run(),
        Cmd::None => println!("Invalid command!"),
    };
}
