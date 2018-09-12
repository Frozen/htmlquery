use tendril::StrTendril;
use std::collections::HashMap;
use tendril::SliceExt;

/// node attributes like ```class="body"```
type Attributes = HashMap<StrTendril, StrTendril>;

/// enum that represents dom node
/// it may be tag node or text node
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Tag { name: StrTendril, attrs: Attributes },
    Text(StrTendril),
}

impl Node {
    pub fn tag<T: Into<StrTendril>>(name: T, attrs: Attributes) -> Node {
        Node::Tag {
            name: name.into(),
            attrs,
        }
    }

    pub fn text<T>(text: T) -> Node
    where
        T: Into<StrTendril>,
    {
        Node::Text(text.into())
    }

    pub fn get_attrs(&self) -> Option<&Attributes> {
        match self {
            Node::Tag { attrs, .. } => Some(attrs),
            _ => None,
        }
    }

    pub fn get_attr(&self, name: &str) -> Option<&StrTendril> {
        self.get_attrs().and_then(|e| e.get(name.as_bytes()))
    }

    pub fn get_classes(&self) -> Vec<StrTendril> {
        if let Some(s) = self.get_attr("class") {
            return s.split_whitespace()
                .map(|e| e.to_tendril())
                .filter(|x| !x.is_empty())
                .collect();
        }
        vec![]
    }

    pub fn get_id(&self) -> Option<&StrTendril> {
        self.get_attr("id")
    }

    pub fn has_class(&self, s: &str) -> bool {
        self.get_classes().contains(&s.to_tendril())
    }

    pub fn get_name(&self) -> Option<&StrTendril> {
        match self {
            Node::Tag { name, .. } => Some(name),
            _ => None,
        }
    }

    //    pub fn get_id(&self) -> Option<&StrTendril> {
    //        match self {
    //            Mode::Tag { id, .. } => Some(id),
    //            _ => None,
    //        }
    //    }
}

#[cfg(test)]
mod tests {
    use super::Node;
    use tendril::SliceExt;

    #[test]
    fn test_node_methods() {
        let node = Node::tag("bla", hashmap!["class".to_tendril() => "main".to_tendril()]);

        assert_eq!(None, node.get_id());
        assert_eq!(true, node.has_class("main"));
    }

}
