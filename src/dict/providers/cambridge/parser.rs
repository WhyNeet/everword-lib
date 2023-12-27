use std::collections::HashMap;

use scraper::{ElementRef, Html};

use crate::{dict::model::Defenition, none_return};

use super::statics::{
    DEF_BLOCK_SELECTOR, DEF_ENTRY_SELECTOR, DEF_EXAMPLES_SELECTOR, DEF_PART_OF_SPEECH_SELECTOR,
};

pub fn parse(doc: &str) -> HashMap<String, Vec<Defenition>> {
    let document = Html::parse_document(doc);
    let defs = document.select(&DEF_ENTRY_SELECTOR);

    /*
        Get two mappings:
         - defenition -> part of speech
         - defenition -> examples
    */
    let (def_part_of_speech, mut def_examples) =
        defs.into_iter()
            // (HashMap::<defenition, part_of_speech>, HashMap::<defenition, examples>)
            .fold(
                (
                    HashMap::<String, String>::new(),
                    HashMap::<String, Vec<String>>::new(),
                ),
                |(mut parts_of_speech_acc, mut examples_acc), def| {
                    let mut text = def.select(&DEF_BLOCK_SELECTOR);
                    let text = text.next().unwrap().text().collect::<String>();
                    let text = text.trim().strip_suffix(':').unwrap_or(&text).to_string();

                    if !text.chars().all(|c| {
                        c.is_alphanumeric() || c.is_whitespace() || c.is_ascii_punctuation()
                    }) {
                        return (parts_of_speech_acc, examples_acc);
                    }

                    let mut part_of_speech = def.select(&DEF_PART_OF_SPEECH_SELECTOR);
                    let part_of_speech = part_of_speech.next().or_else(|| {
                        let parent = none_return!(def.parent());
                        let header = none_return!(parent.prev_sibling());
                        let header = none_return!(ElementRef::wrap(header));
                        let part_of_speech = header.select(&DEF_PART_OF_SPEECH_SELECTOR).next();

                        part_of_speech
                    });

                    if part_of_speech.is_none() {
                        return (parts_of_speech_acc, examples_acc);
                    }

                    let part_of_speech = part_of_speech.unwrap().text().collect::<String>();

                    let examples = def.select(&DEF_EXAMPLES_SELECTOR);
                    let mut examples = examples
                        .into_iter()
                        .map(|example| example.text().collect::<String>().trim().to_string())
                        .collect::<Vec<String>>();

                    parts_of_speech_acc.insert(text.clone(), part_of_speech);
                    examples_acc
                        .entry(text)
                        .or_insert(Vec::new())
                        .append(&mut examples);

                    (parts_of_speech_acc, examples_acc)
                },
            );

    /*
        Merge two mappings into: part of speech -> Defenition { ... }

        Reason: defenitions can be duplicated by this provider, so merge their examples.
    */
    def_part_of_speech.into_iter().fold(
        HashMap::<String, Vec<Defenition>>::new(),
        |mut acc, (def, part_of_speech)| {
            let examples = def_examples.remove(&def).unwrap();

            acc.entry(part_of_speech)
                .or_insert(Vec::new())
                .push(Defenition::new(def, examples));

            acc
        },
    )
}
