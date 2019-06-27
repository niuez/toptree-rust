use std::ptr::NonNull;
use crate::node::*;
use crate::link::*;
use crate::expose::*;
use crate::debug::*;

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
    fn identity() -> Self {
        Diameter {
            diam: 0,
            max_dist_left: 0,
            max_dist_right: 0,
            length: 0,
        }
    }
    fn compress(a: Self, b: Self, rake: Self) -> Self {
        Diameter {
            diam: *[ a.diam, b.diam, a.max_dist_right + b.max_dist_left, a.max_dist_right + rake.max_dist_right, b.max_dist_left + rake.max_dist_right, rake.diam].into_iter().max().unwrap(),
            max_dist_left: std::cmp::max(std::cmp::max(a.max_dist_left, a.length + b.max_dist_left), a.length + rake.max_dist_right),
            max_dist_right: std::cmp::max(std::cmp::max(b.max_dist_right, b.length + a.max_dist_right), b.length + rake.max_dist_right),
            length: a.length + b.length
        }
    }
    fn rake(a: Self, b: Self) -> Self {
        Diameter {
            diam: *[ a.diam, b.diam, a.max_dist_right + b.max_dist_right ].into_iter().max().unwrap(),
            max_dist_left: std::cmp::max(std::cmp::max(a.max_dist_left, a.length + b.max_dist_right), std::cmp::max(b.max_dist_left, b.length + a.max_dist_right)),
            max_dist_right: std::cmp::max(a.max_dist_right, b.max_dist_right),
            length: 0,
        }
    }
}

pub fn diameter_test() {
    println!("diameter");
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
            es.push(link(v[*a], v[*b], Diameter::new(*w)));
            println!("{:?}", (*a, *b, *w));
            //test_comp_endpoints(v[0].as_ref().1.unwrap());
        }
        for i in 0..13 {
            let dummy = NonNull::new_unchecked(Box::into_raw(Box::new(Vertex(i + 13, None))));
            v.push(dummy);
            let el = link(v[i], dummy, Diameter::new(0));
            es.push(el);
        }
        println!("diameter = {}", expose(v[0].as_ref().1.unwrap()).fold().diam);
    }
}
