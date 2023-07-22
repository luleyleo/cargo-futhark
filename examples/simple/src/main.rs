type Backend = simple_lib::backends::C;

fn main() {
    let config = simple_lib::Config::<Backend>::new();
    let context = simple_lib::Context::new(config);

    let input = &[1.0, 2.0, 3.0];
    let average = 2.0;

    assert_eq!(average, 2.0);
}
