use futhark_rust::{Generator, Target};

fn main() {
    Generator::new("futhark/lib.fut")
        .expect("Futhark source not found.")
        .with_target(Target::C)
        .run()
        .expect("Failed to run generator.");
}
