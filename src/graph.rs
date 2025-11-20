use uuid::Uuid;
use gpui::Point;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HandleType {
    Source,
    Target,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HandlePosition {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub struct Handle {
    pub id: String,
    pub handle_type: HandleType,
    pub position: HandlePosition,
    pub offset: Option<f32>, // Percentage or pixel offset from center? Let's say 0.0 to 1.0 along the edge? Or just rely on flex/absolute positioning?
                             // For now, let's stick to simple positioning logic in render.
                             // Actually, the user might want multiple handles on one side.
                             // Let's add an index or just rely on the order in the vector?
                             // Let's keep it simple: The Node struct holds the handles.
}

impl Handle {
    pub fn new(id: impl Into<String>, handle_type: HandleType, position: HandlePosition) -> Self {
        Self {
            id: id.into(),
            handle_type,
            position,
            offset: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node<D> {
    pub id: Uuid,
    pub position: Point<f32>,
    pub data: D,
    pub handles: Vec<Handle>,
}

impl<D> Node<D> {
    pub fn new(data: D, position: Point<f32>) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            data,
            handles: Vec::new(),
        }
    }

    pub fn with_handles(mut self, handles: Vec<Handle>) -> Self {
        self.handles = handles;
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

    pub fn with_handles(mut self, source_handle: impl Into<String>, target_handle: impl Into<String>) -> Self {
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
