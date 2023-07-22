use crate::manifest::ValueType;

#[derive(Debug, Clone, Copy)]
pub struct ArrayType {
    pub elements: ValueType,
    pub rank: usize,
}

impl ArrayType {
    pub fn struct_name(&self) -> String {
        format!("{}_{}d", self.elements.rust_name(), self.rank).to_ascii_uppercase()
    }

    pub fn type_name(&self) -> String {
        format!("futhark_{}_{}d", self.elements.rust_name(), self.rank)
    }

    pub fn fn_new_name(&self) -> String {
        format!("futhark_new_{}_{}d", self.elements.rust_name(), self.rank)
    }

    pub fn fn_values_name(&self) -> String {
        format!(
            "futhark_values_{}_{}d",
            self.elements.rust_name(),
            self.rank
        )
    }

    pub fn fn_shape_name(&self) -> String {
        format!("futhark_shape_{}_{}d", self.elements.rust_name(), self.rank)
    }

    pub fn fn_free_name(&self) -> String {
        format!("futhark_free_{}_{}d", self.elements.rust_name(), self.rank)
    }
}
