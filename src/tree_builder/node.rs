/// RC pointed dom
use std::cell::{Cell, RefCell};
use std::fmt;
use std::rc::{Rc, Weak};
use super::RawToken;


/// Single node
/// It keeps a strong reference to the children, and a weak handle to the 
/// parent node to prevent ownership errors.
pub struct Node {
    parent: Cell<Option<WeakHandle>>,
    children: RefCell<Vec<Handle>>,
    pub data: RefCell<RawToken>
}

/// Handle - strong reference to a node 
pub type Handle = Rc<Node>;
/// Weak handle to parent 
pub type WeakHandle = Weak<Node>;


/// For debugging 
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Node: {:?} - Children: {:?}",
            self.data.borrow(),
            self.children.borrow()
        )
    }
}

impl Node {
    /// Create a new instance
    pub fn new(data: RawToken) -> Handle {
        Rc::new(
            Node {
                parent: Cell::new(None),
                children: RefCell::new(Vec::new()),
                data: RefCell::new(data)
            }
        )
    }

    /// Add a node to the children of this node 
    pub fn append_node(parent: &Handle, child: Handle){
        let old_parent = child.parent.replace(Some(Rc::downgrade(parent)));

        assert!(old_parent.is_none());

        // insert the child to the parent
        parent.children.borrow_mut().push(child);
    }
}