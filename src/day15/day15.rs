#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day15() {
        println!("part1:");
        part1();
        println!("part2:");
        part2();
    }
}

fn input() -> String {
    String::from(include_str!("input.txt"))
}

// *** Some days it's OK to be lazy: thank you example code. ***

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
#[derive(Copy, Clone)]
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

// *** *** *** *** *** ***

fn solve(dup: usize) {
    let input = input();

    let mut grid: Vec<Vec<usize>> = Vec::new();
    for line in input.lines() {
        let line: Vec<usize> = line.bytes().map(|b| (b - b'0') as usize).collect();
        grid.push(line);
    }
    let w = grid[0].len();
    let h = grid.len();
    let w_tot = dup * w;
    let h_tot = dup * h;

    let mut adjacencies: Vec<Vec<Edge>> = Vec::new();
    for _ in 0..w_tot * h_tot {
        adjacencies.push(Vec::new());
    }
    for (y, row) in grid.iter().enumerate() {
        for dup_y in 0..dup {
            for (x, &cost) in row.iter().enumerate() {
                let y = y + h * dup_y;
                for dup_x in 0..dup {
                    let x = x + w * dup_x;
                    let cost = (cost + dup_x + dup_y - 1) % 9 + 1;
                    let node = x + w_tot * y;
                    let edge = Edge { node, cost };
                    if x > 0 {
                        adjacencies[node - 1].push(edge);
                    }
                    if x < w_tot - 1 {
                        adjacencies[node + 1].push(edge);
                    }
                    if y > 0 {
                        adjacencies[node - w_tot].push(edge);
                    }
                    if y < h_tot - 1 {
                        adjacencies[node + w_tot].push(edge);
                    }
                }
            }
        }
    }

    let min_risk = shortest_path(&adjacencies, 0, adjacencies.len() - 1).unwrap();

    println!("{}", min_risk);
}

pub fn part1() {
    solve(1);
}

pub fn part2() {
    solve(5);
}
