use cargo_futhark::{Generator, Result, Target};

fn main() -> Result<()> {
    Generator::new("src/lib.fut")
        .with_target_if(Target::C, cfg!(feature = "c"))
        .with_target_if(Target::MultiCore, cfg!(feature = "multicore"))
        .with_target_if(Target::OpenCL, cfg!(feature = "opencl"))
        .with_target_if(Target::Cuda, cfg!(feature = "cuda"))
        .with_target_if(Target::ISPC, cfg!(feature = "ispc"))
        .run()
}
