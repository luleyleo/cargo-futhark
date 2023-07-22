mod futhark {
    include!(concat!(env!("OUT_DIR"), "/futhark/futhark_lib.rs"));
}

fn main() {
    println!("Hello, world!");
}
