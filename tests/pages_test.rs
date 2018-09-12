extern crate file;
extern crate htmlquery;
extern crate tendril;

use htmlquery::parse::parse;

fn get_file(name: &str) -> String {
    file::get_text(name).unwrap()
}

#[test]
fn test_pages() {
    let parsed = parse(&get_file("./tests/data/rustbook_index.html"));

    let active = parsed.all("a.active");
    assert_eq!(1, active.len());
    assert_eq!(Some("a"), active[0].get_name().map(|e| &**e));

    let script_tags = parsed.all("script");
    assert_eq!(11, script_tags.len());
}

#[test]
fn test_html1() {
    let parsed = parse(&get_file("./tests/data/1.html"));

    let active = parsed.all(".tags a");

    assert_eq!(5, active.len());
    assert_eq!(Some(&"a".into()), active[0].get_attr("href"));
}

#[test]
fn test_html2() {
    let parsed = parse(&get_file("./tests/data/2.html"));

    let a_tags = parsed.all(".abc a");

    assert_eq!(1, a_tags.len());
    assert_eq!(
        Some("a".to_string()),
        a_tags[0].get_attr("href").map(|e| e.into())
    );
}

#[test]
fn test_multiple_classes() {
    let parsed = parse(&get_file("./tests/data/multiple_classes.html"));

    let active = parsed.all(".main.first a");

    assert_eq!(1, active.len());
    assert_eq!(Some(&"a".into()), active[0].get_attr("href"));
}

#[test]
fn test_empty_query() {
    let parsed = parse(&get_file("./tests/data/multiple_classes.html"));

    let active = parsed.all("");
    assert_eq!(0, active.len());
    assert_eq!(None, parsed.first(""));
}
