// Library dependencies
use rust_newsletter_backend::run;

/**
 * Main function using the Tokio async runtime
 */
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Call the run function to start the server
    run()?.await
}
