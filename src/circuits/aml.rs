use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error},
    poly::Rotation,
};
use pasta_curves::pallas;

const NUM_BITS: usize = 16; // max amount = 65535 (zmień wg AML)

#[derive(Clone, Debug)]
pub struct AmlRangeCircuit {
    pub amount: Value<pallas::Base>,
}

#[derive(Clone, Debug)]
pub struct AmlConfig {
    pub amount: Column<Advice>,
    pub bits: [Column<Advice>; NUM_BITS],
}

impl Circuit<pallas::Base> for AmlRangeCircuit {
    type Config = AmlConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            amount: Value::unknown(),
        }
    }

    fn configure(meta: &mut ConstraintSystem<pallas::Base>) -> Self::Config {
        let amount = meta.advice_column();
        let bits = array_init::array_init(|_| meta.advice_column());

        meta.enable_equality(amount);
        bits.iter().for_each(|c| meta.enable_equality(*c));

        // bit constraint: b * (b - 1) = 0
        for bit in bits.iter() {
            meta.create_gate("boolean constraint", |meta| {
                let b = meta.query_advice(*bit, Rotation::cur());
                b.clone() * (b - pallas::Base::one())
            });
        }

        // amount = sum(bits * 2^i)
        meta.create_gate("recompose amount", |meta| {
            let mut acc = meta.query_advice(amount, Rotation::cur());
            for (i, bit) in bits.iter().enumerate() {
                let b = meta.query_advice(*bit, Rotation::cur());
                acc = acc - b * pallas::Base::from(1u64 << i);
            }
            acc
        });

        AmlConfig { amount, bits }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<pallas::Base>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "aml range proof",
            |mut region| {
                let amount_val = self.amount;

                region.assign_advice(
                    || "amount",
                    config.amount,
                    0,
                    || amount_val,
                )?;

                let amount_u64 = amount_val
                    .map(|a| a.get_lower_64())
                    .unwrap_or(0);

                for i in 0..NUM_BITS {
                    let bit = (amount_u64 >> i) & 1;
                    region.assign_advice(
                        || format!("bit {}", i),
                        config.bits[i],
                        0,
                        || Value::known(pallas::Base::from(bit)),
                    )?;
                }

                Ok(())
            },
        )
    }
}
