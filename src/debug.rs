use crate::node::*;


pub fn test_comp_endpoints<T: Cluster + std::fmt::Debug>(node: CompNode<T>) {
    unsafe {
        //node.push();
        match node {
            CompNode::Node(n) => {
                println!("NODE {:?} = {:?}", [node.endpoints(0), node.endpoints(1)].iter().map(|v| v.as_ref().0).collect::<Vec<_>>(), node.fold());
                println!("left");
                test_comp_endpoints(n.as_ref().child(0));
                println!("right");
                test_comp_endpoints(n.as_ref().child(1));
            }
            CompNode::Leaf(_) => {
                println!("LEAF {:?} = {:?}", [node.endpoints(0), node.endpoints(1)].iter().map(|v| v.as_ref().0).collect::<Vec<_>>(), node.fold());
            }
        }
    }
}

pub fn test_comp_print<T: Cluster + std::fmt::Debug>(node: CompNode<T>) {
    unsafe {
        match node {
            CompNode::Node(_) => {
                println!("NODE {:?} = {:?}", [node.endpoints(0), node.endpoints(1)].iter().map(|v| v.as_ref().0).collect::<Vec<_>>(), node.fold());
            }
            CompNode::Leaf(_) => {
                println!("LEAF {:?} = {:?}", [node.endpoints(0), node.endpoints(1)].iter().map(|v| v.as_ref().0).collect::<Vec<_>>(), node.fold());
            }
        }
    }
}
