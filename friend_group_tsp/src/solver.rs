use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::usize::MAX;
mod friendgroupdata;
pub use friendgroupdata::real_adjacency_matrix_creator;

// TODO: Finish this

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,       // How much the path we've taken has cost.
    position: usize, // Index of the current node.
    visited: u8,     // Bitmask to track what's been visited (we only need up to 9 nodes).
}

pub fn dijkstra_path(source: usize, nodes: &[usize], end_node: usize, adj_matrix: &Vec<Vec<usize>>) {
    let N = nodes.len();
    let mut dist = vec![vec![MAX; 1 << nodes.len()]; N];
    let mut heap = BinaryHeap::new();

    dist[source][1] = 0;
    heap.push(State { cost: 0, position: source, visited: 1 });

    // While there are still states to explore, pop from the heap.
    while let Some(State { cost, position, visited }) = heap.pop() {
        // if weve visited the nodes, and we've reached the end node
        if visited == (1 << nodes.len()) - 1 && position == end_node {
            // return the cost.
            return Some(cost);
        }
        // if the popped state's cost is greater than the minimum cost,
        if cost > dist[position][visited] {
            // just go to the next iteration
            continue;
        }

        // look for neighbors of the current node ('position')
        for (next, &weight) in adj_matrix[position].iter().enumerate() {
            // if there is a weight
            if let Some(weight) = weight {
                // calculate the new visited bitmask by marking `next` node as visited
                let next_visited = visited | (1 << nodes.iter().position(|&x| x == next).unwrap_or(N));
                // find the new cost by adding the edge weight to the current cost
                let next_cost = cost + weight;
                // if the new path is shorter, update and push to heap
                if next_cost < dist[next][next_visited] {
                    dist[next][next_visited] = next_cost;
                    heap.push(State { cost: next_cost, position: next, visited: next_visited });
                }
            }
        }
    }

    None // No path found
}