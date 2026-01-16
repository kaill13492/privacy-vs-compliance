use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance},
};
use halo2_gadgets::poseidon::{
    primitives::{ConstantLength, P128Pow5T3 as PoseidonSpec},
    Hash as PoseidonHash,
};
use pasta_curves::pallas;

/// Poseidon(amount, blinding) == public commitment
#[derive(Clone, Debug)]
pub struct TransferCircuit {
    pub amount: Value<pallas::Base>,
    pub blinding: Value<pallas::Base>,
}

#[derive(Clone, Debug)]
pub struct TransferConfig {
    pub advice: [Column<Advice>; 2],
    pub commitment: Column<Instance>,
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
        let advice = [meta.advice_column(), meta.advice_column()];
        let commitment = meta.instance_column();

        advice.iter().for_each(|c| meta.enable_equality(*c));
        meta.enable_equality(commitment);

        TransferConfig { advice, commitment }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<pallas::Base>,
    ) -> Result<(), Error> {
        let hash = PoseidonHash::<
            pallas::Base,
            PoseidonSpec,
            ConstantLength<2>,
            3,
            2,
        >::init();

        let commitment = layouter.assign_region(
            || "poseidon hash",
            |mut region| {
                let a = region.assign_advice(
                    || "amount",
                    config.advice[0],
                    0,
                    || self.amount,
                )?;

                let b = region.assign_advice(
                    || "blinding",
                    config.advice[1],
                    0,
                    || self.blinding,
                )?;

                let output =
                    hash.hash(layouter.namespace(|| "hash"), [a, b])?;

                Ok(output)
            },
        )?;

        layouter.constrain_instance(
            commitment.cell(),
            config.commitment,
            0,
        )?;

        Ok(())
    }
}
