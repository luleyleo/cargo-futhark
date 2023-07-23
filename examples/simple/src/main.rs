use simple_example_lib::{backends, Array_F64_1D, Config, Context};

type Backend = backends::C;

fn main() {
    let config = Config::<Backend>::new();
    let context = Context::new(config);

    let input = &[1.0, 2.0, 3.0];
    let input = Array_F64_1D::new(&context, input, input.len());
    let double = context.entry_double(&input).unwrap();
    let average = context.entry_average(&double).unwrap();

    println!("result: {}", average);
}
