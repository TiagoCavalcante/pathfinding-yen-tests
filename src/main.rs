use graphs::Graph;
use pathfinding::prelude::yen;

fn main() {
  let size = 100;
  let density = 0.1;

  let start = 0;
  let end = 10;

  let mut graph = Graph::new(size);
  graph.fill_undirected(density);

  let path = yen(
    &start,
    |&vertex| {
      graph
        .get_neighbors(vertex)
        .iter()
        .map(|&neighbor| (neighbor, 1))
        .collect::<Vec<_>>()
    },
    |&vertex| vertex == end,
    10,
  );

  println!("{:?}", path);
}
