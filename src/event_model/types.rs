/// We will eventually have multiple users working on the same model.
/// Therefore we will need to identify individual cursors across the network.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CursorId(uuid::Uuid);

impl CursorId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        CursorId(uuid::Uuid::new_v4())
    }

    pub fn uuid(&self) -> uuid::Uuid {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(uuid::Uuid);

impl NodeId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        NodeId(uuid::Uuid::new_v4())
    }

    pub fn uuid(&self) -> uuid::Uuid {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    Role,
    Command,
    Event,
    View,
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_string: &'static str = match self {
            NodeType::Role => "Role",
            NodeType::Command => "Command",
            NodeType::Event => "Event",
            NodeType::View => "View",
        };
        write!(f, "{as_string}")
    }
}
