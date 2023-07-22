type Backend = simple_lib::backends::C;

fn main() {
    let config = simple_lib::Config::<Backend>::new();
    let context = simple_lib::Context::new(config);

    let input = &[1.0, 2.0, 3.0];
    let input = simple_lib::F64_1D::new(&context, input, input.len());
    let double = context.entry_double(&input).unwrap();
    let average = context.entry_average(&double).unwrap();

    println!("result: {}", average);
}
