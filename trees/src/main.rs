use std::cell::RefCell;
use std::rc::{Rc, Weak};

// parent aware tree node with RefCell
#[derive(Debug)]
struct ParentAwareNode {
    value: i32,
    // children should not own parent
    parent: RefCell<Weak<ParentAwareNode>>,
    // parent should own children
    children: RefCell<Vec<Rc<ParentAwareNode>>>,
}

// non parent aware, but with Box and including traversal
#[derive(Debug)]
struct BoxRefNode {
    value: i32,
    children: Vec<Box<BoxRefNode>>
}

impl BoxRefNode {
    fn traverse(&self) {
        println!("traversing: {}", self.value);
        for child in self.children.iter() {
            (*child).traverse();
        }
    }
}

fn main() {
    // parent aware example from rust book
    let leaf = Rc::new(ParentAwareNode {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());    

    let branch = Rc::new(ParentAwareNode {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // non parent aware, but implemented using Box
    let mut root = BoxRefNode{
        value: 1,
        children: vec![]
    };

    root.children.push(Box::new(BoxRefNode {
        value: 2,
        children: vec![]
    }));

    root.children.push(Box::new(BoxRefNode {
        value: 3,
        children: vec![Box::new(BoxRefNode {
            value: 4,
            children: vec![]
        })]
    }));

    println!("BoxRefNode root: {:?}", root);
    root.traverse();
}
