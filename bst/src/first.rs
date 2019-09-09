// first.rs

// A link can be empty, or point to a node (note this is not public)

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>)
}

impl Link {

  pub fn insert(&mut self, number: i32) -> bool {

    // If inserting into an empty link, place the element in this link (making it no longer empty).
    // If inserting into a non-empty link:
    // return false if the element is in this node; otherwise,
    // recurse to the left if the new value is less than the node's value
    // recurse to the right if the new value is greater than the node's value
    // You may not end up needing mem::replace like in TMLL.

    return true;

  }

  pub fn search(&mut self, number: i32) -> bool {

    // If searching an empty link, return false; the element can't be found.
    // If searching a non-empty link:
    // return true if the element is in this node; otherwise,
    // recurse to the left if the target value is less than the node's value
    // recurse to the right if the target value is greater than the node's value

    return true;
  }

}

// A node contains a value, and two child links (left and right)

#[derive(Debug)]
pub struct Node {
  value: i32,
  left: Box<Node>,
  right: Box<Node>
}

// public structure BST with root of type "Link"

#[derive(Debug)]
pub struct BST {
  root: Link
}

impl BST {
  pub fn new() -> Self {
    BST { root: Link::Empty }
  }

  // insert an element into the BST, Return true if inserted, false if already present
  pub fn insert(&mut self, number: i32) -> bool {

    // If root is empty, return false

    // If we search and don't find the element, insert and return true
    //let new_node = Node {
    //  value: number,
    //};

    return true;
  }

  // search for an element in the bst. Return true iff element is found
  pub fn search(&mut self, number: i32) -> bool {
    return true;
  }

}
