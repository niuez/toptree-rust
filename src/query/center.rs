use crate::node::*;
use crate::link::*;
use crate::select::*;
use crate::expose::*;

#[derive(Clone, Debug)]
struct Center {
    radius: usize,
    max_dist_left: usize,
    max_dist_right: usize,
    length: usize
}

impl Center {
    fn new(l: usize) -> Self {
        Center {
            radius: l,
            max_dist_left: l,
            max_dist_right: l,
            length: l,
        }
    }
}

impl Cluster for Center {
    type V = usize;
    fn identity() -> Self {
        Center {
            radius: 0,
            max_dist_left: 0,
            max_dist_right: 0,
            length: 0,
        }
    }
    fn compress(a: Self, b: Self, _: usize, _: usize, _: usize) -> Self {
        Center {
            radius: std::cmp::max(a.max_dist_right, b.max_dist_left),
            max_dist_left: std::cmp::max(a.max_dist_left, a.length + b.max_dist_left),
            max_dist_right: std::cmp::max(b.max_dist_right, b.length + a.max_dist_right),
            length: a.length + b.length
        }
    }
    fn rake(a: Self, b: Self, _: usize, _: usize, _: usize) -> Self {
        Center {
            radius: 0,
            max_dist_left: std::cmp::max(a.max_dist_left, a.length + b.max_dist_right),
            max_dist_right: std::cmp::max(a.max_dist_right, b.max_dist_right),
            length: a.length,
        }
    }
    fn reverse(&mut self) {
        std::mem::swap(&mut self.max_dist_left, &mut self.max_dist_right);
    }
}

pub fn center_test() {
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
        es.push(link(v[*a], v[*b], Center::new(*w)));
    }
    let center = select(v[0], |a, b, _, _, _| {
        if a.max_dist_right >= b.max_dist_left { 0 }
        else { 1 }
    });
    println!("center vertices {}, {}", center.0.value(), center.1.value());
    println!("radius {}, {}", expose(center.0).fold().radius, expose(center.1).fold().radius);
}
