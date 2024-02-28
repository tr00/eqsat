
#[derive(Debug)]
struct UnionFind {
    vec: Vec<i32>,
}

impl UnionFind {

    pub fn new() -> Self {
        return UnionFind { vec: Vec::new() };
    }

    pub fn len(&self) -> usize {
        return self.vec.len();
    }

    pub fn push(&mut self) -> i32 {
        let len = self.len();

        self.vec.push(-1);

        return len as i32;
    }

    pub fn root(&root, x: i32) -> i32 {
        let mut r = x;

        while self.vec[r] >= 0 {
            r = self.vec[r];
        }

        return r;
    }

    pub fn join(&mut self, a: i32, b: i32) -> i32 {
        let mut ra = root(a);
        let mut rb = root(b);

        if ra == rb {
            return rb;
        }

        let mut sa = -self.vec[ra];
        let mut sb = -self.vec[rb];

        if sa > sb {
            (ra, rb) = (rb, ra);
            (sa, sb) = (sb, sa);
        }

        self.vec[rb] -= sa;
        self.vec[ra] = rb;
        
        return rb;
    }

    pub fn same(&self, a: i32, b: i32) -> bool {
        return self.root(a) == self.root(b);
    }
}


