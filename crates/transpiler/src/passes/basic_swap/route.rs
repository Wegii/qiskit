
use pyo3::prelude::*;

use qiskit_circuit::dag_circuit::{DAGCircuit, NodeType};
use qiskit_circuit::{VarsMode};
use qiskit_circuit::operations::{Operation};
use qiskit_circuit::{PhysicalQubit};

use std::convert::Infallible;

use crate::target::{Target};
use crate::passes::sabre::route::{PyRoutingTarget};

use rustworkx_core::dictmap::*;
use rustworkx_core::shortest_path::{dijkstra, distance_matrix};

/// Run basic swap on a circuit
///
/// Returns:
///     A two-tuple of the newly routed :class:`.DAGCircuit`, and the layout that maps virtual
///     qubits to their assigned physical qubits at the *end* of the circuit execution.
#[pyfunction]
#[pyo3(signature=(dag, target))]
pub fn basic_routing(
    dag: &mut DAGCircuit,
    target: &Target
) -> PyResult<Option<DAGCircuit>> {

    //println!("{} days", 31);

    // Construct dag builder
    let new_dag = dag.copy_empty_like(VarsMode::Alike)?;
    //let mut new_dag = new_dag.into_builder();
    let new_dag = new_dag.into_builder();

    // Compute distance matrix
    let routing_target = PyRoutingTarget::from_target(target)?;
    let Some(target) = routing_target.0.as_ref() else {
        // All-to-all coupling.
        println!("Error");
        return Ok(Some(dag.clone()));
    };

    for node in dag.topological_op_nodes()? {

        if let NodeType::Operation(inst) = &dag[node] {
            

            if inst.op.num_qubits() > 1 {

                // Calculate distance

                let qargs = dag.get_qargs(inst.qubits);
                let index0 = qargs[0].index();
                let index1 = qargs[1].index();

                // Calculate physical distance using coupling map
                let dist = &target.distance;
                let distance = dist[[index0, index1]];

                println!("{}", distance);
                
                if distance > 1.0 {
                     
                    // Calculate shortest path between two qubit using dijkstra algorithm from rustworkx 
                    let shortest_path = {
                        let mut shortest_paths: DictMap<PhysicalQubit, Vec<PhysicalQubit>> = DictMap::new();
                        (dijkstra(
                            &target.neighbors,
                            qargs[0],
                            Some(qargs[1]),
                            |_| Ok(1.),
                            Some(&mut shortest_paths),
                        ) as Result<Vec<_>, Infallible>)
                            .expect("error is infallible");
                        shortest_paths
                            .swap_remove(&qargs[1])
                            .expect("target is required to be connected")
                    };

                    println!("{}", shortest_path)
                    /*
                
                    // Add swaps along the shortest path. Divide path in half (swap from target to middle of path, as well as target
                    // to middle of path), in order to reduce depth of circuit.
                    let split: usize = shortest_path.len() / 2;
                    current_swaps.reserve(shortest_path.len() - 2);
                    for i in 0..split {
                        current_swaps.push([shortest_path[i], shortest_path[i + 1]]);  
                    }
                    for i in 0..split - 1 {
                        let end = shortest_path.len() - 1 - i;
                        current_swaps.push([shortest_path[end], shortest_path[end - 1]]);
                    }

                    // Apply swaps on the dag
                    let apply_swap = |swap: &[PhysicalQubit; 2],
                                    layout: &mut NLayout,
                                    dag: &mut DAGCircuitBuilder|
                    -> PyResult<NodeIndex> {
                        layout.swap_physical(swap[0], swap[1]);
                        let swap = PackedInstruction::from_standard_gate(
                            StandardGate::Swap,
                            None,
                            dag.insert_qargs(&[Qubit(map_fn(swap[1]).0), Qubit(map_fn(swap[0]).0)]),
                        );
                        dag.push_back(swap)
                    };
                    current_swaps.iter().for_each(|&swap| self.apply_swap(swap));
                    */

                } else {
                    // Swapping not needed! Add the gate to the dag
                    // TODO: Add gate to dag
                }
            } else {
                // one-qubit gate. Add gate to the dag
                // TODO: Add gate to dag
            }
        } else {
            unreachable!("Op nodes contain a non-operation");
        }   
    }


    // TODO: qubits

    
    
    // TODO: current_swaps





    // Ok(new_dag)
    Ok(Some(new_dag.build()))
}