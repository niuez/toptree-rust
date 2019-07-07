use crate::node::*;
use crate::expose::*;

fn select_rake<T: Cluster, F: Fn(T, T, T::V, T::V, T::V) -> usize>(mut rake: RakeNode<T>, sel: F, right: &mut (T, T::V, T::V)) -> CompNode<T> {
    unsafe {
        rake.push();
        while let RakeNode::Node(r) = rake {
            r.as_ref().child(0).push();
            r.as_ref().child(0).fix();
            r.as_ref().child(1).push();
            r.as_ref().child(1).fix();
            let (rf, r0, _r1) = (T::rake(r.as_ref().child(1).fold(), right.0.clone(), r.as_ref().child(1).endpoints(0).value(), right.1, r.as_ref().child(1).endpoints(1).value()), r.as_ref().child(1).endpoints(0).value(), r.as_ref().child(1).endpoints(1).value());
            let dir = sel(r.as_ref().child(0).fold(), rf, 
                          r.as_ref().child(0).endpoints(0).value(), r0, r.as_ref().child(0).endpoints(1).value());
            rake = r.as_ref().child(dir);
            *right = (T::rake(r.as_ref().child(1 - dir).fold(), right.0.clone(), r.as_ref().child(1 - dir).endpoints(0).value(), right.1, r.as_ref().child(1 - dir).endpoints(1).value()), r.as_ref().child(1 - dir).endpoints(0).value(), r.as_ref().child(1 - dir).endpoints(1).value());
            rake.push();
        }
        if let RakeNode::Leaf(comp) = rake { comp }
        else { unreachable!() }
    }
}

pub fn select<T: Cluster, F: Fn(T, T, T::V, T::V, T::V) -> usize>(v : Vertex<T>, sel: F) -> (Vertex<T>, Vertex<T>) {
    let mut node = expose(v);
    let mut left = None;
    let mut right = None;
    unsafe {
        node.push();
        while let CompNode::Node(n) = node {
            n.as_ref().child(0).push();
            n.as_ref().child(0).fix();
            n.as_ref().child(1).push();
            n.as_ref().child(1).fix();
            if let Some(mut r) = n.as_ref().rake() { r.push(); r.fix(); }
            let a = n.as_ref().child(0);
            let b = n.as_ref().child(1);
            let r = n.as_ref().rake();

            let (af, a0, a1) = match left.clone() {
                Some((lf, l0, l1)) => (T::compress(lf, a.fold(), l0, a.endpoints(1).value(), l1), l0, a.endpoints(1).value()),
                None => (a.fold(), a.endpoints(0).value(), a.endpoints(1).value()),
            };
            
            let (bf, b0, b1) = match right.clone() {
                Some((rf, _r0, r1)) => (T::compress(b.fold(), rf, b.endpoints(0).value(), r1, b.endpoints(1).value()), b.endpoints(0).value(), r1),
                None => (b.fold(), b.endpoints(0).value(), b.endpoints(1).value()),
            };
            let dir = sel(
                match r {
                    Some(r) => T::rake(af.clone(), r.fold(), a0, r.endpoints(0).value(), a1),
                    None => af.clone(),
                },
                bf.clone(),
                a0, b1, a1
            );
            node = if dir == 0 {
                let mut rbf = bf.clone();
                rbf.reverse();
                let rb0 = b1;
                let rb1 = b0;
                let (mut rf, r0, r1) = (T::rake(r.unwrap().fold(), rbf.clone(), r.unwrap().endpoints(0).value(), rb0, r.unwrap().endpoints(1).value()), r.unwrap().endpoints(0).value(), r.unwrap().endpoints(1).value());
                let dir = sel(af, rf.clone(), a0, r0, a1);
                if dir == 0 {
                    rf.reverse();
                    right = Some((rf, r1, r0));
                    n.as_ref().child(0)
                }
                else {
                    right = match left.take() {
                        Some((lf, l0, l1)) => {
                            Some((T::rake(lf, rbf, l0, rb0, l1), l0, l1))
                        }
                        None => Some((rbf, rb0, rb1)),
                    };
                    select_rake(n.as_ref().rake().unwrap(), &sel, right.as_mut().unwrap())
                }
            }
            else {
                left = Some((af, a0, a1));
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
