use crate::node::*;
use crate::link::*;
use crate::cut::*;
use crate::expose::*;

#[derive(Clone, Debug)]
struct Farthest {
    ans: usize,
    max_dist_left: usize,
    max_dist_right: usize,
    length: usize
}

impl Farthest {
    fn new(l: usize) -> Self {
        Farthest {
            ans: l,
            max_dist_left: l,
            max_dist_right: l,
            length: l,
        }
    }
}

impl Cluster for Farthest {
    fn identity() -> Self {
        Farthest {
            ans: 0,
            max_dist_left: 0,
            max_dist_right: 0,
            length: 0,
        }
    }
    fn compress(a: Self, b: Self) -> Self {
        Farthest {
            ans: std::cmp::max(a.max_dist_right, b.max_dist_left),
            max_dist_left: std::cmp::max(a.max_dist_left, a.length + b.max_dist_left),
            max_dist_right: std::cmp::max(b.max_dist_right, b.length + a.max_dist_right),
            length: a.length + b.length
        }
    }
    fn rake(a: Self, b: Self) -> Self {
        Farthest {
            ans: 0,
            max_dist_left: std::cmp::max(a.max_dist_left, a.length + b.max_dist_right),
            max_dist_right: std::cmp::max(a.max_dist_right, b.max_dist_right),
            length: a.length,
        }
    }
    fn reverse(&mut self) {
        std::mem::swap(&mut self.max_dist_left, &mut self.max_dist_right);
    }
}

use std::io::Read;


/* https://atcoder.jp/contests/tkppc/tasks/tkppc2015_j */
pub fn farthest_test() {
    println!("farthest");
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut v: Vec<_> = (0..1).map(|_| Vertex::new(())).collect();
    let edges :Vec<(usize, usize, usize)>= (0..q).map(|_| {
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            )
    }).collect();
    let mut es = Vec::new();
    for (t, a, c) in edges.iter() {
        if *t == 1 {
            let new_v = Vertex::new(());
            v.push(new_v);
            link(v[*a], new_v, Farthest::new(*c));
            es.push((*a, v.len() - 1));
        }
        else if *t == 2 {
            let p = es[*a - 1].0;
            let q = es[*a - 1].1;
            cut(v[p], v[q]);
            link(v[p], v[q], Farthest::new(*c));
        }
        else if *t == 3 {
            println!("farthest from {} = {}", *a, expose(v[*a]).fold().ans);
        }
    }
}
