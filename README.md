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

## 重心

```rust
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
```

select

```rust
let (x, y) = select(v[a], |a, b, av, bv, cv| {
    if a.inter_weight + av + cv >= b.inter_weight + bv + cv { 0 }
    else { 1 }
    });
```

## TODO

- update query
