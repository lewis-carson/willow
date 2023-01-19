# Lazily evaluated tree

Sometimes, you need to traverse a tree so large that you can't afford to load it all into memory. 

For example, you might have a tree of files on disk, and you want to traverse it, but you don't want to load all the files into memory. Or, you may want to write a game playing computer and want to abstract away move generation.

With `willow` you could write search algorithm that *assumes moves have already been generated*.

This is a simple crate that allows you to do that. By implementing the `Node` trait and creating a new `Tree`, you can traverse in a lazy fashion.

The tree is based on an arena data structure, making it extremely efficient to traverse.

See the examples directory for more information.

My good thanks to `arenatree` for providing the basis of this crate.