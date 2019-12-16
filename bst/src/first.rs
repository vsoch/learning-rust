// first.rs

// A link can be empty, or point to a node (note this is not public)

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>)
}

impl Link {

  pub fn insert(&mut self, number: i32) -> bool {

    match self {

      // Insert into empty link, create new node.
      Link::Empty => {
          let mut node = Link::More(Box::new(Node {value: number, left:Empty, right:Empty}));
          self = Link::More(node);
          return true;
      }

      Link::More(ref mut node) => {

        // return false if the element is in this node
        if node.value == number {
          return false;
        }
 
        // recurse to the left if the new value is less than the node's value
        if number < node.value {
          return node.left.insert(number);
        }

        // recurse to the right if the new value is greater than the node's value
        return node.right.insert(number);
    }
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

    match self.root {

      // If root is empty, set number as root.
      Link::Empty => {
        let root = Box::new(Node {value: number, left: Empty, right: Empty});
        self.root = Link::More(root);
        return true
      }
    }

    // Return the result of inserting the number into the node
    return self.root.insert(number)

  }

  // search for an element in the bst. Return true iff element is found
  pub fn search(&mut self, number: i32) -> bool {
    return true;
  }

}
