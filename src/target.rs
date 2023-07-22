use enumflags2::bitflags;

use std::fmt::Display;

#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    C,
    MultiCore,
    OpenCL,
    Cuda,
    ISPC,
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Target {
    pub fn struct_name(&self) -> &'static str {
        match self {
            Target::C => "C",
            Target::MultiCore => "MultiCore",
            Target::OpenCL => "OpenCL",
            Target::Cuda => "Cuda",
            Target::ISPC => "Ispc",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Target::C => "c",
            Target::MultiCore => "multicore",
            Target::OpenCL => "opencl",
            Target::Cuda => "cuda",
            Target::ISPC => "ispc",
        }
    }
}
