mod amnis;
mod channel;
mod error;
mod function;
mod gas_plan;
mod variable;

pub use amnis::Amnis;
pub use error::Error;
pub use variable::Variable;

pub type Result<T> = std::result::Result<T, Error>;
