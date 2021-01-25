use serde::Deserialize;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::{BinaryHeap, HashSet};

#[derive(Deserialize, Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Deserialize, Debug)]
pub struct Graph {
    intersections: Vec<Point>,
    roads: Vec<Vec<usize>>,
}

struct PathTree {
    nodes: Vec<Node>,
}

struct Node {
    previous: Option<usize>,
    intersection: usize,
}

impl PathTree {
    fn start(intersection: &usize) -> Self {
        PathTree {
            nodes: vec![Node {
                previous: None,
                intersection: *intersection,
            }],
        }
    }

    fn add(&mut self, prev: &usize, intersection: &usize) -> usize {
        self.nodes.push(Node {
            previous: Some(*prev),
            intersection: *intersection,
        });
        self.nodes.len() - 1
    }

    fn get_path(&self, start: &usize) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut current: &Node = &self.nodes[*start];
        loop {
            match current.previous {
                Some(idx) => {
                    result.push(current.intersection as i32);
                    current = &self.nodes[idx]
                }
                None => {
                    result.push(current.intersection as i32);
                    result.reverse();
                    return result;
                }
            }
        }
    }
}

struct FrontierNode {
    distance: f64,
    path_index: usize,
}

impl FrontierNode {
    fn new(d: f64, idx: &usize) -> Self {
        FrontierNode {
            distance: d,
            path_index: *idx,
        }
    }

    fn get_intersection(&self, path: &PathTree) -> usize {
        path.nodes[self.path_index].intersection
    }

    fn get_path(&self, path: &PathTree) -> Vec<i32> {
        path.get_path(&self.path_index)
    }
}

impl Ord for FrontierNode {
    fn cmp(&self, other: &Self) -> Ordering {
        let a: i64 = (self.distance * 100_000_000_000.0) as i64;
        let b: i64 = (other.distance * 100_000_000_000.0) as i64;
        a.cmp(&b)
    }
}

impl PartialOrd for FrontierNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}
impl PartialEq for FrontierNode {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}
impl Eq for FrontierNode {}

fn find_distance(a: &Point, b: &Point) -> f64 {
    let deltax = a.x - b.x;
    let deltay = a.y - b.y;
    return (deltax.powi(2) + deltay.powi(2)).sqrt();
}

pub fn shortest_path(graph: &Graph, start: usize, end: usize) -> Vec<i32> {
    let mut explored: HashSet<usize> = HashSet::new();
    let mut frontier: BinaryHeap<FrontierNode> = BinaryHeap::new();
    let mut path = PathTree::start(&start);
    frontier.push(FrontierNode::new(0.0, &0));

    loop {
        let node = frontier.pop().unwrap();

        if node.get_intersection(&path) == end {
            return node.get_path(&path);
        }

        for road in graph.roads[node.get_intersection(&path)].iter() {
            if !explored.contains(&road) {
                let distance = node.distance
                    + find_distance(
                        &graph.intersections[node.get_intersection(&path)],
                        &graph.intersections[*road],
                    );
                let distance_from_end =
                    find_distance(&graph.intersections[*road], &graph.intersections[end]);
                let heuristic = node.distance + distance + distance_from_end;

                // Here
                frontier.push(FrontierNode::new(
                    heuristic,
                    &path.add(&node.path_index, road),
                ));
            }
        }

        explored.insert(node.get_intersection(&path));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;

    #[test]
    fn check_graph() -> io::Result<()> {
        let mut f = File::open("graph.json")?;
        let mut buffer = String::new();

        f.read_to_string(&mut buffer)?;
        let graph: Graph = serde_json::from_str(&buffer)?;
        assert_eq!(graph.roads[0], vec![36, 34, 31, 28, 17]);
        Ok(())
    }

    #[test]
    fn check_path_tree() {
        let mut path = PathTree::start(&5);
        path.add(&0, &7);
        path.add(&1, &3);
        assert_eq!(path.get_path(&2), vec![5, 7, 3])
    }

    #[test]
    fn check_frontier_heap() {
        let mut frontier: BinaryHeap<FrontierNode> = BinaryHeap::new();
        frontier.push(FrontierNode::new(4.0, &3));
        frontier.push(FrontierNode::new(7.0, &2));
        frontier.push(FrontierNode::new(2.0, &8));

        assert_eq!(frontier.pop().unwrap().distance, 2.0);
    }

    #[test]
    fn check_shortest_path() -> io::Result<()> {
        let mut f = File::open("graph.json")?;
        let mut buffer = String::new();

        f.read_to_string(&mut buffer)?;
        let graph: Graph = serde_json::from_str(&buffer)?;
        assert_eq!(shortest_path(&graph, 5, 34), vec![5, 16, 37, 12, 34]);
        Ok(())
    }
}
