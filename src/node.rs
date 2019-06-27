use std::ptr::NonNull;
use crate::parent_dir::*;

pub type Link<N> = Option<N>;

#[derive(Clone, Copy, PartialEq)]
pub struct Vertex(pub usize, pub Option<CompNode>);

#[derive(Clone, Copy, PartialEq)]
pub enum CompNode {
    Node(NonNull<Compress>),
    Leaf(NonNull<Edge>),
}

#[derive(Clone, Copy, PartialEq)]
pub enum RakeNode {
    Node(NonNull<Rake>),
    Leaf(CompNode),
}

#[derive(Clone, Copy, PartialEq)]
pub enum ParentNode {
    Compress(NonNull<Compress>),
    Rake(NonNull<Rake>),
}

#[derive(Clone, Copy, PartialEq)]
pub struct Edge {
    v: [NonNull<Vertex>; 2],
    par: Link<ParentNode>,
    me: NonNull<Edge>,


    pub val: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Compress {
    ch: [CompNode; 2],
    v: [NonNull<Vertex>; 2],
    rake: Link<RakeNode>,
    par: Link<ParentNode>,
    me: NonNull<Compress>,
    rev: bool,

    pub guard: bool,


    pub fold: usize
}

#[derive(Clone, Copy, PartialEq)]
pub struct Rake {
    pub ch: [RakeNode; 2],
    pub par: Link<ParentNode>,
}

impl Edge {
    pub fn new(v: NonNull<Vertex>, u: NonNull<Vertex>, val: usize) -> NonNull<Edge> {
        unsafe {
            let mut e = NonNull::new_unchecked(Box::into_raw(Box::new(Edge {
                v: [v, u],
                par: None,
                val: val,
                me: NonNull::dangling(),
            })));
            e.as_mut().me = e;
            e.as_mut().fix();
            e
        }
    }
}

impl Compress {
    pub fn new(left: CompNode, right: CompNode) -> NonNull<Compress> {
        unsafe {
            let mut n = NonNull::new_unchecked(Box::into_raw(Box::new(Compress {
                ch: [left, right],
                v: [NonNull::dangling(), NonNull::dangling()],
                rake: None,
                par: None,
                rev: false,
                me: NonNull::dangling(),
                guard: false,
                fold: 0,
            })));
            n.as_mut().me = n;
            n.as_mut().fix();
            n
        }
    }
}

impl Rake {
    pub fn new(left: RakeNode, right: RakeNode) -> NonNull<Rake> {
        unsafe {
            let mut r = NonNull::new_unchecked(Box::into_raw(Box::new(Rake {
                ch: [left, right],
                par: None,
            })));
            r.as_mut().fix();
            r
        }
    }
}

pub trait TVertex {
    fn fix(&mut self);
    fn push(&mut self);
    fn reverse(&mut self);
    fn parent(&self) -> Link<ParentNode>;
    fn parent_mut(&mut self) -> &mut Link<ParentNode>;
}

pub trait Node: TVertex {
    type Child: TVertex;
    fn child(&self, dir: usize) -> Self::Child;
    fn child_mut(&mut self, dir: usize) -> &mut Self::Child;
}

impl TVertex for Edge {
    fn fix(&mut self) {
        match self.parent() {
            Some(ParentNode::Compress(_)) => {
                if parent_dir_comp(CompNode::Leaf(self.me)).is_none() {
                    unsafe {
                        self.v[0].as_mut().1 = Some(CompNode::Leaf(self.me));
                    }
                }
            }
            Some(ParentNode::Rake(_)) => {
                unsafe {
                    self.v[0].as_mut().1 = Some(CompNode::Leaf(self.me));
                }
            }
            _ => {
                unsafe {
                    self.v[0].as_mut().1 = Some(CompNode::Leaf(self.me));
                    self.v[1].as_mut().1 = Some(CompNode::Leaf(self.me));
                }
            }
        }
    }
    fn push(&mut self) {}
    fn reverse(&mut self) {
        self.v.swap(0, 1);
    }
    fn parent(&self) -> Link<ParentNode> { self.par }
    fn parent_mut(&mut self) -> &mut Link<ParentNode> { &mut self.par }
}

impl Compress {
    pub fn rake(&self) -> Link<RakeNode> { self.rake }
    pub fn rake_mut(&mut self) -> &mut Link<RakeNode> { &mut self.rake }
}

impl TVertex for Compress {
    fn fix(&mut self) {
        self.push();
        self.v[0] = self.ch[0].endpoints(0);
        self.v[1] = self.ch[1].endpoints(1);
        self.fold = self.ch[0].fold() + self.ch[1].fold();
        unsafe { self.ch[0].endpoints(1).as_mut().1 = Some(CompNode::Node(self.me)); }
        assert!(self.ch[0].endpoints(1) == self.ch[1].endpoints(0));
        match self.parent() {
            Some(ParentNode::Compress(_)) => {
                if parent_dir_comp(CompNode::Node(self.me)).is_none() {
                    unsafe {
                        self.v[0].as_mut().1 = Some(CompNode::Node(self.me));
                    }
                }
            },
            Some(ParentNode::Rake(_)) => {
                unsafe {
                    self.v[0].as_mut().1 = Some(CompNode::Node(self.me));
                }
            }
            _ => {
                unsafe {
                    self.v[0].as_mut().1 = Some(CompNode::Node(self.me));
                    self.v[1].as_mut().1 = Some(CompNode::Node(self.me));
                }
            }
        }
    }
    fn push(&mut self) {
        if self.rev {
            self.ch[0].reverse();
            self.ch[1].reverse();
            self.rev = false;
        }
    }
    fn reverse(&mut self) {
        self.ch.swap(0, 1);
        self.v.swap(0, 1);
        self.rev ^= true;
    }
    fn parent(&self) -> Link<ParentNode> { self.par }
    fn parent_mut(&mut self) -> &mut Link<ParentNode> { &mut self.par }
}

impl Node for Compress {
    type Child = CompNode;
    fn child(&self, dir: usize) -> Self::Child { self.ch[dir] }
    fn child_mut(&mut self, dir: usize) -> &mut Self::Child { &mut self.ch[dir] }
}

impl TVertex for Rake {
    fn fix(&mut self) {}
    fn push(&mut self) {
    }
    fn reverse(&mut self) {
    }
    fn parent(&self) -> Link<ParentNode> { self.par }
    fn parent_mut(&mut self) -> &mut Link<ParentNode> { &mut self.par }
}

impl Node for Rake {
    type Child = RakeNode;
    fn child(&self, dir: usize) -> Self::Child { self.ch[dir] }
    fn child_mut(&mut self, dir: usize) -> &mut Self::Child { &mut self.ch[dir] }
}

impl CompNode {
    pub fn endpoints(&self, dir: usize) -> NonNull<Vertex> {
        unsafe {
            match *self {
                CompNode::Node(node) => node.as_ref().v[dir],
                CompNode::Leaf(leaf) => leaf.as_ref().v[dir],
            }
        }
    }
    pub fn fold(&self) -> usize {
        unsafe {
            match *self {
                CompNode::Node(node) => node.as_ref().fold,
                CompNode::Leaf(leaf) => leaf.as_ref().val,
            }
        }
    }
}

impl TVertex for CompNode {
    fn fix(&mut self) {
        unsafe {
            match *self {
                CompNode::Node(mut node) => node.as_mut().fix(),
                CompNode::Leaf(mut leaf) => leaf.as_mut().fix(),
            }
        }
    }
    fn push(&mut self) {
        unsafe {
            match *self {
                CompNode::Node(mut node) => node.as_mut().push(),
                CompNode::Leaf(mut leaf) => leaf.as_mut().push(),
            }
        }
    }
    fn reverse(&mut self) {
        unsafe {
            match *self {
                CompNode::Node(mut node) => node.as_mut().reverse(),
                CompNode::Leaf(mut leaf) => leaf.as_mut().reverse(),
            }
        }
    }
    fn parent(&self) -> Link<ParentNode> {
        unsafe {
            match *self {
                CompNode::Node(node) => node.as_ref().parent(),
                CompNode::Leaf(leaf) => leaf.as_ref().parent(),
            }
        }
    }
    fn parent_mut(&mut self) -> &mut Link<ParentNode> {
        unsafe {
            match self {
                CompNode::Node(ref mut node) => node.as_mut().parent_mut(),
                CompNode::Leaf(ref mut leaf) => leaf.as_mut().parent_mut(),
            }
        }
    }
}

impl TVertex for RakeNode {
    fn fix(&mut self) {
        unsafe {
            match *self {
                RakeNode::Node(mut node) => node.as_mut().fix(),
                RakeNode::Leaf(mut leaf) => leaf.fix(),
            }
        }
    }
    fn push(&mut self) {
        unsafe {
            match *self {
                RakeNode::Node(mut node) => node.as_mut().push(),
                RakeNode::Leaf(mut leaf) => leaf.push(),
            }
        }
    }
    fn reverse(&mut self) {
        unsafe {
            match *self {
                RakeNode::Node(mut node) => node.as_mut().reverse(),
                RakeNode::Leaf(mut leaf) => leaf.reverse(),
            }
        }
    }
    fn parent(&self) -> Link<ParentNode> {
        unsafe {
            match *self {
                RakeNode::Node(node) => node.as_ref().parent(),
                RakeNode::Leaf(leaf) => leaf.parent(),
            }
        }
    }
    fn parent_mut(&mut self) -> &mut Link<ParentNode> {
        unsafe {
            match self {
                RakeNode::Node(ref mut node) => node.as_mut().parent_mut(),
                RakeNode::Leaf(ref mut leaf) => leaf.parent_mut(),
            }
        }
    }
}

impl TVertex for ParentNode {
    fn fix(&mut self) {
        unsafe {
            match *self {
                ParentNode::Compress(mut node) => node.as_mut().fix(),
                ParentNode::Rake(mut leaf) => leaf.as_mut().fix(),
            }
        }
    }
    fn push(&mut self) {
        unsafe {
            match *self {
                ParentNode::Compress(mut node) => node.as_mut().push(),
                ParentNode::Rake(mut leaf) => leaf.as_mut().push(),
            }
        }
    }
    fn reverse(&mut self) {
        unsafe {
            match *self {
                ParentNode::Compress(mut node) => node.as_mut().reverse(),
                ParentNode::Rake(mut leaf) => leaf.as_mut().reverse(),
            }
        }
    }
    fn parent(&self) -> Link<ParentNode> {
        unsafe {
            match *self {
                ParentNode::Compress(node) => node.as_ref().parent(),
                ParentNode::Rake(leaf) => leaf.as_ref().parent(),
            }
        }
    }
    fn parent_mut(&mut self) -> &mut Link<ParentNode> {
        unsafe {
            match self {
                ParentNode::Compress(ref mut node) => node.as_mut().parent_mut(),
                ParentNode::Rake(ref mut leaf) => leaf.as_mut().parent_mut(),
            }
        }
    }
}
