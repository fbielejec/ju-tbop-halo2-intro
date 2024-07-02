use halo2_proofs::circuit::{AssignedCell, Layouter, SimpleFloorPlanner, Value};
use halo2_proofs::dev::MockProver;
use halo2_proofs::plonk::{Circuit, ConstraintSystem, Error};
use halo2curves::bn256::Fr;

use crate::circuit_config::FibonacciConfig;
use crate::gate::create_fibonacci_gate;

/// Column setup.
mod circuit_config;
/// gate.
mod gate;

/// relation
#[derive(Default)]
struct FibonacciCircuit {}

// TODO
impl FibonacciCircuit {
    pub fn assign_row_zero(
        &self,
        mut layouter: impl Layouter<Fr>,
        config: &FibonacciConfig,
    ) -> Result<
        (
            AssignedCell<Fr, Fr>,
            AssignedCell<Fr, Fr>,
            AssignedCell<Fr, Fr>,
        ),
        Error,
    > {
        // | row  | a | b | c | instance |
        // | 0    | 1 | 1 | 2 | 1        |
        // | 1    | 1 | 2 | 3 | 1        |
        // |      |   |   |   | 55       |

        layouter.assign_region(
            || "row 0",
            |mut region| {
                config.selector.enable(&mut region, 0)?;

                let a_cell = region.assign_advice_from_instance(
                    || "fib(0) = 1",
                    config.instance,
                    0, // row where the instance is (public input)
                    config.col_a,
                    0,
                )?;

                let b_cell = region.assign_advice_from_instance(
                    || "fib(1) = 1",
                    config.instance,
                    1, // row where the next instance is
                    config.col_b,
                    0,
                )?;

                let c_cell = region.assign_advice(
                    || "fib(3) = f(0) + f(1)",
                    config.col_c,
                    0,
                    || a_cell.value() + b_cell.value(),
                )?;

                Ok((a_cell, b_cell, c_cell))
            },
        )
    }

    pub fn assign_row(
        &self,
        mut layouter: impl Layouter<Fr>,
        config: &FibonacciConfig,
    ) -> Result<AssignedCell<Fr, Fr>, Error> {
        todo!("")
    }
}

impl Circuit<Fr> for FibonacciCircuit {
    // We are using our own column setup.
    type Config = FibonacciConfig;
    // We use the simplest layouting.
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    // Setting up the table shape (its columns and available gates).
    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self::Config {
        // ====== Columns ======
        let config = FibonacciConfig::new(meta);
        // ====== Gates   ======
        create_fibonacci_gate(meta, &config);

        config
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fr>,
    ) -> Result<(), Error> {
        self.assign_row_zero(layouter.namespace(|| "row 0"), &config)?;

        Ok(())
    }
}

fn main() {
    let a = Fr::from(1);
    let b = Fr::from(1);
    let c = Fr::from(55);

    let relation_instance = FibonacciCircuit {};

    let public_input = vec![a, b, c];

    MockProver::run(4, &relation_instance, vec![public_input])
        .unwrap()
        .assert_satisfied();
}
