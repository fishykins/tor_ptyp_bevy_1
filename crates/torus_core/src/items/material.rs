use crate::items::DefaultFloat;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Material {
    /// Material name
    pub name: String,
    pub display_name: String,
    pub density: DefaultFloat,
}
