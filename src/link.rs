use std::ptr::NonNull;
use crate::node::*;
use crate::expose::*;

pub fn link(v: NonNull<Vertex>, u: NonNull<Vertex>, weight: usize) -> NonNull<Edge> {
    unsafe {
        if v.as_ref().1.is_none() && u.as_ref().1.is_none() {
            let mut e = NonNull::new_unchecked(Box::into_raw(Box::new(Edge {
                v: [v, u],
                par: None,
                val: weight,
                me: NonNull::dangling(),
            })));
            e.as_mut().me = e;
            e.as_mut().fix();

            e
        }
        else {
            let nnu = u.as_ref().1;
            let nnv = v.as_ref().1;
            let mut e = NonNull::new_unchecked(Box::into_raw(Box::new(Edge {
                v: [v, u],
                par: None,
                val: weight,
                me: NonNull::dangling(),
            })));
            e.as_mut().me = e;
            e.as_mut().fix();
            let mut left = match nnu {
                None => {
                    CompNode::Leaf(e)
                }
                Some(uu) => {
                    let mut uu = expose(uu);
                    if uu.endpoints(1) == u {
                        uu.reverse();
                        uu.push();
                    }
                    if uu.endpoints(0) == u {
                        let mut nu = NonNull::new_unchecked(Box::into_raw(Box::new(Compress {
                            ch: [CompNode::Leaf(e), uu],
                            v: [NonNull::dangling(), NonNull::dangling()],
                            rake: None,
                            par: None,
                            rev: false,
                            me: NonNull::dangling(),
                            guard: false,
                            fold: 0,
                        })));
                        nu.as_mut().me = nu;
                        nu.as_mut().fix();
                        *e.as_mut().parent_mut() = Some(ParentNode::Compress(nu));
                        e.as_mut().fix();
                        *uu.parent_mut() = Some(ParentNode::Compress(nu));
                        uu.fix();
                        CompNode::Node(nu)
                    }
                    else {
                        let mut nu = match uu {
                            CompNode::Node(nu) => nu,
                            _ => unreachable!(),
                        };
                        let mut left_ch = nu.as_ref().child(0);
                        *nu.as_mut().child_mut(0) = CompNode::Leaf(e);
                        *e.as_mut().parent_mut() = Some(ParentNode::Compress(nu));
                        e.as_mut().fix();
                        let beta = nu.as_ref().rake();
                        let mut rake = match beta {
                            Some(mut b) => {
                                let rake = NonNull::new_unchecked(Box::into_raw(Box::new(Rake {
                                    ch: [b, RakeNode::Leaf(left_ch)],
                                    par: None,
                                })));
                                *b.parent_mut() = Some(ParentNode::Rake(rake));
                                *left_ch.parent_mut() = Some(ParentNode::Rake(rake));
                                left_ch.fix();
                                RakeNode::Node(rake)
                            }
                            None => {
                                RakeNode::Leaf(left_ch)
                            }
                        };
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
                    let mut vv = expose(vv);
                    if vv.endpoints(0) == v {
                        vv.reverse();
                        vv.push();
                    }
                    if vv.endpoints(1) == v {
                        let mut top = NonNull::new_unchecked(Box::into_raw(Box::new(Compress {
                            ch: [vv, left],
                            v: [NonNull::dangling(), NonNull::dangling()],
                            rake: None,
                            par: None,
                            rev: false,
                            me: NonNull::dangling(),
                            guard: false,
                            fold: 0,
                        })));
                        *vv.parent_mut() = Some(ParentNode::Compress(top));
                        vv.fix();
                        *left.parent_mut() = Some(ParentNode::Compress(top));
                        left.fix();
                        top.as_mut().me = top;
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
                                let mut rake = NonNull::new_unchecked(Box::into_raw(Box::new(Rake {
                                    ch: [a, RakeNode::Leaf(right_ch)],
                                    par: None,
                                })));
                                *a.parent_mut() = Some(ParentNode::Rake(rake));
                                a.fix();
                                *right_ch.parent_mut() = Some(ParentNode::Rake(rake));
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
                        //test_comp_print(nv.as_ref().child(0));
                        //test_comp_print(nv.as_ref().child(1));
                        //println!("-------------");
                        nv.as_mut().fix();
                    }
                }
            }
            e
        }
    }
}
