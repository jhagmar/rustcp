use std::{
    cmp::Ordering,
    ops::{Add, AddAssign},
};

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

pub fn quickselect<T>(nums: &mut [T], lo: usize, hi: usize, k: usize) -> T
where
    T: std::cmp::PartialOrd + Copy,
{
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

pub struct SegmentTree<T> {
    n: usize,
    tree: Vec<T>,
}

impl<T> SegmentTree<T>
where
    T: Copy + Add<Output = T> + Default + AddAssign,
{
    pub fn new(nums: &[T]) -> Self {
        let n = nums.len();
        let mut tree: Vec<_> = std::iter::repeat(T::default())
            .take(n)
            .chain(nums.iter().copied())
            .collect();
        for i in (1..n).rev() {
            tree[i] = tree[2 * i] + tree[2 * i + 1];
        }
        Self { n, tree }
    }

    pub fn update(&mut self, index: usize, val: T) {
        let mut i = index + self.n;
        self.tree[i] = val;
        while i > 0 {
            let (child1, child2) = (i, if i % 2 == 0 { i + 1 } else { i - 1 });
            i /= 2;
            self.tree[i] = self.tree[child1] + self.tree[child2];
        }
    }

    pub fn sum_range(&self, left: usize, right: usize) -> T {
        let (mut left, mut right) = (left as usize + self.n, right as usize + self.n);
        let mut sum = T::default();
        while left <= right {
            if left % 2 == 1 {
                sum += self.tree[left];
                left += 1;
            }
            if right % 2 == 0 {
                sum += self.tree[right];
                right -= 1;
            }
            left /= 2;
            right /= 2;
        }
        sum
    }
}

/// Calculates length of longest increasing subsequence.
///
/// Time complexity: O(n * log(n))
/// Space complexity: O(n)
///
/// # Arguments
///
/// * `nums` a slice of numbers, or anything that is `Ord + Copy`
///
/// Examples
///
/// ```
/// # use rustcp::lis_len;
/// assert_eq!(lis_len(&[10,9,2,5,3,7,101,18]), 4);
/// ```
///
/// ```
/// # use rustcp::lis_len;
/// assert_eq!(lis_len(&[0,1,0,3,2,3]), 4);
/// ```
///
/// ```
/// # use rustcp::lis_len;
/// assert_eq!(lis_len(&[7,7,7,7,7,7,7]), 1);
/// ```
pub fn lis_len<T>(nums: &[T]) -> usize
where
    T: Ord + Copy,
{
    let mut sub = Vec::with_capacity(nums.len());
    for num in nums {
        if let Err(index) = sub.binary_search(num) {
            if index == sub.len() {
                sub.push(*num);
            } else {
                sub[index] = *num;
            }
        }
    }
    sub.len()
}

pub type TopoSortGraph = [Vec<usize>];
#[derive(Copy, Clone)]
enum TopoSortMark {
    Cleared,
    Temporary,
    Permanent,
}

use std::collections::VecDeque;

fn toposort_dfs(graph: &TopoSortGraph, marks: &mut [TopoSortMark], n: usize, mut rez: VecDeque<usize>) -> Option<VecDeque<usize>> {
    match marks[n] {
        TopoSortMark::Cleared => {
            marks[n] = TopoSortMark::Temporary;
            for m in &graph[n] {
                match toposort_dfs(graph, marks, *m, rez) {
                    Some(r) => rez = r,
                    None => return None,
                }
            }
            rez.push_front(n);
            marks[n] = TopoSortMark::Permanent;
            Some(rez)
        },
        TopoSortMark::Temporary => None,
        TopoSortMark::Permanent => Some(rez),
    }
}

/// Performs topological sorting.
///
/// Time complexity: O(N + E), all (N)odes and (E)dges are visited once
/// Space complexity: O(N) for allocation of marks
///
/// # Arguments
///
/// * `graph` a graph of dependencies
///
/// Examples
///
/// ```
/// # use rustcp::toposort;
/// let graph = [vec![], vec![0], vec![1]];
/// assert_eq!(toposort(&graph), Some(vec![2,1,0]));
/// ```
///
/// ```
/// # use rustcp::toposort;
/// let graph = [vec![2], vec![0], vec![1]];
/// assert_eq!(toposort(&graph), None);
/// ```
pub fn toposort(graph: &TopoSortGraph) -> Option<Vec<usize>> {
    let mut marks = vec![TopoSortMark::Cleared; graph.len()];
    let mut rez = VecDeque::new();
    for n in 0..graph.len() {
        match toposort_dfs(graph, &mut marks, n, rez) {
            Some(r) => rez = r,
            None => return None,
        }
    }
    Some(rez.into_iter().collect())
}

#[cfg(test)]
mod tests;
