use halo2_proofs::circuit::{AssignedCell, Layouter, SimpleFloorPlanner};
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

#[allow(clippy::type_complexity)]
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
        // | row  | a | b | c | instance | selector
        // | 0    | 1 | 1 | 2 | 1        | 1
        // | 1    | 1 | 2 | 3 | 1        |

        layouter.assign_region(
            || "row 0",
            |mut region| {
                // turn on the selector at the given offset for this region
                config.selector.enable(&mut region, 0)?;

                let a_cell = region.assign_advice_from_instance(
                    || "a",
                    config.instance,
                    0, // row where the instance is (public input)
                    config.col_a,
                    0,
                )?;

                let b_cell = region.assign_advice_from_instance(
                    || "b",
                    config.instance,
                    1, // row where the next instance is
                    config.col_b,
                    0,
                )?;

                let c_cell = region.assign_advice(
                    || "a + b = c",
                    config.col_c,
                    0,
                    || a_cell.value().copied() + b_cell.value(),
                )?;

                Ok((a_cell, b_cell, c_cell))
            },
        )
    }

    pub fn assign_row(
        &self,
        mut layouter: impl Layouter<Fr>,
        prev_b: &AssignedCell<Fr, Fr>,
        prev_c: &AssignedCell<Fr, Fr>,
        config: &FibonacciConfig,
    ) -> Result<AssignedCell<Fr, Fr>, Error> {
        // | row  | a | b | c |
        // | 0    | 1 | 1 | 2 |
        // | 1    | 1 | 2 | 3 |
        // | 2    | 2 | 3 | 5 |
        // | 3    | 3 | 5 | 8 |

        layouter.assign_region(
            || "next row",
            |mut region| {
                // turn on the selector at the given offset for this region
                config.selector.enable(&mut region, 0)?;

                // Copy the value from b & c in previous row to a & b in current row

                prev_b.copy_advice(|| "a", &mut region, config.col_a, 0)?;

                prev_c.copy_advice(|| "b", &mut region, config.col_b, 0)?;

                let c_cell = region.assign_advice(
                    || "a + b = c",
                    config.col_c,
                    0,
                    || prev_b.value().copied() + prev_c.value(),
                )?;

                Ok(c_cell)
            },
        )
    }

    pub fn expose_public(
        &self,
        mut layouter: impl Layouter<Fr>,
        cell: &AssignedCell<Fr, Fr>,
        config: &FibonacciConfig,
    ) -> Result<(), Error> {
        //
        // | row  | advice_3 (c) | instance |
        // | 0    |              | 1        |
        // | 1    |              | 1        |
        // | 2    | f(9)         | 55       |

        layouter.constrain_instance(cell.cell(), config.instance, 2)
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
        let (_, mut prev_b, mut prev_c) =
            self.assign_row_zero(layouter.namespace(|| "row 0"), &config)?;

        for _i in 3..10 {
            let c_cell =
                self.assign_row(layouter.namespace(|| "next row"), &prev_b, &prev_c, &config)?;
            prev_b = prev_c;
            prev_c = c_cell;
        }

        self.expose_public(layouter.namespace(|| "out"), &prev_c, &config)?;

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
