use hierarchy::Token as NodeIndex;
use hierarchy::Hierarchy;
use super::node::Node;
use html5ever::rcdom::{Handle, NodeData};
use tendril::SliceExt;
use std::collections::HashMap;
use index::Index;
use dom::Dom;

pub(crate) struct Walker {
    store: Hierarchy<Node>,
    indexer: Index,
}

impl Walker {
    pub(crate) fn new(store: Hierarchy<Node>, indexer: Index) -> Walker {
        Walker { store, indexer }
    }

    pub(crate) fn walk(&mut self, handle: Handle, parent: Option<NodeIndex>) {
        let rs = match handle.data {
            NodeData::Text { ref contents } => Some(Node::Text(contents.borrow().clone())),
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
            NodeData::Document => None,
            _ => None,
        };

        if let Some(node) = rs {
            let cur_index = if let Some(parent_index) = parent {
                self.store.add_sub_node(parent_index, node.clone())
            } else {
                self.store.add_root_node(node.clone())
            };

            self.indexer.index_node(&node, cur_index);

            for child in handle.children.borrow().iter() {
                self.walk(child.clone(), Some(cur_index));
            }
        } else {
            for child in handle.children.borrow().iter() {
                self.walk(child.clone(), parent);
            }
        }
    }

    pub(crate) fn into_query(self) -> Dom {
        Dom::new(self.store, self.indexer)
    }
}
