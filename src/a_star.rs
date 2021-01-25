use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Deserialize, Debug)]
struct Graph {
    intersections: Vec<Point>,
    roads: Vec<Vec<i32>>,
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
}
