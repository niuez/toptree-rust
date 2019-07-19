use crate::node::*;
use crate::parent_dir::*;
use crate::splay::*;

pub fn expose_raw<T: Cluster>(mut node: CompNode<T>) -> CompNode<T> {
    loop {
        if let CompNode::Node(comp) = node {
            splay_comp(comp);
        }
        let mut n = match node.parent() {
            None => break,
            Some(ParentNode::Rake(mut par)) => {
                unsafe { par.as_mut().push(); }
                splay_rake(par);
                if let Some(ParentNode::Compress(n)) = unsafe { par.as_ref().parent() } {
                    n
                }
                else { unreachable!() }
            }
            Some(ParentNode::Compress(mut n)) => {
                unsafe { n.as_mut().push(); }
                unsafe {
                    if n.as_ref().guard && parent_dir_comp_guard(node).is_some() { break }
                }
                n
            }
        };

        splay_comp(n);

        let dir = match parent_dir_comp_guard(CompNode::Node(n)) {
            Some((dir, _)) => dir,
            None => 0,
        };
        if dir == 1 {
            unsafe {
                n.as_ref().child(dir).reverse();
                n.as_ref().child(dir).push();
                node.reverse();
                node.push();
            }
        }
        if let Some((n_dir, mut rake)) = parent_dir_rake(RakeNode::Leaf(node)) {
            unsafe {
                let mut nch = n.as_mut().child(dir);
                nch.push();
                rake.as_mut().push();

                *rake.as_mut().child_mut(n_dir) = RakeNode::Leaf(nch);
                *nch.parent_mut() = Some(ParentNode::Rake(rake));
                *n.as_mut().child_mut(dir) = node;
                *node.parent_mut() = Some(ParentNode::Compress(n));

                nch.fix();
                rake.as_mut().fix();
                node.fix();
                n.as_mut().fix();
                splay_rake(rake);
            }
        }
        else {
            unsafe {
                let mut nch = n.as_mut().child(dir);
                nch.push();

                *n.as_mut().rake_mut() = Some(RakeNode::Leaf(nch));
                *nch.parent_mut() = Some(ParentNode::Compress(n));
                *n.as_mut().child_mut(dir) = node;
                *node.parent_mut() = Some(ParentNode::Compress(n));

                nch.fix();
                node.fix();
                n.as_mut().fix();
            }
        }
        if let CompNode::Leaf(_) = node {
            node = CompNode::Node(n);
        }
    }
    node
}

pub fn expose<T: Cluster>(ver: Vertex<T>) -> CompNode<T> {
    expose_raw(ver.handle().unwrap())
}

pub fn soft_expose<T: Cluster>(v: Vertex<T>, u: Vertex<T>) {
    unsafe {
        let mut root = expose(v);
        if v.handle() == u.handle() {
            if root.endpoints(1) == v || root.endpoints(0) == u {
                root.reverse();
                root.push();
            }
            return;
        }
        if let CompNode::Node(mut root) = root {
            root.as_mut().guard = true;
            let mut soot = expose(u);
            root.as_mut().guard = false;
            soot.push();
            root.as_mut().fix();
            if let Some((0, _)) = parent_dir_comp(soot) {
                root.as_mut().reverse();
                root.as_mut().push();
            }
        }
    }
}
