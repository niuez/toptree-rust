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
        let a = Self::rake(a, rake);
        Diameter {
            diam: *[ a.diam, b.diam, a.max_dist_right + b.max_dist_left].into_iter().max().unwrap(),
            max_dist_left: std::cmp::max(a.max_dist_left, a.length + b.max_dist_left),
            max_dist_right: std::cmp::max(b.max_dist_right, b.length + a.max_dist_right),
            length: a.length + b.length
        }
    }
    fn rake(a: Self, b: Self) -> Self {
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
    unsafe {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();

        let mut v: Vec<_> = (0..n).map(|i| Vertex(i, None)).map(|v| NonNull::new_unchecked(Box::into_raw(Box::new(v)))).collect();
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
            println!("{:?}", (*a, *b, *w));
            //test_comp_endpoints(v[0].as_ref().1.unwrap());
        }
        println!("diameter = {}", expose(v[0].as_ref().1.unwrap()).fold().diam);
    }
}
