use std::collections::HashMap;
mod utils;
mod graph;

//=============================is a directed acyclic graph (DAG)===================

//=============================calculate maximum outdegree ========================
pub fn calculate_outdegree(graph: &graph::Graph)-> i32{
    let mut max_outdegree:i32 = 0;
    for (_key, value) in &graph.map{
        let outdegree:i32 = value.len() as i32;
        if outdegree > max_outdegree{
            max_outdegree = outdegree;
        }
    }
    max_outdegree
    
}

//==============================main function=====================================
fn main() {
    //read the given csv file
    let csv_data = utils::read_csv().unwrap();
    let mut graph_1 = graph::Graph::new();
    let mut indegree_map:HashMap<i32, i32> = HashMap::new();

    println!("Size of the data list: {:?}", &csv_data.len());

    // add graph information to the hashmap
    for edge in csv_data{
        let first_element = 0;
        let second_element  = 1;

        let key = edge[first_element];
        let value = edge[second_element];

        // calculate indegree
        if indegree_map.contains_key(&value){
            let count = indegree_map.get(&value).unwrap();
            indegree_map.insert(value, count+1);
        }
        else {
            indegree_map.insert(value, 1);
        }
        // ======================================


        //check if the key is present or create
        if !(graph_1.map.contains_key(&edge[first_element])){
            graph_1.map.insert(key, vec![value]);
        }
        else {
            graph_1.map.get_mut(&key).unwrap().push(value);
        }
    }

    // count maximum outdegree and indegree
    let out_deg:i32 = calculate_outdegree(&graph_1);
    println!("max_out_degree: {}", out_deg);

    let in_deg = indegree_map.values().max().unwrap();
    println!("max_in_degree: {}", in_deg);

    
}