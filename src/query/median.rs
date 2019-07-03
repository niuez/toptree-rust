use crate::node::*;
use crate::link::*;
use crate::select::*;
use crate::expose::*;

#[derive(Clone)]
struct Median {
    inter_weight: usize,
    left_sum: usize,
    right_sum: usize,
    ans: usize,
    length: usize,
}

impl Median {
    fn new(l: usize, v: [Vertex<usize, Self>; 2]) -> Self {
        Median {
            inter_weight: 0,
            ans: 0,
            left_sum: 0,
            right_sum: 0,
            length: l,
        }
    }
}

impl Cluster for Median {
    type VType = Vertex<usize, Self>;
    fn identity() -> Self {
        Median {
            inter_weight: 0,
            left_sum: 0,
            right_sum: 0,
            ans: 0,
            length: 0,
        }
    }
    fn compress(a: Self, b: Self, av: [Vertex<usize, Self>; 2], bv: [Vertex<usize, Self>; 2]) -> Self {
        Median {
            inter_weight: a.inter_weight + b.inter_weight + av[1].value(),
            ans: a.right_sum + b.left_sum + a.length * av[0].value() + b.length * bv[1].value(),
            left_sum: a.left_sum + b.left_sum + a.length * (b.inter_weight + av[1].value()),
            right_sum: b.right_sum + b.right_sum + b.length * (a.inter_weight + av[1].value()),
            length: a.length + b.length,
        }
    }
    fn rake(a: Self, b: Self, av: [Vertex<usize, Self>; 2], bv: [Vertex<usize, Self>; 2]) -> Self {
        Median {
            inter_weight: a.inter_weight + b.inter_weight + bv[0].value(),
            ans: 0,
            left_sum: a.left_sum + b.right_sum + a.length * b.inter_weight + (a.length + b.length) * bv[0].value(),
            right_sum: a.right_sum + b.left_sum + b.length * bv[0].value(),
            length: a.length,
        }
    }
    fn reverse(&mut self) {
        std::mem::swap(&mut self.left_sum, &mut self.right_sum);
    }
}


pub fn median_test() {
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
        es.push(link(v[*a], v[*b], Median::new(*w, [v[*a], v[*b]])));
    }
    let median = select(v[0], |a, b, av, bv| {
        if a.inter_weight + av[0].value() + av[1].value() >= b.inter_weight + bv[0].value() + bv[1].value() { 0 }
        else { 1 }
    });
    println!("center vertices {}, {}", median.0.value(), median.1.value());
    println!("radius {}, {}", expose(median.0).fold().ans, expose(median.1).fold().ans);
}
