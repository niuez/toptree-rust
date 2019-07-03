use crate::node::*;
use crate::expose::*;

fn select_rake<T: Cluster, F: Fn(T, T) -> usize>(mut rake: RakeNode<T>, sel: F) -> CompNode<T> {
    unsafe {
        rake.push();
        while let RakeNode::Node(r) = rake {
            let dir = sel(r.as_ref().child(0).fold(), r.as_ref().child(1).fold());
            rake = r.as_ref().child(dir);
            rake.push();
        }
        if let RakeNode::Leaf(comp) = rake { comp }
        else { unreachable!() }
    }
}

pub fn select<T: Cluster, F: Fn(T, T) -> usize>(v : Vertex<T>, sel: F) -> (Vertex<T>, Vertex<T>) {
    let mut node = expose(v);
    unsafe {
        node.push();
        while let CompNode::Node(n) = node {
            n.as_ref().child(0).push();
            n.as_ref().child(1).push();
            if let Some(mut r) = n.as_ref().rake() { r.push(); }
            let a = n.as_ref().child(0).fold();
            let b = n.as_ref().child(1).fold();
            let r = match n.as_ref().rake() {
                Some(r) => r.fold(),
                None => T::identity(),
            };
            let dir = sel(T::rake(a.clone(), r.clone()), b);
            node = if dir == 0 {
                let dir = sel(a, r);
                if dir == 0 { n.as_ref().child(0) }
                else { select_rake(n.as_ref().rake().unwrap(), &sel) }
            }
            else {
                n.as_ref().child(1)
            };
            node.push();
        }
    }
    if let CompNode::Leaf(_) = node {
        soft_expose(node.endpoints(0), node.endpoints(1));
        (node.endpoints(0), node.endpoints(1))
    }
    else { unreachable!() }
}
