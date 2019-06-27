use std::ptr::NonNull;
use crate::parent_dir::*;

pub trait Cluster: Clone {
    fn identity() -> Self;
    fn compress(left: Self, right: Self, rake: Self) -> Self;
    fn rake(left: Self, right: Self) -> Self;
}

pub type Link<N> = Option<N>;

pub struct Vertex<T: Cluster>(pub usize, pub Option<CompNode<T>>);

pub enum CompNode<T: Cluster> {
    Node(NonNull<Compress<T>>),
    Leaf(NonNull<Edge<T>>),
}

pub enum RakeNode<T: Cluster> {
    Node(NonNull<Rake<T>>),
    Leaf(CompNode<T>),
}

pub enum ParentNode<T: Cluster> {
    Compress(NonNull<Compress<T>>),
    Rake(NonNull<Rake<T>>),
}

pub struct Edge<T: Cluster> {
    v: [NonNull<Vertex<T>>; 2],
    par: Link<ParentNode<T>>,
    me: NonNull<Edge<T>>,


    pub val: T,
}

pub struct Compress<T: Cluster> {
    ch: [CompNode<T>; 2],
    v: [NonNull<Vertex<T>>; 2],
    rake: Link<RakeNode<T>>,
    par: Link<ParentNode<T>>,
    me: NonNull<Compress<T>>,
    rev: bool,

    pub guard: bool,


    pub fold: T
}

pub struct Rake<T: Cluster> {
    ch: [RakeNode<T>; 2],
    par: Link<ParentNode<T>>,

    fold: T,
}

impl<T: Cluster> Edge<T> {
    pub fn new(v: NonNull<Vertex<T>>, u: NonNull<Vertex<T>>, val: T) -> NonNull<Edge<T>> {
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

impl<T: Cluster> Compress<T> {
    pub fn new(left: CompNode<T>, right: CompNode<T>) -> NonNull<Compress<T>> {
        unsafe {
            let mut n = NonNull::new_unchecked(Box::into_raw(Box::new(Compress {
                ch: [left, right],
                v: [NonNull::dangling(), NonNull::dangling()],
                rake: None,
                par: None,
                rev: false,
                me: NonNull::dangling(),
                guard: false,
                fold: T::identity(),
            })));
            n.as_mut().me = n;
            n.as_mut().fix();
            n
        }
    }
}

impl<T: Cluster> Rake<T> {
    pub fn new(left: RakeNode<T>, right: RakeNode<T>) -> NonNull<Rake<T>> {
        unsafe {
            let mut r = NonNull::new_unchecked(Box::into_raw(Box::new(Rake {
                ch: [left, right],
                par: None,
                fold: T::identity()
            })));
            r.as_mut().fix();
            r
        }
    }
}

pub trait TVertex<T: Cluster> {
    fn fix(&mut self);
    fn push(&mut self);
    fn reverse(&mut self);
    fn parent(&self) -> Link<ParentNode<T>>;
    fn parent_mut(&mut self) -> &mut Link<ParentNode<T>>;
}

pub trait Node<T: Cluster>: TVertex<T> {
    type Child: TVertex<T>;
    fn child(&self, dir: usize) -> Self::Child;
    fn child_mut(&mut self, dir: usize) -> &mut Self::Child;
}

impl<T: Cluster> TVertex<T> for Edge<T> {
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
    fn parent(&self) -> Link<ParentNode<T>> { self.par }
    fn parent_mut(&mut self) -> &mut Link<ParentNode<T>> { &mut self.par }
}

impl<T: Cluster> Compress<T> {
    pub fn rake(&self) -> Link<RakeNode<T>> { self.rake }
    pub fn rake_mut(&mut self) -> &mut Link<RakeNode<T>> { &mut self.rake }
}

impl<T: Cluster> TVertex<T> for Compress<T> {
    fn fix(&mut self) {
        self.push();
        self.v[0] = self.ch[0].endpoints(0);
        self.v[1] = self.ch[1].endpoints(1);
        //self.fold = self.ch[0].fold() + self.ch[1].fold();
        self.fold = T::compress(self.ch[0].fold(), self.ch[1].fold(), match self.rake {
            Some(r) => r.fold(),
            None => T::identity(),
        });
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
    fn parent(&self) -> Link<ParentNode<T>> { self.par }
    fn parent_mut(&mut self) -> &mut Link<ParentNode<T>> { &mut self.par }
}

impl<T: Cluster> Node<T> for Compress<T> {
    type Child = CompNode<T>;
    fn child(&self, dir: usize) -> Self::Child { self.ch[dir] }
    fn child_mut(&mut self, dir: usize) -> &mut Self::Child { &mut self.ch[dir] }
}

impl<T: Cluster> TVertex<T> for Rake<T> {
    fn fix(&mut self) {
        self.fold = T::rake(self.ch[0].fold(), self.ch[1].fold());
    }
    fn push(&mut self) {
    }
    fn reverse(&mut self) {
    }
    fn parent(&self) -> Link<ParentNode<T>> { self.par }
    fn parent_mut(&mut self) -> &mut Link<ParentNode<T>> { &mut self.par }
}

impl<T: Cluster> Node<T> for Rake<T> {
    type Child = RakeNode<T>;
    fn child(&self, dir: usize) -> Self::Child { self.ch[dir] }
    fn child_mut(&mut self, dir: usize) -> &mut Self::Child { &mut self.ch[dir] }
}

impl<T: Cluster> CompNode<T> {
    pub fn endpoints(&self, dir: usize) -> NonNull<Vertex<T>> {
        unsafe {
            match *self {
                CompNode::Node(node) => node.as_ref().v[dir],
                CompNode::Leaf(leaf) => leaf.as_ref().v[dir],
            }
        }
    }
    pub fn fold(&self) -> T {
        unsafe {
            match *self {
                CompNode::Node(node) => node.as_ref().fold.clone(),
                CompNode::Leaf(leaf) => leaf.as_ref().val.clone(),
            }
        }
    }
}

impl<T: Cluster> RakeNode<T> {
    pub fn fold(&self) -> T {
        unsafe {
            match *self {
                RakeNode::Node(node) => node.as_ref().fold.clone(),
                RakeNode::Leaf(leaf) => leaf.fold(),
            }
        }
    }
}

impl<T: Cluster> TVertex<T> for CompNode<T> {
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
    fn parent(&self) -> Link<ParentNode<T>> {
        unsafe {
            match *self {
                CompNode::Node(node) => node.as_ref().parent(),
                CompNode::Leaf(leaf) => leaf.as_ref().parent(),
            }
        }
    }
    fn parent_mut(&mut self) -> &mut Link<ParentNode<T>> {
        unsafe {
            match self {
                CompNode::Node(ref mut node) => node.as_mut().parent_mut(),
                CompNode::Leaf(ref mut leaf) => leaf.as_mut().parent_mut(),
            }
        }
    }
}

impl<T: Cluster> TVertex<T> for RakeNode<T> {
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
    fn parent(&self) -> Link<ParentNode<T>> {
        unsafe {
            match *self {
                RakeNode::Node(node) => node.as_ref().parent(),
                RakeNode::Leaf(leaf) => leaf.parent(),
            }
        }
    }
    fn parent_mut(&mut self) -> &mut Link<ParentNode<T>> {
        unsafe {
            match self {
                RakeNode::Node(ref mut node) => node.as_mut().parent_mut(),
                RakeNode::Leaf(ref mut leaf) => leaf.parent_mut(),
            }
        }
    }
}

impl<T: Cluster> TVertex<T> for ParentNode<T> {
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
    fn parent(&self) -> Link<ParentNode<T>> {
        unsafe {
            match *self {
                ParentNode::Compress(node) => node.as_ref().parent(),
                ParentNode::Rake(leaf) => leaf.as_ref().parent(),
            }
        }
    }
    fn parent_mut(&mut self) -> &mut Link<ParentNode<T>> {
        unsafe {
            match self {
                ParentNode::Compress(ref mut node) => node.as_mut().parent_mut(),
                ParentNode::Rake(ref mut leaf) => leaf.as_mut().parent_mut(),
            }
        }
    }
}

impl<T: Cluster> PartialEq for CompNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (CompNode::Node(a), CompNode::Node(b)) => a == b,
            (CompNode::Leaf(a), CompNode::Leaf(b)) => a == b,
            _ => false,
        }
    }
}

impl<T: Cluster> PartialEq for RakeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (RakeNode::Node(a), RakeNode::Node(b)) => a == b,
            (RakeNode::Leaf(a), RakeNode::Leaf(b)) => a == b,
            _ => false,
        }
    }
}


impl<T: Cluster> PartialEq for ParentNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (ParentNode::Compress(a), ParentNode::Compress(b)) => a == b,
            (ParentNode::Rake(a), ParentNode::Rake(b)) => a == b,
            _ => false,
        }
    }
}


impl<T: Cluster> Clone for CompNode<T> {
    fn clone(&self) -> Self {
        match *self {
            CompNode::Node(a) => CompNode::Node(a),
            CompNode::Leaf(a) => CompNode::Leaf(a),
        }
    }
}

impl<T: Cluster> Clone for RakeNode<T> {
    fn clone(&self) -> Self {
        match *self {
            RakeNode::Node(a) => RakeNode::Node(a),
            RakeNode::Leaf(a) => RakeNode::Leaf(a),
        }
    }
}

impl<T: Cluster> Clone for ParentNode<T> {
    fn clone(&self) -> Self {
        match *self {
            ParentNode::Compress(a) => ParentNode::Compress(a),
            ParentNode::Rake(a) => ParentNode::Rake(a),
        }
    }
}

impl<T: Cluster> Clone for Vertex<T> {
    fn clone(&self) -> Self {
        Vertex(self.0, self.1)
    }
}

impl<T: Cluster> Copy for CompNode<T> {}
impl<T: Cluster> Copy for RakeNode<T> {}
impl<T: Cluster> Copy for ParentNode<T> {}
impl<T: Cluster> Copy for Vertex<T> {}
