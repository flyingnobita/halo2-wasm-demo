use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Circuit, ConstraintSystem, Error},
};

use super::chips::square_chip::{Circuit1Config, SquareChip};

pub struct Circuit1<F: FieldExt> {
    pub a: Value<F>,
}

impl<F: FieldExt> Circuit<F> for Circuit1<F> {
    type Config = Circuit1Config;

    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            a: Value::default(),
        }
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        SquareChip::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let chip = SquareChip::construct(config);

        let (_, b) = chip.load(layouter.namespace(|| ""), self.a)?;

        chip.expose_public(layouter.namespace(|| "out"), &b, 0)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use halo2_proofs::{circuit::Value, dev::MockProver, pasta::Fp};

    use super::Circuit1;

    #[test]
    fn test_circuit_1() {
        let k = 3;

        let a = Fp::from(2);
        let b = a * a;
        // let b = Fp::from(4);

        let circuit = Circuit1 { a: Value::known(a) };

        let public_inputs = vec![b];

        let prover = MockProver::run(k, &circuit, vec![public_inputs.clone()]).unwrap();
        prover.assert_satisfied();
        println!("public_input: {:?}", public_inputs);
    }
}
