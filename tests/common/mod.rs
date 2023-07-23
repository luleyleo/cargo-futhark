use std::{env, fs, path::Path};

use eyre::{Context, Result};

pub fn setup_env() {
    env::set_var("HOST", "x86_64-unknown-linux-gnu");
    env::set_var("TARGET", "x86_64-unknown-linux-gnu");
    env::set_var("OPT_LEVEL", "0");
    env::set_var("OUT_DIR", env!("CARGO_TARGET_TMPDIR"));
    env::set_var("CARGO_MANIFEST_DIR", env!("CARGO_MANIFEST_DIR"));
}

pub fn create_out_dir() -> Result<()> {
    let out_dir = env::var("OUT_DIR").wrap_err("OUT_DIR is not set.")?;
    let out_dir = Path::new(&out_dir);

    if out_dir.exists() {
        fs::remove_dir_all(out_dir).wrap_err("Failed to remove existing output directory.")?;
    }

    fs::create_dir_all(out_dir).wrap_err("Failed to create output directory.")?;

    Ok(())
}
