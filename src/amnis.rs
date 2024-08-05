use crate::gas::GasPlan;
use crate::io::AmnisIO;
use crate::Result;

#[async_trait::async_trait]
pub trait Amnis {
    fn new(gas_plan: GasPlan) -> Self
    where
        Self: Sized;

    async fn handle(&mut self, input: Box<dyn AmnisIO>) -> Result<Box<dyn AmnisIO>>;
}
