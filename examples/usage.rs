use std::fs;

use willowtree::Tree;

#[derive(Debug)]
struct Node {
    path: String,
}

impl Node {
    fn new(path: String) -> Self {
        Self { path }
    }
}

impl willowtree::Node for Node {
    fn children(&self) -> Vec<Self> {
        // check if path is file using fs
        // if so, return empty vec

        let children = fs::read_dir(&self.path);

        match children {
            Ok(children) => children
                .map(|file| {
                    let path = file.unwrap().path().to_str().unwrap().to_string();

                    Node::new(path)
                })
                .collect(),
            Err(_) => vec![],
        }
    }

    fn leaf(&self) -> bool {
        let md = fs::metadata(&self.path);

        match md {
            Ok(metadata) => metadata.is_file(),
            Err(_) => true,
        }
    }
}

fn walk(tree: &mut Tree<Node>, id: willowtree::Id, depth: usize) {
    if depth == 5 {
        return;
    }

    for child in &mut tree.children(id) {
        walk(tree, *child, depth + 1);
    }
}



fn main() {
    let mut tree = Tree::<Node>::new(Node::new("/".to_string()));

    let root = tree.root();

    walk(&mut tree, root, 0);
}
