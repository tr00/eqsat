
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

    pub fn root(&self, x: i32) -> i32 {
        let mut r = x as usize;

        while self.vec[r] >= 0 {
            r = self.vec[r] as usize;
        }

        return r as i32;
    }

    pub fn join(&mut self, a: i32, b: i32) -> i32 {
        let mut ra = self.root(a) as usize;
        let mut rb = self.root(b) as usize;

        if ra == rb {
            return rb as i32;
        }

        let mut sa = -self.vec[ra];
        let mut sb = -self.vec[rb];

        if sa > sb {
            (ra, rb) = (rb, ra);
            (sa, sb) = (sb, sa);
        }

        self.vec[rb] -= sa;
        self.vec[ra] = rb as i32;
        
        return rb as i32;
    }

    pub fn same(&self, a: i32, b: i32) -> bool {
        return self.root(a) == self.root(b);
    }
}

#[cfg(test)]
mod tests {

    use crate::unionfind::*;
    
    #[test]
    fn uf32() {
        let mut uf = UnionFind::new();

        let x1 = uf.push();
        let x2 = uf.push();

        assert_ne!(x1, x2);

        uf.join(x1, x2);

        let r1 = uf.root(x1);
        let r2 = uf.root(x2);

        assert_eq!(r1, r2);
    }
}


