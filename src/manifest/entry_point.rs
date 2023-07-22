use crate::manifest::Type;

#[derive(Debug, Clone)]
pub struct EntryPoint {
    pub name: String,
    pub inputs: Vec<Type>,
    pub outputs: Vec<Type>,
}

impl EntryPoint {
    pub fn fn_name(&self) -> String {
        format!("futhark_entry_{}", self.name)
    }

    pub fn context_fn_name(&self) -> String {
        format!("entry_{}", self.name)
    }
}
