use privacy_vs_compliance::merkle::MerkleTree;

#[test]
fn merkle_root_is_deterministic() {
    let mut tree = MerkleTree::new();
    tree.insert([1u8; 32]);
    tree.insert([2u8; 32]);

    let root1 = tree.root();
    let root2 = tree.root();

    assert_eq!(root1, root2);
}
