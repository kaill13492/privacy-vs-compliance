use halo2_proofs::dev::MockProver;
use pasta_curves::pallas;
use privacy_vs_compliance::circuits::aml::AmlRangeCircuit;

#[test]
fn aml_range_proof_valid() {
    let amount = pallas::Base::from(4200);

    let circuit = AmlRangeCircuit {
        amount: halo2_proofs::circuit::Value::known(amount),
    };

    let prover = MockProver::run(6, &circuit, vec![]).unwrap();
    prover.assert_satisfied();
}
