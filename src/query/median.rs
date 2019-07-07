use crate::node::*;
use crate::link::*;
use crate::select::*;
use crate::expose::*;
use crate::path_query::*;

#[derive(Clone)]
struct Median {
    inter_weight: usize,
    left_sum: usize,
    right_sum: usize,
    ans: usize,
    length: usize,
}

impl Median {
    fn new(l: usize) -> Self {
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
    type V = usize;
    fn identity() -> Self {
        Median {
            inter_weight: 0,
            left_sum: 0,
            right_sum: 0,
            ans: 0,
            length: 0,
        }
    }
    fn compress(a: Self, b: Self, av: usize, bv: usize, cv: usize) -> Self {
        Median {
            inter_weight: a.inter_weight + b.inter_weight + cv,
            ans: a.right_sum + b.left_sum + a.length * av + b.length * bv,
            left_sum: a.left_sum + b.left_sum + a.length * (b.inter_weight + cv),
            right_sum: b.right_sum + a.right_sum + b.length * (a.inter_weight + cv),
            length: a.length + b.length,
        }
    }
    fn rake(a: Self, b: Self, _av: usize, bv: usize, _cv: usize) -> Self {
        Median {
            inter_weight: a.inter_weight + b.inter_weight + bv,
            ans: 0,
            left_sum: a.left_sum + b.right_sum + a.length * b.inter_weight + (a.length + b.length) * bv,
            right_sum: a.right_sum + b.right_sum + b.length * bv,
            length: a.length,
        }
    }
    fn reverse(&mut self) {
        std::mem::swap(&mut self.left_sum, &mut self.right_sum);
    }
}


pub fn median_test() {
    println!("median test");
    let v: Vec<_> = (0..13).map(|_| Vertex::new(1)).collect();
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
        es.push(link(v[*a], v[*b], Median::new(*w)));
    }
    let median = select(v[0], |a, b, av, bv, cv| {
        if a.inter_weight + av + cv >= b.inter_weight + bv + cv { 0 }
        else { 1 }
    });
    println!("median vertices {}, {}", v.iter().position(|vv| *vv == median.0).unwrap(), v.iter().position(|vv| *vv == median.1).unwrap());
    println!("weight {}, {}", expose(median.0).fold().ans, expose(median.1).fold().ans);
    println!("9 weight => {}", expose(v[9]).fold().ans);
    println!("9 = median.1 => {}", v[9] == median.1);

    println!("verify");

    for i in 0..13 {
        let mut sum = 0;
        for j in 0..13 {
            if i == j { continue }
            sum += path_query(v[i], v[j]).length;
        }
        println!("center {} -> sum = {}", i, sum);
        println!("expose sum = {}", expose(v[i]).fold().ans);
        println!("inter = {}", expose(v[i]).fold().inter_weight);
    }
}

pub fn median_easy() {
    println!("median easy");
    let v: Vec<_> = (0..11).map(|_| Vertex::new(1)).collect();
    let edges = [
        (0usize, 1usize, 1usize),
        (1usize, 2usize, 2usize),
        (2, 3, 3),
        (3, 4, 4),
        (4, 5, 5),
        (5, 6, 6),
        (6, 7, 7),
        (7, 8, 8),
        (8, 9, 9),
        (9, 10, 10),
    ];
    let mut es = Vec::new();
    for (a, b, w) in edges.iter() {
        es.push(link(v[*a], v[*b], Median::new(*w)));
    }
    let median = select(v[0], |a, b, av, bv, cv| {
        println!("left {} right {}", a.inter_weight + av + cv, b.inter_weight + bv + cv);
        if a.inter_weight + av + cv >= b.inter_weight + bv + cv { 0 }
        else { 1 }
    });
    println!("median vertices {}, {}", v.iter().position(|vv| *vv == median.0).unwrap(), v.iter().position(|vv| *vv == median.1).unwrap());
    println!("weight {}, {}", expose(median.0).fold().ans, expose(median.1).fold().ans);
    for i in 0..11 {
        let mut sum = 0;
        for j in 0..11 {
            if i == j { continue }
            sum += path_query(v[i], v[j]).length;
        }
        println!("center {} -> sum = {}", i, sum);
        println!("expose sum = {}", expose(v[i]).fold().ans);
        println!("==================");
    }
}

pub fn median_easy2() {
    println!("median easy2");
    let v: Vec<_> = (0..7).map(|_| Vertex::new(1)).collect();
    let edges = [
        (0usize, 1usize, 1usize),
        (1usize, 2usize, 2usize),
        (2, 3, 3),
        (3, 4, 4),
        (3, 5, 5),
        (3, 6, 6),
    ];
    let mut es = Vec::new();
    for (a, b, w) in edges.iter() {
        es.push(link(v[*a], v[*b], Median::new(*w)));
    }
    let median = select(v[0], |a, b, av, bv, cv| {
        println!("left {} right {}", a.inter_weight + av + cv, b.inter_weight + bv + cv);
        if a.inter_weight + av + cv >= b.inter_weight + bv + cv { 0 }
        else { 1 }
    });
    println!("median vertices {}, {}", v.iter().position(|vv| *vv == median.0).unwrap(), v.iter().position(|vv| *vv == median.1).unwrap());
    println!("weight {}, {}", expose(median.0).fold().ans, expose(median.1).fold().ans);
    for i in 0..7 {
        let mut sum = 0;
        for j in 0..7 {
            if i == j { continue }
            sum += path_query(v[i], v[j]).length;
        }
        println!("center {} -> sum = {}", i, sum);
        println!("expose sum = {}", expose(v[i]).fold().ans);
        println!("==================");
    }
}
