use crate::circuit_config::StandardPlonkConfig;
use halo2_proofs::arithmetic::Field;
use halo2_proofs::plonk::ConstraintSystem;
use halo2_proofs::poly::Rotation;

/// Register the universal gate in the constraint system using registered columns.
pub fn create_universal_plonk_gate<Fr: Field>(
    meta: &mut ConstraintSystem<Fr>,
    config: &StandardPlonkConfig,
) {
    meta.create_gate("mul", |meta| {
        // Query the advice columns. `Rotation` is the relative offset from the current table row.
        let [input] = [config.input].map(|column| meta.query_advice(column, Rotation::cur()));

        // Query the selectors.
        let [q_product] = [config.q_product].map(|column| meta.query_fixed(column));

        // Query the constant column.
        let constant = meta.query_fixed(config.constant);

        // Query the instance column.
        let instance = meta.query_instance(config.instance, Rotation::cur());

        // Return a list of expressions that should be equal to zero.
        vec![q_product * (input * instance + constant)]
    });
}
