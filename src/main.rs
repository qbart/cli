use std::env;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;

// TODO:
// prevent index out of bounds for arguments/params


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

struct Config {
    kind: String,
    params: Vec<String>,
}

enum Cmd {
    Help,
    Rvm(RvmGenerator),
    None,
}

struct RvmGenerator {
    version: String,
    gemset: String,
}

impl RvmGenerator {
    fn new(params: &[String]) -> RvmGenerator {
        let rvm_params: Vec<&str> = params[0].split("@").collect();

        RvmGenerator {
            version: String::from(rvm_params[0]),
            gemset: String::from(rvm_params[1]),
        }
    }
}

impl Runnable for RvmGenerator {
    fn run(&self) {
        println!("RVM");
        let ruby_version_path = Path::new("./.ruby-version");
        let ruby_gemset_path = Path::new("./.ruby-gemset");

        println!("{} >> {}", self.version, ruby_version_path.display());
        let mut file_version = match File::create(&ruby_version_path) {
            Ok(f) => f,
            Err(e) => panic!("Failed to create {}: {}", ruby_version_path.display(), e.description()),
        };
        match file_version.write_all(self.version.as_bytes()) {
            Err(e) => panic!("Failed to write to file {}: {}", ruby_version_path.display(), e.description()),
            _ => (),
        };

        println!("{} >> {}", self.gemset, ruby_gemset_path.display());
        let mut file_gemset = match File::create(&ruby_gemset_path) {
            Ok(f) => f,
            Err(e) => panic!("Failed to create {}: {}", ruby_gemset_path.display(), e.description()),
        };
        match file_gemset.write_all(self.gemset.as_bytes()) {
            Err(e) => panic!("Failed to write to file {}: {}", ruby_gemset_path.display(), e.description()),
            _ => (),
        };

        println!("Ok");
    }
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

trait Runnable {
    fn run(&self);
}