use std::collections::HashMap;
use csv::Error;
mod utils;
mod graph;

#[derive(Debug, Clone)]
pub struct Node{
    pub flag: i32,
    pub parent: Option<i32>,
    pub node: i32,
}

pub struct Graph{
    pub map: HashMap<i32, Vec<Node>>
}

impl Graph {
    pub fn new()->Self{
        Graph {
            map: HashMap::new()
        }
    }
}

//=============================is a directed acyclic graph (DAG)===================
/*

Mark each key as non-visited (0), visiting (1), or visited (2).
declare stack
stert iterating the keys of the graph
ierate all the keys
if the key is already visited (2) skip
if the key is non-visited (0) start DFS
push the key to stack
iterate till stack is empty
pop a key from the stack
check its state
if visited (2) continue
if visiting (1) mark it as visited (2) and continue
if non-visited (0)
mark it as visiting (1)
push it back to stack
check all its nodes
for each node
get its state
if node is visiting (1) return false : a cycle
if node is non-visited (0) push it to stack
*/


pub fn is_dag(graph: &Graph) -> bool {
    // 0 = unvisited, 1 = visiting, 2 = visited
    let mut node_state: HashMap<i32, i32> = HashMap::new();
    let mut stack: Vec<i32> = Vec::new();
    
    for &start_key in graph.map.keys() {

        if node_state.get(&start_key).unwrap_or(&0) != &0 {
            continue;
        }
        else{
            // Start DFS
            stack.push(start_key);
            
            while let Some(current) = stack.pop() {
                let current_state = *node_state.get(&current).unwrap_or(&0);
                
                if current_state == 2 {
                    continue;
                }
                
                if current_state == 1 {
                    node_state.insert(current, 2);
                    continue;
                }
                
                node_state.insert(current, 1);
                stack.push(current);
                
                // Check nodes of key
                if let Some(neighbors) = graph.map.get(&current) {
                    for neighbor in neighbors {
                        let neighbor_node = neighbor.node;
                        let neighbor_state = *node_state.get(&neighbor_node).unwrap_or(&0);
                        
                        if neighbor_state == 1 {
                            return false;
                        }
                        
                        if neighbor_state == 0 {
                            stack.push(neighbor_node);
                        }
                    }
                }
            }
        }
        
       
    }
    
    true
}

//=============================calculate maximum outdegree ========================
pub fn calculate_outdegree(graph: &Graph)-> i32{
    let mut max_outdegree:i32 = 0;
    for (_key, value) in &graph.map{
        let outdegree:i32 = value.len() as i32;
        if outdegree > max_outdegree{
            max_outdegree = outdegree;
        }
    }
    max_outdegree
    
}

//=============================create graph=======================================
pub fn create_graph(csv_data:Vec<Vec<i32>>)-> Result<Graph, Error>{

    let mut graph_1 = Graph::new();
    let mut indegree_map:HashMap<i32, i32> = HashMap::new();
    
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

        // ====================================================

        //check if the key is present or create graph
        if !(graph_1.map.contains_key(&edge[first_element])){
            graph_1.map.insert(key, vec![
                Node{
                    flag: 0,
                    parent: None,
                    node: value, 
                }
            ]);
        }
        else {
            graph_1.map.get_mut(&key).unwrap().push(
                Node{
                    flag: 0,
                    parent: None,   
                    node: value, 
                }
            );
        }
    }

    // count maximum outdegree and indegree
    let out_deg:i32 = calculate_outdegree(&graph_1);
    println!("max_out_degree: {}", out_deg);

    let in_deg = indegree_map.values().max().unwrap();
    println!("max_in_degree: {}", in_deg);

    Ok(graph_1)
}

//==============================main function=====================================
fn main() {

    //read the given csv file
    let csv_data = utils::read_csv().unwrap();
    println!("Size of the data list: {:?}", &csv_data.len());

    // generate graph
    let graph_result = create_graph(csv_data).unwrap();
    // for (key,value) in &graph_result.map{
    //     //print the correct key and value count
    //     println!("key: {}, value_count: {}", key, value.len());
    // }

    // check is dag
    let is_dag_result:bool = is_dag(&graph_result);
    println!("is_DAG: {}", is_dag_result);

    

    
}
