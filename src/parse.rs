use std::io;
use std::iter::repeat;
use std::default::Default;
use std::string::String;

use html5ever::parse_document;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::tendril::TendrilSink;
use html5ever::LocalName;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Cursor;
use tendril::{SliceExt, StrTendril, Tendril};
use hierarchy::Hierarchy;
use hierarchy::Token as NodeIndex;
use super::node::Node;
use super::walker::Walker;
//use dom::DOM;
use index::Index;
use query::Query;

//pub fn walk<'a>(
//    indent: usize,
//    handle: Handle,
//    store: &mut Hierarchy<Node>,
//    parent_index: usize,
//    classes: &mut HashMap<StrTendril, NodeIndex>,
//) {
//    print!("{}", repeat(" ").take(indent).collect::<String>());
//    //    let mut nodes = vec![];
//    let rs = match handle.data {
//        NodeData::Document => {
//            println!("#Document");
//            Some(Node::new_document())
//        }
//
//        //        NodeData::Doctype { ref name, ref public_id, ref system_id }
////        => println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),
//
////        NodeData::Text { ref contents }
////            => println!("#text: {}", escape_default(&contents.borrow())),
//
////        NodeData::Comment { ref contents }
////        => println!("<!-- {} -->", escape_default(contents)),
//        NodeData::Element {
//            ref name,
//            ref attrs,
//            ..
//        } => {
//            let mut node_attrs = HashMap::new();
//            for attr in attrs.borrow().iter() {
//                node_attrs.insert((*attr.name.local).into(), (*attr.value).into());
//            }
//            Some(Node::new((*name.local).to_tendril(), node_attrs))
//        }
//
//        NodeData::ProcessingInstruction { .. } => unreachable!(),
//        _ => None,
//    };
//
//    if let Some(node) = rs {
//        let cur_index = store.add_sub_node(parent_index, node.clone());
//
//        for child in handle.children.borrow().iter() {
//            walk(indent + 4, child.clone(), store, cur_index, classes);
//        }
//        // TODO index
//        //        if let Some(v) = node.attrs.get(&"class".to_tendril()) {
//        //            for row in get_classes_from_attr_value(v).iter() {
//        //                classes.insert(row.to_tendril(), cur_index);
//        //            }
//        //        }
//    }
//
//    //    DOM {
//    //        nodes: nodes,
//    //    }
//}

//impl DomIterator {
//    pub fn new(root_node: hierarchy::Index, h: Hierarchy<Node>) -> DomIterator {
//
//        let mut cur_node = root_node.clone();
//
//        while cur_node.child.borrow().len() > 0 {
//            let x = {
//                cur_node.child.borrow()[0].clone()
//            };
//            cur_node = x;
////            mem::replace(&mut cur_node, x);
////             = ;
//        }
//
//        DomIterator {
//            cur_node: cur_node,
//            root_node: root_node,
//            index: 0,
//        }
//    }
//}

pub fn parse(s: &str) -> Query {
    let stdin = io::stdin();
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut Cursor::new(s.as_bytes()))
//            .from_iter(s.bytes());
//            .
//            .read_from(&mut stdin.lock())
        .unwrap();

    let mut hierarchy = Hierarchy::new();

    //        let root_index = hierarchy.add_root_node(Node::default());

    //        let mut classes: HashMap<StrTendril, NodeIndex> = HashMap::new();

    let indexer = Index::new();
    let mut walker = Walker::new(hierarchy, indexer);

    walker.walk(dom.document, None);

    //        walk(0, dom.document, &mut hierarchy, root_index, &mut classes);

    //    walker.into_dom()

    //    index: Indexer,
    //    Query::new(hierarchy, indexer)
    walker.into_query()

    //        DOM {
    //            h: hierarchy,
    //        }
}

#[cfg(test)]
mod tests {

    use super::*;

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
