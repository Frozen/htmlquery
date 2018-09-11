use super::dom::DOM;
use super::node::Node;
use hierarchy::Token as NodeIndex;
use index::Index;
use std::collections::HashSet;
use hierarchy::Hierarchy;
use tendril::StrTendril;

pub struct Query {
    h: Hierarchy<Node>,
    index: Index,
}

//
impl Query {
    pub(crate) fn new(h: Hierarchy<Node>, index: Index) -> Query {
        Query { h, index }
    }

    fn get_tags_by_conditions(&self, mut tokens: Tokens) -> Vec<NodeIndex> {
        let mut initial = {
            let token = tokens.pop().unwrap();
            match token {
                Token::Class(ref s) => self.index.search_by_class(&s),
                Token::Id(ref s) => self.index.search_by_id(&s),
                Token::Tag(ref s) => self.index.search_by_tag(&s),
            }
        };

        //        println!("get_tags_by_conditions initial {:?}", initial);

        let mut out = vec![];
        for tag_id in initial {
            if node_passes_conditions(&self.h[*tag_id], &tokens) {
                out.push(*tag_id);
            }
        }
        out
    }
}

//pub trait Queryable {
//    type Item;
//
//    fn first(s: &str) -> Option<&Self::Item>;
//    fn all(&self, s: &str) -> Result<Vec<&Node>, QueryError>;
//}

#[derive(Debug)]
pub enum QueryError {

}

impl Query {
    //    type Item = NodeIndex;

    //    fn first(s: &str) -> Option<&<Self as Queryable>::Item> {
    //        unimplemented!()
    //    }

    pub fn all(&self, s: &str) -> Result<Vec<&Node>, QueryError> {
        let mut parsed = parse_query(s);

        let last = parsed.pop().unwrap();

        let initial: Vec<NodeIndex> = self.get_tags_by_conditions(last);

        let rs = parsed
            .iter()
            .rev()
            .fold(initial, |acc: Vec<NodeIndex>, cur: &Tokens| {
                let mut out: Vec<NodeIndex> = vec![];

                for node_index in acc {
                    for i in self.h.parents(node_index) {
                        if node_passes_conditions(&self.h[i], &cur) {
                            out.push(node_index);
                            break;
                        }
                    }
                }
                out
            });

        println!("~~~~~rs {:?}", rs);

        let mut out = vec![];
        for r in rs {
            out.push(&self.h[r])
        }
        return Ok(out);
    }
}

pub struct QueryResult {
    indexes: Vec<NodeIndex>,
}

impl QueryResult {
    pub fn new(indexes: Vec<NodeIndex>) -> QueryResult {
        QueryResult { indexes }
    }
}

fn parse_query(s: &str) -> Vec<Tokens> {
    let mut out = Vec::default();

    for token in s.split_whitespace() {
        out.push(parse_token(token));
    }

    out
}

fn node_passes_conditions(node: &Node, v: &Tokens) -> bool {
    for i in v {
        let rs = match i {
            Token::Id(ref id) => node.get_attrs().and_then(|e| e.get("id".as_bytes())) == Some(id),
            Token::Class(ref class) => {
                //                node.get_attrs().and_then(|e| e.get("class".as_bytes())).and_then(|e| e.split_whitespace().find(|r| r == &class)).is_some()
                if node.get_attrs().is_none() {
                    return false;
                }
                let attrs = node.get_attrs().unwrap();
                if attrs
                    .get("class".as_bytes())
                    .and_then(|e| {
                        e.split_whitespace()
                            .find(|v| v.to_string() == class.to_string())
                    })
                    .is_none()
                {
                    return false;
                }
                true
            }
            Token::Tag(ref tag) => node.get_name() == Some(tag),
        };
        if !rs {
            return false;
        }
    }
    return true;
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
enum Token {
    Tag(StrTendril),
    Class(StrTendril),
    Id(StrTendril),
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        //        Token::Tag("aaa".into())
        println!("+++++++ {:?}", s);
        match s.chars().nth(0).unwrap() {
            '.' => Token::Class(s[1..].into()),
            '#' => Token::Id(s[1..].into()),
            _ => Token::Tag(s.into()),
        }
    }
}

//impl Token {
//    pub fn push(&mut self, c: char) {
//        match self {
//            Tag(ref mut )
//        }
//    }
//}

type Tokens = Vec<Token>;

//struct Tokens {
//    inner: Vec<String>,
//    current: Option<String>,
//}
//
//impl Tokens {
//    pub fn new() -> Tokens {
//        Tokens { inner: Vec::new() }
//    }
//
//    fn set_class(&mut self, c: char) {
//        self.inner.push("".to)
//        self.inner.push(c);
//    }
//}

fn parse_token(s: &str) -> Tokens {
    assert!(!s.is_empty());
    s.chars()
        .fold(vec![], |mut acc, x| {
            match x {
                '.' => {
                    acc.push("".to_string());
                    let len = acc.len();
                    acc[len - 1].push(x);
                }
                '#' => {
                    acc.push("".to_string());
                    let len = acc.len();
                    acc[len - 1].push(x);
                }
                _ => {
                    if acc.last().is_none() {
                        acc.push("".to_string());
                    }
                    let len = acc.len();
                    acc[len - 1].push(x);
                }
            }

            acc
        })
        .iter()
        .map(|e: &String| -> Token { e.to_string().into() })
        .collect::<Tokens>()
}

#[cfg(test)]
mod tests {

    use super::parse_token;
    use super::Token;
    use parse::parse;
    use tendril::{SliceExt, StrTendril, Tendril};

    #[test]
    fn test_parse_token() {
        let a = "abc.de#xyz";

        assert_eq!(
            vec![
                Token::Tag("abc".into()),
                Token::Class("de".into()),
                Token::Id("xyz".into()),
            ],
            parse_token(a)
        );
    }

    #[test]
    fn test_dom() {
        let html = r#"<div class="main">"#;
        let rs = parse(html);

        let q = rs.all(".main").unwrap()[0];

        assert_eq!(Some(&"div".into()), q.get_name());

        println!("q := {:?}", q);
    }

}
