use lazy_static::lazy_static;
use scraper::Selector;

lazy_static! {
    pub static ref DEF_ENTRY_SELECTOR: Selector = Selector::parse("div.pr.dsense").unwrap();
    pub static ref DEF_PART_OF_SPEECH_SELECTOR: Selector = Selector::parse("span.pos").unwrap();
    pub static ref DEF_BLOCK_SELECTOR: Selector = Selector::parse("div.def").unwrap();
    pub static ref DEF_EXAMPLES_SELECTOR: Selector =
        Selector::parse(".sense-body > .def-block .def-body .examp").unwrap();
}
