pub mod rvm;

use self::rvm::RvmGenerator;

pub enum Cmd {
    Help,
    Rvm(RvmGenerator),
    None,
}
