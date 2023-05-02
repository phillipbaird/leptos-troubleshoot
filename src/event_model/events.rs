use super::types::*;

#[derive(Debug, Clone)]
pub enum Event {
    CursorCreated {
        id: CursorId,
        label: String,
        row: usize,
        col: usize,
    },
    NodeCreated {
        id: NodeId,
        label: String,
        node_type: NodeType,
        row: usize,
        col: usize,
    },
    NodeDeselected {
        cursor_id: CursorId,
        node_id: NodeId,
    },
    NodeSelected {
        cursor_id: CursorId,
        node_id: NodeId,
    },
}
