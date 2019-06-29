# top tree on Rust

top treeをrustで書いたやつです.

## 二点間距離クエリ

```rust
impl Cluster for usize {
    fn identity() -> Self { 0 }
    fn compress(left: Self, right: Self) -> Self { left + right }
    fn rake(a: Self, _: Self) -> Self { a }
    fn reverse(&mut self) {}
}
```

## 直径クエリ

```rust
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
    fn compress(a: Self, b: Self) -> Self {
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
```

## 最遠点クエリ

https://atcoder.jp/contests/tkppc/tasks/tkppc2015_j サンプル通った

```rust
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
```

## TODO

- update query
