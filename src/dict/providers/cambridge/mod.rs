pub mod parser;
pub mod statics;

use async_trait::async_trait;

use crate::dict::traits::{Defenitions, Scrape};

use self::parser::parse;

pub struct CambridgeScraper;

#[async_trait]
impl Scrape for CambridgeScraper {
    async fn scrape(self, word: &str) -> anyhow::Result<Defenitions> {
        let doc = reqwest::get(format!(
            "https://dictionary.cambridge.org/us/dictionary/english/{word}"
        ))
        .await?
        .text()
        .await?;

        Ok(parse(&doc))
    }
}
