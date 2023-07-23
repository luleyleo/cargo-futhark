use cargo_futhark::{Generator, Result, Target};

fn main() -> Result<()> {
    Generator::new("futhark/lib.fut")
        .with_target(Target::C)
        .run()
}
