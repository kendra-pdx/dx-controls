use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use dx_graph::force::{Arena, ForceGraph, SimulationParams};
use petgraph::prelude::*;

const DT: f32 = 0.016;
const WARM_UP_TICKS: usize = 20;

fn path_graph(n: usize) -> UnGraph<usize, ()> {
    let mut g = UnGraph::new_undirected();
    let nodes: Vec<_> = (0..n).map(|i| g.add_node(i)).collect();
    for pair in nodes.windows(2) {
        g.add_edge(pair[0], pair[1], ());
    }
    g
}

fn grid_graph(side: usize) -> UnGraph<usize, ()> {
    let mut g = UnGraph::new_undirected();
    let nodes: Vec<_> = (0..side * side).map(|i| g.add_node(i)).collect();
    for row in 0..side {
        for col in 0..side {
            let i = row * side + col;
            if col + 1 < side {
                g.add_edge(nodes[i], nodes[i + 1], ());
            }
            if row + 1 < side {
                g.add_edge(nodes[i], nodes[i + side], ());
            }
        }
    }
    g
}

fn complete_graph(n: usize) -> UnGraph<usize, ()> {
    let mut g = UnGraph::new_undirected();
    let nodes: Vec<_> = (0..n).map(|i| g.add_node(i)).collect();
    for i in 0..n {
        for j in (i + 1)..n {
            g.add_edge(nodes[i], nodes[j], ());
        }
    }
    g
}

fn warmed_up(graph: UnGraph<usize, ()>) -> ForceGraph<usize, ()> {
    let mut sim = ForceGraph::new(graph, Arena::default(), SimulationParams::default());
    for _ in 0..WARM_UP_TICKS {
        sim.update(DT);
    }
    sim
}

fn bench_update_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("update_path");
    for n in [10, 50, 100, 200] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let mut sim = warmed_up(path_graph(n));
            b.iter(|| sim.update(DT));
        });
    }
    group.finish();
}

fn bench_update_grid(c: &mut Criterion) {
    let mut group = c.benchmark_group("update_grid");
    for side in [3, 7, 10, 14] {
        let n = side * side;
        group.bench_with_input(BenchmarkId::from_parameter(n), &side, |b, &side| {
            let mut sim = warmed_up(grid_graph(side));
            b.iter(|| sim.update(DT));
        });
    }
    group.finish();
}

fn bench_update_complete(c: &mut Criterion) {
    let mut group = c.benchmark_group("update_complete");
    for n in [10, 50, 100] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let mut sim = warmed_up(complete_graph(n));
            b.iter(|| sim.update(DT));
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_update_path,
    bench_update_grid,
    bench_update_complete
);
criterion_main!(benches);
