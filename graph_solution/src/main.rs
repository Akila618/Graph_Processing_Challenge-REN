use crate::graph::*;

mod utils;
mod graph;

fn main() {
    //read the given csv file
    let csv_data = utils::read_csv().unwrap();
    println!("Size of the data list: {:?}", &csv_data.len());

    //created Hashmap<i32, Vec<i32>>
    let mut graph_1 = graph::Graph::new();
    
    // add graph information to the hashmap
    for edge in csv_data{
        let first_element = 0;
        let second_element  = 1;
        //[2,3] : check if the key is present in map if not create and insert data
        if !(graph_1.graph.contains_key(&edge[first_element])){
            graph_1.graph.insert(edge[first_element], vec![edge[second_element]]);
        }
        else {
            graph_1.graph.get_mut(&edge[first_element]).unwrap().push(edge[second_element]);
        }
    }

}