use crate::{manifest::Manifest, template, Target};
use bindgen::callbacks::ParseCallbacks;
use enumflags2::BitFlags;
use eyre::{bail, ensure, Context, Result};
use rerun_except::rerun_except;
use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
};

mod names {
    pub const TARGET_DIR: &str = "futhark";
    pub const RAW_TARGET_DIR: &str = "futhark_raw";

    pub const LIBRARY: &str = "futhark_lib";
    pub const MANIFEST: &str = "futhark_lib.json";

    pub const H_FILE: &str = "futhark_lib.h";
    pub const C_FILE: &str = "futhark_lib.c";
    pub const RS_FILE: &str = "futhark_lib.rs";
}

fn cargo_out_dir() -> Result<PathBuf> {
    env::var("OUT_DIR")
        .wrap_err("OUT_DIR is undefined.")
        .map(|dir| PathBuf::from(dir))
}

fn cargo_manifest_dir() -> Result<PathBuf> {
    env::var("CARGO_MANIFEST_DIR")
        .wrap_err("CARGO_MANIFEST_DIR is undefined.")
        .map(|dir| PathBuf::from(dir))
}

pub struct Generator {
    source: PathBuf,
    watch: bool,
    targets: BitFlags<Target>,
}

impl Generator {
    pub fn new(source: impl Into<PathBuf>) -> Self {
        Generator {
            source: source.into(),
            watch: true,
            targets: BitFlags::empty(),
        }
    }

    pub fn watch_sources(&mut self, watch: bool) -> &mut Self {
        self.watch = watch;
        self
    }

    pub fn with_target(&mut self, target: Target) -> &mut Self {
        self.targets |= target;
        self
    }

    pub fn with_target_if(&mut self, target: Target, condition: bool) -> &mut Self {
        if condition {
            self.targets |= target;
        }
        self
    }

    pub fn run(&mut self) -> Result<()> {
        ensure!(self.source.is_file(), "Futhark source file does not exist.");

        ensure!(
            !self.targets.is_empty(),
            "At least one target must be built."
        );

        self.build_targets().wrap_err("Failed to build targets.")?;

        self.generate_library()
            .wrap_err("Failed to generate Rust library.")?;

        Ok(())
    }
}

impl Generator {
    fn generate_library(&mut self) -> Result<(), eyre::ErrReport> {
        let any_target = self.targets.iter().next().unwrap();
        let manifest_path = cargo_out_dir()?
            .join(names::TARGET_DIR)
            .join(any_target.name())
            .join(names::MANIFEST);
        let manifest = Manifest::from_json_file(&manifest_path).wrap_err_with(|| {
            format!(
                "Failed to load manifest file at {}.",
                manifest_path.display()
            )
        })?;

        let rust_lib = template::combined(&manifest, self.targets).to_string();
        let rust_lib_path = cargo_out_dir()?
            .join(names::TARGET_DIR)
            .join(names::RS_FILE);

        let mut rust_lib_file = fs::File::create(&rust_lib_path)
            .wrap_err("Failed to create generated Rust library file.")?;
        writeln!(rust_lib_file, "{}", rust_lib)
            .wrap_err("Failed to write generated Rust library.")?;
        rust_lib_file.flush().wrap_err("Failed to flush.")?;

        let rustfmt_status = Command::new("rustfmt")
            .arg(rust_lib_path)
            .status()
            .wrap_err("Failed to run rustfmt.")?
            .success();

        Ok(if !rustfmt_status {
            bail!("Failed to format generated Rust library.");
        })
    }

    fn build_targets(&self) -> Result<()> {
        if self.watch {
            watch_source(&self.source).wrap_err("Failed to watch source files for changes.")?;
        }

        if self.targets.contains(Target::C) {
            build_target(&self.source, Target::C).wrap_err("Failed to build C target.")?;
        }

        if self.targets.contains(Target::MultiCore) {
            build_target(&self.source, Target::MultiCore)
                .wrap_err("Failed to build Multi-Core target.")?;
        }

        if self.targets.contains(Target::OpenCL) {
            build_target(&self.source, Target::OpenCL)
                .wrap_err("Failed to build OpenCL target.")?;

            println!("cargo:rustc-link-lib=OpenCL");
        }

        if self.targets.contains(Target::Cuda) {
            build_target(&self.source, Target::Cuda).wrap_err("Failed to build Cuda target.")?;

            println!("cargo:rustc-link-search=/opt/cuda/lib64");
            println!("cargo:rustc-link-lib=cuda");
            println!("cargo:rustc-link-lib=cudart");
            println!("cargo:rustc-link-lib=nvrtc");
        }

        Ok(())
    }
}

fn build_target(source: &Path, target: Target) -> Result<()> {
    let out_dir = cargo_out_dir()?;
    let target_dir = out_dir.join(names::TARGET_DIR).join(target.name());
    fs::create_dir_all(&target_dir).wrap_err("Could not create target dir.")?;

    let raw_target_dir = out_dir.join(names::RAW_TARGET_DIR).join(target.name());
    fs::create_dir_all(&raw_target_dir).wrap_err("Could not create raw target dir.")?;

    let futhark_status = Command::new("futhark")
        .args(&[target.name(), "--library", "-o"])
        .arg(raw_target_dir.join(names::LIBRARY))
        .arg(source.as_os_str())
        .status()
        .wrap_err("Failed to run Futhark compiler.")?
        .success();

    if !futhark_status {
        bail!("Failed to compile Futhark code.");
    }

    fs::copy(
        raw_target_dir.join(names::MANIFEST),
        target_dir.join(names::MANIFEST),
    )
    .wrap_err("Failed to copy manifest file")?;

    let prefix = format!("futhark_{target}_");

    prefix_items(
        &prefix,
        raw_target_dir.join(names::H_FILE),
        target_dir.join(names::H_FILE),
    )
    .wrap_err("Failed to prefix header file items.")?;

    prefix_items(
        &prefix,
        raw_target_dir.join(names::C_FILE),
        target_dir.join(names::C_FILE),
    )
    .wrap_err("Failed to prefix source file items.")?;

    bindgen::Builder::default()
        .clang_arg("-I/opt/cuda/include")
        .header(target_dir.join(names::H_FILE).to_string_lossy())
        .allowlist_function("free")
        .allowlist_function("futhark_.*")
        .allowlist_type("futhark_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(PrefixRemover::new(prefix))
        .generate()
        .wrap_err("Failed to generate bindings.")?
        .write_to_file(target_dir.join(names::RS_FILE))
        .wrap_err("Failed to write bindings to file.")?;

    cc::Build::new()
        .file(target_dir.join(names::C_FILE))
        .include("/opt/cuda/include")
        .static_flag(true)
        .warnings(false)
        .try_compile(&format!("futhark-lib-{compiler}", compiler = target))
        .wrap_err("Failed to compile the generated c code.")?;

    Ok(())
}

fn watch_source(source: &Path) -> Result<()> {
    let old_manifest_dir = cargo_manifest_dir()?;

    env::set_var("CARGO_MANIFEST_DIR", source.parent().unwrap().as_os_str());

    rerun_except(&[])
        .map_err(|err| eyre::eyre!("{}", err))
        .wrap_err("Failed to watch files.")?;

    env::set_var("CARGO_MANIFEST_DIR", old_manifest_dir);

    Ok(())
}

fn prefix_items(prefix: &str, input: impl AsRef<Path>, output: impl AsRef<Path>) -> Result<()> {
    let mut out = BufWriter::new(File::create(output).wrap_err("Failed to open output file.")?);

    let memblock_prefix = &format!("{prefix}_memblock_");
    let lexical_realloc_error_prefix = &format!("{prefix}_lexical_realloc_error");

    for line in fs::read_to_string(input)?.lines() {
        let new_line = line
            .replace("memblock_", memblock_prefix)
            .replace("lexical_realloc_error", lexical_realloc_error_prefix)
            .replace("futhark_", &prefix);

        writeln!(out, "{}", new_line).wrap_err("Failed to write line to output file.")?;
    }

    out.flush().wrap_err("Failed to flush output file.")?;

    Ok(())
}

#[derive(Debug)]
struct PrefixRemover {
    prefix: String,
}

impl PrefixRemover {
    pub fn new(prefix: impl ToOwned<Owned = String>) -> Box<dyn ParseCallbacks> {
        Box::new(PrefixRemover {
            prefix: prefix.to_owned(),
        })
    }
}

impl ParseCallbacks for PrefixRemover {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        if original_item_name.contains(&self.prefix) {
            return Some(original_item_name.replace(&self.prefix, "futhark_"));
        }

        None
    }
}
