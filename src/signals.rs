use leptos::*;

use crate::constants::*;

#[derive(Debug, Clone)]
pub struct Cursor {
    pub id: uuid::Uuid,
    pub selected_nodes: ReadSignal<Vec<SelectedNode>>,
    pub selection_transform: ReadSignal<Pos>,
    set_selected_nodes: WriteSignal<Vec<SelectedNode>>,
    _set_selection_transform: WriteSignal<Pos>,
}

impl Cursor {
    pub fn new(cx: Scope, id: uuid::Uuid) -> Self {
        let (selected_nodes, set_selected_nodes) = create_signal(cx, Vec::new());
        let (selection_transform, _set_selection_transform) = create_signal(cx, Pos { x: 0, y: 0 });
        Cursor {
            id,
            selected_nodes,
            selection_transform,
            set_selected_nodes,
            _set_selection_transform,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: uuid::Uuid,
    pub row: ReadSignal<usize>,
    pub col: ReadSignal<usize>,
    pub transform: Signal<String>,
    _set_row: WriteSignal<usize>,
    _set_col: WriteSignal<usize>,
}

impl Node {
    pub fn new(cx: Scope, id: uuid::Uuid, row: usize, col: usize) -> Self {
        let (row, _set_row) = create_signal(cx, row);
        let (col, _set_col) = create_signal(cx, col);
        let transform = Signal::derive(cx, move || node_transform(row(), col()));
        Node {
            id,
            row,
            col,
            transform,
            _set_row,
            _set_col,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelectedNode {
    pub id: uuid::Uuid,
    pub source_id: uuid::Uuid,
    pub transform: Signal<String>,
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

#[derive(Debug, Clone)]
pub enum Event {
    CursorCreated {
        id: uuid::Uuid,
    },
    NodeCreated {
        id: uuid::Uuid,
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
            Event::CursorCreated { id } => {
                let new_cursor = Cursor::new(self.cx, id.clone());
                self.set_cursors.update(|cs| cs.push(new_cursor));
            }
            Event::NodeCreated { id, row, col } => {
                self.set_nodes
                    .update(|ns| ns.push(Node::new(self.cx, id, row, col)));
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
