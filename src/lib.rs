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

pub fn hoare_partition<T>(nums: &mut [T], mut lo: usize, mut hi: usize, pivot_index: usize) -> usize
where
    T: std::cmp::PartialOrd + Copy,
{
    let pivot = nums[pivot_index];
    let n = hi;

    while hi <= n && lo < hi {
        while nums[lo] < pivot {
            lo += 1;
        }

        while hi > 0 && nums[hi] >= pivot {
            hi -= 1;
        }

        if lo < hi {
            nums.swap(lo, hi);
        }
    }
    lo
}

pub fn lomuto_partition<T>(nums: &mut [T], left: usize, right: usize, pivot_index: usize) -> usize
where
    T: std::cmp::PartialOrd + Copy,
{
    let pivot = nums[pivot_index];

    nums.swap(pivot_index, right);
    let mut store_index = left;

    for i in left..right {
        if nums[i] < pivot {
            nums.swap(store_index, i);
            store_index += 1;
        }
    }

    nums.swap(store_index, right);
    store_index
}


fn quickselect<T>(nums: &mut [T], lo: usize, hi: usize, k: usize) -> T
where
    T: std::cmp::PartialOrd + Copy {
    if lo == hi {
        nums[lo]
    } else {
        let pivot_index = lo + (hi - lo) / 2;
        let pivot = nums[pivot_index];

        let pivot_index = lomuto_partition(nums, lo, hi, pivot_index);

        match k.cmp(&pivot_index) {
            Ordering::Equal => pivot,
            Ordering::Less => quickselect(nums, lo, pivot_index - 1, k),
            Ordering::Greater => quickselect(nums, pivot_index + 1, hi, k),
        }
    }
}

#[cfg(test)]
mod tests;
