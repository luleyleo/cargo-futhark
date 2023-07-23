use cargo_futhark::{Generator, Result, Target};

fn main() -> Result<()> {
    Generator::new("src/lib.fut").with_target(Target::C).run()
}
