/*
 * Main library file for the Rust project
 * This file contains the main functionality of the application,
 * whist the executable file is used to run the application.
*/

// Base dependencies
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, dev::Server, web};
use std::net::TcpListener;

/**
 * General async function to handle the greeting endpoints
 * - Empty name parameter will return "Hello, World!"
 * - Name parameter will return "Hello, {name}!"
 *
 * Responder trait - Anything that can be converted to an HTTP response
 */
async fn greet_response(req: HttpRequest) -> impl Responder {
    // Either get the name or default to "World"
    let name: &str = req.match_info().get("name").unwrap_or("World");

    // Use the format macro to create the response
    return format!("Hello, {}!", name);
}

/**
 * General health check endpoint handler
 */
async fn health_check() -> impl Responder {
    // Return a simple text response for health check
    HttpResponse::Ok().finish()
}

/**
 * HTTP server run function for the general application
 * binaries - Marked as public for the executable to use
 */
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // Start the HTTP server
    let server = HttpServer::new(|| {
        /*
         * Instantiate a new app with the pattern of
         * route + guard
         */
        App::new()
            // Define the root endpoint (GET request)
            .route("/", web::get().to(greet_response))
            // Register a health check endpoint
            .route("/health", web::get().to(health_check))
            // General greeting endpoint with optional name parameter
            .route("/{name}", web::get().to(greet_response))
    })
    // Bind to the specified address and port
    .listen(listener)?
    // Start the server and listen for incoming requests
    .run();

    // Return the server instance
    Ok(server)
}
