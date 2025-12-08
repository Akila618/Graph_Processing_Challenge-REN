use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Graph{
    pub graph: HashMap<i32, Vec<i32>>
}

impl Graph {
    pub fn new()->Self{
        Graph {
            graph: HashMap::new()
        }
    }
}