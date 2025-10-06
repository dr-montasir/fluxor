<div align="center">
    <a href="https://github.com/dr-montasir/fluxor">
        <img src="fluxor-icon-64x64.svg" width="100">
        <h2>FLUXOR</h2>
    </a>
    <a href="https://github.com/dr-montasir/fluxor" target="_blank">
        <img alt="github" src="https://img.shields.io/badge/github-dr%20montasir%20/%20fluxor-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">
    </a>
    <a href="https://crates.io/crates/fluxor" target="_blank">
        <img alt="crates.io" src="https://img.shields.io/crates/v/fluxor.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">
    </a>
    <a href="https://docs.rs/fluxor" target="_blank">
        <img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-fluxor-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="22">
    </a>
    <a href="https://choosealicense.com/licenses/apache-2.0" target="_blank">
       <img alt="license" src="https://img.shields.io/badge/license-apache_2.0-4a98f7.svg?style=for-the-badge&labelColor=555555&logo=apache" height="22">
    </a>
    <a href="https://choosealicense.com/licenses/mit" target="_blank">
       <img alt="license" src="https://img.shields.io/badge/license-mit-4a98f7.svg?style=for-the-badge&labelColor=555555&logo=apache" height="22">
    </a>
    <a href="https://crates.io/crates/fluxor" target="_blank">
        <img 
            alt="downloads" 
            src="https://img.shields.io/crates/d/fluxor.svg?style=for-the-badge&labelColor=555555&logo=&color=428600"
            height="22"
        >
    </a>
    <a href="https://deps.rs/crate/fluxor" target="_blank">
        <img 
            alt="Dependency Status" 
            src="https://deps.rs/crate/fluxor/latest/status.svg?style=for-the-badge"
            height="22"
        >
    </a>
</div>
**Fluxor** is a versatile Rust web framework designed for data science and computing science applications. Inspired by frameworks like Express.js, Flask, and Shiny, Fluxor provides a robust environment for developing applications that require efficient data handling and server management.

[SEE CHANGELOG](https://github.com/dr-montasir/fluxor/blob/main/CHANGELOG.md)

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
- [Documentation](#documentation)
- [Modules](#modules)
- [Roadmap](#roadmap)
- [Examples](#examples)
- [Fluxor - CLI](#fluxor---cli)
- [Logo Definition and Meaning](#logo-definition-and-meaning)
- [Contributing](#contributing)
- [License](#license)
- [Author](#author)

## Features

- **Asynchronous**: Built on the Tokio runtime for powerful non-blocking I/O.
- **Robust**: Offers a rich set of functionalities tailored for scientific computing and web development.
- **Extendable**: Easily integrate with other data storage solutions such as MongoDB, PostgreSQL, and Redis.
- **CLI Tool**: A dedicated CLI (`fluxor_cli`) for project scaffolding to expedite the development process.
- **Modular**: Comprises various modules like math, data handling, logging, and more to facilitate streamlined development.

## Getting Started

To begin using Fluxor, ensure that you have [Rust](https://www.rust-lang.org/tools/install) installed on your system. You can either create a new Fluxor project or add Fluxor to an existing project.

### Option 1: Adding Fluxor to an Existing Project

If you're starting from scratch, you can add Fluxor directly to a new or existing Rust project using Cargo:

1. Create a new Rust project:

```terminal

cargo new <project_name>
cd <project_name>

```

2. Add Fluxor as a dependency:

```terminal

cargo add fluxor

```

### Option 2: Creating a New Fluxor Project with the Fluxor CLI

If you prefer to use the Fluxor CLI to create a new project, you can do so with the following commands:

1. First, install the Fluxor CLI:

```terminal

cargo install fluxor_cli

```

2. Create a new Fluxor project:

```terminal
      
fluxor new <project_name> --version latest --example helloworld
    
```

Replace `<project_name>` with your desired project name.

Once the project scaffolding is complete, navigate into your project directory:

```terminal
      
cd <project_name>
    
```

Now, you can build and run your Fluxor application:

```terminal
      
cargo run
    
```

Your application should now be running on `http://127.0.0.1:8080`.

## Documentation

Comprehensive documentation is available in the [docs](https://docs.rs/fluxor/latest/fluxor) directory for further guidance on using Fluxor, modules, configuration, and deployment.

## Modules

Fluxor is organized into several key modules:

- **Core**: The backbone of the framework providing essential functionalities.
- **Client**: A simple HTTP client built within the Fluxor framework for testing API endpoints, eliminating the need for external tools like Postman. This module allows developers to easily construct and send HTTP requests, view responses, and manage various request parameters, facilitating effective and streamlined API testing directly within the Fluxor environment.
- **Data**: Modules for interacting with different data storage systems: MongoDB, PostgreSQL, and Redis.
- **Encode**: A module for encoding and decoding data using Serde and Serde JSON. It provides functions to convert Rust data structures to JSON strings and back, facilitating data serialization and deserialization.
- **Math**: Contains functionalities for performing mathematical computations.
- **StyledLog**: Provides an efficient and aesthetically pleasing logging mechanism.
- **Cans**: An elegant and lightweight Rust-based literal template engine for managing web content, enhanced with a world module for streamlined access to regional and city information, as well as robust MIME type management.
- **Wtime**: Provides a variety of functions for obtaining the current UTC and local times, along with generating customizable timestamps to suit your needs.
- **Fluxio**: A wrapper around the Hyper crate, designed specifically for use within the Fluxor project. This module simplifies HTTP client functionalities, enhancing performance and integration within the Fluxor framework.

## Roadmap

- **Website**: Develop a dedicated website for Fluxor that showcases its features and provides resources for users.
- **Comprehensive Documentation**: Expand the existing documentation to cover more use cases and detailed explanations of functionalities.
- **More Examples and Scenarios**: Add more examples and scenarios in `fluxor_cli` to help developers understand how to utilize the framework effectively.
- **Middleware Enhancements**: Already implemented additional features for handling request parameters (e.g., `req.params.extra`), similar to database integration. Future releases will aim to add more middleware functions to enhance Fluxor’s capabilities.

## Examples

### Hello World Example

A basic Fluxor application that responds with "Hello, World!" when accessed via a web browser.

```rust
use fluxor::prelude::*;

fn hello(_req: Req, _params: Params) -> Reply {
    boxed(async {
        Ok(Response::builder()
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(Body::from("<h1>👋 Hello, World!</h1>"))
            .unwrap())
    })
}

#[tokio::main]
async fn main() {
    let mut app = Fluxor::new();			// Initialize the application.
    app.route(GET, "/", hello);				// Set the route (method, path, handler).
    app.run("127.0.0.1", "8080").await;		// Start the HTTP server (host, port).
}
```

### API Examples

A simple Fluxor API endpoint that returns a JSON response (method: GET).

```rust
use fluxor::prelude::*;

fn hello(_req: Req, _params: Params) -> Reply {
    boxed(async move {
        let json_response = format!(r#"{{"message": "👋 Hello, World!"}}"#);
        
        Ok(Response::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(json_response))
            .unwrap())
    })
}

#[tokio::main]
async fn main() {
    let mut app = Fluxor::new();							// Initialize the application.
    app.route(GET, "/", hello);								// Set the route (method, path, handler).
    app.route(GET, "/http-client", serve_http_client);  	// A simple http client to test your application.
    app.run("127.0.0.1", "8080").await;						// Start the HTTP server (host, port).
}   
```

A simple Fluxor API endpoint that returns a JSON response (method: POST).

```rust
use fluxor::prelude::*;

fn hello(_req: Req, _params: Params) -> Reply {
    boxed(async move {
       let json_response = format!(r#"{{"message": "👋 Hello, World!"}}"#);
        
        Ok(Response::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(json_response))
            .unwrap())
    })
}

#[tokio::main]
async fn main() {
    let mut server = Fluxor::new();                     	// Initialize the application.
    server.route(POST, "/", hello);                     	// Set the route (method, path, handler).
    server.route(GET, "/http-client", serve_http_client);  	// A simple HTTP client to test your application.
    server.run("127.0.0.1", "8080").await;              	// Start the HTTP server (host, port).
}
```

## Fluxor - CLI

The `fluxor_cli` allows users to quickly scaffold new Fluxor projects. Here's how to utilize it:

```terminal
      
fluxor new <project_name> --version <version> --example <example-name>
    
```

- **project_name**: The name of your new project.
- **version**: The version of Fluxor to use (default is 'latest').
- **example-name**: Choose between example templates like `helloworld` or `helloworld-api`.

### All Fluxor CLI Examples

### 1. Hello World
- helloworld
- helloworld-api
- helloworld-api-server

### 2. Routes
- routes
- routes-project

### 3. Assets
- assets

### 4. DotEnv
- dotenv

### 5. Cans
- cans-template-engine

Use the example name after the flag --example (e.g., `helloworld`):

```terminal
fluxor new my_project --version latest --example helloworld

fluxor new my_app --version latest --example routes

fluxor new routes_app --version latest --example routes-project

fluxor new assets_example --version latest --example assets

fluxor new dotenv_example --version latest --example dotenv

fluxor new template_app --version latest --example cans-template-engine
```

## Logo Definition and Meaning

The logo consists of a stylized letter "S" and an inverted letter "F." The letter "S" represents "server," while the letter "F" symbolizes the name of the framework "Fluxor." The logo features two colors: orange, representing the Rust programming language, and cyan, symbolizing "Fluxio," a Rust programming library for HTTP. Together, the logo conveys the image of a flame, indicating that the server is running.

## Contributing

Contributions are welcome! If you'd like to contribute to Fluxor, please fork the repository, create a new branch, and submit a pull request. For larger changes, please discuss your ideas via an issue before implementing them.

## License

Fluxor is licensed under either of the following licenses:

- MIT License
- Apache License, Version 2.0

See the LICENSE file for more details.

---

## Author

[Dr. Montasir Mirghani](https://github.com/dr-montasir)
