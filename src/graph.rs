use std::collections::{HashMap, HashSet, VecDeque};
#[derive(Debug)]
pub struct Graph {
    pub(crate) edges: HashMap<String, HashSet<String>>, 
}
impl Graph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }
    pub fn add_vertex(&mut self, category: &str) {
        self.edges.entry(category.to_string()).or_insert(HashSet::new());
    }
    pub fn add_edge(&mut self, category: &str, country: &str) {
        self.edges
            .entry(category.to_string())
            .or_insert(HashSet::new())
            .insert(country.to_string());
    }

   pub fn get_vertices(&self) -> Vec<String> {
       self.edges.keys().cloned().collect()
       
    }
    
    pub fn calculate_distance(&self, start_vertex: &String, end_vertex: &String) -> Option<usize> {
        if !self.edges.contains_key(start_vertex) || !self.edges.contains_key(end_vertex) {
            return None; 
        }
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();
        queue.push_back(start_vertex.clone());
        visited.insert(start_vertex.clone(), 0);

        while let Some(current_vertex) = queue.pop_front() {
            if &current_vertex == end_vertex {
                return Some(visited[&current_vertex]);
            }

            if let Some(neighbors) = self.edges.get(&current_vertex) {
                for neighbor in neighbors.iter() {
                    if neighbor.is_empty() {
                        continue;
                    }
                    if !visited.contains_key(neighbor) {
                        let distance = visited[&current_vertex] + 1;
                        visited.insert(neighbor.clone(), distance);
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        None
    }
 
}