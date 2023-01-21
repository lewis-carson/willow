use indextree::{Arena, NodeId};

#[derive(Debug)]
pub struct Tree<T> {
    arena: Arena<T>,
    root: NodeId,
    hash_table: std::collections::HashMap<T, NodeId>,
}

pub trait Node: std::hash::Hash + std::cmp::Eq + std::fmt::Debug {
    fn children(&self) -> Vec<Self>
    where
        Self: Sized;

    fn leaf(&self) -> bool;
}

impl<T> Tree<T>
where
    T: Node,
{
    pub fn new(r: T) -> Self {
        let mut arena = Arena::new();

        let root = arena.new_node(r);

        Self {
            arena,
            root,
            hash_table: std::collections::HashMap::new(),
        }
    }

    pub fn root(&self) -> NodeId {
        self.root
    }

    pub fn is_rendered(&self, id: NodeId) -> bool {
        id.children(&self.arena).count() > 0
    }

    pub fn rendered_children(&self, id: NodeId) -> Vec<NodeId> {
        id.children(&self.arena).collect()
    }

    fn add_child(&mut self, id: NodeId, child: T) -> NodeId {
        let c = self.arena.new_node(child);
        id.append(c, &mut self.arena);
        c
    }

    pub fn children(&mut self, id: NodeId) -> Vec<NodeId> {
        let node = self.arena.get(id).unwrap();

        if node.get().leaf() {
            return vec![];
        }

        if self.is_rendered(id) {
            return self.rendered_children(id);
        }

        for child in node.get().children() {
            // check if child is already in the tree
            if let Some(c) = self.hash_table.get(&child) {
                // if so, append the existing node to the parent
                // instead of creating a new one
                id.append(*c, &mut self.arena);
                continue;
            }

            self.add_child(id, child);
        }

        id.children(&self.arena).collect()
    }

    pub fn get(&self, id: NodeId) -> Option<&T> {
        self.arena.get(id).map(|node| node.get())
    }
}

pub type Id = indextree::NodeId;
