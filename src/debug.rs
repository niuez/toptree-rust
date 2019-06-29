use crate::node::*;


pub fn test_comp_endpoints<S: Copy + std::fmt::Debug, T: Cluster + std::fmt::Debug>(node: CompNode<S, T>) {
    unsafe {
        //node.push();
        match node {
            CompNode::Node(n) => {
                println!("NODE {:?} = {:?}", [node.endpoints(0), node.endpoints(1)].iter().map(|v| v.as_ref().value()).collect::<Vec<_>>(), node.fold());
                println!("left");
                test_comp_endpoints(n.as_ref().child(0));
                println!("right");
                test_comp_endpoints(n.as_ref().child(1));
            }
            CompNode::Leaf(_) => {
                println!("LEAF {:?} = {:?}", [node.endpoints(0), node.endpoints(1)].iter().map(|v| v.as_ref().value()).collect::<Vec<_>>(), node.fold());
            }
        }
    }
}

pub fn test_comp_set<S: Copy + std::fmt::Debug, T: Cluster + std::fmt::Debug>(mut node: CompNode<S, T>) {
    unsafe {
        node.push();
        match node {
            CompNode::Node(n) => {
                println!("NODE {:?} = {:?}", [node.endpoints(0), node.endpoints(1)].iter().map(|v| v.as_ref().value()).collect::<Vec<_>>(), node.fold());
                println!("left");
                test_comp_print(n.as_ref().child(0));
                println!("right");
                test_comp_print(n.as_ref().child(1));
            }
            CompNode::Leaf(_) => {
                println!("LEAF {:?} = {:?}", [node.endpoints(0), node.endpoints(1)].iter().map(|v| v.as_ref().value()).collect::<Vec<_>>(), node.fold());
            }
        }
    }
}

pub fn test_comp_print<S: Copy + std::fmt::Debug, T: Cluster + std::fmt::Debug>(node: CompNode<S, T>) {
    unsafe {
        match node {
            CompNode::Node(_) => {
                println!("NODE {:?} = {:?}", [node.endpoints(0), node.endpoints(1)].iter().map(|v| v.as_ref().value()).collect::<Vec<_>>(), node.fold());
            }
            CompNode::Leaf(_) => {
                println!("LEAF {:?} = {:?}", [node.endpoints(0), node.endpoints(1)].iter().map(|v| v.as_ref().value()).collect::<Vec<_>>(), node.fold());
            }
        }
    }
}
