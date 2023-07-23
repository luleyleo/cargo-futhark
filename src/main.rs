use std::{fs, path::Path};

use clap::{Parser, Subcommand};
use eyre::{ensure, Context, Result};
use include_dir::{include_dir, Dir};

static PROJECT_TEMPLATE: Dir = include_dir!("$CARGO_MANIFEST_DIR/project-template");

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(author = "Leopold Luley")]
#[command(version = "0.1")]
#[command(about = "Use `cargo futhark` instead")]
enum Cli {
    #[command(about = "Manage Cargo-Futhark projects")]
    Futhark {
        #[command(subcommand)]
        command: Commands,
    },
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create a new Cargo-Futhark project")]
    New { name: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli {
        Cli::Futhark {
            command: Commands::New { name },
        } => new_project(name),
    }
}

fn new_project(name: &str) -> Result<()> {
    let project = Path::new(name);

    ensure!(
        !project.exists(),
        "A directory with the name '{name}' already exists."
    );

    fs::create_dir(project).wrap_err("Failed to create project directory.")?;

    PROJECT_TEMPLATE
        .extract(project)
        .wrap_err("Failed to extract template.")?;

    let cargo_toml_path = project.join("Cargo.toml");
    let replacements = replace_in_file(cargo_toml_path, "{{lib-name}}", name)
        .wrap_err("Failed to replace project name in Cargo.toml file.")?;
    ensure!(
        replacements == 1,
        "Failed to replace project name in Cargo.toml file."
    );

    println!("The new project '{name}' was created successfully.");

    if is_subproject() {
        println!();
        println!("You might want to add it to your existing Cargo.toml file:");
        println!("[workspace]");
        println!("members = [\"{name}\"]");
    }

    Ok(())
}

fn replace_in_file(path: impl AsRef<Path>, from: &str, to: &str) -> Result<usize> {
    let content = fs::read_to_string(&path).wrap_err("Failed to read source file.")?;
    let occurrences = content.match_indices(from).count();
    let new_content = content.replace(from, to);
    fs::write(&path, new_content).wrap_err("Failed to write after replacement.")?;

    Ok(occurrences)
}

fn is_subproject() -> bool {
    let absolute_path = match Path::new(".").canonicalize() {
        Ok(path) => path,
        Err(_) => return false,
    };

    let mut dir = absolute_path.as_path();

    loop {
        if dir.join("Cargo.toml").is_file() {
            return true;
        }

        if dir.parent().is_none() {
            return false;
        }

        dir = dir.parent().unwrap();
    }
}
