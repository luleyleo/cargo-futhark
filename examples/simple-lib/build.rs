use futhark_rust::{Generator, Result, Target};

fn main() -> Result<()> {
    Generator::new("futhark/lib.fut")
        .with_target(Target::C)
        .run()
}
