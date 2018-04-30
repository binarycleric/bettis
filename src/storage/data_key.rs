#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct DataKey {
    key: String,
}

impl DataKey {
    pub fn new(string: String) -> Self {
        Self { key: string }
    }
}
