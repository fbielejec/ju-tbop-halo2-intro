use crate::circuit_config::FibonacciConfig;
use halo2_proofs::arithmetic::Field;
use halo2_proofs::plonk::ConstraintSystem;
use halo2_proofs::poly::Rotation;

/// Register the universal gate in the constraint system using registered columns.
pub fn create_fibonacci_gate<Fr: Field>(meta: &mut ConstraintSystem<Fr>, config: &FibonacciConfig) {
    meta.create_gate("add", |meta| {
        let s = meta.query_selector(config.selector);
        let a = meta.query_advice(config.col_a, Rotation::cur());
        let b = meta.query_advice(config.col_b, Rotation::cur());
        let c = meta.query_advice(config.col_c, Rotation::cur());

        // Return a list of expressions that should be equal to zero.
        vec![s * (a + b - c)]
    });
}
