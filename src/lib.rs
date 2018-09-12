extern crate hierarchy;
extern crate html5ever;
extern crate linked_hash_set;

#[cfg(test)]
#[macro_use]
extern crate maplit;
extern crate tendril;

pub mod parse;
mod dom;
mod index;
mod node;
mod walker;
mod search;
use dom::Dom;
use parse::parse;

pub fn parse_html(s: &str) -> Dom {
    parse(s)
}
