#[derive(Debug, Clone, Default)]
pub struct Agent {
    pub owner: u32,
}

impl Agent {
    pub fn new(owner: u32) -> Self {
        Self { owner }
    }
}