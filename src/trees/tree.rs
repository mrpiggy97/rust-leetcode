pub struct Node{
    left : Box<Node>,
    right : Box<Node>,
    val : i32
}

pub struct Tree{
    root : Node
}