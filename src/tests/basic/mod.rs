use super::*;

#[test]
fn union_find_basic_1() {
    let mut uf = UnionFind::new(3);
    assert_eq!(uf.find(0), 0);
    assert_eq!(uf.find(1), 1);
    assert_eq!(uf.find(2), 2);
    assert!(!uf.connected(0, 1));
    assert!(!uf.connected(0, 2));
    assert!(!uf.connected(1, 2));
    assert_eq!(uf.n_sets(), 3);
    uf.union_set(0, 1);
    assert!(uf.connected(0, 1));
    assert!(!uf.connected(0, 2));
    assert!(!uf.connected(1, 2));
    assert_eq!(uf.n_sets(), 2);
}

#[test]
fn hoare_partition_basic_1() {
    let mut data = [5, 3, 1, 6, 9];
    assert_eq!(hoare_partition(&mut data, 0, 4, 0), 2);
    for i in 0..2 {
        assert!(data[i] < 5);
    }
    for i in 2..5 {
        assert!(data[i] >= 5);
    }
}

#[test]
fn lomuto_partition_basic_1() {
    let mut data = [5, 3, 1, 6, 9];
    assert_eq!(lomuto_partition(&mut data, 0, 4, 0), 2);
    for i in 0..1 {
        assert!(data[i] < 5);
    }
    assert_eq!(data[2], 5);
    for i in 3..5 {
        assert!(data[i] > 5);
    }
}

#[test]
fn quickselect_basic_1() {
    let data = [5, 3, 1, 6, 9];
    let sorted = {
        let mut t = data.clone();
        t.sort();
        t
    };
    for (k, s) in sorted.into_iter().enumerate() {
        assert_eq!(quickselect(&mut data.clone(), 0, 4, k), s);
    }
}

#[test]
fn segment_tree_basic_1() {
    let data = [5, 3, 1, 6, 9];
    let mut st = SegmentTree::new(&data);
    assert_eq!(st.sum_range(0, 4), 24);
    assert_eq!(st.sum_range(0, 1), 8);
    assert_eq!(st.sum_range(2, 4), 16);
    assert_eq!(st.sum_range(1, 3), 10);
    st.update(2, 2);
    assert_eq!(st.sum_range(0, 4), 25);
    assert_eq!(st.sum_range(0, 1), 8);
    assert_eq!(st.sum_range(2, 4), 17);
    assert_eq!(st.sum_range(1, 3), 11);
}
