#[macro_use]
extern crate html5ever;
//extern crate string_cache;
extern crate hierarchy;
extern crate linked_hash_set;
#[macro_use]
extern crate maplit;
extern crate tendril;

pub mod parse;
pub mod query;
mod index;
mod node;
mod walker;
mod dom;

use dom::DOM;
use query::Query;
use parse::parse;

pub fn parse_html(s: &str) -> Query {
    parse(s)
    //    let dom = parse(s);
    //    Query::new(dom)
}
