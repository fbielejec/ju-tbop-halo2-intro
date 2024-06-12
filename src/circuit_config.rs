use halo2_proofs::arithmetic::Field;
use halo2_proofs::plonk::{Advice, Column, ConstraintSystem, Fixed, Instance};

/// Standard Plonk circuit column set.
#[derive(Copy, Clone)]
pub struct StandardPlonkConfig {
    // ======= COLUMNS FOR VALUES KNOWN ONLY TO THE PROVER =========================================
    /// Column for the left input to a gate.
    pub input: Column<Advice>,

    // ======= COLUMNS FOR VALUES THAT ARE FIXED FOREVER AND PUBLICLY KNOWN ========================
    /// Selector for the product of the left and right inputs to a gate.
    pub q_product: Column<Fixed>,
    /// Column for fixed constants.
    pub constant: Column<Fixed>,

    // ======= COLUMNS FOR VALUES THAT ARE FIXED FOR A SINGLE PROVER-VERIFIER INTERACTION AND ARE
    // PUBLICLY KNOWN ==============================================================================
    /// Column for the public input.
    pub instance: Column<Instance>,
}

impl StandardPlonkConfig {
    /// Given a constraint system, returns a new configuration.
    ///
    /// This method should register all required columns in the constraint system and gather them
    /// into a configuration object.
    ///
    /// This is also the place where equality constraints can be enabled for particular columns.
    pub fn new<Fr: Field>(meta: &mut ConstraintSystem<Fr>) -> Self {
        Self {
            input: meta.advice_column(),

            q_product: meta.fixed_column(),
            constant: meta.fixed_column(),

            instance: meta.instance_column(),
        }
    }
}
