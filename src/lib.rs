pub mod context;
pub mod gas_plan;
pub mod variable;

pub mod error;
use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub trait Interpreter {
    fn new(ctx: context::Context) -> Self;
    fn execute() -> impl std::future::Future<Output = Result<()>>;
}
