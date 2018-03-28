pub mod rvm;

use self::rvm::RvmGenerator;

pub enum Cmd {
    Help,
    Rvm(RvmGenerator),
    None,
}

pub struct Config {
    kind: String,
    params: Vec<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let kind = args[1].clone();
        let params: Vec<String> = args[2..].to_vec();

        Config { kind, params }
    }

    pub fn resolve(&self) -> Cmd {
        match self.kind.as_ref() {
            "help" => Cmd::Help,
            "rvm" => Cmd::Rvm(RvmGenerator::new(&self.params)),
            _ => Cmd::None,
        }
    }
}
