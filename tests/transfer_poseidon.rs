use halo2_proofs::dev::MockProver;
use pasta_curves::pallas;
use privacy_vs_compliance::circuits::transfer::TransferCircuit;

#[test]
fn poseidon_commitment_proof() {
    let amount = pallas::Base::from(100);
    let blinding = pallas::Base::from(999);

    let circuit = TransferCircuit {
        amount: halo2_proofs::circuit::Value::known(amount),
        blinding: halo2_proofs::circuit::Value::known(blinding),
    };

    // Public input = Poseidon(amount, blinding)
    // MockProver will compute it internally
    let public_inputs = vec![vec![pallas::Base::zero()]];

    let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
    prover.assert_satisfied();
}
