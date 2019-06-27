use std::ptr::NonNull;
use crate::node::*;
use crate::parent_dir::*;

pub fn rotate_comp<T: Cluster>(mut t: NonNull<Compress<T>>, mut x: NonNull<Compress<T>>, dir: usize) {
    unsafe {
        let y = x.as_ref().parent();
        if let Some(mut yy) = y { 
            yy.push()
        }
        let par = parent_dir_comp_guard(CompNode::Node(x));
        let rake_par = parent_dir_comp_rake(CompNode::Node(x));
        *x.as_mut().child_mut(dir ^ 1) = t.as_ref().child(dir);
        *t.as_ref().child(dir).parent_mut() = Some(ParentNode::Compress(x));
        *t.as_mut().child_mut(dir) = CompNode::Node(x);
        *x.as_mut().parent_mut() = Some(ParentNode::Compress(t));
        x.as_mut().fix();
        t.as_mut().fix();
        *t.as_mut().parent_mut() = y;
        if let Some((xdir, mut yy)) = par {
            *yy.as_mut().child_mut(xdir) = CompNode::Node(t);
            //println!("yy===========================");
            //test_comp_endpoints(CompNode::Node(yy));
            yy.as_mut().fix();
        }
        else if let Some((xdir, mut yy)) = rake_par {
            *yy.as_mut().child_mut(xdir) = RakeNode::Leaf(CompNode::Node(t));
            yy.as_mut().fix();
        }
    }
}

pub fn rotate_rake<T: Cluster>(mut t: NonNull<Rake<T>>, mut x: NonNull<Rake<T>>, dir: usize) {
    unsafe {
        let y = x.as_ref().parent();
        if let Some(mut yy) = y { yy.push() }
        let par = parent_dir_rake(RakeNode::Node(x));
        *x.as_mut().child_mut(dir ^ 1) = t.as_ref().child(dir);
        *t.as_ref().child(dir).parent_mut() = Some(ParentNode::Rake(x));
        *t.as_mut().child_mut(dir) = RakeNode::Node(x);
        *x.as_mut().parent_mut() = Some(ParentNode::Rake(t));
        x.as_mut().fix();
        t.as_mut().fix();
        *t.as_mut().parent_mut() = y;
        if let Some((xdir, mut yy)) = par {
            *yy.as_mut().child_mut(xdir) = RakeNode::Node(t);
            yy.as_mut().fix();
        }
        else if let Some(ParentNode::Compress(mut yy)) = y {
            *yy.as_mut().rake_mut() = Some(RakeNode::Node(t));
            //test_comp_print(CompNode::Node(yy));
            //test_comp_print(yy.as_ref().child(0));
            //test_comp_print(yy.as_ref().child(1));

            yy.as_mut().fix();
        }
    }
}

pub fn splay_comp<T: Cluster>(mut t: NonNull<Compress<T>>) {
    unsafe {
        t.as_mut().push();
        t.as_mut().fix();
        while let Some((_,mut q)) = parent_dir_comp(CompNode::Node(t)) {
            q.as_mut().push();
            if let Some((_, mut r)) = parent_dir_comp(CompNode::Node(q)) {
                r.as_mut().push();
                q.as_mut().push();
                t.as_mut().push();
                let qt_dir = parent_dir_comp(CompNode::Node(t)).unwrap().0;
                let rq_dir = parent_dir_comp(CompNode::Node(q)).unwrap().0;
                if rq_dir == qt_dir {
                    rotate_comp(q, r, rq_dir ^ 1);
                    rotate_comp(t, q, qt_dir ^ 1);
                }
                else {
                    rotate_comp(t, q, qt_dir ^ 1);
                    rotate_comp(t, r, rq_dir ^ 1);
                }
            }
            else {
                let qt_dir = parent_dir_comp(CompNode::Node(t)).unwrap().0;
                t.as_mut().push();
                //println!("t =====================");
                //test_comp_print(CompNode::Node(t));
                //println!("=============");
                //test_comp_endpoints(CompNode::Node(q));
                rotate_comp(t, q, qt_dir ^ 1);
            }
        }
    }
    //println!("====================== end splay =================================");
}

pub fn splay_rake<T: Cluster>(mut t: NonNull<Rake<T>>) {
    unsafe {
        t.as_mut().push();
        t.as_mut().fix();
        while let Some((_, mut q)) = parent_dir_rake(RakeNode::Node(t)) {
            q.as_mut().push();
            if let Some((_, mut r)) = parent_dir_rake(RakeNode::Node(q)) {
                r.as_mut().push();
                q.as_mut().push();
                t.as_mut().push();
                let qt_dir = parent_dir_rake(RakeNode::Node(t)).unwrap().0;
                let rq_dir = parent_dir_rake(RakeNode::Node(q)).unwrap().0;
                if rq_dir == qt_dir {
                    rotate_rake(q, r, rq_dir ^ 1);
                    rotate_rake(t, q, qt_dir ^ 1);
                }
                else {
                    rotate_rake(t, q, qt_dir ^ 1);
                    rotate_rake(t, r, rq_dir ^ 1);
                }
            }
            else {
                let qt_dir = parent_dir_rake(RakeNode::Node(t)).unwrap().0;
                t.as_mut().push();
                rotate_rake(t, q, qt_dir ^ 1);
            }
        }
    }
}
