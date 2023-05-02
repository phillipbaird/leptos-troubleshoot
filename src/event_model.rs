#[derive(Debug, Clone)]
pub enum Event {
    CursorCreated {
        id: uuid::Uuid,
    },
    NodeCreated {
        id: uuid::Uuid,
        label: String,
        row: usize,
        col: usize,
    },
    NodeDeselected {
        cursor_id: uuid::Uuid,
        node_id: uuid::Uuid,
    },
    NodeSelected {
        cursor_id: uuid::Uuid,
        node_id: uuid::Uuid,
    },
}
