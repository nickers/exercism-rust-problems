pub mod graph {

    use std::collections::HashMap;

    pub mod graph_items {
        pub mod node {
            #[derive(PartialEq)]
            #[derive(Debug)]
            pub struct Node;
        }
        pub mod edge {
            #[derive(PartialEq)]
            #[derive(Debug)]
            pub struct Edge;
        }
    }

    #[derive(Debug)]
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

        pub fn with_nodes(&self, _nodes: &Vec<graph_items::node::Node>) -> Self {
            unimplemented!("x")
        }

        pub fn with_edges(&self, _edges: &Vec<graph_items::edge::Edge>) -> Self {
            unimplemented!("y")
        }

        pub fn with_attrs(&self, _attrs: &[(&str, &str)]) -> Self {
            unimplemented!("s")
        }

        pub fn get_node(&self, _name: &str) -> Result<graph_items::node::Node, &str> {
            unimplemented!("")
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
        pub fn new(_name: &str) -> Self {
            unimplemented!("node");
        }

        pub fn with_attrs(&self, _attrs: &[(&str, &str)]) -> Self {
            unimplemented!("s")
        }

        pub fn get_attr(&self, _name: &str) -> Option<&str> {
            unimplemented!("node::get_attr")
        }
    }

}
