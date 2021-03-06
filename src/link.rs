use std::ptr::NonNull;
use crate::node::*;
use crate::expose::*;

pub fn link<T: Cluster>(v: Vertex<T>, u: Vertex<T>, weight: T) -> NonNull<Edge<T>> {
    unsafe {
        if v.handle().is_none() && u.handle().is_none() {
            Edge::new(v, u, weight)
        }
        else {
            let nnu = u.handle();
            let nnv = v.handle();
            let mut e = Edge::new(v, u, weight);
            let mut left = match nnu {
                None => {
                    CompNode::Leaf(e)
                }
                Some(uu) => {
                    let mut uu = expose_raw(uu);
                    uu.push();
                    if uu.endpoints(1) == u {
                        uu.reverse();
                        uu.push();
                    }
                    if uu.endpoints(0) == u {
                        let mut nu = Compress::new(CompNode::Leaf(e), uu);
                        *e.as_mut().parent_mut() = Some(ParentNode::Compress(nu));
                        e.as_mut().fix();
                        *uu.parent_mut() = Some(ParentNode::Compress(nu));
                        uu.fix();
                        nu.as_mut().fix();

                        CompNode::Node(nu)
                    }
                    else {
                        let mut nu = match uu {
                            CompNode::Node(nu) => nu,
                            _ => unreachable!(),
                        };
                        let mut left_ch = nu.as_ref().child(0);
                        left_ch.push();

                        *nu.as_mut().child_mut(0) = CompNode::Leaf(e);
                        *e.as_mut().parent_mut() = Some(ParentNode::Compress(nu));
                        e.as_mut().fix();

                        let beta = nu.as_ref().rake();
                        let mut rake = match beta {
                            Some(mut b) => {
                                b.push();
                                let rake = Rake::new(b, RakeNode::Leaf(left_ch));
                                *b.parent_mut() = Some(ParentNode::Rake(rake));
                                *left_ch.parent_mut() = Some(ParentNode::Rake(rake));
                                b.fix();
                                left_ch.fix();
                                RakeNode::Node(rake)
                            }
                            None => {
                                RakeNode::Leaf(left_ch)
                            }
                        };
                        rake.fix();
                        *nu.as_mut().rake_mut() = Some(rake);
                        *rake.parent_mut() = Some(ParentNode::Compress(nu));
                        rake.fix();
                        nu.as_mut().fix();

                        CompNode::Node(nu)
                    }
                }
            };
            match nnv {
                None => {}
                Some(vv) => {
                    let mut vv = expose_raw(vv);
                    vv.push();
                    if vv.endpoints(0) == v {
                        vv.reverse();
                        vv.push();
                    }
                    if vv.endpoints(1) == v {
                        let mut top = Compress::new(vv, left);
                        *vv.parent_mut() = Some(ParentNode::Compress(top));
                        vv.fix();
                        *left.parent_mut() = Some(ParentNode::Compress(top));
                        left.fix();
                        top.as_mut().fix();
                    }
                    else {
                        let mut nv = match vv {
                            CompNode::Node(nv) => nv,
                            _ => unreachable!(),
                        };
                        let mut right_ch = nv.as_ref().child(1);
                        right_ch.reverse();
                        right_ch.push();
                        *nv.as_mut().child_mut(1) = left;
                        *left.parent_mut() = Some(ParentNode::Compress(nv));
                        left.fix();

                        let alpha = nv.as_ref().rake();
                        let mut rake = match alpha {
                            Some(mut a) => {
                                a.push();
                                let mut rake = Rake::new(a, RakeNode::Leaf(right_ch));
                                *a.parent_mut() = Some(ParentNode::Rake(rake));
                                *right_ch.parent_mut() = Some(ParentNode::Rake(rake));
                                a.fix();
                                right_ch.fix();
                                rake.as_mut().fix();

                                RakeNode::Node(rake)
                            }
                            None => {
                                RakeNode::Leaf(right_ch)
                            }
                        };
                        *nv.as_mut().rake_mut() = Some(rake);
                        *rake.parent_mut() = Some(ParentNode::Compress(nv));
                        rake.fix();
                        nv.as_mut().fix();
                    }
                }
            }
            e
        }
    }
}
