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