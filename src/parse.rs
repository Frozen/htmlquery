use std::default::Default;
use html5ever::parse_document;
use html5ever::rcdom::RcDom;
use html5ever::tendril::TendrilSink;
use std::io::Cursor;
use hierarchy::Hierarchy;
use super::walker::Walker;
use index::Index;
use dom::Dom;

pub fn parse(s: &str) -> Dom {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut Cursor::new(s.as_bytes()))
        .unwrap();

    let indexer = Index::new();
    let mut walker = Walker::new(Hierarchy::new(), indexer);

    walker.walk(dom.document, None);
    walker.into_query()
}

#[cfg(test)]
mod tests {

    use tendril::{SliceExt, StrTendril};

    #[test]
    fn test_tendril_equal() {
        let short1: StrTendril = "a123bc".into();
        let short2: StrTendril = "a123bc".to_tendril();
        assert_eq!(short1, short2);

        let long1: StrTendril = "a123bc111111111111111111".into();
        let long2: StrTendril = "a123bc111111111111111111".to_tendril();
        assert_eq!(long1, long2);
    }
}
