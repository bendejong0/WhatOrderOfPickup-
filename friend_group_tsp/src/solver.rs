use crate::friendgroupdata::real_adjacency_matrix_creator;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::usize::MAX;

// TODO: Finish this

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,       // How much the path we've taken has cost.
    position: usize, // Index of the current node.
    visited: usize,  // Bitmask to track what's been visited (we only need up to 9 nodes).
}

impl PartialOrd for State{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

pub fn dijkstra_path(source: usize, nodes: &[usize], end_node: usize, adj_matrix: &Vec<Vec<usize>>) -> Option<u32>{
    let N = nodes.len();
    let mut dist = vec![vec![MAX; 1 << nodes.len()]; N];
    let mut heap = BinaryHeap::<State>::new();

    dist[source][1 << source] = 0;
    heap.push(State { cost: 0, position: source, visited: 1 << source});

    // While there are still states to explore, pop from the heap.
    while let Some(State { cost, position, visited }) = heap.pop() {
        // if weve visited the nodes, and we've reached the end node
        if visited == (1 << nodes.len()) - 1 && position == end_node {
            // return the cost.
            return Some(cost);
        }
        // if the popped state's cost is greater than the minimum cost,
        if cost > dist[position][visited] as u32 {
            // just go to the next iteration
            continue;
        }

        // look for neighbors of the current node ('position')
        for (next, &weight) in adj_matrix[position].iter().enumerate() {
            // calculate the new visited bitmask by marking `next` node as visited
            let next_visited = visited | (1 << nodes.iter().position(|&x| x == next).unwrap_or(N));
            // find the new cost by adding the edge weight to the current cost
            let next_cost = cost + weight as u32;
            // if the new path is shorter, update and push to heap
            if next_cost < dist[next][next_visited] as u32 {
                dist[next][next_visited] = next_cost as usize;
                heap.push(State { cost: next_cost, position: next, visited: next_visited });
            }
        }
    }

    None // No path found
}