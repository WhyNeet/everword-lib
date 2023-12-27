mod constants;
use constants::TESTING_DOCUMENT;

use everword_lib::dict::{
    cambridge::{self, statics::DEF_BLOCK_SELECTOR},
    model::Defenition,
};
use scraper::Html;

#[tokio::test]
async fn defs_selector_works() {
    let doc = reqwest::get("https://dictionary.cambridge.org/us/dictionary/english/word")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let document = Html::parse_document(&doc);
    let mut defs = document.select(&DEF_BLOCK_SELECTOR);

    assert!(defs.next().is_some());
}

#[test]
fn parser_works() {
    let defs = cambridge::parser::parse(TESTING_DOCUMENT);

    dbg!(&defs);

    assert_eq!(defs.len(), 1);
    assert_eq!(defs["noun"].len(), 3);
    assert_eq!(
        defs["noun"][0],
        Defenition::new("a piece of equipment that you speak into to make your voice louder, or to record your voice or other sounds".to_string(), vec!["The interviewer asked her to speak into/use the microphone.".to_string(), "My laptop has a built-in microphone.".to_string()])
    );
}
