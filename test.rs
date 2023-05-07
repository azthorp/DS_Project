#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_connected_components() {
        let graph = Graph::create_undirected(4, &vec![(0, 1), (1, 2), (2, 3)]);
        let components = compute_connected_components(&graph);
        assert_eq!(components, vec![Some(0), Some(0), Some(0), Some(0)]);
    }

    #[test]
    fn test_compute_average_distance() {
        let graph = Graph::create_undirected(4, &vec![(0, 1), (1, 2), (2, 3)]);
        let components = compute_connected_components(&graph);
        let avg_distance = compute_average_distance(&graph, &components);
        assert_eq!(avg_distance, 1.5);
    }
}


