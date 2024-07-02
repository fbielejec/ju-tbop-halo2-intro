use halo2_proofs::arithmetic::Field;
use halo2_proofs::plonk::{Advice, Column, ConstraintSystem, Fixed, Instance, Selector};

/// Standard Plonk circuit column set.
#[derive(Copy, Clone)]
pub struct FibonacciConfig {
    // ======= COLUMNS FOR VALUES KNOWN ONLY TO THE PROVER =========================================
    pub col_a: Column<Advice>,
    pub col_b: Column<Advice>,
    pub col_c: Column<Advice>,

    // /// Column for the left input to a gate.
    // pub left_input: Column<Advice>,
    // /// Column for the right input to a gate.
    // pub right_input: Column<Advice>,
    // /// Column for the output of a gate.
    // pub output: Column<Advice>,

    // ======= COLUMNS FOR VALUES THAT ARE FIXED FOREVER AND PUBLICLY KNOWN ========================

    // /// Selector for the left input to a gate.
    // pub q_left: Column<Fixed>,
    // /// Selector for the right input to a gate.
    // pub q_right: Column<Fixed>,
    // /// Selector for the output of a gate.
    // pub q_output: Column<Fixed>,
    // /// Selector for the product of the left and right inputs to a gate.
    // pub q_product: Column<Fixed>,
    // /// Column for fixed constants.
    // pub constant: Column<Fixed>,
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
