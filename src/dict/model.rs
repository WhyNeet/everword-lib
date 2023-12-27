use serde::Serialize;

#[derive(Serialize, PartialEq, Debug)]
pub struct Defenition {
    text: String,
    examples: Vec<String>,
}

impl Defenition {
    pub fn new(text: String, examples: Vec<String>) -> Self {
        Self { text, examples }
    }
}
