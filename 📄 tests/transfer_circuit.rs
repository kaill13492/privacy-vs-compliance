use halo2_proofs::{
    dev::MockProver,
};
use privacy_vs_compliance::circuits::transfer::TransferCircuit;

#[test]
fn transfer_commitment_proof() {
    let amount = 100u64;
    let blinding = [7u8; 32];

    let commitment = TransferCircuit::commitment(amount, blinding);

    let circuit = TransferCircuit {
        amount: halo2_proofs::circuit::Value::known(amount),
        blinding: halo2_proofs::circuit::Value::known(blinding),
    };

    let public_inputs = vec![commitment
        .iter()
        .map(|b| halo2_proofs::pasta::Fp::from(*b as u64))
        .collect()];

    let prover = MockProver::run(4, &circuit, public_inputs).unwrap();
    prover.assert_satisfied();
}
