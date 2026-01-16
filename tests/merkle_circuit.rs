use halo2_proofs::dev::MockProver;
use pasta_curves::pallas;
use privacy_vs_compliance::circuits::merkle::MerkleInclusionCircuit;

#[test]
fn merkle_inclusion_proof() {
    // Tree of depth 2:
    // root = H(H(leaf, s1), s2)

    let leaf = pallas::Base::from(10);
    let s1 = pallas::Base::from(20);
    let s2 = pallas::Base::from(30);

    let circuit = MerkleInclusionCircuit {
        leaf: halo2_proofs::circuit::Value::known(leaf),
        path: vec![
            halo2_proofs::circuit::Value::known(s1),
            halo2_proofs::circuit::Value::known(s2),
        ],
        index_bits: vec![
            halo2_proofs::circuit::Value::known(pallas::Base::zero()),
            halo2_proofs::circuit::Value::known(pallas::Base::zero()),
        ],
    };

    let public_inputs = vec![vec![pallas::Base::zero()]];

    let prover = MockProver::run(9, &circuit, public_inputs).unwrap();
    prover.assert_satisfied();
}
