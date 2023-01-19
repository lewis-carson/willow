use indextree::{Arena, NodeId};

pub struct Tree<T> {
    arena: Arena<T>,
    root: NodeId,
}

pub trait Node {
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

        Self { arena, root }
    }

    pub fn root(&self) -> NodeId {
        self.root
    }

    pub fn children(&mut self, id: NodeId) -> Vec<NodeId> {
        let node = self.arena.get(id).unwrap();

        if node.get().leaf() {
            return vec![];
        }

        if id.children(&self.arena).count() > 0 {
            return id.children(&self.arena).collect();
        }

        for child in node.get().children() {
            let c = self.arena.new_node(child);
            id.append(c, &mut self.arena);
        }

        id.children(&self.arena).collect()
    }

    pub fn get(&self, id: NodeId) -> Option<&T> {
        self.arena.get(id).map(|node| node.get())
    }
}

pub type Id = indextree::NodeId;
