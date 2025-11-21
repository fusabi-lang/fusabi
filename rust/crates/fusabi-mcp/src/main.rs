use anyhow::Result;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use fusabi_mcp::McpServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_writer(std::io::stderr)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // Create and run the server
    let server = McpServer::new();
    server.run().await
}