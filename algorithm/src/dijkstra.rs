use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vertex<'a> {
    name: &'a str,
}

#[allow(dead_code)]
impl<'a> Vertex<'a> {
    fn new(name: &'a str) -> Vertex<'a> {
        Vertex { name }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Visit<V> {
    vertex: V,
    distance: usize,
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}

#[allow(dead_code)]
fn dijkstra_distance<'a>(
    //shortest distance from start node
    start: Vertex<'a>,
    adjacency_list: &HashMap<Vertex<'a>, Vec<(Vertex<'a>, usize)>>,
) -> HashMap<Vertex<'a>, usize> {
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    distances.insert(start, 0);
    to_visit.push(Visit {
        vertex: start,
        distance: 0,
    });

    while let Some(Visit { vertex, distance }) = to_visit.pop() {
        if !visited.insert(vertex) {
            continue;
        }

        if let Some(neighbors) = adjacency_list.get(&vertex) {
            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(neighbor)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    distances.insert(*neighbor, new_distance);
                    to_visit.push(Visit {
                        vertex: *neighbor,
                        distance: new_distance,
                    });
                }
            }
        }
    }
    distances
}

#[allow(dead_code)]
fn dijkstra_path<'a>(
    start: Vertex<'a>,
    target: Vertex<'a>,
    adjacency_list: &HashMap<Vertex<'a>, Vec<(Vertex<'a>, usize)>>,
) -> Vec<Vertex<'a>> {
    //shortest path from start node to target node
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();
    let mut previous_nodes: HashMap<Vertex, Vertex> = HashMap::new();
    let mut path = vec![];
    let mut node = target;

    distances.insert(start, 0);
    to_visit.push(Visit {
        vertex: start,
        distance: 0,
    });

    while let Some(Visit { vertex, distance }) = to_visit.pop() {
        if !visited.insert(vertex) {
            continue;
        }

        if let Some(neighbors) = adjacency_list.get(&vertex) {
            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(neighbor)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    distances.insert(*neighbor, new_distance);
                    previous_nodes.insert(*neighbor, vertex);
                    to_visit.push(Visit {
                        vertex: *neighbor,
                        distance: new_distance,
                    });
                }
            }
        }
    }

    while node != start {
        path.push(node);
        let value = match previous_nodes.get(&node) {
            Some(value) => value,
            None => return vec![],
        };
        node = *value;
    }
    path.push(start);
    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::Vertex;
    use crate::dijkstra::{dijkstra_distance, dijkstra_path};
    use std::collections::HashMap;

    #[test]
    fn given_example() {
        //shortest distance from start node to given node
        let start_node = Vertex::new("start_node"); //node_0
        let node_1 = Vertex::new("node_1");
        let node_2 = Vertex::new("node_2");
        let node_3 = Vertex::new("node_3");
        let node_4 = Vertex::new("node_4");
        let node_5 = Vertex::new("node_5");
        let node_6 = Vertex::new("node_6");

        let mut adjacency_list = HashMap::new();
        adjacency_list.insert(start_node, vec![(node_4, 3), (node_1, 7), (node_5, 10)]);
        adjacency_list.insert(
            node_1,
            vec![
                (start_node, 7),
                (node_4, 2),
                (node_3, 10),
                (node_2, 4),
                (node_5, 6),
            ],
        );
        adjacency_list.insert(node_2, vec![(node_1, 4), (node_3, 2)]);
        adjacency_list.insert(
            node_3,
            vec![
                (node_1, 10),
                (node_2, 2),
                (node_6, 4),
                (node_4, 11),
                (node_5, 9),
            ],
        );
        adjacency_list.insert(node_4, vec![(node_1, 2), (node_3, 11), (node_6, 5)]);
        adjacency_list.insert(node_5, vec![(start_node, 10), (node_1, 6), (node_3, 9)]);
        adjacency_list.insert(node_6, vec![(node_4, 5), (node_3, 4)]);

        let distances = dijkstra_distance(start_node, &adjacency_list);
        assert_eq!(distances.get(&node_1), Some(&(5)));
        assert_eq!(
            dijkstra_path(start_node, node_1, &adjacency_list),
            vec![start_node, node_4, node_1]
        );

        assert_eq!(distances.get(&node_2), Some(&(9)));
        assert_eq!(
            dijkstra_path(start_node, node_2, &adjacency_list),
            vec![start_node, node_4, node_1, node_2]
        );

        assert_eq!(distances.get(&node_3), Some(&(11)));
        assert_eq!(
            dijkstra_path(start_node, node_3, &adjacency_list),
            vec![start_node, node_4, node_1, node_2, node_3]
        );

        assert_eq!(distances.get(&node_4), Some(&(3)));
        assert_eq!(
            dijkstra_path(start_node, node_4, &adjacency_list),
            vec![start_node, node_4]
        );

        assert_eq!(distances.get(&node_5), Some(&(10)));
        assert_eq!(
            dijkstra_path(start_node, node_5, &adjacency_list),
            vec![start_node, node_5]
        );

        assert_eq!(distances.get(&node_6), Some(&(8)));
        assert_eq!(
            dijkstra_path(start_node, node_6, &adjacency_list),
            vec![start_node, node_4, node_6]
        );
    }
}
