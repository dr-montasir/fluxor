pub mod hello_world;

pub use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::process::Command as ProcessCommand;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    New {
        name: String,
        #[clap(short, long, default_value = "latest")]
        version: String,
        #[clap(short, long, default_value = "helloworld")]
        example: String,
    },
}

pub fn fetch_latest_version(crate_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = ProcessCommand::new("cargo")
        .arg("search")
        .arg(crate_name)
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Failed to fetch latest version of {}: {}",
            crate_name,
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Cargo search output: {}", stdout); // Debug print

    // Use a regex to extract the version number
    let re = Regex::new(r#"(\d+\.\d+\.\d+)"#).unwrap();
    let version = stdout
        .lines()
        .find(|line| line.contains(crate_name))
        .and_then(|line| re.find(line))
        .map(|version| version.as_str())
        .ok_or_else(|| format!("Crate '{}' not found", crate_name))?;

    println!("Parsed version: {}", version); // Debug print

    Ok(version.to_string())
}

pub fn create_fluxor_web_project(name: &str, version: &str, example: &str) {
    let project_path = Path::new(name);

    // Check if project directory already exists
    if project_path.exists() {
        eprintln!("Project directory already exists.");
        return;
    }

    // Create project directory
    fs::create_dir_all(&project_path).expect("Failed to create project directory");

    // Fetch fluxor version if "latest" is specified
    let fluxor_version = if version == "latest" {
        fetch_latest_version("fluxor").expect("Failed to fetch latest version of fluxor")
    } else {
        version.to_string()
    };

    // Create .gitignore file
    let gitignore = r#"/target
    "#;
       
    fs::write(project_path.join(".gitignore"), gitignore)
        .expect("Failed to create .gitignore");

    // Create .gitignore file
    let readme = format!(r#"# {}

This project has been initialized with the assistance of the [Fluxor CLI](https://crates.io/crates/fluxor_cli), a command-line tool that allows developers to quickly and efficiently create project starters for the [Fluxor web framework](https://crates.io/crates/fluxor)."
"#, name);
        
    fs::write(project_path.join("README.md"), readme)
        .expect("Failed to create readme file");

    // Create Cargo.toml specific for the example
    let cargo_toml = match example {
        // Hello World Examples
        "helloworld" => hello_world::hello_world_cargo_toml(name, &fluxor_version),
        "helloworld-api" => hello_world::hello_world_cargo_toml(name, &fluxor_version),
        "helloworld-api-server" => hello_world::hello_world_api_server_cargo_toml(name, &fluxor_version),
        _ => {
            eprintln!("Unknown example specified: {}", example);
            return;
        }
    };

    fs::write(project_path.join("Cargo.toml"), cargo_toml)
        .expect("Failed to create Cargo.toml");

    // Create src directory under the project
    let src_path = project_path.join("src");
    fs::create_dir_all(&src_path).expect("Failed to create src directory");

    // Create main.rs based on the specified example
    match example {
        "helloworld" => {
            hello_world::hello_world_template(&src_path);
        }
        "helloworld-api" => {
            hello_world::hello_world_api_template(&src_path);
        }
        "helloworld-api-server" => {
            hello_world::hello_world_api_server_template(&src_path);
        }
        _ => {
            eprintln!("Unknown example specified: {}", example);
            return;
        }
    }

    println!("Fluxor project '{}' created successfully using the '{}' example.", name, example);
}
