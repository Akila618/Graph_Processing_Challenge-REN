use std::{collections::HashMap, process::Output};

use crate::Graph;
//use nalgebra_sparse::csr::CsrMatrix;
use sprs::{CsMat, TriMat};



//creating the trasition matrix for the graph
pub fn create_transition_matrix(graph: &Graph) -> CsMat<f64>{ 

    /*
    0 -> [1|2]
    1 -> [3]
    2 -> [3]
    3 -> []
    
    iterate through each node in the graph
    get the key
    if the vector is empty of the key m then 1/size
    update the m th row of the transition matrix by setting each element to 1/size
    if thee vector is not empty
    get the length of the vector
    calculate the probability as 1/length
    update the m th row of the transition matrix by setting each element at the index of the vector to the probability
    
    */
    let size = graph.map.len();

    // The TriMat will temporarily hold all non-zero entries.
    let mut trimat = TriMat::new((size, size));

    // a map to match the keys and idexes of the matrix ex: key of graph 10 -> index 0 of key_index_map
    let mut key_index_map: HashMap<i32, usize> = HashMap::new();

    let mut index:usize = 0;

    for key in graph.map.keys() {
        key_index_map.insert(*key, index);
        index += 1;
    }
    
    for (selected_node, target_nodes) in &graph.map {

        let out_degree = target_nodes.len() as f64;

        if out_degree == 0.0 {

            let prob = 1.0 / size as f64;
            let mut counter = 0;

            while counter < size as i32{
                let row = *key_index_map.get(selected_node).unwrap() as usize;
                let col = counter as usize;
                trimat.add_triplet(row, col, prob);
                counter += 1;
            }

        } else {

            let len = target_nodes.len();
            let prob = 1.0 / len as f64;

            let mut counter = 0;

            while counter < target_nodes.len() as i32{

                let target_node = target_nodes[counter as usize];
                let row = *key_index_map.get(selected_node).unwrap() as usize;
                let col = *key_index_map.get(&target_node).unwrap() as usize;

                trimat.add_triplet(row, col, prob);

                counter += 1;
            }
            
        }
    }

    // create the csr matrix from the triplet matrix
    trimat.to_csr()

}

