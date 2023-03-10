use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Circuit, ConstraintSystem, Error},
};

use super::chips::cube_chip::{CubeChip, CubeChipConfig};

pub struct Circuit2<F: FieldExt> {
    pub a: Value<F>,
}

impl<F: FieldExt> Circuit<F> for Circuit2<F> {
    type Config = CubeChipConfig;

    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            a: Value::default(),
        }
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        CubeChip::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let chip = CubeChip::construct(config);

        let (_, b) = chip.load(layouter.namespace(|| ""), self.a)?;

        chip.expose_public(layouter.namespace(|| "out"), &b, 0)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use halo2_proofs::{circuit::Value, dev::MockProver, pasta::Fp};

    use super::Circuit2;

    #[test]
    fn test_circuit_2() {
        let k = 3;

        let a = Fp::from(2);
        let b = a * a * a;

        let circuit = Circuit2 { a: Value::known(a) };

        let public_inputs = vec![b];

        let prover = MockProver::run(k, &circuit, vec![public_inputs.clone()]).unwrap();
        prover.assert_satisfied();
        println!("public_input: {:?}", public_inputs);
    }
}
