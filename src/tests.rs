use crate::graph::Graph;

#[test]
fn test_graph_add_vertex() {
    let mut graph = Graph::new();
    graph.add_vertex("country");
    graph.add_vertex("listed_in");
    graph.add_vertex("rating");
    graph.add_vertex("release_year");
    assert_eq!(graph.edges.len(), 4);
    assert!(graph.edges.contains_key("country"));
    assert!(graph.edges.contains_key("listed_in"));
    assert!(graph.edges.contains_key("rating"));
    assert!(graph.edges.contains_key("release_year"));
}

#[test]
fn test_graph_add_edge() {
    let mut graph = Graph::new();
    graph.add_edge("country", "United States");
    graph.add_edge("listed_in", "Horror");
    graph.add_edge("rating", "PG-13");
    graph.add_edge("release_year", "2021");
    assert_eq!(graph.edges.len(), 4);
    assert!(graph.edges.get("country").unwrap().contains("United States"));
    assert!(graph.edges.get("listed_in").unwrap().contains("Horror"));
    assert!(graph.edges.get("rating").unwrap().contains("PG-13"));
    assert!(graph.edges.get("release_year").unwrap().contains("2021"));
}

#[test]
fn test_get_vertices() {
    let mut graph = Graph::new();
    graph.add_vertex("The Shawshank Redemption");
    graph.add_vertex("The Godfather");
    graph.add_vertex("The Dark Knight");
    let vertices = graph.get_vertices();
    assert_eq!(vertices.len(), 3);
    assert!(vertices.contains(&"The Shawshank Redemption".to_string()));
    assert!(vertices.contains(&"The Godfather".to_string()));
    assert!(vertices.contains(&"The Dark Knight".to_string()));
}


#[test]
fn test_calculate_distance() {
    let mut graph = Graph::new();
    graph.add_vertex("listed_in");
    graph.add_vertex("country");
    graph.add_vertex("rating");
    graph.add_edge("listed_in", "country");
    graph.add_edge("country", "rating");
    let distance = graph.calculate_distance(&"country".to_string(), &"rating".to_string());
    assert_eq!(distance, Some(1));
}
