// Base dependencies
use actix_web::{App, HttpRequest, HttpServer, Responder, web};

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
 * Main function using the Tokio async runtime
 */
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Start the HTTP server
    HttpServer::new(|| {
        /*
         * Instantiate a new app with the pattern of
         * route + guard
         */
        App::new()
            // Define the root endpoint (GET request)
            .route("/", web::get().to(greet_response))
            // General greeting endpoint with optional name parameter
            .route("/{name}", web::get().to(greet_response))
    })
    // Bind to the specified address and port
    .bind("127.0.0.1:8000")?
    // Start the server and listen for incoming requests
    .run()
    .await
}
