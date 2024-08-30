use std::pin::Pin;

use tokio::io::AsyncRead;

use crate::gas::GasPlan;
use crate::io::Output;

#[async_trait::async_trait]
pub trait Amnis {
    fn new(gas_plan: GasPlan) -> Self
    where
        Self: Sized;

    async fn handle(&self, input: Pin<Box<dyn AsyncRead + Send>>) -> Output;
}
