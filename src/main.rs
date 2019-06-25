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
    v: [NonNull<Vertex>; 2],
    par: Link<ParentNode>,
    me: NonNull<Edge>,


    val: usize,
}

#[derive(Clone, Copy, PartialEq)]
struct Compress {
    ch: [CompNode; 2],
    v: [NonNull<Vertex>; 2],
    rake: Link<RakeNode>,
    par: Link<ParentNode>,
    me: NonNull<Compress>,
    rev: bool,


    fold: usize
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
        t.as_mut().fix();
        while let Some((qt_dir, mut q)) = parent_dir_comp(CompNode::Node(t)) {
            if let Some((rq_dir, mut r)) = parent_dir_comp(CompNode::Node(q)) {
                r.as_mut().push();
                q.as_mut().push();
                t.as_mut().push();
                t.as_mut().fix();
                q.as_mut().fix();
                r.as_mut().fix();
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
                t.as_mut().fix();
                q.as_mut().fix();
                rotate_comp(t, q, qt_dir ^ 1);
            }
        }
    }
}

fn splay_rake(mut t: NonNull<Rake>) {
    unsafe {
        t.as_mut().push();
        t.as_mut().fix();
        while let Some((qt_dir, mut q)) = parent_dir_rake(RakeNode::Node(t)) {
            if let Some((rq_dir, mut r)) = parent_dir_rake(RakeNode::Node(q)) {
                r.as_mut().push();
                q.as_mut().push();
                t.as_mut().push();
                t.as_mut().fix();
                q.as_mut().fix();
                r.as_mut().fix();
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
                t.as_mut().fix();
                q.as_mut().fix();
                rotate_rake(t, q, qt_dir ^ 1);
            }
        }
    }
}

fn expose(mut node: CompNode) -> CompNode {
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
                let mut nch = n.as_mut().child(dir);
                {
                    let mut now = nch;
                    let mut stack = Vec::new();
                    while let Some(ParentNode::Compress(par)) = now.parent() {
                        stack.push(par);
                        now = CompNode::Node(par);
                    }
                    while let Some(mut par) = stack.pop() {
                        par.as_mut().push();
                    }
                }
                *rake.as_mut().child_mut(n_dir) = RakeNode::Leaf(n.as_ref().child(dir));
                *n.as_mut().child(dir).parent_mut() = Some(ParentNode::Rake(rake));
                *n.as_mut().child_mut(dir) = node;
                *node.parent_mut() = Some(ParentNode::Compress(n));
                nch.fix();
                rake.as_mut().fix();
                node.fix();
                splay_rake(rake);
                n.as_mut().fix();
            }
        }
        else {
            unsafe {
                let mut nch = n.as_mut().child(dir);
                {
                    let mut now = nch;
                    let mut stack = Vec::new();
                    while let Some(ParentNode::Compress(par)) = now.parent() {
                        stack.push(par);
                        now = CompNode::Node(par);
                    }
                    while let Some(mut par) = stack.pop() {
                        par.as_mut().push();
                    }
                }

                *n.as_mut().rake_mut() = Some(RakeNode::Leaf(n.as_ref().child(dir)));
                *n.as_mut().child(dir).parent_mut() = Some(ParentNode::Compress(n));
                *n.as_mut().child_mut(dir) = node;
                *node.parent_mut() = Some(ParentNode::Compress(n));
                nch.fix();
                node.fix();
                n.as_mut().fix();
            }
        }
        if let CompNode::Leaf(_) = node {
            while let Some(ParentNode::Compress(mut par)) = node.parent() {
                unsafe { par.as_mut().fix() }
                node = CompNode::Node(par);
            }
        }
    }
    node
}

fn soft_expose(v: NonNull<Vertex>, u: NonNull<Vertex>) {
    unsafe {
        if u.as_ref().1.unwrap().endpoints(0) == u {
            expose(u.as_ref().1.unwrap().endpoints(1).as_ref().1.unwrap());
        }
        let mut ur = expose(u.as_ref().1.unwrap());
        if ur.endpoints(0) == u {
            ur.reverse();
            ur.push();
        }
        //test_comp_endpoints(ur);
        let mut root = expose(v.as_ref().1.unwrap());

        //println!("root = {}, {}", root.endpoints(0).as_ref().0, root.endpoints(1).as_ref().0);
        //println!("v u = {}, {}", v.as_ref().0, u.as_ref().0);
        //test_comp_print(v.as_ref().1.unwrap());
        //test_comp_print(u.as_ref().1.unwrap());
        //test_comp_endpoints(root);

        if u.as_ref().1 == v.as_ref().1 {
            if root.endpoints(1) == v {
                root.reverse();
            }
            else if root.endpoints(0) == u {
                root.reverse();
            }
            return;
        }
        let mut root = match root {
            CompNode::Node(n) => n,
            _ => unreachable!(),
        };
        match u.as_ref().1.unwrap() {
            CompNode::Node(mut t) => {
                t.as_mut().push();
                t.as_mut().fix();
                while let Some((qt_dir, mut q)) = parent_dir_comp(CompNode::Node(t)) {
                    if CompNode::Node(q) == CompNode::Node(root) { break }
                    q.as_mut().push();
                    t.as_mut().push();
                    t.as_mut().fix();
                    q.as_mut().fix();
                    rotate_comp(t, q, qt_dir ^ 1);
                }
            }
            _ => unreachable!()
        }
        if root.as_ref().child(0) == u.as_ref().1.unwrap() {
            root.as_mut().reverse();
        }
        assert!(root.as_ref().child(1) == u.as_ref().1.unwrap());
    }
}

fn query(v: NonNull<Vertex>, u: NonNull<Vertex>) -> usize {
    unsafe {
        soft_expose(v, u);
        let mut root = v.as_ref().1.unwrap();
        root.push();
        //test_comp_endpoints(root);
        //println!("root = {}, {}", root.endpoints(0).as_ref().0, root.endpoints(1).as_ref().0);
        if root.endpoints(0) == v && root.endpoints(1) == u {
            root.fold()
        }
        else if root.endpoints(0) == v {
            if let CompNode::Node(mut n) = root {
                n.as_mut().push();
                n.as_ref().child(0).fold()
            }
            else { unreachable!() }
        }
        else if root.endpoints(1) == u {
            if let CompNode::Node(mut n) = root {
                n.as_mut().push();
                n.as_ref().child(1).fold()
            }
            else { unreachable!() }
        }
        else {
            if let CompNode::Node(mut n) = root {
                n.as_mut().push();
                if let CompNode::Node(mut n2) = n.as_ref().child(1) {
                    n2.as_mut().push();
                    n2.as_ref().child(0).fold()
                }
                else { unreachable!() }
            }
            else { unreachable!() }
        }
    }
}

fn test_comp_endpoints(mut node: CompNode) {
    unsafe {
        //node.push();
        match node {
            CompNode::Node(node) => {
                println!("node---");
                println!("{:?} = {}  rev {}", node.as_ref().v.iter().map(|v| v.as_ref().0).collect::<Vec<_>>(), node.as_ref().fold, node.as_ref().rev);
                println!("left");
                test_comp_endpoints(node.as_ref().child(0));
                println!("right");
                test_comp_endpoints(node.as_ref().child(1));
            }
            CompNode::Leaf(leaf) => {
                println!("leaf---");
                println!("{:?} = {}", leaf.as_ref().v.iter().map(|v| v.as_ref().0).collect::<Vec<_>>(), leaf.as_ref().val);
            }
        }
    }
}

fn test_comp_print(node: CompNode) {
    unsafe {
        match node {
            CompNode::Node(node) => {
                println!("node---");
                println!("{:?}", node.as_ref().v.iter().map(|v| v.as_ref().0).collect::<Vec<_>>());
            }
            CompNode::Leaf(leaf) => {
                println!("leaf---");
                println!("{:?}", leaf.as_ref().v.iter().map(|v| v.as_ref().0).collect::<Vec<_>>());
            }
        }
    }
}


fn main() {
    unsafe {
        let v: Vec<_> = (0..8).map(|i| Vertex(i, None)).map(|v| NonNull::new_unchecked(Box::into_raw(Box::new(v)))).collect();
        let e_weight = [1, 2, 3, 4];
        let mut e: Vec<_> = (0..4).map(|i| NonNull::new_unchecked(Box::into_raw(Box::new(Edge {
            v: [v[i], v[i + 1]],
            par: None,
            me: NonNull::dangling(),

            val: e_weight[i],

        })))).map(|mut e| {
            e.as_mut().me = e;
            e.as_mut().fix();
            CompNode::Leaf(e)
        }).collect();

        let mut e51 = NonNull::new_unchecked(Box::into_raw(Box::new(Edge {
            v: [v[5], v[1]],
            par: None,
            me: NonNull::dangling(),

            val: 5,
        })));
        e51.as_mut().me = e51;
        e51.as_mut().fix();
        let mut e51 = CompNode::Leaf(e51);

        let mut e63 = NonNull::new_unchecked(Box::into_raw(Box::new(Edge {
            v: [v[6], v[3]],
            par: None,
            me: NonNull::dangling(),

            val: 6,
        })));
        e63.as_mut().me = e63;
        e63.as_mut().fix();
        let mut e63 = CompNode::Leaf(e63);

        let mut e73 = NonNull::new_unchecked(Box::into_raw(Box::new(Edge {
            v: [v[7], v[3]],
            par: None,
            me: NonNull::dangling(),

            val: 7,
        })));
        e73.as_mut().me = e73;
        e73.as_mut().fix();
        let mut e73 = CompNode::Leaf(e73);

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
            me: NonNull::dangling(),

            fold: 0,
        })));
        p1.as_mut().me = p1;
        p1.as_mut().fix();
        *e51.parent_mut() = Some(ParentNode::Compress(p1));
        *e[0].parent_mut() = Some(ParentNode::Compress(p1));
        *e[1].parent_mut() = Some(ParentNode::Compress(p1));
        let mut p2 = NonNull::new_unchecked(Box::into_raw(Box::new(Compress {
            ch: [e[2], e[3]],
            v: [v[2], v[4]],
            rake: Some(RakeNode::Node(rake)),
            par: None,
            rev: false,
            me: NonNull::dangling(),

            fold: 0,
        })));
        p2.as_mut().me = p2;
        p2.as_mut().fix();
        *rake.as_mut().parent_mut() = Some(ParentNode::Compress(p2));
        *e[2].parent_mut() = Some(ParentNode::Compress(p2));
        *e[3].parent_mut() = Some(ParentNode::Compress(p2));
        let mut p3 = NonNull::new_unchecked(Box::into_raw(Box::new(Compress {
            ch: [CompNode::Node(p1), CompNode::Node(p2)],
            v: [v[0], v[4]],
            rake: None,
            par: None,
            rev: false,
            me: NonNull::dangling(),

            fold: 0,
        })));
        p3.as_mut().me = p3;
        p3.as_mut().fix();
        *p1.as_mut().parent_mut() = Some(ParentNode::Compress(p3));
        *p2.as_mut().parent_mut() = Some(ParentNode::Compress(p3));
        /*
        test_comp_endpoints(CompNode::Node(p3));
        println!("----------handle---------");
        for vv in v.iter() {
            println!("handle {}", vv.as_ref().0);
            test_comp_print(vv.as_ref().1.unwrap());
        }
        */
        /*
        splay_comp(p1);
        println!("splay");
        test_comp_endpoints(CompNode::Node(p3));
        println!("p1-------------------------");
        test_comp_endpoints(CompNode::Node(p1));
        for vv in v.iter() {
            println!("handle {}", vv.as_ref().0);
            test_comp_print(vv.as_ref().1.unwrap());
        }
        println!("expose e51");
        expose(e51);
        test_comp_endpoints(CompNode::Node(p1));
        for vv in v.iter() {
            println!("handle {}", vv.as_ref().0);
            test_comp_print(vv.as_ref().1.unwrap());
        }
        println!("expose e73");
        expose(e73);
        test_comp_endpoints(CompNode::Node(p1));
        for vv in v.iter() {
            println!("handle {}", vv.as_ref().0);
            test_comp_print(vv.as_ref().1.unwrap());
        }
        println!("expose e01");
        expose(e[0]);
        test_comp_endpoints(CompNode::Node(p1));
        for vv in v.iter() {
            println!("handle {}", vv.as_ref().0);
            test_comp_print(vv.as_ref().1.unwrap());
        }
        */
        /*
        soft_expose(v[5], v[3]);
        println!("finish--------------");
        test_comp_endpoints(v[5].as_ref().1.unwrap());
        soft_expose(v[1], v[3]);
        println!("finish--------------");
        test_comp_endpoints(v[1].as_ref().1.unwrap());
        */

        println!("query 0, 4 = {}", query(v[0], v[4]));
        println!("query 0, 3 = {}", query(v[0], v[3]));
        println!("query 1, 3 = {}", query(v[1], v[3]));
        println!("query 1, 5 = {}", query(v[1], v[5]));
        println!("query 2, 0 = {}", query(v[2], v[0]));
        println!("query 5, 7 = {}", query(v[5], v[7]));
        println!("query 6, 7 = {}", query(v[6], v[7]));
    }
}
