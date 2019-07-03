use crate::node::*;
use crate::link::*;
use crate::cut::*;
//use crate::select::*;
use crate::expose::*;

#[derive(Clone, Debug)]
struct Diameter {
    diam: usize,
    max_dist_left: usize,
    max_dist_right: usize,
    length: usize
}

impl Diameter {
    fn new(l: usize) -> Self {
        Diameter {
            diam: l,
            max_dist_left: l,
            max_dist_right: l,
            length: l,
        }
    }
}

impl Cluster for Diameter {
    type V = usize;
    fn identity() -> Self {
        Diameter {
            diam: 0,
            max_dist_left: 0,
            max_dist_right: 0,
            length: 0,
        }
    }
    fn compress(a: Self, b: Self, _: usize, _: usize, _: usize) -> Self {
        Diameter {
            diam: *[ a.diam, b.diam, a.max_dist_right + b.max_dist_left].into_iter().max().unwrap(),
            max_dist_left: std::cmp::max(a.max_dist_left, a.length + b.max_dist_left),
            max_dist_right: std::cmp::max(b.max_dist_right, b.length + a.max_dist_right),
            length: a.length + b.length
        }
    }
    fn rake(a: Self, b: Self, _: usize, _: usize, _: usize) -> Self {
        Diameter {
            diam: *[ a.diam, b.diam, a.max_dist_right + b.max_dist_right ].into_iter().max().unwrap(),
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

pub fn diameter_test() {
    println!("diameter");
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let v: Vec<_> = (0..n).map(|i| Vertex::new(i)).collect();
    let edges :Vec<(usize, usize, usize)>= (0..n-1).map(|_| {
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            )
    }).collect();
    let mut es = Vec::new();
    for (a, b, w) in edges.iter() {
        es.push(link(v[*a], v[*b], Diameter::new(*w)));
        //println!("{:?}", (*a, *b, *w));
        //test_comp_endpoints(v[0].as_ref().1.unwrap());
    }
    println!("diameter = {}", expose(v[0]).fold().diam);
}

pub fn diameter_cut_test() {
    println!("diameter cut");
    let v: Vec<_> = (0..13).map(|i| Vertex::new(i)).collect();
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
        es.push(link(v[*a], v[*b], Diameter::new(*w)));
        //println!("{:?}", (*a, *b, *w));
        //test_comp_endpoints(v[0].as_ref().1.unwrap());
    }
    /* let center = select(v[0], |a, b| {
        if a.max_dist_right >= b.max_dist_left { 0 }
        else { 1 }
    }); 
    println!("center vertices {}, {}", center.0.value(), center.1.value()); */
    cut(v[0], v[5]);
    println!("0 diameter = {}", expose(v[0]).fold().diam);
    println!("5 diameter = {}", expose(v[5]).fold().diam);
}
