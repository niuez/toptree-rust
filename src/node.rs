use std::ptr::NonNull;
use crate::parent_dir::*;
use crate::link::*;
use crate::expose::*;

pub trait Cluster: Clone {
    type V: Default + Copy + std::fmt::Debug;
    fn identity() -> Self;
    fn compress(left: Self, right: Self, a: Self::V, b: Self::V, c: Self::V) -> Self;
    fn rake(left: Self, right: Self, a: Self::V, b: Self::V, c: Self::V) -> Self;
    fn reverse(&mut self);
}

pub type Link<N> = Option<N>;

pub struct VertexRaw<T: Cluster> {
    val: T::V,
    handle: Option<CompNode<T>>
}

impl<T: Cluster> VertexRaw<T> {
    pub fn new(val: T::V) -> Self {
        VertexRaw {
            val: val,
            handle: None,
        }
    }
    pub fn dummy() -> Self {
        VertexRaw {
            val: T::V::default(),
            handle: None,
        }
    }
    pub fn handle(&self) -> Option<CompNode<T>> {
        self.handle
    }
    pub fn handle_mut(&mut self) -> &mut Option<CompNode<T>> {
        &mut self.handle
    }
    pub fn value(&self) -> T::V {
        self.val
    }
    pub fn value_set(&mut self, val: T::V) {
        let mut root = expose_raw(self.handle().unwrap());
        self.val = val;
        root.fix();
    }
}

pub struct Vertex<T: Cluster> {
    vertex: NonNull<VertexRaw<T>>,
}

impl<T: Cluster> Vertex<T> {
    pub fn dangling() -> Self {
        Vertex { vertex: NonNull::dangling() }
    }
    pub fn new(val: T::V) -> Self {
        unsafe {
            let v = Vertex { vertex: NonNull::new_unchecked(Box::into_raw(Box::new(VertexRaw::new(val)))) };
            let dummy = Vertex { vertex: NonNull::new_unchecked(Box::into_raw(Box::new(VertexRaw::dummy()))) };
            link(v, dummy, T::identity());
            v
        }
    }
    pub fn handle(&self) -> Option<CompNode<T>> {
        unsafe { self.vertex.as_ref().handle() }
    }
    pub fn handle_mut(&mut self) -> &mut Option<CompNode<T>> {
        unsafe { self.vertex.as_mut().handle_mut() }
    }
    pub fn value(&self) -> T::V {
        unsafe { self.vertex.as_ref().value() }
    }
    pub fn value_set(&mut self, val: T::V) {
        unsafe { self.vertex.as_mut().value_set(val); }
    }
}

impl<T: Cluster> Clone for Vertex<T> {
    fn clone(&self) -> Self {
        Vertex { vertex: self.vertex.clone() }
    }
}
impl<T: Cluster> Copy for Vertex<T> {}
impl<T: Cluster> PartialEq for Vertex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.vertex == other.vertex
    }
}

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
    v: [Vertex<T>; 2],
    par: Link<ParentNode<T>>,
    me: NonNull<Edge<T>>,


    pub val: T,
}

pub struct Compress<T: Cluster> {
    ch: [CompNode<T>; 2],
    v: [Vertex<T>; 2],
    rake: Link<RakeNode<T>>,
    par: Link<ParentNode<T>>,
    me: NonNull<Compress<T>>,
    rev: bool,

    pub guard: bool,


    pub fold: T
}

pub struct Rake<T: Cluster> {
    ch: [RakeNode<T>; 2],
    v: [Vertex<T>; 2],
    par: Link<ParentNode<T>>,

    fold: T,
}

impl<T: Cluster> Edge<T> {
    pub fn new(v: Vertex<T>, u: Vertex<T>, val: T) -> NonNull<Edge<T>> {
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
                v: [Vertex::dangling(), Vertex::dangling()],
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
                v: [Vertex::dangling(), Vertex::dangling()],
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
                    *self.v[0].handle_mut() = Some(CompNode::Leaf(self.me));
                }
            }
            Some(ParentNode::Rake(_)) => {
                *self.v[0].handle_mut() = Some(CompNode::Leaf(self.me));
            }
            _ => {
                *self.v[0].handle_mut() = Some(CompNode::Leaf(self.me));
                *self.v[1].handle_mut() = Some(CompNode::Leaf(self.me));
            }
        }
    }
    fn push(&mut self) {}
    fn reverse(&mut self) {
        self.v.swap(0, 1);
        self.val.reverse();
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

        self.fold = T::compress(
            match self.rake {
                Some(r) => T::rake(self.ch[0].fold(), r.fold(), self.ch[0].endpoints(0).value(), r.endpoints(0).value(), self.ch[0].endpoints(1).value()),
                None => self.ch[0].fold(),
            },
            self.ch[1].fold(), self.ch[0].endpoints(0).value(), self.ch[1].endpoints(1).value(), self.ch[0].endpoints(1).value()
            );
        *self.ch[0].endpoints(1).handle_mut() = Some(CompNode::Node(self.me));
        /*println!("fix=====");
        for i in 0..2 {
            for j in 0..2 {
                println!("{}, {} = {}", i, j, self.ch[0].endpoints(i) == self.ch[1].endpoints(j));
            }
        }*/
        //assert!(self.ch[0].endpoints(1) == self.ch[1].endpoints(0));
        match self.parent() {
            Some(ParentNode::Compress(_)) => {
                if parent_dir_comp(CompNode::Node(self.me)).is_none() {
                    *self.v[0].handle_mut() = Some(CompNode::Node(self.me));
                }
            },
            Some(ParentNode::Rake(_)) => {
                *self.v[0].handle_mut() = Some(CompNode::Node(self.me));
            }
            _ => {
                *self.v[0].handle_mut() = Some(CompNode::Node(self.me));
                *self.v[1].handle_mut() = Some(CompNode::Node(self.me));
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
        self.push();
        self.v = [self.ch[0].endpoints(0), self.ch[0].endpoints(1)];
        self.fold = T::rake(self.ch[0].fold(), self.ch[1].fold(), self.ch[0].endpoints(0).value(), self.ch[1].endpoints(0).value(), self.ch[0].endpoints(1).value());
    }
    fn push(&mut self) {
    }
    fn reverse(&mut self) {
        self.push();
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
    pub fn endpoints(&self, dir: usize) -> Vertex<T> {
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
    pub fn endpoints(&self, dir: usize) -> Vertex<T> {
        unsafe {
            match *self {
                RakeNode::Node(node) => node.as_ref().v[dir],
                RakeNode::Leaf(leaf) => leaf.endpoints(dir),
            }
        }
    }
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

impl<T: Cluster> Copy for CompNode<T> {}
impl<T: Cluster> Copy for RakeNode<T> {}
impl<T: Cluster> Copy for ParentNode<T> {}
