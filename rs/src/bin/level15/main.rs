use advent::grid_from_input;
use petgraph::{
  algo::astar,
  graph::{DiGraph, NodeIndex},
  visit::{EdgeRef, IntoEdgeReferences},
  Graph,
};
use std::collections::HashMap;

fn raw_input() -> &'static str {
  include_str!("input.txt")
}

fn input() -> (DiGraph<u32, ()>, NodeIndex, NodeIndex) {
  let grid = grid_from_input(raw_input());
  let mut node_map: HashMap<(usize, usize), NodeIndex> = HashMap::new();
  let mut graph = DiGraph::<u32, ()>::default();

  for r in 0..grid.rows() {
    for c in 0..grid.cols() {
      let node = graph.add_node(grid[r][c]);
      node_map.insert((r, c), node);
    }
  }

  for r in 0..grid.rows() {
    for c in 0..grid.cols() {
      let &this_node = node_map.get(&(r, c)).unwrap();

      let node_to_right = node_map.get(&(r, c + 1));
      match node_to_right {
        Some(&nr) => {
          graph.add_edge(this_node, nr, ());
          graph.add_edge(nr, this_node, ());
        }
        None => (),
      }

      let node_below = node_map.get(&(r + 1, c));
      match node_below {
        Some(&nb) => {
          graph.add_edge(this_node, nb, ());
          graph.add_edge(nb, this_node, ());
        }
        None => (),
      }
    }
  }

  let start = node_map[&(0, 0)];
  let goal = node_map[&(grid.rows() - 1, grid.cols() - 1)];

  (graph, start, goal)
}

fn main() {
  let (graph, start, goal) = input();
  let is_goal = |node| node == goal;

  let edge_cost = |e: <&Graph<u32, ()> as IntoEdgeReferences>::EdgeRef| {
    return graph[e.target()];
  };

  let estimate_cost = |_| 0;
  let result = astar(&graph, start, is_goal, edge_cost, estimate_cost);
  let (total_risk, _) = result.unwrap();
  println!("{}", total_risk)
}
