use crate::types::{handle::HandleType, position::Position};

#[derive(Clone, Debug)]
pub struct Handle {
    pub id: String,
    pub handle_type: HandleType,
    pub position: Position,
    pub offset: Option<f32>, // Percentage or pixel offset from center? Let's say 0.0 to 1.0 along the edge? Or just rely on flex/absolute positioning?
                             // For now, let's stick to simple positioning logic in render.
                             // Actually, the user might want multiple handles on one side.
                             // Let's add an index or just rely on the order in the vector?
                             // Let's keep it simple: The Node struct holds the handles.
}

impl Handle {
    pub fn new(id: impl Into<String>, handle_type: HandleType, position: Position) -> Self {
        Self {
            id: id.into(),
            handle_type,
            position,
            offset: None,
        }
    }
}
