use crate::node::*;
use crate::link::*;
use crate::select::*;
use crate::expose::*;
use crate::path_query::*;
use crate::cut::*;

#[derive(Clone, Debug)]
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

use std::io::Read;

pub fn yuki772() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let mut v: Vec<_> = (0..n).map(|_| Vertex::new(1)).collect();
    let mut sum = 0;
    for _ in 0..q {
        let query: usize = iter.next().unwrap().parse().unwrap();
        if query == 1 {
            let a: usize = iter.next().unwrap().parse().unwrap();
            let b: usize = iter.next().unwrap().parse().unwrap();
            let c: usize = iter.next().unwrap().parse().unwrap();
            let (a, b) = ((a - 1 + sum) % n, (b - 1 + sum) % n);
            //println!("link {} {}", a, b);
            link(v[a], v[b], Median::new(c));
        }
        else if query == 2 {
            let a: usize = iter.next().unwrap().parse().unwrap();
            let b: usize = iter.next().unwrap().parse().unwrap();
            let (a, b) = ((a - 1 + sum) % n, (b - 1 + sum) % n);
            //println!("cut {} {}", a, b);
            cut(v[a], v[b]);
        }
        else if query == 3 {
            let a: usize = iter.next().unwrap().parse().unwrap();
            let a = (a - 1 + sum) % n;
            let val = v[a].value();
            v[a].value_set(1 - val);
            let (x, y) = select(v[a], |a, b, av, bv, cv| {
                if a.inter_weight + av + cv >= b.inter_weight + bv + cv { 0 }
                else { 1 }
            });
            //println!("query 3 = {}", a);
            let ans = std::cmp::min(expose(x).fold().ans, expose(y).fold().ans);
            /* let ans = (0..n).filter(|i| {
                let root = expose(v[a]);
                expose(v[*i]);
                root.parent().is_some() || *i == a
            }).map(|i| expose(v[i]).fold().ans).min().unwrap(); */
            sum = (sum + ans % n) % n;
            println!("{}", ans);
        }
    }

}
