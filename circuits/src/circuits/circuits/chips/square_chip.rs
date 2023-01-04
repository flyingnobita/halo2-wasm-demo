use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{AssignedCell, Layouter, Value},
    plonk::{Advice, Column, ConstraintSystem, Error, Instance, Selector},
    poly::Rotation,
};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct SquareChipConfig {
    pub col_a: Column<Advice>,
    pub col_b: Column<Advice>,
    pub col_instance: Column<Instance>,
    pub col_selector: Selector,
}

#[derive(Debug, Clone)]
pub struct SquareChip<F: FieldExt> {
    config: SquareChipConfig,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> SquareChip<F> {
    pub fn construct(config: SquareChipConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    pub fn configure(meta: &mut ConstraintSystem<F>) -> SquareChipConfig {
        let col_a = meta.advice_column();
        let col_b = meta.advice_column();
        let col_selector = meta.selector();
        let col_instance = meta.instance_column();

        meta.enable_equality(col_a);
        meta.enable_equality(col_b);
        meta.enable_equality(col_instance);

        meta.create_gate("square", |meta| {
            //
            // col_a | col_b | col_instance | col_selector
            //   a   |   a   |   instance   |   selector

            let selector = meta.query_selector(col_selector);
            let a = meta.query_advice(col_a, Rotation::cur());
            let b = meta.query_advice(col_b, Rotation::cur());

            vec![selector * (a.clone() * a - b)]
        });

        SquareChipConfig {
            col_a,
            col_b,
            col_selector,
            col_instance,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn load(
        &self,
        mut layouter: impl Layouter<F>,
        value: Value<F>,
    ) -> Result<(AssignedCell<F, F>, AssignedCell<F, F>), Error> {
        layouter.assign_region(
            || "first row",
            |mut region| {
                self.config.col_selector.enable(&mut region, 0)?;

                let cell_a = region.assign_advice(|| "a", self.config.col_a, 0, || value)?;
                let cell_b = region.assign_advice(
                    || "a * a",
                    self.config.col_b,
                    0,
                    || cell_a.value().copied() * cell_a.value(),
                )?;

                Ok((cell_a, cell_b))
            },
        )
    }

    pub fn expose_public(
        &self,
        mut layouter: impl Layouter<F>,
        cell: &AssignedCell<F, F>,
        row: usize,
    ) -> Result<(), Error> {
        layouter.constrain_instance(cell.cell(), self.config.col_instance, row)
    }
}
