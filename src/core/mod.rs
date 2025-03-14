use dotenv::dotenv;
use fluxio::service::{make_service_fn, service_fn};
use fluxio::{Body, Request, Response, Server, StatusCode};
use std::collections::HashMap;
use std::convert::Infallible;
use std::env;
use std::future::Future;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use tokio::fs as async_fs;
use wtime::{
    calc::get_day_name,
    local::{format_local_ts, local_ts_sec},
};

use styledlog::Colorize;
use cans::mime::{set_mime_types, insert_mime_type, remove_mime_type};

pub type Req = fluxio::Request<fluxio::Body>;
pub type Reply = Pin<Box<dyn Future<Output = Result<fluxio::Response<fluxio::Body>, std::convert::Infallible>> + Send>>;

/// Parameters for the Fluxor application including the directory and any extra parameters.
#[derive(Clone)]
pub struct Params {
    pub dir: Arc<PathBuf>,              // Directory path (required for application)
    pub extra: HashMap<String, String>, // Required and optional parameters
}

/// Struct representing a route in the web server.
#[derive(Clone)]
pub struct Route {
    pub method: fluxio::Method,                         // HTTP method (GET, POST, etc.)
    pub path: String,                                   // Path for the route
    pub handler: fn(Request<Body>, Params) -> Reply,    // Handler function associated with the route
}

impl Route {
    /// Matches the incoming request path with the route path and captures parameters.
    /// 
    /// # Arguments
    /// 
    /// * `req`: The incoming request.
    /// 
    /// # Returns
    /// 
    /// Option containing a HashMap of captured parameters if the route matches, else None.
    pub fn is_match(&self, req: &Req) -> Option<HashMap<String, String>> {
        let path_segments: Vec<&str> = self.path.split('/').collect();
        let request_segments: Vec<&str> = req.uri().path().split('/').collect();

        if path_segments.len() != request_segments.len() {
            return None; // Path not a match
        }

        let mut params = HashMap::new();

        for (route_segment, request_segment) in path_segments.iter().zip(request_segments.iter()) {
            if route_segment.starts_with('<') && route_segment.ends_with('>') {
                let key = route_segment
                    .trim_matches('<')
                    .trim_matches('>')
                    .to_string();
                params.insert(key, request_segment.to_string());
            } else if route_segment != request_segment {
                return None; // Path segments do not match
            }
        }

        Some(params)
    }
}

/// Core struct for the Fluxor web server managing routes, parameters, and MIME types.
pub struct Fluxor {
    pub params: Params,                         // Parameters for the server
    pub routes: Vec<Route>,                     // List of routes
    pub mime_types: HashMap<String, String>,    // Store MIME types
}

impl Fluxor {
    /// Creates a new instance of Fluxor, initializing parameters and MIME types.
    /// 
    /// # Returns
    /// 
    /// A new Fluxor instance.
    pub fn new() -> Self {
        let mime_types = set_mime_types(); // Initialize MIME types at the start
        Self {
            params: Params {
                dir: Arc::new(PathBuf::new()),
                extra: HashMap::new(),
            },
            routes: Vec::new(),
            mime_types, // Set the initialized MIME types
        }
    }

    /// Sets the directory for static file serving.
    /// 
    /// # Arguments
    /// 
    /// * `dir`: A string representing the directory path.
    pub fn set_dir(&mut self, dir: String) {
        self.params.dir = Arc::new(PathBuf::from(dir));
    }

    /// Adds a new route to the Fluxor server.
    /// 
    /// # Arguments
    /// 
    /// * `path`: The path for the route.
    /// * `method`: The HTTP method for the route (GET, POST, etc.).
    /// * `handler`: A function that handles requests to that route.
    pub fn route(
        &mut self,
        path: &str,
        method: fluxio::Method,
        handler: fn(Request<Body>, Params) -> Reply,
    ) {
        self.routes.push(Route {
            method,
            path: path.to_string(),
            handler,
        });
    }

    /// Includes new MIME types in the server configuration.
    /// 
    /// # Arguments
    /// 
    /// * `new_types`: A HashMap of file extensions and their corresponding MIME types.
    pub fn include_mime_types(&mut self, new_types: HashMap<String, String>) {
        for (ext, mime) in new_types {
            insert_mime_type(&mut self.mime_types, &ext, &mime);
        }
    }

    /// Excludes specified MIME types from the server configuration.
    /// 
    /// # Arguments
    /// 
    /// * `extensions`: A vector of file extensions to be removed.
    pub fn exclude_mime_types(&mut self, extensions: Vec<&str>) {
        for ext in extensions {
            remove_mime_type(&mut self.mime_types, ext);
        }
    }

    /// Starts the HTTP server on the provided host and port, processing incoming requests.
    /// 
    /// # Arguments
    /// 
    /// * `host`: The host IP address (e.g., "127.0.0.1").
    /// * `port`: The port number (e.g., "8080").
    pub async fn run(&self, host: &str, port: &str) {
        let make_svc = make_service_fn(|_conn| {
            let params = self.params.clone();
            let routes = self.routes.clone();
            let mime_types = self.mime_types.clone(); // Pass MIME types to the service
            async {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let params = params.clone();
                    let routes = routes.clone();
                    handle_request(req, params, routes, mime_types.clone()) // Pass the MIME types
                }))
            }
        });

        let addr = format!("{}:{}", host, port);
        let addr: SocketAddr = addr.parse().expect("Invalid address/port combination");

        Server::bind(&addr).serve(make_svc);

        let server = Server::bind(&addr).serve(make_svc);

        // Get the current local formated timestamp
        let timestamp = format_local_ts();
        // Get the name of the day
        let day_name = get_day_name(local_ts_sec());
        let formated_addr = format!("{}", addr);
        let address = format!(
            "{}{}",
            "http://".blue().bold(),
            formated_addr.blue().italic().bold()
        );

        let project_name = env_var("PROJECT_NAME", "Fluxor");

        let startup_message = format!(
            "{} {}\n{}\n🌐 Server running {}: {} 📬\n{}\n🕔 {} {}\n{}\nPress Ctrl-C to shut down the server.",
            project_name.bright_green().bold(), "has started.".bright_green().bold(), "[INFO]".cyan(), "on".magenta(), address.underline(), "[TIME]".cyan(), day_name, timestamp.blue(), "[SHUTDOWN]".cyan(),
        );

        let server_view = env_var("SERVER_VIEW", "show");

        server_log(&server_view, &startup_message); // Log the server status

        if let Err(e) = server.await {
            println!(
                "\n[ERROR]\n❌ Server error: '{:?}'\n[TIME]\n🕔 {} {}",
                e, day_name, timestamp
            );
        }
    }
}

/// Serves static files with error handling.
/// 
/// # Arguments
/// 
/// * `req`: The incoming request.
/// * `params`: Parameters containing directory information.
/// * `mime_types`: A HashMap of MIME types to use for the response.
/// 
/// # Returns
/// 
/// A Result containing a Response with the file content or a 404 error.
async fn serve_static_file(req: Req, params: &Params, mime_types: &HashMap<String, String>) -> Result<Response<Body>, Infallible> {
    let path = params.dir.join(req.uri().path().trim_start_matches('/'));
    match async_fs::read(&path).await {
        Ok(content) => {
            let extension = path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or_default();
            let content_type = mime_types.get(extension).unwrap(); // Default MIME type

            Ok(Response::builder()
                .header("Content-Type", content_type)
                .body(Body::from(content))
                .unwrap())
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("404 Not Found"))
            .unwrap()),
    }
}

/// Unified request handler that processes incoming requests against defined routes.
/// 
/// # Arguments
/// 
/// * `req`: The incoming request.
/// * `params`: Parameters for the request context.
/// * `routes`: Vector of available routes.
/// * `mime_types`: HashMap of MIME types to be used for static file serving.
/// 
/// # Returns
/// 
/// A Result containing a Response for the incoming request, which may include a static file or a 404 error.
async fn handle_request(
    req: Req,
    params: Params,
    routes: Vec<Route>,
    mime_types: HashMap<String, String>, // Add MIME types to parameters
) -> Result<Response<Body>, Infallible> {
    for route in routes.iter() {
        if route.method == *req.method() {
            if let Some(captured_params) = route.is_match(&req) {
                let mut new_params = params.clone();
                new_params.extra.extend(captured_params); // Add captured params to the existing ones

                // Check if there's a '/' at the end of the URL without a subsequent value
                if new_params.extra.values().any(|value| value.is_empty()) {
                    return Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::from("Resource not found"))
                        .unwrap());
                }

                let future = (route.handler)(req, new_params);
                return future.await;
            }
        }
    }

    // If no route matches, serve static files or return 404
    serve_static_file(req, &params, &mime_types).await
}

/// Extracts query parameters from an incoming request.
/// 
/// # Arguments
/// 
/// * `req`: The incoming request.
/// 
/// # Returns
/// 
/// A HashMap of query parameters and their corresponding values.
pub fn extract_query(req: &Req) -> HashMap<String, String> {
    let mut query_params = HashMap::new();
    if let Some(query) = req.uri().query() {
        for (key, value) in url::form_urlencoded::parse(query.as_bytes()) {
            query_params.insert(key.to_string(), value.to_string());
        }
    }
    query_params
}

/// Extracts a specific query parameter by its key.
/// 
/// # Arguments
/// 
/// * `query_params`: A HashMap of all query parameters.
/// * `key`: The key of the parameter to retrieve.
/// 
/// # Returns
/// 
/// The value of the specified query parameter or an empty string if not found.
pub fn get_param(query_params: &HashMap<String, String>, key: &str) -> String {
    query_params.get(key).cloned().unwrap_or_default()
}

/// Boxes futures for unified return types.
/// 
/// # Arguments
/// 
/// * `future`: The future to be boxed.
/// 
/// # Returns
/// 
/// A boxed version of the future.
pub fn boxed<F>(future: F) -> Reply
where
    F: Future<Output = Result<Response<Body>, Infallible>> + Send + 'static,
{
    Box::pin(future)
}

/// Retrieves the value of an environment variable or returns a default value.
/// 
/// # Arguments
/// 
/// * `var_name`: The name of the environment variable.
/// * `default`: The default value if the variable is not set.
/// 
/// # Returns
/// 
/// The value of the environment variable or the default value.
pub fn env_var(var_name: &str, default: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| default.to_string())
}

/// Loads environment variables from a `.env` file if it exists.
pub fn load_dotenv() {
    dotenv().ok(); // Load environment variables from a .env file
}

/// Logs server startup messages based on user-defined criteria ("show" or "hide").
/// 
/// # Arguments
/// 
/// * `action`: A string indicating whether to show or hide the log message.
/// * `text`: The log message to display.
pub fn server_log(action: &str, text: &str) {
    match action {
        "show" => {
            println!("{}", text);
        }
        "hide" => {
            // Do nothing
        }
        _ => {
            // Nothing printed for any other input
        }
    }
}

use crate::client::HTTP_CLIENT; // Importing HTTP_CLIENT to serve HTTP responses

/// Function that serves HTTP client responses with defined content.
/// 
/// # Arguments
/// 
/// * `_req`: The incoming request (not used here).
/// * `_params`: Parameters for the request context (not used here).
/// 
/// # Returns
/// 
/// A boxed future that resolves to an HTTP response containing the client content.
pub fn serve_http_client(_req: Req, _params: Params) -> Reply {
    boxed(async {
        Ok(Response::builder()
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(Body::from(HTTP_CLIENT))
            .unwrap())
    })
}
