use std::{collections::HashMap, hash::RandomState};

use petgraph::algo::toposort;
use petgraph::{algo::all_simple_paths, prelude::*};

pub fn part_one(input: &str) -> Option<usize> {
    let mut graph = DiGraph::new();
    let mut indexes = HashMap::new();
    indexes.insert("out", graph.add_node("out"));

    let lines = input.lines();
    for line in lines {
        let node = line.split_once(':').unwrap().0;
        let from = if let Some(i) = indexes.get(node) {
            *i
        } else {
            let from = graph.add_node(node);
            indexes.insert(node, from);
            from
        };
        let edges = line.split_once(':').unwrap().1.split_whitespace();
        for edge in edges {
            let to = if let Some(i) = indexes.get(edge) {
                *i
            } else {
                let to = graph.add_node(edge);
                indexes.insert(edge, to);
                to
            };
            graph.add_edge(from, to, 1);
        }
    }
    let solution = all_simple_paths::<Vec<_>, _, RandomState>(
        &graph,
        *indexes.get("you").unwrap(),
        *indexes.get("out").unwrap(),
        0,
        None,
    )
    .count();
    Some(solution)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut graph = DiGraph::new();
    let mut indexes = HashMap::new();
    indexes.insert("out", graph.add_node("out"));

    let lines = input.lines();
    for line in lines {
        let node = line.split_once(':').unwrap().0;
        let from = if let Some(i) = indexes.get(node) {
            *i
        } else {
            let from = graph.add_node(node);
            indexes.insert(node, from);
            from
        };
        let edges = line.split_once(':').unwrap().1.split_whitespace();
        for edge in edges {
            let to = if let Some(i) = indexes.get(edge) {
                *i
            } else {
                let to = graph.add_node(edge);
                indexes.insert(edge, to);
                to
            };
            graph.add_edge(from, to, 1);
        }
    }
    let svr = *indexes.get("svr").unwrap();
    let dac = *indexes.get("dac").unwrap();
    let fft = *indexes.get("fft").unwrap();
    let out = *indexes.get("out").unwrap();

    let topo = toposort(&graph, None).expect("graph is not a DAG");

    let a = count_paths_between(&graph, &topo, svr, dac); // svr -> dac
    let b = count_paths_between(&graph, &topo, svr, fft); // svr -> fft
    let c = count_paths_between(&graph, &topo, dac, fft); // dac -> fft
    let d = count_paths_between(&graph, &topo, fft, dac); // fft -> dac
    let e = count_paths_between(&graph, &topo, dac, out); // dac -> out
    let f = count_paths_between(&graph, &topo, fft, out); // fft -> out

    let solution = a * c * f + b * d * e;

    Some(solution)
}

fn count_paths_between(
    graph: &DiGraph<&str, i32>,
    topo: &[NodeIndex],
    start: NodeIndex,
    target: NodeIndex,
) -> usize {
    let mut ways = vec![0usize; graph.node_count()];
    ways[start.index()] = 1;

    for &node in topo {
        let w = ways[node.index()];
        if w == 0 {
            continue;
        }
        for edge in graph.edges(node) {
            let next = edge.target();
            ways[next.index()] += w;
        }
    }

    ways[target.index()]
}

adventofcode::advent_of_code!(2025, 11, Some(5), Some(2));
