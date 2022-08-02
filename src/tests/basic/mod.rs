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
