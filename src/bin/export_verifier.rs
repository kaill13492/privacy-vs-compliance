use halo2_proofs::{
    plonk::keygen_vk,
    poly::commitment::Params,
};
use halo2_solidity_verifier::SolidityGenerator;
use pasta_curves::pallas;
use privacy_vs_compliance::circuits::transfer::TransferCircuit;

use std::fs::File;
use std::io::Write;

fn main() {
    let params = Params::<pallas::Affine>::new(8);

    let empty_circuit = TransferCircuit {
        amount: halo2_proofs::circuit::Value::unknown(),
        blinding: halo2_proofs::circuit::Value::unknown(),
    };

    let vk = keygen_vk(&params, &empty_circuit).unwrap();

    let solidity = SolidityGenerator::new(&params, &vk)
        .render()
        .unwrap();

    let mut file = File::create("Verifier.sol").unwrap();
    file.write_all(solidity.as_bytes()).unwrap();

    println!("✅ Solidity verifier generated: Verifier.sol");
}

cargo run --bin export_verifier
