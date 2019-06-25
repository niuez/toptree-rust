use std::ptr::NonNull;

type Link<N> = Option<N>;

#[derive(Clone, Copy, PartialEq)]
struct Vertex(usize, Option<CompNode>);

#[derive(Clone, Copy, PartialEq)]
enum CompNode {
    Node(NonNull<Compress>),
    Leaf(NonNull<Edge>),
}

#[derive(Clone, Copy, PartialEq)]
enum RakeNode {
    Node(NonNull<Rake>),
    Leaf(CompNode),
}

#[derive(Clone, Copy, PartialEq)]
enum ParentNode {
    Compress(NonNull<Compress>),
    Rake(NonNull<Rake>),
}

#[derive(Clone, Copy, PartialEq)]
struct Edge {
    v: [Vertex; 2],
    par: Link<ParentNode>
}

#[derive(Clone, Copy, PartialEq)]
struct Compress {
    ch: [CompNode; 2],
    v: [Vertex; 2],
    rake: Link<RakeNode>,
    par: Link<ParentNode>,
    rev: bool,
}

#[derive(Clone, Copy, PartialEq)]
struct Rake {
    ch: [RakeNode; 2],
    par: Link<ParentNode>,
    rev: bool,
}

trait TVertex {
    fn fix(&mut self);
    fn push(&mut self);
    fn reverse(&mut self);
    fn parent(&self) -> Link<ParentNode>;
    fn parent_mut(&mut self) -> &mut Link<ParentNode>;
}

trait Node: TVertex {
    type Child: TVertex;
    fn child(&self, dir: usize) -> Self::Child;
    fn child_mut(&mut self, dir: usize) -> &mut Self::Child;
}

impl TVertex for Edge {
    fn fix(&mut self) {}
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
    fn fix(&mut self) {}
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

impl TVertex for CompNode {
    fn fix(&mut self) {}
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
    fn fix(&mut self) {}
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
    fn fix(&mut self) {}
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

fn parent_dir_comp(child: CompNode) -> Option<(usize, NonNull<Compress>)> {
    unsafe {
        match child.parent() {
            Some(ParentNode::Compress(p)) => {
                if p.as_ref().child(0) == child { Some((0, p)) }
                else if p.as_ref().child(1) == child { Some((1, p)) }
                else { None }
            }
            _ => None,
        }
    }
}

fn parent_dir_comp_rake(child: CompNode) -> Option<(usize, NonNull<Rake>)> { unsafe { match child.parent() { Some(ParentNode::Rake(p)) => {
                if p.as_ref().child(0) == RakeNode::Leaf(child) { Some((0, p)) }
                else if p.as_ref().child(1) == RakeNode::Leaf(child) { Some((1, p)) }
                else { None }
            }
            _ => None,
        }
    }
}


fn parent_dir_rake(child: RakeNode) -> Option<(usize, NonNull<Rake>)> {
    unsafe {
        match child.parent() {
            Some(ParentNode::Rake(p)) => {
                if p.as_ref().child(0) == child { Some((0, p)) }
                else if p.as_ref().child(0) == child { Some((1, p)) }
                else { None }
            }
            _ => None,
        }
    }
}

fn rotate_comp(mut t: NonNull<Compress>, mut x: NonNull<Compress>, dir: usize) {
    unsafe {
        let y = x.as_ref().parent();
        let par = parent_dir_comp(CompNode::Node(x));
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
            yy.as_mut().fix();
        }
        if let Some((xdir, mut yy)) = rake_par {
            *yy.as_mut().child_mut(xdir) = RakeNode::Leaf(CompNode::Node(t));
            yy.as_mut().fix();
        }
    }
}

fn rotate_rake(mut t: NonNull<Rake>, mut x: NonNull<Rake>, dir: usize) {
    unsafe {
        let y = x.as_ref().parent();
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
        if let Some(ParentNode::Compress(mut yy)) = y {
            *yy.as_mut().rake_mut() = Some(RakeNode::Node(t));
            yy.as_mut().fix();
        }
    }
}

fn splay_comp(mut t: NonNull<Compress>) {
    unsafe {
        t.as_mut().push();
        while let Some((qt_dir, mut q)) = parent_dir_comp(CompNode::Node(t)) {
            if let Some((rq_dir, mut r)) = parent_dir_comp(CompNode::Node(q)) {
                r.as_mut().push();
                q.as_mut().push();
                t.as_mut().push();
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
                q.as_mut().push();
                t.as_mut().push();
                rotate_comp(t, q, qt_dir ^ 1);
            }
        }
    }
}

fn splay_rake(mut t: NonNull<Rake>) {
    unsafe {
        t.as_mut().push();
        while let Some((qt_dir, mut q)) = parent_dir_rake(RakeNode::Node(t)) {
            if let Some((rq_dir, mut r)) = parent_dir_rake(RakeNode::Node(q)) {
                r.as_mut().push();
                q.as_mut().push();
                t.as_mut().push();
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
                q.as_mut().push();
                t.as_mut().push();
                rotate_rake(t, q, qt_dir ^ 1);
            }
        }
    }
}

fn expose(mut node: CompNode) {
    loop {
        if let CompNode::Node(comp) = node {
            splay_comp(comp);
        }
        let mut n = match node.parent() {
            None => break,
            Some(ParentNode::Rake(par)) => {
                splay_rake(par);
                if let Some(ParentNode::Compress(n)) = unsafe { par.as_ref().parent() } {
                    n
                }
                else { unreachable!() }
            }
            Some(ParentNode::Compress(n)) => {
                n
            }
        };
        let dir = match parent_dir_comp(CompNode::Node(n)) {
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
                *rake.as_mut().child_mut(n_dir) = RakeNode::Leaf(n.as_ref().child(dir));
                *n.as_mut().child(dir).parent_mut() = Some(ParentNode::Rake(rake));
                rake.as_mut().fix();
                *n.as_mut().child_mut(dir) = node;
                *node.parent_mut() = Some(ParentNode::Compress(n));
                splay_rake(rake);
                n.as_mut().fix();
            }
        }
        else {
            unsafe {
                *n.as_mut().rake_mut() = Some(RakeNode::Leaf(n.as_ref().child(dir)));
                *n.as_mut().child(dir).parent_mut() = Some(ParentNode::Compress(n));
                *n.as_mut().child_mut(dir) = node;
                *node.parent_mut() = Some(ParentNode::Compress(n));
                n.as_mut().fix();
            }
        }
        if let CompNode::Leaf(_) = node {
            while let Some(ParentNode::Compress(par)) = node.parent() {
                node = CompNode::Node(par);
            }
        }
    }
}

fn test_comp_endpoints(node: CompNode) {
    unsafe {
        match node {
            CompNode::Node(node) => {
                println!("node---");
                println!("{:?}", node.as_ref().v.iter().map(|v| v.0).collect::<Vec<_>>());
                println!("left");
                test_comp_endpoints(node.as_ref().child(0));
                println!("right");
                test_comp_endpoints(node.as_ref().child(1));
            }
            CompNode::Leaf(leaf) => {
                println!("leaf---");
                println!("{:?}", leaf.as_ref().v.iter().map(|v| v.0).collect::<Vec<_>>());
            }
        }
    }
}

fn main() {
    unsafe {
        let v: Vec<_> = (0..8).map(|i| Vertex(i, None)).collect();
        let mut e: Vec<_> = (0..4).map(|i| NonNull::new_unchecked(Box::into_raw(Box::new(Edge {
            v: [v[i], v[i + 1]],
            par: None,
        })))).map(|e| CompNode::Leaf(e)).collect();
        let mut e51 = CompNode::Leaf(NonNull::new_unchecked(Box::into_raw(Box::new(Edge {
            v: [v[5], v[1]],
            par: None,
        }))));
        let mut e63 = CompNode::Leaf(NonNull::new_unchecked(Box::into_raw(Box::new(Edge {
            v: [v[6], v[3]],
            par: None,
        }))));
        let mut e73 = CompNode::Leaf(NonNull::new_unchecked(Box::into_raw(Box::new(Edge {
            v: [v[7], v[3]],
            par: None,
        }))));
        let mut rake = NonNull::new_unchecked(Box::into_raw(Box::new(Rake {
            ch: [RakeNode::Leaf(e63), RakeNode::Leaf(e73)],
            par: None,
            rev: false,
        })));
        *e63.parent_mut() = Some(ParentNode::Rake(rake));
        *e73.parent_mut() = Some(ParentNode::Rake(rake));
        let mut p1 = NonNull::new_unchecked(Box::into_raw(Box::new(Compress {
            ch: [e[0], e[1]],
            v: [v[0], v[2]],
            rake: Some(RakeNode::Leaf(e51)),
            par: None,
            rev: false,
        })));
        *e51.parent_mut() = Some(ParentNode::Compress(p1));
        *e[0].parent_mut() = Some(ParentNode::Compress(p1));
        *e[1].parent_mut() = Some(ParentNode::Compress(p1));
        let mut p2 = NonNull::new_unchecked(Box::into_raw(Box::new(Compress {
            ch: [e[2], e[3]],
            v: [v[2], v[4]],
            rake: Some(RakeNode::Node(rake)),
            par: None,
            rev: false,
        })));
        *rake.as_mut().parent_mut() = Some(ParentNode::Compress(p2));
        *e[2].parent_mut() = Some(ParentNode::Compress(p2));
        *e[3].parent_mut() = Some(ParentNode::Compress(p2));
        let p3 = NonNull::new_unchecked(Box::into_raw(Box::new(Compress {
            ch: [CompNode::Node(p1), CompNode::Node(p2)],
            v: [v[0], v[4]],
            rake: None,
            par: None,
            rev: false,
        })));
        *p1.as_mut().parent_mut() = Some(ParentNode::Compress(p3));
        *p2.as_mut().parent_mut() = Some(ParentNode::Compress(p3));
        test_comp_endpoints(CompNode::Node(p3));
        splay_comp(p1);
        println!("splay");
        test_comp_endpoints(CompNode::Node(p3));
        println!("p1-------------------------");
        test_comp_endpoints(CompNode::Node(p1));
        println!("expose e51");
        expose(e51);
        test_comp_endpoints(CompNode::Node(p1));
        println!("expose e73");
        expose(e73);
        test_comp_endpoints(CompNode::Node(p1));
    }
}
