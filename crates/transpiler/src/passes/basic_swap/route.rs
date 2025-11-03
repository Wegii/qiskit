
use pyo3::prelude::*;

use qiskit_circuit::dag_circuit::{DAGCircuit};

/// Run basic swap on a circuit
///
/// Returns:
///     A two-tuple of the newly routed :class:`.DAGCircuit`, and the layout that maps virtual
///     qubits to their assigned physical qubits at the *end* of the circuit execution.
#[pyfunction]
#[pyo3(signature=(dag))]
pub fn basic_routing(
    dag: &DAGCircuit
) -> PyResult<DAGCircuit> {

    println!("{} days", 31);

    return Ok(dag.clone());
}