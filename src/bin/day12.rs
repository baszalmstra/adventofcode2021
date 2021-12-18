use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    is_big: bool,
    is_end: bool,
    is_start: bool,
    idx: usize,
}

struct Graph {
    nodes: Vec<Node>,
    node_by_name: HashMap<String, usize>,
    edges_from: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        let mut graph = Graph {
            nodes: vec![],
            node_by_name: Default::default(),
            edges_from: Default::default(),
        };
        graph.get_node_idx("start");
        graph.get_node_idx("end");

        graph
    }

    pub fn start_node_idx(&self) -> usize {
        0
    }

    pub fn get_node_idx(&mut self, name: impl Into<String>) -> usize {
        let name = name.into();
        *self.node_by_name.entry(name.clone()).or_insert_with(|| {
            let idx = self.nodes.len();
            let node = Node {
                is_big: name.contains(|c: char| c.is_uppercase()),
                is_end: name == "end",
                is_start: name == "start",
                idx: self.nodes.len(),
            };
            self.nodes.push(node);
            idx
        })
    }
}

fn parse(input: &str) -> Graph {
    let mut graph = Graph::new();
    for line in input.lines() {
        let (left, right) = line.split_once('-').unwrap();
        let left_idx = graph.get_node_idx(left);
        let right_idx = graph.get_node_idx(right);
        graph
            .edges_from
            .entry(left_idx)
            .or_default()
            .push(right_idx);
        graph
            .edges_from
            .entry(right_idx)
            .or_default()
            .push(left_idx);
    }
    graph
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day12/input")?;
    let graph = parse(&input);

    fn count_paths(
        graph: &Graph,
        start: usize,
        visited_map: &mut HashMap<usize, usize>,
        small_node_visited_count: usize,
        mut small_cave_visited: bool,
    ) -> usize {
        // If this is the end, we are done and found a path
        if graph.nodes[start].is_end {
            return 1;
        }

        // Increment the number of times we visited this node
        *visited_map.entry(start).or_insert(0) += 1;

        // If this is a small cave and we visisted it more than once, we have visisted a single
        // small cave more than once.
        if !graph.nodes[start].is_big
            && *visited_map.get_mut(&start).unwrap() >= small_node_visited_count
        {
            small_cave_visited = true;
        }

        // Iterate over all outgoing edges
        let mut total_paths = 0;
        for node in graph
            .edges_from
            .get(&start)
            .into_iter()
            .flat_map(|nodes| nodes.iter().copied())
        {
            let node = &graph.nodes[node];
            let visited_count = visited_map.get(&node.idx).copied().unwrap_or(0);
            if node.is_start && visited_count >= 1 {
                continue;
            }
            if !node.is_big && small_cave_visited && visited_count >= 1 {
                continue;
            }
            if !node.is_big && !small_cave_visited && visited_count >= small_node_visited_count {
                continue;
            }

            total_paths += count_paths(
                graph,
                node.idx,
                visited_map,
                small_node_visited_count,
                small_cave_visited,
            )
        }

        *visited_map.get_mut(&start).unwrap() -= 1;

        total_paths
    }

    let count = count_paths(
        &graph,
        graph.start_node_idx(),
        &mut HashMap::new(),
        1,
        false,
    );
    println!("Solution 1: {}", count);

    let count = count_paths(
        &graph,
        graph.start_node_idx(),
        &mut HashMap::new(),
        2,
        false,
    );
    println!("Solution 2: {}", count);

    Ok(())
}
