//! tests/health_check.rs
//!
//! Integration tests for the `/health_check` endpoint.
//!
//! Uses `reqwest` to simulate HTTP requests against a live server while
//! tokio::test is used to run the test asynchronously.

// Library dependencies
use rust_newsletter_backend::run;

/**
 * Function to spawn our test server so we can simulate
 * the actual server running in a given port
 */
fn spawn_app() {
    // Get the server instance from the run function
    let server = run().expect("Failed to start server");

    // Run the server in the background
    let _ = tokio::spawn(server);
}

/**
 * Actual testing function
 */
#[tokio::test]
async fn test_health_check() {
    // Spawn the test server
    spawn_app();

    // Create a client to make requests
    let client = reqwest::Client::new();

    // Perform the health check request
    let response = client
        .get("http://127.0.0.1:8000/health")
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
