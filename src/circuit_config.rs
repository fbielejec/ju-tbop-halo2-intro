use halo2_proofs::arithmetic::Field;
use halo2_proofs::plonk::{Advice, Column, ConstraintSystem, Instance, Selector};

/// Standard Plonk circuit column set.
#[derive(Copy, Clone)]
pub struct FibonacciConfig {
    // ======= COLUMNS FOR VALUES KNOWN ONLY TO THE PROVER =========================================
    pub col_a: Column<Advice>,
    pub col_b: Column<Advice>,
    pub col_c: Column<Advice>,

    // ======= COLUMNS FOR VALUES THAT ARE FIXED FOREVER AND PUBLICLY KNOWN ========================
    pub selector: Selector,

    // ======= COLUMNS FOR VALUES THAT ARE FIXED FOR A SINGLE PROVER-VERIFIER INTERACTION AND ARE
    // PUBLICLY KNOWN ==============================================================================
    /// Column for the public input.
    pub instance: Column<Instance>,
}

impl FibonacciConfig {
    /// Given a constraint system, returns a new configuration.
    ///
    /// This method should register all required columns in the constraint system and gather them
    /// into a configuration object.
    ///
    /// This is also the place where equality constraints can be enabled for particular columns.
    pub fn new<Fr: Field>(meta: &mut ConstraintSystem<Fr>) -> Self {
        let col_a = meta.advice_column();
        let col_b = meta.advice_column();
        let col_c = meta.advice_column();
        let selector = meta.selector();
        let instance = meta.instance_column();

        meta.enable_equality(col_a);
        meta.enable_equality(col_b);
        meta.enable_equality(col_c);
        meta.enable_equality(instance);

        Self {
            col_a,
            col_b,
            col_c,
            selector,
            instance,
        }
    }
}
