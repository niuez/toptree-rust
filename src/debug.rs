use crate::node::*;

pub fn test_comp_endpoints(node: CompNode) {
    unsafe {
        //node.push();
        match node {
            CompNode::Node(node) => {
                println!("NODE {:?} = {}", node.as_ref().v.iter().map(|v| v.as_ref().0).collect::<Vec<_>>(), node.as_ref().fold);
                println!("left");
                test_comp_endpoints(node.as_ref().child(0));
                println!("right");
                test_comp_endpoints(node.as_ref().child(1));
            }
            CompNode::Leaf(leaf) => {
                println!("LEAF {:?} = {}", leaf.as_ref().v.iter().map(|v| v.as_ref().0).collect::<Vec<_>>(), leaf.as_ref().val);
            }
        }
    }
}

pub fn test_comp_print(node: CompNode) {
    unsafe {
        match node {
            CompNode::Node(node) => {
                println!("NODE {:?} = {}", node.as_ref().v.iter().map(|v| v.as_ref().0).collect::<Vec<_>>(), node.as_ref().fold);
            }
            CompNode::Leaf(leaf) => {
                println!("LEAF {:?} = {}", leaf.as_ref().v.iter().map(|v| v.as_ref().0).collect::<Vec<_>>(), leaf.as_ref().val);
            }
        }
    }
}
