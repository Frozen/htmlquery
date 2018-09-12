use std::collections::HashMap;
use tendril::StrTendril;
use hierarchy::Token as NodeIndex;
use node::Node;
use linked_hash_set::LinkedHashSet as HashSet;

pub fn get_classes_from_attr_value(s: &str) -> Vec<&str> {
    s.split_whitespace().filter(|x| !x.is_empty()).collect()
}

#[derive(Debug, Default)]
pub(crate) struct Index {
    /// index by node id
    ids: HashMap<StrTendril, HashSet<NodeIndex>>,

    /// index by classnames
    classnames: HashMap<StrTendril, HashSet<NodeIndex>>,

    /// index by name of tag
    tag_names: HashMap<StrTendril, HashSet<NodeIndex>>,

    // returns default value if no match
    #[doc(hidden)]
    _default: HashSet<NodeIndex>,
}

impl Index {
    pub(crate) fn new() -> Index {
        Index {
            ids: HashMap::new(),
            tag_names: HashMap::new(),
            classnames: HashMap::new(),

            _default: HashSet::new(),
        }
    }

    pub fn index_node(&mut self, node: &Node, node_index: NodeIndex) {
        if let Some(attrs) = node.get_attrs() {
            self.add_to_index_by_attributes(attrs, node_index);
        }

        if let Some(name) = node.get_name() {
            self.add_to_index_by_tag_name(name, node_index);
        }
    }

    fn add_to_index_by_attributes(
        &mut self,
        attrs: &HashMap<StrTendril, StrTendril>,
        node_index: NodeIndex,
    ) {
        if let Some(id) = attrs.get("id".as_bytes()) {
            self.ids
                .entry(id.clone())
                .or_insert_with(HashSet::new)
                .insert(node_index);
        }

        // if contains "class" in attributes
        if let Some(classes) = attrs.get("class".as_bytes()) {
            for class in get_classes_from_attr_value(&*classes) {
                self.classnames
                    .entry(class.into())
                    .or_insert_with(HashSet::new)
                    .insert(node_index);
            }
        }
    }

    fn add_to_index_by_tag_name(&mut self, name: &StrTendril, node_index: NodeIndex) {
        self.tag_names
            .entry(name.clone())
            .or_insert_with(HashSet::new)
            .insert(node_index);
    }

    pub fn search_by_class(&self, a: &StrTendril) -> &HashSet<NodeIndex> {
        self.classnames.get(a).unwrap_or(&self._default)
    }

    pub fn search_by_tag(&self, a: &StrTendril) -> &HashSet<NodeIndex> {
        self.tag_names.get(a).unwrap_or(&self._default)
    }

    pub fn search_by_id(&self, a: &StrTendril) -> &HashSet<NodeIndex> {
        self.ids.get(a).unwrap_or(&self._default)
    }
}

#[cfg(test)]
mod tests {
    use super::get_classes_from_attr_value;
    use node::Node;
    use std::collections::HashMap;
    use tendril::StrTendril;
    use super::Index;
    use linked_hash_set::LinkedHashSet as HashSet;
    use std::iter::FromIterator;

    #[test]
    fn test_parse() {
        assert_eq!(
            vec!["a", "b", "c"],
            get_classes_from_attr_value("a b     c")
        );
    }

    fn create_attr<T, E>(t: T, e: E) -> HashMap<StrTendril, StrTendril>
    where
        T: Into<StrTendril>,
        E: Into<StrTendril>,
    {
        let mut out = HashMap::new();
        out.insert(t.into(), e.into());
        out
    }

    #[test]
    fn test_index_search() {
        let a = Node::tag("a", create_attr("link", "ya.ru"));
        let body = Node::tag("body", create_attr("class", "main"));
        let div = Node::tag("div", create_attr("class", "main"));

        let mut indexer = Index::new();
        indexer.index_node(&a, 1);
        indexer.index_node(&body, 2);
        indexer.index_node(&div, 3);

        assert_eq!(
            &HashSet::from_iter(vec![2, 3]),
            indexer.search_by_class(&"main".into())
        );
        assert_eq!(
            &HashSet::from_iter(vec![3]),
            indexer.search_by_tag(&"div".into())
        );
    }

    fn get_tendril<T: Into<StrTendril>>(s: T) {
        let _b = s.into();
    }

    #[test]
    fn smoke() {
        get_tendril("a");
        let a: StrTendril = "a".into();
        get_tendril(a);
    }
}
