// Library dependencies
use rust_newsletter_backend::run;
use std::net::TcpListener;

/**
 * Main function using the Tokio async runtime
 */
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Get the TCP listener for the server
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to address");

    // Call the run function to start the server
    run(listener)?.await
}
