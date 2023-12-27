use std::collections::HashMap;

use async_trait::async_trait;

use super::model::Defenition;

#[async_trait]
pub trait Scrape {
    async fn scrape(word: &str) -> anyhow::Result<HashMap<String, Vec<Defenition>>>;
}
