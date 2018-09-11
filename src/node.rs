use tendril::{SliceExt, StrTendril, Tendril};
use std::collections::HashMap;

/// node attributes like ```class="body"```
type Attributes = HashMap<StrTendril, StrTendril>;

//static defaultAttrs: Attributes = Attributes::default();

/// enum that represents dom node
/// it may be tag node or text node
#[derive(Debug, Clone)]
pub enum Node {
    Tag { name: StrTendril, attrs: Attributes },
    Text(StrTendril),
}

impl Node {
    pub fn tag(name: StrTendril, attrs: Attributes) -> Node {
        Node::Tag { name, attrs }
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
        return self.get_attrs()
            .and_then(|e| e.get(name.as_bytes()))
            .map(|e| e);
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
