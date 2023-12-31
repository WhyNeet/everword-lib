use crate::dict::traits::Scrape;

use super::cambridge::CambridgeScraper;

pub struct ProviderFactory;

impl ProviderFactory {
    pub fn get_provider(id: &str) -> Option<impl Scrape> {
        match id {
            "cambridge" => Some(CambridgeScraper),
            _ => None,
        }
    }

    pub fn get_all() -> Box<[Provider]> {
        Box::new([Provider {
            id: Box::from("cambridge"),
            name: Box::from("Cambridge Dictionary"),
        }])
    }
}

#[derive(serde::Serialize)]
pub struct Provider {
    id: Box<str>,
    name: Box<str>,
}
