pub struct Graph<T> {
    nodes: Vec<T>,
    edges: Vec<(T, T)>,
}

impl<T> Default for Graph<T> {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl<T> Graph<T> {
    pub fn add_node(&mut self, node: T) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: (T, T)) {
        self.edges.push(edge);
    }
}
