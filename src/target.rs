use enumflags2::bitflags;

use std::fmt::Display;

/// Futhark targets.
#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// Futhark `c` target.
    ///
    /// # Requirements
    /// None
    C,

    /// Futhark `multicore` target.
    ///
    /// # Requirements
    /// None
    MultiCore,

    /// Futhark `opencl` target.
    ///
    /// # Requirements
    /// - OpenCL headers
    /// - OpenCL loader
    OpenCL,

    /// Futhark `cuda` target.
    ///
    /// # Requirements
    /// - Nvidia CUDA SDK
    Cuda,

    /// Futhark `ispc` target.
    ///
    /// # Requirements
    /// - Intel ISPC SDK
    ISPC,
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Target {
    pub(crate) fn struct_name(&self) -> &'static str {
        match self {
            Target::C => "C",
            Target::MultiCore => "MultiCore",
            Target::OpenCL => "OpenCL",
            Target::Cuda => "Cuda",
            Target::ISPC => "Ispc",
        }
    }

    pub(crate) fn name(&self) -> &'static str {
        match self {
            Target::C => "c",
            Target::MultiCore => "multicore",
            Target::OpenCL => "opencl",
            Target::Cuda => "cuda",
            Target::ISPC => "ispc",
        }
    }
}
