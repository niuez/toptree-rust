use std::ptr::NonNull;
use crate::node::*;
use crate::parent_dir::*;
use crate::splay::*;

pub fn expose(mut node: CompNode) -> CompNode {
    loop {
        //println!("function expose --- node");
        //test_comp_print(node);
        //println!("endpoints ---------------------");
        /*test_comp_endpoints(
            {
                let mut nn = node;
                while let Some((_dir, par)) = parent_dir_comp(nn) {
                    nn = CompNode::Node(par);
                }
                nn
            }
            );*/
        if let CompNode::Node(comp) = node {
            splay_comp(comp);
        }
        let mut n = match node.parent() {
            None => break,
            Some(ParentNode::Rake(par)) => {
                splay_rake(par);
                //unsafe { println!("{}", par.as_ref().parent().is_none()); }
                //unsafe { println!("{}", if let Some(ParentNode::Rake(_)) = par.as_ref().parent() { true } else { false }); }
                if let Some(ParentNode::Compress(n)) = unsafe { par.as_ref().parent() } {
                    n
                }
                else { unreachable!() }
            }
            Some(ParentNode::Compress(n)) => {
                unsafe {
                    if n.as_ref().guard && parent_dir_comp_guard(node).is_some() { break }
                }
                n
            }
        };
        //println!("splay_comp_n ---------------------");
        //test_comp_endpoints(CompNode::Node(n));
        splay_comp(n);
        //println!("aaa=====");
        //test_comp_endpoints(CompNode::Node(n));
        //println!("node");
        //test_comp_print(node);
        let dir = match parent_dir_comp_guard(CompNode::Node(n)) {
            Some((dir, _)) => dir,
            None => 0,
        };
        if dir == 1 {
            unsafe {
                n.as_ref().child(dir).reverse();
                node.reverse();
            }
        }
        if let Some((n_dir, mut rake)) = parent_dir_rake(RakeNode::Leaf(node)) {
            unsafe {
                let mut nch = n.as_mut().child(dir);
                *rake.as_mut().child_mut(n_dir) = RakeNode::Leaf(nch);
                *nch.parent_mut() = Some(ParentNode::Rake(rake));
                *n.as_mut().child_mut(dir) = node;
                *node.parent_mut() = Some(ParentNode::Compress(n));
                nch.fix();
                rake.as_mut().fix();
                node.fix();
                splay_rake(rake);
                //println!("=================2===================");
                //test_comp_endpoints(CompNode::Node(n));
                n.as_mut().fix();
            }
        }
        else {
            unsafe {
                let mut nch = n.as_mut().child(dir);
                *n.as_mut().rake_mut() = Some(RakeNode::Leaf(nch));
                *nch.parent_mut() = Some(ParentNode::Compress(n));
                *n.as_mut().child_mut(dir) = node;
                *node.parent_mut() = Some(ParentNode::Compress(n));
                nch.fix();
                node.fix();
                //println!("=================1===================");
                //test_comp_endpoints(CompNode::Node(n));
                n.as_mut().fix();
            }
        }
        if let CompNode::Leaf(_) = node {
            node = CompNode::Node(n);
        }
    }
    node
}

pub fn soft_expose(v: NonNull<Vertex>, u: NonNull<Vertex>) {
    unsafe {
        let mut root = expose(v.as_ref().1.unwrap());
        if v.as_ref().1 == u.as_ref().1 { return; }

        if root.endpoints(0) == v {
            root.reverse();
            root.push();
        }
        if root.endpoints(1) == v {
            expose(u.as_ref().1.unwrap());
        }
        else if let CompNode::Node(mut r) = root {
            r.as_mut().guard = true;
            //println!("guard ---------------");
            //test_comp_print(root);
            //test_comp_print(u.as_ref().1.unwrap());
            let soot = expose(u.as_ref().1.unwrap());
            r.as_mut().guard = false;
            r.as_mut().fix();
            if parent_dir_comp(soot).unwrap().0 == 0 {
                root.reverse();
            }
        }
        else {
            unreachable!()
        }
    }
}
