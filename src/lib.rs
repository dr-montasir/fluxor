#![doc(html_logo_url = "https://github.com/dr-montasir/fluxor/raw/HEAD/fluxor-icon-64x64.svg")]
#![doc = r"<div align='center'><a href='https://github.com/dr-montasir/fluxor' target='_blank'><img src='https://github.com/dr-montasir/fluxor/raw/HEAD/fluxor-icon-64x64.svg' alt='Fluxor' width='80' height='auto' /></a><br><br><a href='https://github.com/dr-montasir/fluxor' target='_blank'>FLUXOR</a><br><br>Fluxor is a versatile Rust web framework designed for data science and computing science applications.</div>"]

//!
//! ## Features
//! 
//! - **Asynchronous**: Built on the Tokio runtime for powerful non-blocking I/O.
//! - **Robust**: Offers a rich set of functionalities tailored for scientific computing and web development.
//! - **Extendable**: Easily integrate with other data storage solutions such as MongoDB, PostgreSQL, and Redis.
//! - **CLI Tool**: A dedicated CLI (`fluxor_cli`) for project scaffolding to expedite the development process.
//! - **Modular**: Comprises various modules like math, data handling, logging, and more to facilitate streamlined development.
//! 
//! ## Getting Started
//! 
//! To begin using Fluxor, ensure that you have [Rust](https://www.rust-lang.org/tools/install) installed on your system. You can either create a new Fluxor project or add Fluxor to an existing project.
//! 
//! ### Option 1: Adding Fluxor to an Existing Project
//! 
//! If you're starting from scratch, you can add Fluxor directly to a new or existing Rust project using Cargo:
//! 
//! 1. Create a new Rust project:
//! 
//! ```terminal
//! 
//! cargo new <project_name>
//! cd <project_name>
//! 
//! ```
//! 
//! 2. Add Fluxor as a dependency:
//! 
//! ```terminal
//! 
//! cargo add fluxor
//! 
//! ```
//! 
//! ### Option 2: Creating a New Fluxor Project with the Fluxor CLI
//! 
//! If you prefer to use the Fluxor CLI to create a new project, you can do so with the following commands:
//! 
//! 1. First, install the Fluxor CLI:
//! 
//! ```terminal
//! 
//! cargo install fluxor_cli
//! 
//! ```
//! 
//! 2. Create a new Fluxor project:
//! 
//! ```terminal
//!       
//! fluxor new <project_name> --version latest --example helloworld
//!     
//! ```
//! 
//! Replace `<project_name>` with your desired project name.
//! 
//! Once the project scaffolding is complete, navigate into your project directory:
//! 
//! ```terminal
//!       
//! cd <project_name>
//!     
//! ```
//! 
//! Now, you can build and run your Fluxor application:
//! 
//! ```terminal
//!       
//! cargo run
//!     
//! ```
//! 
//! Your application should now be running on `http://127.0.0.1:8080`.
//! 
//! ## Examples
//! 
//! ### Hello World Example
//! 
//! A basic Fluxor application that responds with "Hello, World!" when accessed via a web browser.
//!
//! ```no_run
//!       
//! use fluxor::prelude::*;
//! 
//! fn hello(_req: Req, _params: Params) -> Reply {
//!     boxed(async {
//!         Ok(Response::builder()
//!             .header("Content-Type", "text/html; charset=UTF-8")
//!             .body(Body::from("<h1>👋 Hello, World!</h1>"))
//!             .unwrap())
//!     })
//! }
//!       
//! #[tokio::main]
//! async fn main() {
//!     let mut app = Fluxor::new();			// Initialize the application.
//!     app.route(GET, "/", hello);				// Set the route (method, path, handler).
//!     app.run("127.0.0.1", "8080").await;		// Start the HTTP server (host, port).
//! }
//!       
//! ```
//!
//! ### API Examples
//! 
//! A simple Fluxor API endpoint that returns a JSON response (method: GET).
//!
//! ```no_run
//!       
//! use fluxor::prelude::*;
//! 
//! fn hello(_req: Req, _params: Params) -> Reply {
//!     boxed(async move {
//!         let json_response = format!(r#"{{"message": "👋 Hello, World!"}}"#);
//!         
//!         Ok(Response::builder()
//!             .header("Content-Type", "application/json")
//!             .body(Body::from(json_response))
//!             .unwrap())
//!     })
//! }
//! 
//! #[tokio::main]
//! async fn main() {
//!     let mut app = Fluxor::new();							// Initialize the application.
//!     app.route(GET, "/", hello);								// Set the route (method, path, handler).
//!     app.route(GET, "/http-client", serve_http_client);  	// A simple http client to test your application.
//!     app.run("127.0.0.1", "8080").await;						// Start the HTTP server (host, port).
//! }
//!     
//! ```
//!       
//! A simple Fluxor API endpoint that returns a JSON response (method: POST).
//!
//! ```no_run
//!       
//! use fluxor::prelude::*;
//! 
//! fn hello(_req: Req, _params: Params) -> Reply {
//!     boxed(async move {
//!        let json_response = format!(r#"{{"message": "👋 Hello, World!"}}"#);
//!         
//!         Ok(Response::builder()
//!             .header("Content-Type", "application/json")
//!             .body(Body::from(json_response))
//!             .unwrap())
//!     })
//! }
//! 
//! #[tokio::main]
//! async fn main() {
//!     let mut server = Fluxor::new();                     	// Initialize the application.
//!     server.route(POST, "/", hello);                     	// Set the route (method, path, handler).
//!     server.route(GET, "/http-client", serve_http_client);  	// A simple HTTP client to test your application.
//!     server.run("127.0.0.1", "8080").await;              	// Start the HTTP server (host, port).
//! }
//! ```
//! 

// Private Core Module
mod core;

// Public Client Module
mod client;

// Public WTime Module
pub mod wtime;

// Public Fluxio Module
pub mod fluxio;

// Public Cans Module
pub mod cans;

// Public Cans Module
pub mod math;

// Public StyledLog Module
pub mod styledlog;

// Public Data Module
pub mod data;

// Prelude Module
#[allow(dead_code)]
pub mod prelude {
    // Use Core Module
    pub use crate::core::*;

    // Use Fluxio Module
    pub use crate::fluxio::{Body, Response, StatusCode, Method, method::*};

    // external modules
    // Use Math Module
    pub use crate::math;

    // Use tokio Crate
    pub use tokio;

    // Use cans Crate module content
    pub use crate::cans::content::*;
}

