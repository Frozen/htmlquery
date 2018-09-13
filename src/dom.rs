use super::node::Node;
use hierarchy::Token as NodeIndex;
use index::Index;
use hierarchy::Hierarchy;
use tendril::StrTendril;

pub struct Dom {
    h: Hierarchy<Node>,
    index: Index,
}

//
impl Dom {
    pub(crate) fn new(h: Hierarchy<Node>, index: Index) -> Dom {
        Dom { h, index }
    }

    fn get_tags_by_conditions(&self, mut tokens: Tokens) -> Vec<NodeIndex> {
        let initial = {
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

impl Dom {
    pub fn first(&self, s: &str) -> Option<&Node> {
        self.all(s).iter().map(|e| *e).nth(0)
    }

    pub fn all(&self, s: &str) -> Vec<&Node> {
        let mut parsed = parse_query(s);

        // empty string
        if parsed.is_empty() {
            return vec![];
        }

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

        let mut out = vec![];
        for r in rs {
            out.push(&self.h[r])
        }
        out
    }
}

fn parse_query(s: &str) -> Vec<Tokens> {
    let mut out = Vec::default();

    for token in s.split_whitespace() {
        out.push(parse_token(token));
    }

    out
}

fn node_passes_conditions(node: &Node, v: &[Token]) -> bool {
    for i in v {
        let rs = match i {
            Token::Id(ref id) => node.get_attrs().and_then(|e| e.get("id".as_bytes())) == Some(id),
            Token::Class(ref class) => node.has_class(class),
            Token::Tag(ref tag) => node.get_name() == Some(tag),
        };
        if !rs {
            return false;
        }
    }
    true
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
enum Token {
    Tag(StrTendril),
    Class(StrTendril),
    Id(StrTendril),
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        match s.chars().nth(0).unwrap() {
            '.' => Token::Class(s[1..].into()),
            '#' => Token::Id(s[1..].into()),
            _ => Token::Tag(s.into()),
        }
    }
}

type Tokens = Vec<Token>;

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

        let q = rs.all(".main")[0];

        assert_eq!(Some(&"div".into()), q.get_name());
    }

}
