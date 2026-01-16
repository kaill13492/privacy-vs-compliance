use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance},
};
use halo2_gadgets::poseidon::{
    primitives::{ConstantLength, P128Pow5T3 as PoseidonSpec},
    Hash as PoseidonHash,
};
use pasta_curves::pallas;

/// Proves: leaf ∈ MerkleTree(root)
#[derive(Clone, Debug)]
pub struct MerkleInclusionCircuit {
    pub leaf: Value<pallas::Base>,
    pub path: Vec<Value<pallas::Base>>,
    pub index_bits: Vec<Value<pallas::Base>>, // 0 or 1
}

#[derive(Clone, Debug)]
pub struct MerkleConfig {
    pub advice: Column<Advice>,
    pub root: Column<Instance>,
}

impl Circuit<pallas::Base> for MerkleInclusionCircuit {
    type Config = MerkleConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            leaf: Value::unknown(),
            path: vec![Value::unknown(); self.path.len()],
            index_bits: vec![Value::unknown(); self.index_bits.len()],
        }
    }

    fn configure(meta: &mut ConstraintSystem<pallas::Base>) -> Self::Config {
        let advice = meta.advice_column();
        let root = meta.instance_column();

        meta.enable_equality(advice);
        meta.enable_equality(root);

        MerkleConfig { advice, root }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<pallas::Base>,
    ) -> Result<(), Error> {
        let poseidon = PoseidonHash::<
            pallas::Base,
            PoseidonSpec,
            ConstantLength<2>,
            3,
            2,
        >::init();

        let mut current = self.leaf;

        for (i, sibling) in self.path.iter().enumerate() {
            let index = self.index_bits[i];

            current = layouter.assign_region(
                || format!("merkle level {}", i),
                |mut region| {
                    let cur = region.assign_advice(
                        || "current",
                        config.advice,
                        0,
                        || current,
                    )?;

                    let sib = region.assign_advice(
                        || "sibling",
                        config.advice,
                        1,
                        || *sibling,
                    )?;

                    let bit = region.assign_advice(
                        || "index",
                        config.advice,
                        2,
                        || index,
                    )?;

                    // conditional swap:
                    // if bit == 0: (cur, sib)
                    // if bit == 1: (sib, cur)
                    let left = cur.value().zip(bit.value()).map(|(c, b)| {
                        if b == pallas::Base::zero() { c } else { sib.value().unwrap() }
                    });

                    let right = cur.value().zip(bit.value()).map(|(c, b)| {
                        if b == pallas::Base::zero() { sib.value().unwrap() } else { c }
                    });

                    let l = region.assign_advice(
                        || "left",
                        config.advice,
                        3,
                        || left,
                    )?;

                    let r = region.assign_advice(
                        || "right",
                        config.advice,
                        4,
                        || right,
                    )?;

                    let out = poseidon.hash(
                        layouter.namespace(|| "poseidon"),
                        [l, r],
                    )?;

                    Ok(out.value().copied())
                },
            )?;
        }

        layouter.constrain_instance(
            current.cell().unwrap(),
            config.root,
            0,
        )?;

        Ok(())
    }
}
