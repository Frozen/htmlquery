use hierarchy::Token as NodeIndex;
use hierarchy::Hierarchy;
use super::node::Node;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use tendril::{SliceExt, StrTendril, Tendril};
use std::collections::HashMap;
use dom::DOM;
use index::Index;
use query::Query;

pub(crate) struct Walker {
    store: Hierarchy<Node>,
    indexer: Index,
}

impl Walker {
    pub(crate) fn new(store: Hierarchy<Node>, indexer: Index) -> Walker {
        Walker { store, indexer }
    }

    pub(crate) fn walk(&mut self, handle: Handle, parent: Option<NodeIndex>) {
        //        print!("{}", repeat(" ").take(indent).collect::<String>());
        //    let mut nodes = vec![];
        //        println!("handle data {:?}", handle.data);
        let rs = match handle.data {
            //            NodeData::Document => {
//                println!("#Document");
//                Some(Node::new_document())
//            }

            //        NodeData::Doctype { ref name, ref public_id, ref system_id }
//        => println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),
            NodeData::Text { ref contents } => {
                //            println!("#text: {}", escape_default(&contents.borrow()))
                Some(Node::Text(contents.borrow().clone()))
            }

            //        NodeData::Comment { ref contents }
            //        => println!("<!-- {} -->", escape_default(contents)),
            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                let mut node_attrs = HashMap::new();
                for attr in attrs.borrow().iter() {
                    node_attrs.insert((*attr.name.local).into(), (*attr.value).into());
                }
                Some(Node::tag((*name.local).to_tendril(), node_attrs))
            }

            NodeData::ProcessingInstruction { .. } => unreachable!(),
            NodeData::Document => {
                //                println!("NodeData::Document");
                None
            }
            _ => {
                //                println!("_x ");
                None
            }
        };

        //        println!("=========== rs {:?}", rs);

        if let Some(node) = rs {
            //            println!("node {:?}", &node);
            let cur_index = if let Some(parent_index) = parent {
                self.store.add_sub_node(parent_index, node.clone())
            } else {
                self.store.add_root_node(node.clone())
            };

            for child in handle.children.borrow().iter() {
                self.walk(child.clone(), Some(cur_index));
            }

            self.indexer.index_node(&node, cur_index);

        // TODO index
        //        if let Some(v) = node.attrs.get(&"class".to_tendril()) {
        //            for row in get_classes_from_attr_value(v).iter() {
        //                classes.insert(row.to_tendril(), cur_index);
        //            }
        //        }
        } else {
            for child in handle.children.borrow().iter() {
                self.walk(child.clone(), parent);
            }
        }
    }

    pub(crate) fn into_query(self) -> Query {
        Query::new(self.store, self.indexer)
    }
}
