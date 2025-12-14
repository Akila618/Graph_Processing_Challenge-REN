use std::{collections::HashMap};
use crate::Graph;
use sprs::{CsMat, TriMat};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct PageRank{
    pub min_value: f64,
    pub max_value: f64,
}

//==============================create transition matrix======================================
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
    
    let mut all_nodes: HashSet<i32> = HashSet::new();
    
    // add all source keys
    for &key in graph.map.keys() {
        all_nodes.insert(key);
    }
    
    for target_nodes in graph.map.values() {
        for &target in target_nodes {
            all_nodes.insert(target);
        }
    }
    
    let size = all_nodes.len();

    // temporarily hold all entries.
    let mut trimat = TriMat::new((size, size));

    // a map to match the keys and idexes of the matrix ex: key of graph 10 -> index 0 of key_index_map
    let mut key_index_map: HashMap<i32, usize> = HashMap::new();

    let mut index:usize = 0;

    // map nodes to indices
    for &node in &all_nodes {

        key_index_map.insert(node, index);
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
    
    for &node in &all_nodes {
        if !graph.map.contains_key(&node) {

            let prob = 1.0 / size as f64;
            let row = *key_index_map.get(&node).unwrap() as usize;
            
            for col in 0..size {
                trimat.add_triplet(row, col, prob);
            }
        }
    }

    // create the csr matrix from the triplet matrix
    trimat.to_csr()

}


// ==============================calculate page rank==========================================
pub fn calculate_pagerank(tr_matrix: CsMat<f64>, damping_factor: f64, total_iterations: usize) -> PageRank {

    /*
    R(i) = (1 - d) / N + d * (sum of (rank of in degree nodes)/(out degree of in degree nodes))
    also 
    R(i) = (1 - d)E(teleport_vector) + d * (M * Ri); M = transition matrix, Ri = rank vector at iteration i
    */

    let size = &tr_matrix.rows();
    // R0 = 1/N
    let initial_rank = 1.0 / *size as f64;

    // E = (1 - d) / N
    let e_value = (1.0 - damping_factor) / *size as f64; 

    let mut pagerank_vector = vec![initial_rank; *size];
    
    let mut new_pagerank_vector = vec![0.0; *size]; 

    for _counter in 0..total_iterations {
        
        for i in 0..*size {
            new_pagerank_vector[i] = e_value;
        } 
        
        for i in 0..*size {
            
            // outer_view is optimum than using 2 inner loops
            let row = tr_matrix.outer_view(i).expect("error geeting row");
              
            let calculation = damping_factor * pagerank_vector[i];
            
            for (j, &probability) in row.iter() {
                new_pagerank_vector[j] += calculation * probability;
            }
        }

        for i in 0..*size {
            pagerank_vector[i] = new_pagerank_vector[i];
        }
    }

    let mut min_value = 0.0;
    let mut max_value = 0.0;

    for &value in &pagerank_vector {

        if min_value == 0.0 || value < min_value {
            min_value = value;
        }

        if max_value == 0.0 || value > max_value {
            max_value = value;
        }

    }

    PageRank {
        min_value: (min_value * 1_000_000.0).round() / 1_000_000.0,
        max_value: (max_value * 1_000_000.0).round() / 1_000_000.0,
    }
}

// TEST -------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_rank() {
        let matrix = CsMat::new(
            (3, 3), 
            vec![0, 1, 2, 3],
            vec![1, 2, 1],
            vec![1.0, 1.0, 1.0],
        );
        let pagerank = calculate_pagerank(matrix, 0.85, 100);
        assert!(pagerank.min_value > 0.0);
        assert!(pagerank.max_value <= 1.0);
    }
}