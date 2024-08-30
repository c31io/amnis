mod amnis;
mod channel;
mod core;
mod error;
mod function;
mod gas;
mod io;
mod variable;

pub use amnis::Amnis;
pub use core::AmnisCore;

pub use gas::Gas;
pub use gas::GasPlan;

pub use io::{Utf8Input, Output};

pub use error::Error;
pub use variable::Variable;

pub type Result<T> = std::result::Result<T, Error>;
