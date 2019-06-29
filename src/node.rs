use std::ptr::NonNull;
use crate::parent_dir::*;

pub trait Cluster: Clone {
    fn identity() -> Self;
    fn compress(left: Self, right: Self, rake: Self) -> Self;
    fn rake(left: Self, right: Self) -> Self;
    fn reverse(&mut self);
}

pub type Link<N> = Option<N>;

pub struct Vertex<S, T: Cluster> {
    val: S,
    handle: Option<CompNode<S, T>>
}

impl<S, T: Cluster> Vertex<S, T> {
    pub fn new(val: S) -> Self {
        Vertex {
            val: val,
            handle: None,
        }
    }
    pub fn handle(&self) -> Option<CompNode<S, T>> {
        self.handle
    }
    pub fn handle_mut(&mut self) -> &mut Option<CompNode<S, T>> {
        &mut self.handle
    }
    pub fn value(&self) -> &S {
        &self.val
    }
}

pub enum CompNode<S, T: Cluster> {
    Node(NonNull<Compress<S, T>>),
    Leaf(NonNull<Edge<S, T>>),
}

pub enum RakeNode<S, T: Cluster> {
    Node(NonNull<Rake<S, T>>),
    Leaf(CompNode<S, T>),
}

pub enum ParentNode<S, T: Cluster> {
    Compress(NonNull<Compress<S, T>>),
    Rake(NonNull<Rake<S, T>>),
}

pub struct Edge<S, T: Cluster> {
    v: [NonNull<Vertex<S, T>>; 2],
    par: Link<ParentNode<S, T>>,
    me: NonNull<Edge<S, T>>,


    pub val: T,
}

pub struct Compress<S, T: Cluster> {
    ch: [CompNode<S, T>; 2],
    v: [NonNull<Vertex<S, T>>; 2],
    rake: Link<RakeNode<S, T>>,
    par: Link<ParentNode<S, T>>,
    me: NonNull<Compress<S, T>>,
    rev: bool,

    pub guard: bool,


    pub fold: T
}

pub struct Rake<S, T: Cluster> {
    ch: [RakeNode<S, T>; 2],
    par: Link<ParentNode<S, T>>,

    fold: T,
}

impl<S, T: Cluster> Edge<S, T> {
    pub fn new(v: NonNull<Vertex<S, T>>, u: NonNull<Vertex<S, T>>, val: T) -> NonNull<Edge<S, T>> {
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

impl<S, T: Cluster> Compress<S, T> {
    pub fn new(left: CompNode<S, T>, right: CompNode<S, T>) -> NonNull<Compress<S, T>> {
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

impl<S, T: Cluster> Rake<S, T> {
    pub fn new(left: RakeNode<S, T>, right: RakeNode<S, T>) -> NonNull<Rake<S, T>> {
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

pub trait TVertex<S, T: Cluster> {
    fn fix(&mut self);
    fn push(&mut self);
    fn reverse(&mut self);
    fn parent(&self) -> Link<ParentNode<S, T>>;
    fn parent_mut(&mut self) -> &mut Link<ParentNode<S, T>>;
}

pub trait Node<S, T: Cluster>: TVertex<S, T> {
    type Child: TVertex<S, T>;
    fn child(&self, dir: usize) -> Self::Child;
    fn child_mut(&mut self, dir: usize) -> &mut Self::Child;
}

impl<S, T: Cluster> TVertex<S, T> for Edge<S, T> {
    fn fix(&mut self) {
        match self.parent() {
            Some(ParentNode::Compress(_)) => {
                if parent_dir_comp(CompNode::Leaf(self.me)).is_none() {
                    unsafe {
                        *self.v[0].as_mut().handle_mut() = Some(CompNode::Leaf(self.me));
                    }
                }
            }
            Some(ParentNode::Rake(_)) => {
                unsafe {
                    *self.v[0].as_mut().handle_mut() = Some(CompNode::Leaf(self.me));
                }
            }
            _ => {
                unsafe {
                    *self.v[0].as_mut().handle_mut() = Some(CompNode::Leaf(self.me));
                    *self.v[1].as_mut().handle_mut() = Some(CompNode::Leaf(self.me));
                }
            }
        }
    }
    fn push(&mut self) {}
    fn reverse(&mut self) {
        self.v.swap(0, 1);
        self.val.reverse();
    }
    fn parent(&self) -> Link<ParentNode<S, T>> { self.par }
    fn parent_mut(&mut self) -> &mut Link<ParentNode<S, T>> { &mut self.par }
}

impl<S, T: Cluster> Compress<S, T> {
    pub fn rake(&self) -> Link<RakeNode<S, T>> { self.rake }
    pub fn rake_mut(&mut self) -> &mut Link<RakeNode<S, T>> { &mut self.rake }
}

impl<S, T: Cluster> TVertex<S, T> for Compress<S, T> {
    fn fix(&mut self) {
        self.push();
        self.v[0] = self.ch[0].endpoints(0);
        self.v[1] = self.ch[1].endpoints(1);
        //self.fold = self.ch[0].fold() + self.ch[1].fold();
        self.fold = T::compress(self.ch[0].fold(), self.ch[1].fold(), match self.rake {
            Some(r) => r.fold(),
            None => T::identity(),
        });
        unsafe { *self.ch[0].endpoints(1).as_mut().handle_mut() = Some(CompNode::Node(self.me)); }
        assert!(self.ch[0].endpoints(1) == self.ch[1].endpoints(0));
        match self.parent() {
            Some(ParentNode::Compress(_)) => {
                if parent_dir_comp(CompNode::Node(self.me)).is_none() {
                    unsafe {
                        *self.v[0].as_mut().handle_mut() = Some(CompNode::Node(self.me));
                    }
                }
            },
            Some(ParentNode::Rake(_)) => {
                unsafe {
                    *self.v[0].as_mut().handle_mut() = Some(CompNode::Node(self.me));
                }
            }
            _ => {
                unsafe {
                    *self.v[0].as_mut().handle_mut() = Some(CompNode::Node(self.me));
                    *self.v[1].as_mut().handle_mut() = Some(CompNode::Node(self.me));
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
        self.fold.reverse();
        self.rev ^= true;
        self.push();
    }
    fn parent(&self) -> Link<ParentNode<S, T>> { self.par }
    fn parent_mut(&mut self) -> &mut Link<ParentNode<S, T>> { &mut self.par }
}

impl<S, T: Cluster> Node<S, T> for Compress<S, T> {
    type Child = CompNode<S, T>;
    fn child(&self, dir: usize) -> Self::Child { self.ch[dir] }
    fn child_mut(&mut self, dir: usize) -> &mut Self::Child { &mut self.ch[dir] }
}

impl<S, T: Cluster> TVertex<S, T> for Rake<S, T> {
    fn fix(&mut self) {
        self.push();
        self.fold = T::rake(self.ch[0].fold(), self.ch[1].fold());
    }
    fn push(&mut self) {
    }
    fn reverse(&mut self) {
        self.fold.reverse();
        self.push();
    }
    fn parent(&self) -> Link<ParentNode<S, T>> { self.par }
    fn parent_mut(&mut self) -> &mut Link<ParentNode<S, T>> { &mut self.par }
}

impl<S, T: Cluster> Node<S, T> for Rake<S, T> {
    type Child = RakeNode<S, T>;
    fn child(&self, dir: usize) -> Self::Child { self.ch[dir] }
    fn child_mut(&mut self, dir: usize) -> &mut Self::Child { &mut self.ch[dir] }
}

impl<S, T: Cluster> CompNode<S, T> {
    pub fn endpoints(&self, dir: usize) -> NonNull<Vertex<S, T>> {
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

impl<S, T: Cluster> RakeNode<S, T> {
    pub fn fold(&self) -> T {
        unsafe {
            match *self {
                RakeNode::Node(node) => node.as_ref().fold.clone(),
                RakeNode::Leaf(leaf) => leaf.fold(),
            }
        }
    }
}

impl<S, T: Cluster> TVertex<S, T> for CompNode<S, T> {
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
    fn parent(&self) -> Link<ParentNode<S, T>> {
        unsafe {
            match *self {
                CompNode::Node(node) => node.as_ref().parent(),
                CompNode::Leaf(leaf) => leaf.as_ref().parent(),
            }
        }
    }
    fn parent_mut(&mut self) -> &mut Link<ParentNode<S, T>> {
        unsafe {
            match self {
                CompNode::Node(ref mut node) => node.as_mut().parent_mut(),
                CompNode::Leaf(ref mut leaf) => leaf.as_mut().parent_mut(),
            }
        }
    }
}

impl<S, T: Cluster> TVertex<S, T> for RakeNode<S, T> {
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
    fn parent(&self) -> Link<ParentNode<S, T>> {
        unsafe {
            match *self {
                RakeNode::Node(node) => node.as_ref().parent(),
                RakeNode::Leaf(leaf) => leaf.parent(),
            }
        }
    }
    fn parent_mut(&mut self) -> &mut Link<ParentNode<S, T>> {
        unsafe {
            match self {
                RakeNode::Node(ref mut node) => node.as_mut().parent_mut(),
                RakeNode::Leaf(ref mut leaf) => leaf.parent_mut(),
            }
        }
    }
}

impl<S, T: Cluster> TVertex<S, T> for ParentNode<S, T> {
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
    fn parent(&self) -> Link<ParentNode<S, T>> {
        unsafe {
            match *self {
                ParentNode::Compress(node) => node.as_ref().parent(),
                ParentNode::Rake(leaf) => leaf.as_ref().parent(),
            }
        }
    }
    fn parent_mut(&mut self) -> &mut Link<ParentNode<S, T>> {
        unsafe {
            match self {
                ParentNode::Compress(ref mut node) => node.as_mut().parent_mut(),
                ParentNode::Rake(ref mut leaf) => leaf.as_mut().parent_mut(),
            }
        }
    }
}

impl<S, T: Cluster> PartialEq for CompNode<S, T> {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (CompNode::Node(a), CompNode::Node(b)) => a == b,
            (CompNode::Leaf(a), CompNode::Leaf(b)) => a == b,
            _ => false,
        }
    }
}

impl<S, T: Cluster> PartialEq for RakeNode<S, T> {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (RakeNode::Node(a), RakeNode::Node(b)) => a == b,
            (RakeNode::Leaf(a), RakeNode::Leaf(b)) => a == b,
            _ => false,
        }
    }
}


impl<S, T: Cluster> PartialEq for ParentNode<S, T> {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (ParentNode::Compress(a), ParentNode::Compress(b)) => a == b,
            (ParentNode::Rake(a), ParentNode::Rake(b)) => a == b,
            _ => false,
        }
    }
}


impl<S, T: Cluster> Clone for CompNode<S, T> {
    fn clone(&self) -> Self {
        match *self {
            CompNode::Node(a) => CompNode::Node(a),
            CompNode::Leaf(a) => CompNode::Leaf(a),
        }
    }
}

impl<S, T: Cluster> Clone for RakeNode<S, T> {
    fn clone(&self) -> Self {
        match *self {
            RakeNode::Node(a) => RakeNode::Node(a),
            RakeNode::Leaf(a) => RakeNode::Leaf(a),
        }
    }
}

impl<S, T: Cluster> Clone for ParentNode<S, T> {
    fn clone(&self) -> Self {
        match *self {
            ParentNode::Compress(a) => ParentNode::Compress(a),
            ParentNode::Rake(a) => ParentNode::Rake(a),
        }
    }
}

impl<S, T: Cluster> Copy for CompNode<S, T> {}
impl<S, T: Cluster> Copy for RakeNode<S, T> {}
impl<S, T: Cluster> Copy for ParentNode<S, T> {}
