use std::ptr::NonNull;
use crate::node::*;
use crate::expose::*;
use crate::splay::*;

fn bring<T: Cluster>(mut root: NonNull<Compress<T>>) {
    unsafe {
        match root.as_ref().rake() {
            None => {
                let mut left = root.as_ref().child(0);
                let _ = Box::from_raw(root.as_ptr());
                *left.parent_mut() = None;
                left.fix();
            }
            Some(RakeNode::Leaf(mut new_right)) => {
                new_right.reverse();
                new_right.push();

                *root.as_mut().child_mut(1) = new_right;
                *new_right.parent_mut() = Some(ParentNode::Compress(root));

                *root.as_mut().rake_mut() = None;

                new_right.fix();
                root.as_mut().fix();
            }
            Some(RakeNode::Node(mut rake)) => {
                rake.as_mut().push();
                while let RakeNode::Node(mut right) = rake.as_ref().child(1) {
                    right.as_mut().push();
                    rake = right;
                }
                root.as_mut().guard = true;
                splay_rake(rake);
                root.as_mut().guard = false;
                let mut new_rake = rake.as_ref().child(0);
                let mut new_right = if let RakeNode::Leaf(right) = rake.as_ref().child(1) {
                    right
                }
                else { unreachable!() };

                let _ = Box::from_raw(rake.as_ptr());
                
                new_right.reverse();
                new_right.push();

                *root.as_mut().child_mut(1) = new_right;
                *new_right.parent_mut() = Some(ParentNode::Compress(root));

                *root.as_mut().rake_mut() = Some(new_rake);
                *new_rake.parent_mut() = Some(ParentNode::Compress(root));

                new_rake.fix();
                new_right.fix();
                root.as_mut().fix();
            }
        }
    }
}

pub fn cut<T: Cluster>(v: Vertex<T>, u: Vertex<T>) {
    unsafe {
        soft_expose(v, u);
        let mut root = v.handle().unwrap();
        root.push();
        if let CompNode::Node(root) = root {
            let mut right = root.as_ref().child(1);
            *right.parent_mut() = None;

            right.reverse();
            right.push();

            if let CompNode::Node(right) = right {
                if let CompNode::Leaf(e) = right.as_ref().child(1) {
                    bring(right);
                    bring(root);
                    let _ = Box::from_raw(e.as_ptr());
                }
                else { unreachable!() }
            }
            else { unreachable!() }
        }
        else { unreachable!() }
    }
}
