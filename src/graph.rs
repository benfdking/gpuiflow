use uuid::Uuid;
use gpui::Point;

#[derive(Clone, Debug)]
pub struct Node<D> {
    pub id: Uuid,
    pub position: Point<f32>,
    pub data: D,
}

impl<D> Node<D> {
    pub fn new(data: D, position: Point<f32>) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            data,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub id: Uuid,
    pub source_id: Uuid,
    pub target_id: Uuid,
}

impl Edge {
    pub fn new(source_id: Uuid, target_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_id,
            target_id,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Graph<D> {
    pub nodes: Vec<Node<D>>,
    pub edges: Vec<Edge>,
}

impl<D> Graph<D> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node<D>) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn get_node(&self, id: Uuid) -> Option<&Node<D>> {
        self.nodes.iter().find(|n| n.id == id)
    }
}
