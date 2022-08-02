use std::cmp::Ordering;

pub struct UnionFind {
    root: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            root: (0..n).collect(),
            rank: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.root[x] == x {
            x
        } else {
            self.root[x] = self.find(self.root[x]);
            self.root[x]
        }
    }

    pub fn union_set(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            match self.rank[root_x].cmp(&self.rank[root_y]) {
                Ordering::Greater => self.root[root_y] = root_x,
                Ordering::Less => self.root[root_x] = root_y,
                Ordering::Equal => {
                    self.root[root_y] = root_x;
                    self.rank[root_x] += 1;
                }
            }
            true
        } else {
            false
        }
    }

    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn n_sets(&mut self) -> usize {
        let n = self.root.len();
        let mut found = vec![false; n];
        let mut count = 0;
        for i in 0..n {
            let r = self.find(i);
            if !found[r] {
                count += 1;
                found[r] = true;
            }
        }
        count
    }

}

#[cfg(test)]
mod tests;

