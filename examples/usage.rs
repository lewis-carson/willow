use willow::Tree;

struct Node {}

impl Node {
    fn new() -> Self {
        Self {}
    }
}

impl willow::Node for Node {
    fn children(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        vec![Node::new()]
    }
}

fn walk(tree: &mut Tree<Node>, id: willow::Id, depth: usize) {
    if depth == 1000 {
        return;
    }

    for child in &mut tree.children(id) {
        println!("{}: {:?}", depth, child);
        walk(tree, *child, depth + 1);
    }
}

fn main() {
    let mut tree = Tree::<Node>::new(Node::new());

    let root = tree.root();

    walk(&mut tree, root, 1);
}
