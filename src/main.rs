use amnis::{Amnis, AmnisCore, GasPlan, Utf8Input};
use tokio::io::{AsyncWriteExt, stdin, stdout};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let gas_plan = GasPlan::max();
    let amnis_core = AmnisCore::new(gas_plan);

    let input = Box::pin(Utf8Input::new(stdin()));
    let output = amnis_core.handle(input).await;
    let mut stream = output.to_utf8();
    let mut stdout = stdout();

    while let Some(frame) = stream.next().await {
        stdout.write_all(frame.as_bytes()).await.unwrap();
        stdout.flush().await.unwrap();
    }
}
