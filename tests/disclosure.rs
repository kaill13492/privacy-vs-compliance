use privacy_vs_compliance::disclosure::ViewingKey;

#[test]
fn selective_disclosure_only_reveals_amount() {
    let vk = ViewingKey::new([1u8; 32], [9u8; 32]);
    let proof = vk.disclose_amount(250);

    assert_eq!(proof.revealed_amount, 250);
    assert_eq!(proof.commitment, [1u8; 32]);
}
