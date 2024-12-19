mod fb;
mod server;

#[tokio::main]
async fn main() {
    server::start().await;
}
