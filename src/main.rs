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

    guard: bool,


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

fn parent_dir_comp(child: CompNode) -> Option<(usize, NonNull<Compress>)> {
    unsafe {
        match child.parent() {
            Some(ParentNode::Compress(mut p)) => {
                p.as_mut().push();
                if p.as_ref().guard { None }
                else if p.as_ref().child(0) == child { Some((0, p)) }
                else if p.as_ref().child(1) == child { Some((1, p)) }
                else { None }
            }
            _ => None,
        }
    }
}

fn parent_dir_comp_guard(child: CompNode) -> Option<(usize, NonNull<Compress>)> {
    unsafe {
        match child.parent() {
            Some(ParentNode::Compress(mut p)) => {
                p.as_mut().push();
                if p.as_ref().child(0) == child { Some((0, p)) }
                else if p.as_ref().child(1) == child { Some((1, p)) }
                else { None }
            }
            _ => None,
        }
    }
}


fn parent_dir_comp_rake(child: CompNode) -> Option<(usize, NonNull<Rake>)> { 
    unsafe { 
        match child.parent() {
            Some(ParentNode::Rake(mut p)) => {
                p.as_mut().push();
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
            Some(ParentNode::Rake(mut p)) => {
                p.as_mut().push();
                if p.as_ref().child(0) == child { Some((0, p)) }
                else if p.as_ref().child(1) == child { Some((1, p)) }
                else { None }
            }
            _ => None,
        }
    }
}

fn rotate_comp(mut t: NonNull<Compress>, mut x: NonNull<Compress>, dir: usize) {
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

fn rotate_rake(mut t: NonNull<Rake>, mut x: NonNull<Rake>, dir: usize) {
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

fn splay_comp(mut t: NonNull<Compress>) {
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

fn splay_rake(mut t: NonNull<Rake>) {
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

fn expose(mut node: CompNode) -> CompNode {
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

fn soft_expose(v: NonNull<Vertex>, u: NonNull<Vertex>) {
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

fn link(v: NonNull<Vertex>, u: NonNull<Vertex>, weight: usize) -> NonNull<Edge> {
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
                                    rev: false,
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
                                    rev: false,
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

fn test_comp_endpoints(node: CompNode) {
    unsafe {
        //node.push();
        match node {
            CompNode::Node(node) => {
                println!("NODE {:?} = {}  rev {}", node.as_ref().v.iter().map(|v| v.as_ref().0).collect::<Vec<_>>(), node.as_ref().fold, node.as_ref().rev);
                println!("left");
                test_comp_endpoints(node.as_ref().child(0));
                println!("right");
                test_comp_endpoints(node.as_ref().child(1));
            }
            CompNode::Leaf(leaf) => {
                println!("LEAF {:?} = {}", leaf.as_ref().v.iter().map(|v| v.as_ref().0).collect::<Vec<_>>(), leaf.as_ref().val);
            }
        }
    }
}

fn test_comp_print(node: CompNode) {
    unsafe {
        match node {
            CompNode::Node(node) => {
                println!("NODE {:?} = {}  rev {}", node.as_ref().v.iter().map(|v| v.as_ref().0).collect::<Vec<_>>(), node.as_ref().fold, node.as_ref().rev);
            }
            CompNode::Leaf(leaf) => {
                println!("LEAF {:?} = {}", leaf.as_ref().v.iter().map(|v| v.as_ref().0).collect::<Vec<_>>(), leaf.as_ref().val);
            }
        }
    }
}


fn main() {
    unsafe {
        let mut v: Vec<_> = (0..13).map(|i| Vertex(i, None)).map(|v| NonNull::new_unchecked(Box::into_raw(Box::new(v)))).collect();
        let edges = [
            (0usize, 1usize, 1usize),
            (1, 2, 10),
            (1, 3, 3),
            (1, 4, 4),
            (0, 5, 3),
            (5, 9, 4),
            (9, 10, 7),
            (10, 11, 9),
            (10, 12, 1),
            (0, 6, 3),
            (6, 7, 3),
            (7, 8, 7),
        ];
        let mut es = Vec::new();
        for (a, b, w) in edges.iter() {
            es.push(link(v[*a], v[*b], *w));
            println!("{:?}", (*a, *b, *w));
            //test_comp_endpoints(v[0].as_ref().1.unwrap());
        }
        for i in 0..13 {
            let dummy = NonNull::new_unchecked(Box::into_raw(Box::new(Vertex(i + 13, None))));
            v.push(dummy);
            let el = link(v[i], dummy, 0);
            es.push(el);
        }
        assert!(query(v[1], v[0]) == 1);
        assert!(query(v[0], v[4]) == 5);
        assert!(query(v[1], v[9]) == 8);
        assert!(query(v[3], v[11]) == 27);
        assert!(query(v[6], v[12]) == 18);
        assert!(query(v[12], v[6]) == 18);
        assert!(query(v[2], v[4]) == 14);
        assert!(query(v[5], v[6]) == 6);
    }
}
