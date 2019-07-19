use crate::node::*;
use crate::link::*;
use crate::path_query::*;
use crate::debug::*;

impl Cluster for usize {
    type V = usize;
    fn identity() -> Self { 0 }
    fn compress(left: Self, right: Self, _: usize, _: usize, _: usize) -> Self { left + right }
    fn rake(a: Self, _: Self, _: usize, _: usize, _: usize) -> Self { a }
    fn reverse(&mut self) {}
}

pub fn path_length_test() {
    println!("path_length");
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
        es.push(link(v[*a], v[*b], *w));
        //println!("{:?}", (*a, *b, *w));
        //test_comp_endpoints(v[0].as_ref().1.unwrap());
    }
    assert!(path_query(v[1], v[0]) == 1);
    assert!(path_query(v[0], v[4]) == 5);
    assert!(path_query(v[1], v[9]) == 8);
    assert!(path_query(v[3], v[11]) == 27);
    assert!(path_query(v[6], v[12]) == 18);
    assert!(path_query(v[12], v[6]) == 18);
    for i in 0..13 {
        println!("VERTEX {} ---------------", i);
        test_comp_set(v[i].handle().unwrap());
    }
    assert!(path_query(v[2], v[4]) == 14);
    assert!(path_query(v[5], v[6]) == 6);
}

