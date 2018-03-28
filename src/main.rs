// TODO:
// prevent index out of bounds for arguments/params

mod tools;

use tools::Cmd;
use tools::Config;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
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
