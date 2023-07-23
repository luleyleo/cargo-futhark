use std::{fs, path::Path, process::Command};

use cargo_futhark::{Generator, Target};
use eyre::{ensure, Context, Result};

mod common;

#[test]
fn generate_c_target() -> Result<()> {
    common::setup_env();
    common::create_out_dir()?;

    Generator::new("project-template/src/lib.fut")
        .with_target(Target::C)
        .watch_sources(false)
        .run()
}

#[test]
fn simple_example_matches_template() {
    let example_dir = Path::new("examples").join("simple-lib");
    let template_dir = Path::new("project-template");

    let example_lib_fut = example_dir.join("src").join("lib.fut");
    let example_lib_rs = example_dir.join("src").join("lib.rs");
    let example_lib_build = example_dir.join("build.rs");

    let template_lib_fut = template_dir.join("src").join("lib.fut");
    let template_lib_rs = template_dir.join("src").join("lib.rs");
    let template_lib_build = template_dir.join("build.rs");

    assert_eq!(
        fs::read_to_string(example_lib_fut).unwrap(),
        fs::read_to_string(template_lib_fut).unwrap(),
        "lib.fut differs"
    );

    assert_eq!(
        fs::read_to_string(example_lib_rs).unwrap(),
        fs::read_to_string(template_lib_rs).unwrap(),
        "lib.rs differs"
    );

    assert_eq!(
        fs::read_to_string(example_lib_build).unwrap(),
        fs::read_to_string(template_lib_build).unwrap(),
        "build.rs differs"
    );
}

#[test]
fn run_c_target() -> Result<()> {
    let output = Command::new("cargo")
        .arg("run")
        .arg("-q")
        .arg("--package")
        .arg("simple-example")
        .output()
        .wrap_err("Failed to run `cargo run` command.")?;

    let std_out = String::from_utf8(output.stdout).wrap_err("Failed to parse output as string")?;

    let output = std_out.trim();
    let expected = "result: 4";

    ensure!(
        output == expected,
        "Unexpected output: \"{output}\" != \"{expected}\""
    );

    Ok(())
}
