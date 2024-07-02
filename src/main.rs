//! Brief introduction to halo2.

#![deny(missing_docs)]

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
struct FibonacciCircuit {
    // // Fib(0)
    // a: Fr,
    // // Fib(1)
    // b: Fr,
    // // Fib(9)
    // c: Fr,
}

// TODO
impl FibonacciCircuit {
    pub fn assign_first_row(
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
        layouter.assign_region(
            || "first row",
            |mut region| {
                //


                
                todo!("")
                //
            },
        )
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
        self.assign_first_row(layouter.namespace(|| "first row"), &config)?;

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
