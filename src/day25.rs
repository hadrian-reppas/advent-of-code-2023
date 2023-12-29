use std::cmp::{max, min};
use std::collections::{BTreeMap, HashMap};

use nalgebra::{base::SquareMatrix, Dyn};

fn get_id<'a>(name: &'a str, map: &mut HashMap<&'a str, usize>) -> usize {
    let id = map.len();
    *map.entry(name).or_insert(id)
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    let mut graph = BTreeMap::new();
    let mut map = HashMap::new();
    for line in input.lines() {
        let (node, neighbors) = line.split_once(": ").unwrap();
        let node = get_id(node, &mut map);
        for neighbor in neighbors.split(' ') {
            let neighbor = get_id(neighbor, &mut map);
            graph.entry(node).or_insert(Vec::new()).push(neighbor);
            graph.entry(neighbor).or_insert(Vec::new()).push(node);
        }
    }
    graph.into_values().collect()
}

fn get_sorted_edges(graph: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut laplacian = vec![vec![0.0; graph.len()]; graph.len()];
    for (node, neighbors) in graph.iter().enumerate() {
        laplacian[node][node] = neighbors.len() as f64;
        for neighbor in neighbors {
            laplacian[node][*neighbor] = -1.0;
            laplacian[*neighbor][node] = -1.0;
        }
    }

    let matrix = SquareMatrix::<f64, Dyn, _>::from_row_iterator(
        graph.len(),
        graph.len(),
        laplacian.into_iter().flatten(),
    );
    let eigen = matrix.symmetric_eigen();
    let mut indices: Vec<_> = eigen.eigenvalues.iter().enumerate().collect();
    indices.sort_unstable_by(|(_, a), (_, b)| a.total_cmp(b));
    let index = indices[1].0;

    let mut edges = Vec::new();
    for (node, neighbors) in graph.iter().enumerate() {
        for neighbor in neighbors.iter().filter(|n| **n > node) {
            let node_pos = eigen.eigenvectors[(node, index)];
            let neighbor_pos = eigen.eigenvectors[(*neighbor, index)];
            edges.push((node, *neighbor, (node_pos - neighbor_pos).abs()));
        }
    }
    edges.sort_unstable_by(|(_, _, a), (_, _, b)| b.total_cmp(a));
    edges.into_iter().map(|(u, v, _)| (u, v)).collect()
}

fn component_size(
    graph: &[Vec<usize>],
    edge1: (usize, usize),
    edge2: (usize, usize),
    edge3: (usize, usize),
) -> u64 {
    let include_edge = |u, v| {
        let edge = (min(u, v), max(u, v));
        edge != edge1 && edge != edge2 && edge != edge3
    };
    let mut seen = vec![false; graph.len()];
    let mut stack = vec![0];
    while let Some(node) = stack.pop() {
        if seen[node] {
            continue;
        }
        seen[node] = true;
        for neighbor in &graph[node] {
            if include_edge(node, *neighbor) {
                stack.push(*neighbor);
            }
        }
    }
    seen.into_iter().map(u64::from).sum()
}

pub fn part1(input: &str) -> u64 {
    let graph = parse(input);
    let edges = get_sorted_edges(&graph);

    for (i, edge1) in edges.iter().enumerate().skip(2) {
        for (j, edge2) in edges.iter().enumerate().take(i).skip(1) {
            for edge3 in edges.iter().take(j) {
                let size = component_size(&graph, *edge1, *edge2, *edge3);
                if size < graph.len() as u64 {
                    return size * (graph.len() as u64 - size);
                }
            }
        }
    }
    unreachable!()
}

pub fn part2(_: &str) -> u64 {
    0
}
