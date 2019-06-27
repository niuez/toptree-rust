pub mod node;
pub mod parent_dir;
pub mod splay;
pub mod expose;
pub mod link;
pub mod path_query;

use std::ptr::NonNull;
use node::*;
use link::*;
use path_query::*;

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
            es.push(link(v[*a], v[*b], *w, 0));
            println!("{:?}", (*a, *b, *w));
            //test_comp_endpoints(v[0].as_ref().1.unwrap());
        }
        for i in 0..13 {
            let dummy = NonNull::new_unchecked(Box::into_raw(Box::new(Vertex(i + 13, None))));
            v.push(dummy);
            let el = link(v[i], dummy, 0, 0);
            es.push(el);
        }
        assert!(path_query(v[1], v[0]) == 1);
        assert!(path_query(v[0], v[4]) == 5);
        assert!(path_query(v[1], v[9]) == 8);
        assert!(path_query(v[3], v[11]) == 27);
        assert!(path_query(v[6], v[12]) == 18);
        assert!(path_query(v[12], v[6]) == 18);
        assert!(path_query(v[2], v[4]) == 14);
        assert!(path_query(v[5], v[6]) == 6);
    }
}
