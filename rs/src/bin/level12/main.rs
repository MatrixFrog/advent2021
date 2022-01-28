use clap::Parser;
use petgraph::graph::{Graph, NodeIndex, UnGraph};
use std::collections::HashMap;

#[derive(Parser)]
struct Cli {
  #[clap(short, long)]
  small: bool,
}

#[derive(Debug, Eq, PartialEq)]
enum CaveType {
  Big,
  Small,
  StartOrEnd,
}

impl CaveType {
  fn from_label(label: &str) -> Self {
    match label {
      "start" | "end" => Self::StartOrEnd,
      _ => {
        if label.to_ascii_lowercase() == label {
          Self::Small
        } else {
          Self::Big
        }
      }
    }
  }
}

fn raw_input() -> &'static str {
  include_str!("input.txt")
}
fn small_input() -> &'static str {
  include_str!("input_small.txt")
}

fn parse_nodes<'a>(input: &'a str, g: &mut UnGraph<CaveType, ()>) -> HashMap<&'a str, NodeIndex> {
  let mut node_map = HashMap::new();
  for l in input.lines() {
    let nodes = l.split('-').collect::<Vec<_>>();
    assert_eq!(2, nodes.len());
    for label in nodes {
      if node_map.contains_key(label) {
        // Already added.
      } else {
        let node = g.add_node(CaveType::from_label(label));
        node_map.insert(label, node);
      }
    }
  }
  node_map
}

fn parse_edges(input: &str) -> impl Iterator<Item = (&str, &str)> {
  input.lines().map(|l| {
    let labels = l.split('-').collect::<Vec<_>>();
    assert_eq!(2, labels.len());
    (labels[0], labels[1])
  })
}

fn parse_input(input: &str) -> (UnGraph<CaveType, ()>, NodeIndex, NodeIndex) {
  let mut g = Graph::new_undirected();
  let nodes = parse_nodes(input, &mut g);
  let edges = parse_edges(input);
  for (label0, label1) in edges {
    let node0 = nodes.get(label0).unwrap();
    let node1 = nodes.get(label1).unwrap();
    g.add_edge(*node0, *node1, ());
  }
  (g, *nodes.get("start").unwrap(), *nodes.get("end").unwrap())
}

fn is_valid(g: &UnGraph<CaveType, ()>, path: &[NodeIndex]) -> bool {
  let mut frequencies = HashMap::new();
  for node in path {
    *frequencies.entry(node).or_insert(0) += 1;
  }

  let mut already_found_2 = false;
  for (node_index, freq) in frequencies {
    let node_type = g.node_weight(*node_index).unwrap();
    match node_type {
      CaveType::Big => { /* no problem */ }
      CaveType::StartOrEnd => {
        if freq > 1 {
          return false;
        }
      }
      CaveType::Small => {
        if freq > 2 {
          return false;
        }
        if freq == 2 {
          if already_found_2 {
            return false;
          }
          already_found_2 = true;
        }
      }
    }
  }
  true
}

// Try to extend this path by adding 'neighbor' if doing so is valid. If not, returns None.
fn try_extend(
  g: &UnGraph<CaveType, ()>,
  path: &[NodeIndex],
  neighbor: NodeIndex,
) -> Option<Vec<NodeIndex>> {
  let mut new_path = path.to_owned();
  new_path.push(neighbor);
  if is_valid(g, &new_path) {
    Some(new_path)
  } else {
    None
  }
}

// Returns a Vec of all valid paths that can be formed by
// adding one additional step to the given path.
//
// If there are none (because there is no way to continue without
// illegally double-visiting a cave), returns an empty Vec.
fn extend_path(g: &UnGraph<CaveType, ()>, path: &[NodeIndex]) -> Vec<Vec<NodeIndex>> {
  let mut paths = vec![];
  for neighbor in g.neighbors(*path.last().unwrap()) {
    match try_extend(g, path, neighbor) {
      None => {}
      Some(new_path) => paths.push(new_path),
    }
  }
  paths
}

fn find_paths(g: &UnGraph<CaveType, ()>, start: NodeIndex, end: NodeIndex) -> Vec<Vec<NodeIndex>> {
  let mut paths = vec![vec![start]];
  loop {
    let mut keep_going = false;
    let mut new_paths = vec![];
    for path in paths {
      if *path.last().unwrap() == end {
        new_paths.push(path);
      } else {
        new_paths.extend(extend_path(g, &path));
        keep_going = true
      }
    }
    paths = new_paths;
    if !keep_going {
      break;
    }
  }
  paths
}

fn main() {
  let small = Cli::parse().small;
  let input = if small { small_input() } else { raw_input() };
  let (g, start, end) = parse_input(input);
  let paths = find_paths(&g, start, end);
  println!("{:?}", paths.len())
}
