#[derive(Debug, Clone, Default)]
pub struct Goon {
    pub owner: u32,
}

impl Goon {
    pub fn new(owner: u32) -> Self {
        Self { owner }
    }
}