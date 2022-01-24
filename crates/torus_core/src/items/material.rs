use crate::items::DefaultFloat;

/// A raw in-game material that can be used to build items.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Material {
    /// Material name
    pub name: String,
    pub display_name: String,
    pub density: DefaultFloat,
}
