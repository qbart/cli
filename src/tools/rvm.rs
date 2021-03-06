use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use colored::*;

pub struct RvmGenerator {
    version: String,
    gemset: String,
}

impl RvmGenerator {
    pub fn new(params: &[String]) -> RvmGenerator {
        if params.len() >= 1 {
            let rvm_params: Vec<&str> = params[0].split("@").collect();
            if rvm_params.len() >= 2 {
                return RvmGenerator {
                    version: String::from(rvm_params[0]),
                    gemset: String::from(rvm_params[1]),
                }
            }
        }

        RvmGenerator { version: "".to_string(), gemset: "".to_string() }
    }

    pub fn run(&self) {
        println!("RVM");
        if self.version == "" || self.gemset == "" {
            println!("{}", "Invalid rvm params".red());
        } else {
            self.process();
            println!("{}", "Ok".green());
        }

    }

    fn process(&self) {
        let ruby_version_path = Path::new("./.ruby-version");
        let ruby_gemset_path = Path::new("./.ruby-gemset");

        // generate .ruby-version
        println!("{} >> {}", self.version, ruby_version_path.display());
        let mut file_version = match File::create(&ruby_version_path) {
            Ok(f) => f,
            Err(e) => panic!("Failed to create {}: {}", ruby_version_path.display(), e.description()),
        };
        match file_version.write_all(self.version.as_bytes()) {
            Err(e) => panic!("Failed to write to file {}: {}", ruby_version_path.display(), e.description()),
            _ => (),
        };

        // generate .ruby-gemset
        println!("{} >> {}", self.gemset, ruby_gemset_path.display());
        let mut file_gemset = match File::create(&ruby_gemset_path) {
            Ok(f) => f,
            Err(e) => panic!("Failed to create {}: {}", ruby_gemset_path.display(), e.description()),
        };
        match file_gemset.write_all(self.gemset.as_bytes()) {
            Err(e) => panic!("Failed to write to file {}: {}", ruby_gemset_path.display(), e.description()),
            _ => (),
        };
    }
}
