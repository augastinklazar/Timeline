use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, Debug)]
pub struct Rgba32F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct PrimaryWheels {
    pub lift: (f32, f32, f32),
    pub gamma: (f32, f32, f32),
    pub gain: (f32, f32, f32),
    pub offset: (f32, f32, f32),
}

impl Default for PrimaryWheels {
    fn default() -> Self {
        Self {
            lift: (0.0, 0.0, 0.0),
            gamma: (1.0, 1.0, 1.0),
            gain: (1.0, 1.0, 1.0),
            offset: (0.0, 0.0, 0.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CorrectorNode {
    pub id: usize,
    pub wheels: PrimaryWheels,
    pub contrast: f32,
    pub pivot: f32,
    pub saturation: f32,
}

pub enum GraphNode {
    Corrector(CorrectorNode),
    // Future: ParallelMixer, LayerMixer, etc.
}

pub struct ColorGradeGraph {
    pub nodes: HashMap<usize, GraphNode>,
    pub edges: Vec<(usize, usize)>, // (from_node_id, to_node_id)
}

impl ColorGradeGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, id: usize, node: GraphNode) {
        self.nodes.insert(id, node);
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push((from, to));
    }

    /// Compiles the execution order using Kahn's algorithm for topological sorting.
    /// Returns the ordered list of node IDs to execute sequentially.
    pub fn compile_execution_order(&self) -> Result<Vec<usize>, String> {
        let mut in_degree: HashMap<usize, usize> = self.nodes.keys().map(|&k| (k, 0)).collect();
        let mut adjacency: HashMap<usize, Vec<usize>> = self.nodes.keys().map(|&k| (k, Vec::new())).collect();

        for &(from, to) in &self.edges {
            adjacency.entry(from).or_default().push(to);
            *in_degree.entry(to).or_default() += 1;
        }

        let mut queue: VecDeque<usize> = in_degree.iter()
            .filter_map(|(&k, &deg)| if deg == 0 { Some(k) } else { None })
            .collect();

        let mut order = Vec::new();

        while let Some(node) = queue.pop_front() {
            order.push(node);
            if let Some(neighbors) = adjacency.get(&node) {
                for &next in neighbors {
                    let deg = in_degree.get_mut(&next).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(next);
                    }
                }
            }
        }

        if order.len() != self.nodes.len() {
            return Err("Cycle detected in color node graph!".into());
        }

        Ok(order)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort() {
        let mut graph = ColorGradeGraph::new();

        let n1 = CorrectorNode { id: 1, wheels: PrimaryWheels::default(), contrast: 1.0, pivot: 0.5, saturation: 1.0 };
        let n2 = CorrectorNode { id: 2, wheels: PrimaryWheels::default(), contrast: 1.0, pivot: 0.5, saturation: 1.0 };
        let n3 = CorrectorNode { id: 3, wheels: PrimaryWheels::default(), contrast: 1.0, pivot: 0.5, saturation: 1.0 };

        graph.add_node(1, GraphNode::Corrector(n1));
        graph.add_node(2, GraphNode::Corrector(n2));
        graph.add_node(3, GraphNode::Corrector(n3));

        // Setup sequential execution: Node 1 -> Node 3 -> Node 2
        graph.add_edge(1, 3);
        graph.add_edge(3, 2);

        let order = graph.compile_execution_order().unwrap();
        assert_eq!(order, vec![1, 3, 2]);
    }

    #[test]
    fn test_cycle_detection() {
        let mut graph = ColorGradeGraph::new();

        let n1 = CorrectorNode { id: 1, wheels: PrimaryWheels::default(), contrast: 1.0, pivot: 0.5, saturation: 1.0 };
        let n2 = CorrectorNode { id: 2, wheels: PrimaryWheels::default(), contrast: 1.0, pivot: 0.5, saturation: 1.0 };

        graph.add_node(1, GraphNode::Corrector(n1));
        graph.add_node(2, GraphNode::Corrector(n2));

        // Create an infinite cycle
        graph.add_edge(1, 2);
        graph.add_edge(2, 1);

        assert!(graph.compile_execution_order().is_err());
    }
}
