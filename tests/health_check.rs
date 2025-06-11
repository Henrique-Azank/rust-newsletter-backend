//! tests/health_check.rs
//!
//! Integration tests for the `/health_check` endpoint.
//!
//! Uses `reqwest` to simulate HTTP requests against a live server while
//! tokio::test is used to run the test asynchronously.

// Library dependencies
use rust_newsletter_backend::run;
use std::net::TcpListener;

/**
 * Function to spawn our test server so we can simulate
 * the actual server running in a given port
 */
fn spawn_app() -> String {
    // Instantiate a TCP listener on any available port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to any address");

    // Get the port number from the listener
    let port = listener.local_addr().unwrap().port();

    // Get the server instance from the run function
    let server = run(listener).expect("Failed to start server");

    // Run the server in the background
    let _ = tokio::spawn(server);

    // Return the address of the server
    format!("http://127.0.0.1:{}", port)
}

/**
 * Actual testing function
 */
#[tokio::test]
async fn test_health_check() {
    // Spawn the test server
    let server_address = spawn_app();

    // Create a client to make requests
    let client = reqwest::Client::new();

    // Perform the health check request
    let response = client
        .get(&format!("{}/health", server_address))
        .send()
        .await
        .expect("Failed to execute the health request");

    // Assert the OK value of the response
    assert!(response.status().is_success(), "Health check failed");

    // Assert that the response body is empty (as per the original code)
    assert_eq!(
        response.content_length(),
        Some(0),
        "Expected empty response body"
    );
}
