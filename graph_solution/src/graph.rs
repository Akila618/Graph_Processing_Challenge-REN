use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Graph{
    pub map: HashMap<i32, Vec<i32>>
}

impl Graph {
    pub fn new()->Self{
        Graph {
            map: HashMap::new()
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct Vector{
//     pub vec: Vec<i32>
// }

// impl Vector {
//     pub fn new()->Self{
//         Vector   {
//             vec: Vec::new()
//         }
//     }
// }