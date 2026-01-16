use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance},
    poly::Rotation,
};
use pasta_curves::pallas;
use sha2::{Digest, Sha256};

#[derive(Clone, Debug)]
pub struct TransferCircuit {
    pub amount: Value<u64>,
    pub blinding: Value<[u8; 32]>,
}

#[derive(Clone, Debug)]
pub struct TransferConfig {
    amount: Column<Advice>,
    blinding: Column<Advice>,
    commitment: Column<Instance>,
}

impl TransferCircuit {
    pub fn commitment(amount: u64, blinding: [u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(amount.to_le_bytes());
        hasher.update(blinding);
        hasher.finalize().into()
    }
}

impl Circuit<pallas::Base> for TransferCircuit {
    type Config = TransferConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            amount: Value::unknown(),
            blinding: Value::unknown(),
        }
    }

    fn configure(meta: &mut ConstraintSystem<pallas::Base>) -> Self::Config {
        let amount = meta.advice_column();
        let blinding = meta.advice_column();
        let commitment = meta.instance_column();

        meta.enable_equality(amount);
        meta.enable_equality(blinding);
        meta.enable_equality(commitment);

        TransferConfig {
            amount,
            blinding,
            commitment,
        }
    }

    fn synthesize(
        &self,
        _config: Self::Config,
        _layouter: impl Layouter<pallas::Base>,
    ) -> Result<(), Error> {
        // Hash constraint implemented off-circuit for now
        // Next step: in-circuit hash (Poseidon)
        Ok(())
    }
}
