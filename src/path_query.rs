use std::ptr::NonNull;
use crate::node::*;
use crate::expose::*;

pub fn path_query<T: Clone + Default>(v: NonNull<Vertex<T>>, u: NonNull<Vertex<T>>) -> T {
    unsafe {
        soft_expose(v, u);
        let mut root = v.as_ref().1.unwrap();
        root.push();
        //test_comp_endpoints(root);
        //println!("root = {}, {}", root.endpoints(0).as_ref().0, root.endpoints(1).as_ref().0);
        if root.endpoints(0) == v && root.endpoints(1) == u {
            root.fold()
        }
        else if root.endpoints(0) == v {
            if let CompNode::Node(mut n) = root {
                n.as_mut().push();
                n.as_ref().child(0).fold()
            }
            else { unreachable!() }
        }
        else if root.endpoints(1) == u {
            if let CompNode::Node(mut n) = root {
                n.as_mut().push();
                n.as_ref().child(1).fold()
            }
            else { unreachable!() }
        }
        else {
            if let CompNode::Node(mut n) = root {
                n.as_mut().push();
                if let CompNode::Node(mut n2) = n.as_ref().child(1) {
                    n2.as_mut().push();
                    n2.as_ref().child(0).fold()
                }
                else { unreachable!() }
            }
            else { unreachable!() }
        }
    }
}
