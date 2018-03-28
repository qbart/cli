use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub trait Runnable {
    fn run(&self);
}

pub enum Cmd {
    Help,
    Rvm(RvmGenerator),
    None,
}

pub struct RvmGenerator {
    version: String,
    gemset: String,
}

impl RvmGenerator {
    pub fn new(params: &[String]) -> RvmGenerator {
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
