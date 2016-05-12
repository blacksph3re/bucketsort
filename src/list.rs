use std::mem;

struct Node<T> {
	next : Box<Node<T>>,
	content : &T
}

struct List<T> {
	first : Node<T>
}
