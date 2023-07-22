use enumflags2::BitFlags;
use futhark_rust::{manifest::Manifest, template, Target};

const TEST_MANIFEST: &str = include_str!("test_manifest.json");

fn main() {
    let targets = BitFlags::from(Target::C);
    let manifest = Manifest::from_json(TEST_MANIFEST).unwrap();

    let output = template::combined(&manifest, targets).to_string();

    println!("{}", output);
}
