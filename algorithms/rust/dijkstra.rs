const MAX: i64 = 99999;

use std::collections::HashMap;

fn graph() -> HashMap<&'static str, HashMap<&'static str, i64>> {

    let nodes = vec!["P", "R", "Z", "A", "Y"];

    let mut p:HashMap<&str, i64> = HashMap::new();
    p.insert("A", 105);
    p.insert("Y", 1);
    p.insert("R", 452);

    let mut r:HashMap<&str, i64> = HashMap::new();
    r.insert("P", 10);
    r.insert("Z", 192);

    let mut z:HashMap<&str, i64> = HashMap::new();
    z.insert("P", 12);

    let mut a:HashMap<&str, i64> = HashMap::new();
    a.insert("R", 112);
    a.insert("Y", 1000);

    let mut y:HashMap<&str, i64> = HashMap::new();
    y.insert("R", 2000);

    let adjacents: Vec<HashMap<&str, i64>> = vec![p, r, z, a, y];
    let graph: HashMap<_, _> = nodes.into_iter().zip(adjacents.into_iter()).collect();

    return graph;
}

fn shortest_distance(initial_path: &'static str) -> HashMap<&'static str, i64> {
    let mut shortest_distances = HashMap::new();

    shortest_distances.insert(initial_path, 0);

    for (node, _) in graph() {

        if node != initial_path {
            shortest_distances.insert(node, MAX);
        }
    }

    return shortest_distances;

}

fn dijkstra(
    origin:&'static str,
    destination:&'static str,
    mut graph:HashMap<&'static str, HashMap<&'static str, i64>>
) -> Vec<&'static str> {

    let mut path: Vec<&str> = Vec::new();

    let mut previous:HashMap<&str, &str> = HashMap::new();
    let mut shortest = shortest_distance(&origin);

    while !graph.is_empty() {
        let next_node: &str = shortest.iter().min_by_key(|entry | entry.1).unwrap().0;

        for parent_node in graph.get(next_node) {
            for (adjacent, weight) in parent_node {
                let check_existence = shortest.get(adjacent);
                if check_existence != None {
                    if weight + shortest[next_node] < shortest[adjacent] {
                        shortest.insert(adjacent, weight + shortest[next_node]);
                        previous.insert(adjacent, next_node);
                    }
                }
            }
        }

        shortest.remove(next_node);
        graph.remove(next_node);
    }

    let mut current: &str = destination;

    while current != origin {
        path.insert(0, current);
        current = previous.get(current).unwrap();
    }

    path.insert(0, origin);

    return path;
    
}

fn main(){

    let result = dijkstra("A", "Z", graph());
    println!("{:?}", result);

}
