pub use crate::components::handle::Handle;
pub use crate::types::{handle::HandleType, position::Position};
use gpui::Point;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Node<D> {
    pub id: Uuid,
    pub position: Point<f32>,
    pub data: D,
    pub handles: Vec<Handle>,
    pub node_type: String,
}

impl<D> Node<D> {
    pub fn new(data: D, position: Point<f32>) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            data,
            handles: Vec::new(),
            node_type: "default".to_string(),
        }
    }

    pub fn with_handles(mut self, handles: Vec<Handle>) -> Self {
        self.handles = handles;
        self
    }

    pub fn with_type(mut self, node_type: impl Into<String>) -> Self {
        self.node_type = node_type.into();
        self
    }
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub id: Uuid,
    pub source_id: Uuid,
    pub source_handle_id: Option<String>,
    pub target_id: Uuid,
    pub target_handle_id: Option<String>,
}

impl Edge {
    pub fn new(source_id: Uuid, target_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_id,
            source_handle_id: None,
            target_id,
            target_handle_id: None,
        }
    }

    pub fn with_handles(
        mut self,
        source_handle: impl Into<String>,
        target_handle: impl Into<String>,
    ) -> Self {
        self.source_handle_id = Some(source_handle.into());
        self.target_handle_id = Some(target_handle.into());
        self
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
