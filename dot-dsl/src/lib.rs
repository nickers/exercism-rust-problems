pub mod graph {

    use std::collections::HashMap;

    pub mod graph_items {
        pub mod node {
            #[derive(PartialEq)]
            #[derive(Debug)]
            #[derive(Clone)]
            pub struct Node {
                pub name: String
            }
        }
        pub mod edge {
            #[derive(PartialEq)]
            #[derive(Debug)]
            #[derive(Clone)]
            pub struct Edge;
        }
    }

    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Graph {
        pub edges: Vec<graph_items::edge::Edge>,
        pub nodes: Vec<graph_items::node::Node>,
        pub attrs: HashMap<String, String>
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                edges: Vec::new(),
                nodes: Vec::new(),
                attrs: HashMap::new()
            }
            // unimplemented!("Construct a new Graph struct.");
        }

        pub fn with_nodes(&self, nodes: &Vec<graph_items::node::Node>) -> Self {
            let mut changed = self.clone();
            changed.nodes = nodes.to_vec();
            return changed;
        }

        pub fn with_edges(&self, _edges: &Vec<graph_items::edge::Edge>) -> Self {
            unimplemented!("y")
        }

        pub fn with_attrs(&self, _attrs: &[(&str, &str)]) -> Self {
            unimplemented!("s")
        }

        pub fn get_node(&self, name: &str) -> Option<graph_items::node::Node> {
            match self.nodes.iter().find(|s| s.name == name) {
                Some(found) => Some(found.clone()),
                _ => None
            }
        }
    }

    impl graph_items::edge::Edge {
        pub fn new(_from: &str, _to: &str) -> Self {
            unimplemented!("node");
        }

        pub fn with_attrs(&self, _attrs: &[(&str, &str)]) -> Self {
            unimplemented!("s")
        }
    }

    impl graph_items::node::Node {
        pub fn new(name: &str) -> Self {
            graph_items::node::Node {
                name: name.to_string()
            }
        }

        pub fn with_attrs(&self, _attrs: &[(&str, &str)]) -> Self {
            unimplemented!("s")
        }

        pub fn get_attr(&self, _name: &str) -> Option<&str> {
            unimplemented!("node::get_attr")
        }
    }

}
