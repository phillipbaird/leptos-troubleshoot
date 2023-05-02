use leptos::*;

use crate::constants::*;

#[derive(Debug, Clone)]
pub struct Cursor {
    pub id: uuid::Uuid,
    pub selected_nodes: ReadSignal<Vec<SelectedNode>>,
    set_selected_nodes: WriteSignal<Vec<SelectedNode>>,
}

impl Cursor {
    pub fn new(cx: Scope, id: uuid::Uuid) -> Self {
        let (selected_nodes, set_selected_nodes) = create_signal(cx, Vec::new());
        Cursor {
            id,
            selected_nodes,
            set_selected_nodes,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: uuid::Uuid,
    pub row: usize,
    pub col: usize,
    pub transform: String,
}

impl Node {
    pub fn new(id: uuid::Uuid, row: usize, col: usize) -> Self {
        let transform = node_transform(row, col);
        Node {
            id,
            row,
            col,
            transform,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelectedNode {
    pub id: uuid::Uuid,
    pub source_id: uuid::Uuid,
    pub transform: String,
}

impl Into<SelectedNode> for &Node {
    fn into(self) -> SelectedNode {
        SelectedNode {
            id: uuid::Uuid::new_v4(),
            source_id: self.id.clone(),
            transform: self.transform.clone(),
        }
    }
}

#[derive(Clone)]
pub struct WorkflowSignals {
    cx: Scope,
    pub cursors: ReadSignal<Vec<Cursor>>,
    pub nodes: ReadSignal<Vec<Node>>,
    set_cursors: WriteSignal<Vec<Cursor>>,
    set_nodes: WriteSignal<Vec<Node>>,
}

impl WorkflowSignals {
    pub fn new(cx: Scope) -> Self {
        let (cursors, set_cursors) = create_signal(cx, Vec::new());
        let (nodes, set_nodes) = create_signal(cx, Vec::new());
        WorkflowSignals {
            cx,
            cursors,
            nodes,
            set_cursors,
            set_nodes,
        }
    }

    pub fn create_cursor(&self, id: uuid::Uuid) {
        let new_cursor = Cursor::new(self.cx, id.clone());
        self.set_cursors.update(|cs| cs.push(new_cursor));
    }

    pub fn create_node(&self, id: uuid::Uuid, row: usize, col: usize) {
        self.set_nodes.update(|ns| ns.push(Node::new(id, row, col)));
    }

    pub fn deselect_node(&self, cursor_id: uuid::Uuid, node_id: uuid::Uuid) {
        self.with_cursor(&cursor_id, |c| {
            c.set_selected_nodes
                .update(|sns| sns.retain(|sn| sn.source_id != node_id));
        });
    }

    pub fn select_node(&self, cursor_id: uuid::Uuid, node_id: uuid::Uuid) {
        self.with_cursor(&cursor_id, |c| {
            let selected_node = self.with_node(&node_id, |n| n.into());
            c.set_selected_nodes
                .update(move |sns| sns.push(selected_node))
        });
    }

    fn with_cursor<T>(&self, id: &uuid::Uuid, f: impl Fn(&Cursor) -> T) -> T {
        self.cursors.with_untracked(|cs| {
            cs.iter()
                .find(|c| c.id == *id)
                .map(f)
                .expect("Cursor should exist.")
        })
    }

    fn with_node<T>(&self, id: &uuid::Uuid, f: impl Fn(&Node) -> T) -> T {
        self.nodes.with_untracked(|ns| {
            ns.iter()
                .find(|n| n.id == *id)
                .map(f)
                .expect("Node should exist.")
        })
    }
}

#[inline]
fn cell_left(col: usize) -> isize {
    col as isize * SWIMLANE_COL_WIDTH
}

#[inline]
fn cell_top(row: usize) -> isize {
    row as isize * SWIMLANE_HEIGHT
}

#[inline]
fn node_left(col: usize) -> isize {
    cell_left(col) + NODE_MARGIN_X
}

#[inline]
fn node_top(row: usize) -> isize {
    cell_top(row) + NODE_MARGIN_Y
}

fn node_transform(row: usize, col: usize) -> String {
    format!("translate({},{})", node_left(col), node_top(row))
}
