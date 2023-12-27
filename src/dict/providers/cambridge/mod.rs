pub mod parser;
pub mod statics;

use std::collections::HashMap;

use async_trait::async_trait;

use crate::dict::{model::Defenition, traits::Scrape};

use self::parser::parse;

pub struct CambridgeScraper;

#[async_trait]
impl Scrape for CambridgeScraper {
    async fn scrape(word: &str) -> anyhow::Result<HashMap<String, Vec<Defenition>>> {
        let doc = reqwest::get(format!(
            "https://dictionary.cambridge.org/us/dictionary/english/{word}"
        ))
        .await?
        .text()
        .await?;

        Ok(parse(&doc))
    }
}
