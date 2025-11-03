
use pyo3::prelude::*;

use qiskit_circuit::dag_circuit::{DAGCircuit, NodeType};
use qiskit_circuit::{VarsMode};
use qiskit_circuit::operations::{Operation, StandardGate};
use qiskit_circuit::{PhysicalQubit, Qubit, VirtualQubit};
use qiskit_circuit::packed_instruction::PackedInstruction;

use std::convert::Infallible;

use crate::target::{Target};
use crate::passes::sabre::route::{PyRoutingTarget};

use rustworkx_core::dictmap::*;
use rustworkx_core::shortest_path::{dijkstra};

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

    println!("Starting");

    // Construct dag builder
    let new_dag_copy = dag.copy_empty_like(VarsMode::Alike)?;
    //let mut new_dag = new_dag.into_builder();
    let mut new_dag = new_dag_copy.into_builder();

    // Compute distance matrix
    let routing_target = PyRoutingTarget::from_target(target)?;
    let Some(target) = routing_target.0.as_ref() else {
        // All-to-all coupling.
        println!("Error");
        return Ok(Some(dag.clone()));
    };

    // TODO: Generate layout for mapping virtual to physical qubits
    /* 
    for node in dag.topological_op_nodes()? {

        if let NodeType::Operation(inst) = &dag[node] {
            

            if inst.op.num_qubits() > 1 {
                //println!("two-qubit gate");

                // Calculate distance
                // Get qubits and convert to physical
                let qargs = dag.get_qargs(inst.qubits);
                let index0_n = qargs[0].index() as u32;
                let index1_n = qargs[1].index() as u32;
                let index0 = PhysicalQubit::new(index0_n); //VirtualQubit::new(index0_n).to_phys(&layout);
                let index1 = PhysicalQubit::new(index1_n); //VirtualQubit::new(index1_n).to_phys(&layout); 
                //let a = layout.virt_to_phys[qargs[0].index()]
                //let b = layout.virt_to_phys[qargs[1].index()]

                // Calculate physical distance using coupling map
                let dist = &target.distance;
                let distance = dist[[index0.index(), index1.index()]];

                //println!("{}", distance);
                
                if distance > 1.0 {
                    /*
                    // Calculate shortest path between two qubit using dijkstra algorithm from rustworkx 
                    let shortest_path = {
                        let mut shortest_paths: DictMap<PhysicalQubit, Vec<PhysicalQubit>> = DictMap::new();

                        (dijkstra(
                            &target.neighbors,
                            index0,
                            Some(index1),
                            |_| Ok(1.),
                            Some(&mut shortest_paths),
                        ) as Result<Vec<_>, Infallible>)
                            .expect("error is infallible");
                        
                        // Return path from source to target, by removing source from the local shortest path and
                        // returning it (swap_remove fn of Vec struct).
                        shortest_paths
                            .swap_remove(&index1)
                            .expect("target is required to be connected")
                    };

                    // Add swaps along the shortest path. Divide path in half (swap from target to middle of path, as well as target
                    // to middle of path), in order to reduce depth of circuit.
                    let split: usize = shortest_path.len() / 2;
                    //current_swaps.reserve(shortest_path.len() - 2);
                    for i in 0..split {
                        //current_swaps.push([shortest_path[i], shortest_path[i + 1]]);  
                        //println!("{}", shortest_path[i].0);
                        //println!("{}", shortest_path[i+1].0);
                        let swap = PackedInstruction::from_standard_gate(
                            StandardGate::Swap,
                            None,
                            new_dag.insert_qargs(&[Qubit(shortest_path[i].0), Qubit(shortest_path[i+1].0)]),
                        );
                        let _ = new_dag.push_back(swap);
                    }
                    
                    for i in 0..split - 1 {
                        let end = shortest_path.len() - 1 - i;
                        // current_swaps.push([shortest_path[end], shortest_path[end - 1]]);
                        //println!("{}", shortest_path[end].0);
                        //println!("{}", shortest_path[end-1].0);

                        let swap = PackedInstruction::from_standard_gate(
                            StandardGate::Swap,
                            None,
                            new_dag.insert_qargs(&[Qubit(shortest_path[i].0), Qubit(shortest_path[i+1].0)]),
                        );
                        let _ = new_dag.push_back(swap);
                    }
                    
                    let new_op = PackedInstruction::from_standard_gate(
                        (inst.op).standard_gate(),
                        None,
                        new_dag.insert_qargs(&[Qubit(shortest_path[split-1].0), Qubit(shortest_path[split].0)]),
                    );
                    let _ = new_dag.push_back(new_op);
                    // TODO: Change qubits, since we swapped
                    // let _ = new_dag.push_back(inst.clone());


                    // TODO: Swap back

                    for i in (0..split).rev() {
                        let swap = PackedInstruction::from_standard_gate(
                            StandardGate::Swap,
                            None,
                            new_dag.insert_qargs(&[Qubit(shortest_path[i].0), Qubit(shortest_path[i+1].0)]),
                        );
                        let _ = new_dag.push_back(swap);
                    }
                    
                    for i in (0..split - 1).rev() {
                        let end = shortest_path.len() - 1 - i;
                        // current_swaps.push([shortest_path[end], shortest_path[end - 1]]);
                        //println!("{}", shortest_path[end].0);
                        //println!("{}", shortest_path[end-1].0);

                        let swap = PackedInstruction::from_standard_gate(
                            StandardGate::Swap,
                            None,
                            new_dag.insert_qargs(&[Qubit(shortest_path[i].0), Qubit(shortest_path[i+1].0)]),
                        );
                        let _ = new_dag.push_back(swap);
                    }
                     */

                } else {
                    // Swapping not needed! Add the gate to the dag
                    // TODO: Add gate to dag
                    let _ = new_dag.push_back(inst.clone());
                }
            } else {
                // one-qubit gate. Add gate to the dag
                // TODO: Add gate to dag
                //println!("single qubit gate");
                let _ = new_dag.push_back(inst.clone());

            }
        } else {
            unreachable!("Op nodes contain a non-operation");
        }   
    }

    */
    println!("Done");

    // Ok(new_dag)
    Ok(Some(new_dag.build()))
}