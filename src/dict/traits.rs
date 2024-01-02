use std::collections::HashMap;

use async_trait::async_trait;

use super::model::Defenition;

pub type Defenitions = HashMap<String, Vec<Defenition>>;

#[async_trait]
pub trait Scrape {
    async fn scrape(self, word: &str) -> anyhow::Result<Defenitions>;
}
