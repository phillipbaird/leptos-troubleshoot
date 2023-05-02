use leptos::*;

use crate::{
    constants::*,
    event_model::{
        events::Event,
        types::{CursorId, NodeId, NodeType},
    },
};

#[derive(Debug, Clone)]
pub struct ConnectorPoints {
    pub position: Pos,
    pub ctrl_point: Pos,
}

#[derive(Debug, Clone)]
pub struct Cursor {
    pub id: CursorId,
    pub label: ReadSignal<String>,
    pub transform: Signal<String>,
    pub top: Signal<isize>,
    pub left: Signal<isize>,
    pub selected_nodes: ReadSignal<Vec<SelectedNode>>,
    pub selection_transform: ReadSignal<Pos>,
    row: RwSignal<usize>,
    col: RwSignal<usize>,
    _set_label: WriteSignal<String>,
    set_selected_nodes: WriteSignal<Vec<SelectedNode>>,
    _set_selection_transform: WriteSignal<Pos>,
}

impl Cursor {
    pub fn new(cx: Scope, id: CursorId, label: String, row: usize, col: usize) -> Self {
        let (label, _set_label) = create_signal(cx, label);
        let row = create_rw_signal(cx, row);
        let col = create_rw_signal(cx, col);
        let transform = Signal::derive(cx, move || cell_transform(row(), col()));
        let top = Signal::derive(cx, move || cell_top(row()));
        let left = Signal::derive(cx, move || cell_left(col()));
        let (selected_nodes, set_selected_nodes) = create_signal(cx, Vec::new());
        let (selection_transform, _set_selection_transform) = create_signal(cx, Pos { x: 0, y: 0 });
        Cursor {
            id,
            label,
            transform,
            row,
            col,
            top,
            left,
            selected_nodes,
            selection_transform,
            _set_label,
            set_selected_nodes,
            _set_selection_transform,
        }
    }

    pub fn set_row_col(&self, row: usize, col: usize) {
        self.row.set(row);
        self.col.set(col);
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub label: ReadSignal<String>,
    pub node_type: NodeType,
    pub row: ReadSignal<usize>,
    pub col: ReadSignal<usize>,
    pub transform: Signal<String>,
    _set_label: WriteSignal<String>,
    set_row: WriteSignal<usize>,
    set_col: WriteSignal<usize>,
}

impl Node {
    pub fn new(
        cx: Scope,
        id: NodeId,
        label: String,
        node_type: NodeType,
        row: usize,
        col: usize,
    ) -> Self {
        let (label, _set_label) = create_signal(cx, label);
        let (row, set_row) = create_signal(cx, row);
        let (col, set_col) = create_signal(cx, col);
        let transform = Signal::derive(cx, move || node_transform(row(), col()));
        Node {
            id,
            label,
            node_type,
            row,
            col,
            transform,
            _set_label,
            set_row,
            set_col,
        }
    }

    pub fn set_row_col(&self, row: usize, col: usize) {
        self.set_row.set(row);
        self.set_col.set(col);
    }
}

#[derive(Debug, Clone)]
pub struct SelectedNode {
    pub id: NodeId,
    pub source_id: NodeId,
    pub transform: Signal<String>,
}

impl Into<SelectedNode> for &Node {
    fn into(self) -> SelectedNode {
        SelectedNode {
            id: NodeId::new(),
            source_id: self.id.clone(),
            transform: self.transform.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn to_transform(&self) -> String {
        format!("translate({},{})", self.x, self.y)
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

    pub fn evolve(&self, event: Event) {
        match event {
            Event::CursorCreated {
                id,
                label,
                row,
                col,
            } => {
                let new_cursor = Cursor::new(self.cx, id.clone(), label, row, col);
                self.set_cursors.update(|cs| cs.push(new_cursor));
            }
            Event::NodeCreated {
                id,
                label,
                node_type,
                row,
                col,
            } => {
                self.set_nodes
                    .update(|ns| ns.push(Node::new(self.cx, id, label, node_type, row, col)));
            }
            Event::NodeDeselected { cursor_id, node_id } => {
                self.with_cursor(&cursor_id, |c| {
                    c.set_selected_nodes
                        .update(|sns| sns.retain(|sn| sn.source_id != node_id));
                });
            }
            Event::NodeSelected { cursor_id, node_id } => {
                self.with_cursor(&cursor_id, |c| {
                    let selected_node = self.with_node(&node_id, |n| n.into());
                    c.set_selected_nodes
                        .update(move |sns| sns.push(selected_node))
                });
            }
        }
    }

    fn with_cursor<T>(&self, id: &CursorId, f: impl Fn(&Cursor) -> T) -> T {
        self.cursors.with_untracked(|cs| {
            cs.iter()
                .find(|c| c.id == *id)
                .map(f)
                .expect("Cursor should exist.")
        })
    }

    fn with_node<T>(&self, id: &NodeId, f: impl Fn(&Node) -> T) -> T {
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

fn cell_transform(row: usize, col: usize) -> String {
    format!("translate({},{})", cell_left(col), cell_top(row))
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
