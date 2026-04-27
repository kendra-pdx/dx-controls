//! Forces (applied each `update(dt)`):
//! 1. Spring forces (Kamada-Kawai style) — between all reachable pairs, where the ideal distance is ideal_length * graph_distance and the spring constant scales as 1/g² so topologically closer nodes exert stronger attraction/repulsion toward their ideal separation.
//! 2. Coulomb repulsion — inverse-square repulsion between every pair regardless of connectivity, preventing overlap and handling disconnected components.
//! 3. Boundary repulsion — quadratic ramp that activates within boundary_margin of each wall, pushing nodes inward. A hard clamp after integration ensures nothing escapes the arena.

use derive_more::{Deref, From, Into};
use derive_new::new;
use petgraph::prelude::*;
use rand::RngExt;
use spacial::prelude::{Area2, Point2, Rect2, TwizzleWH, Vec2};
use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};
use tracing::info;

const MAX_SPEED: f32 = 500.0;
const MIN_DISTANCE: f32 = 0.01;

pub trait GraphInfo: Deref<Target = UnGraph<Self::Node, Self::Edge>> {
    type Node;
    type Edge;

    fn node_state(&self, idx: NodeIndex) -> Option<&NodeState>;
}

pub trait GraphInfoMut: GraphInfo + DerefMut<Target = UnGraph<Self::Node, Self::Edge>> {
    fn node_state_mut(&mut self, idx: NodeIndex) -> Option<&mut NodeState>;
}

/// Fixed-size rectangular arena that constrains the simulation.
///
/// The origin `(0, 0)` sits at the centre; nodes are confined to
/// `[-width/2, width/2] × [-height/2, height/2]`.
#[derive(Debug, Clone, Copy, PartialEq, Deref, Into, From, new)]
pub struct Arena(#[new(into)] Area2<Vec2<f32>>);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InitialLayout {
    Circular,
    Random,
}

/// Tuning knobs for the force simulation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulationParams {
    /// Initial layout strategy before the first update tick.
    /// Default: Circular
    pub initial_layout: InitialLayout,

    /// Velocity damping applied each step (`0.0..1.0`). Lower → more damping.
    pub damping: f32,

    /// Base spring constant for graph-distance forces.
    /// Actual per-pair strength is `spring_constant / graph_dist²`.
    pub spring_constant: f32,

    /// Ideal separation *per graph hop* — two nodes with graph distance `g`
    /// will settle at roughly `ideal_length * g` apart.
    pub ideal_length: f32,

    /// Coulomb-like repulsion strength applied between every pair.
    pub repulsion_strength: f32,

    /// Magnitude of the boundary repulsion force.
    pub boundary_strength: f32,

    /// Distance from the arena wall at which boundary repulsion activates.
    pub boundary_margin: f32,

    /// Ignore any graph updates if dt exceeds this value. This happens if the simulation gets paused by the browser.
    pub max_delta_t: f32,
}

/// Snapshot of simulation state returned by [`ForceGraph::update`].
///
/// All metrics exclude anchored nodes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UpdateResult {
    /// Sum of `½ v²` over every non-anchored node.
    pub total_kinetic_energy: f32,

    /// Largest velocity magnitude among non-anchored nodes.
    pub max_velocity: f32,

    /// Largest positional displacement (Euclidean) of any non-anchored node
    /// during this tick.
    pub max_displacement: f32,
}

impl Arena {
    pub fn bounds(&self) -> Rect2 {
        let half_w = self.w() / 2.0;
        let half_h = self.h() / 2.0;
        let top_left = Point2::from([-half_w, -half_h]);
        Rect2::new(top_left, self.0)
    }
}

impl UpdateResult {
    /// `true` when every tracked metric is at or below the given threshold.
    pub fn is_stable(&self, threshold: f32) -> bool {
        self.max_displacement <= threshold && self.max_velocity <= threshold
    }
}

/// Force-directed graph layout simulation over a petgraph [`UnGraph`].
///
/// Three classes of force are applied each tick:
///
/// 1. **Spring forces** between all reachable pairs, scaled by shortest-path
///    graph distance so that topologically close nodes attract more strongly.
/// 2. **Coulomb repulsion** between every pair to prevent overlap.
/// 3. **Boundary repulsion** that ramps up quadratically as a node approaches
///    the arena wall, plus a hard position clamp as a safety net.
pub struct ForceGraph<N, E> {
    graph: UnGraph<N, E>,
    states: Vec<NodeState>,
    distances: Vec<Vec<Option<u32>>>,
    arena: Arena,
    params: SimulationParams,
}

#[derive(Debug, Clone)]
pub struct NodeState {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub anchored: bool,
}

impl Default for Arena {
    fn default() -> Self {
        Self(Area2::from([500.0, 500.0]))
    }
}

impl Default for SimulationParams {
    fn default() -> Self {
        Self {
            initial_layout: InitialLayout::Circular,
            damping: 0.9,
            spring_constant: 1.0,
            ideal_length: 120.0,
            repulsion_strength: 400.0,
            boundary_strength: 2000.0,
            boundary_margin: 50.0,
            max_delta_t: 0.5,
        }
    }
}

impl<N, E> ForceGraph<N, E> {
    pub fn new(graph: UnGraph<N, E>, arena: Arena, params: SimulationParams) -> Self {
        info!(?arena, "setting up initial graph layout within arena");
        let states = match params.initial_layout {
            InitialLayout::Circular => circular_layout(&graph, &arena),
            InitialLayout::Random => randomized_layout(&graph, &arena),
        };
        let distances = all_pairs_distances(&graph);
        Self {
            graph,
            states,
            distances,
            arena,
            params,
        }
    }

    pub fn graph(&self) -> &UnGraph<N, E> {
        &self.graph
    }

    pub fn graph_mut(&mut self) -> impl GraphInfoMut<Node = N, Edge = E> {
        // &mut self.graph
        // #[derive(Deref)]
        struct Info<'g, N, E> {
            graph: &'g mut ForceGraph<N, E>,
        }

        impl<'g, N, E> DerefMut for Info<'g, N, E> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.graph.graph
            }
        }

        impl<'g, N, E> Deref for Info<'g, N, E> {
            type Target = UnGraph<N, E>;

            fn deref(&self) -> &Self::Target {
                &self.graph.graph
            }
        }

        impl<'g, N, E> GraphInfo for Info<'g, N, E> {
            type Node = N;

            type Edge = E;

            fn node_state(&self, idx: NodeIndex) -> Option<&NodeState> {
                self.graph.states.get(idx.index())
            }
        }

        impl<'g, N, E> GraphInfoMut for Info<'g, N, E> {
            fn node_state_mut(&mut self, idx: NodeIndex) -> Option<&mut NodeState> {
                self.graph.states.get_mut(idx.index())
            }
        }

        Info { graph: self }
    }

    pub fn arena(&self) -> &Arena {
        &self.arena
    }

    pub fn position(&self, idx: NodeIndex) -> [f32; 2] {
        self.states[idx.index()].position
    }

    pub fn set_position(&mut self, idx: NodeIndex, position: [f32; 2]) {
        self.states[idx.index()].position = position;
    }

    pub fn set_anchored(&mut self, idx: NodeIndex, anchored: bool) {
        self.states[idx.index()].anchored = anchored;
    }

    pub fn visit_nodes(&self, mut visitor: impl FnMut(NodeIndex, &N, [f32; 2])) {
        for idx in self.graph.node_indices() {
            visitor(idx, &self.graph[idx], self.states[idx.index()].position);
        }
    }

    pub fn visit_edges(&self, mut visitor: impl FnMut(&N, [f32; 2], &N, [f32; 2], &E)) {
        for edge_idx in self.graph.edge_indices() {
            if let Some((src, tgt)) = self.graph.edge_endpoints(edge_idx) {
                visitor(
                    &self.graph[src],
                    self.states[src.index()].position,
                    &self.graph[tgt],
                    self.states[tgt.index()].position,
                    &self.graph[edge_idx],
                );
            }
        }
    }

    /// Advance the simulation by `dt` seconds.
    ///
    /// Returns an [`UpdateResult`] describing how much the layout changed,
    /// which can be used to decide when to stop iterating.
    pub fn update(&mut self, dt: f32) -> UpdateResult {
        let n = self.states.len();
        if n == 0 {
            return UpdateResult {
                total_kinetic_energy: 0.0,
                max_velocity: 0.0,
                max_displacement: 0.0,
            };
        }

        let dt = dt.min(self.params.max_delta_t);

        let mut forces = vec![[0.0_f32; 2]; n];
        self.apply_spring_forces(&mut forces);
        self.apply_repulsion(&mut forces);
        self.apply_boundary_repulsion(&mut forces);
        self.integrate(dt, &forces)
    }

    // --- private force helpers ------------------------------------------------

    /// Kamada-Kawai–style spring: ideal distance = `ideal_length * g`,
    /// spring constant = `spring_constant / g²`.  Pairs without a finite
    /// graph distance are skipped (handled by Coulomb repulsion only).
    fn apply_spring_forces(&self, forces: &mut [[f32; 2]]) {
        let n = self.states.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let Some(g) = self.distances[i][j] else {
                    continue;
                };
                if g == 0 {
                    continue;
                }

                let [xi, yi] = self.states[i].position;
                let [xj, yj] = self.states[j].position;
                let dx = xj - xi;
                let dy = yj - yi;
                let (dx, dy, dist) = safe_displacement(dx, dy);

                let gf = g as f32;
                let ideal = self.params.ideal_length * gf;
                let k = self.params.spring_constant / (gf * gf);
                let mag = k * (dist - ideal);

                let fx = mag * dx / dist;
                let fy = mag * dy / dist;

                forces[i][0] += fx;
                forces[i][1] += fy;
                forces[j][0] -= fx;
                forces[j][1] -= fy;
            }
        }
    }

    /// Coulomb-like inverse-square repulsion between every pair, regardless of
    /// connectivity.
    fn apply_repulsion(&self, forces: &mut [[f32; 2]]) {
        let n = self.states.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let [xi, yi] = self.states[i].position;
                let [xj, yj] = self.states[j].position;
                let dx = xj - xi;
                let dy = yj - yi;
                let (dx, dy, dist) = safe_displacement(dx, dy);
                let dist_sq = dist * dist;

                let mag = self.params.repulsion_strength / dist_sq;
                let fx = mag * dx / dist;
                let fy = mag * dy / dist;

                forces[i][0] -= fx;
                forces[i][1] -= fy;
                forces[j][0] += fx;
                forces[j][1] += fy;
            }
        }
    }

    /// Quadratic ramp pushing nodes away from the arena walls.  The force is
    /// zero at `boundary_margin` distance and rises to `boundary_strength` at
    /// the wall itself.
    fn apply_boundary_repulsion(&self, forces: &mut [[f32; 2]]) {
        let half_w = self.arena.w() / 2.0;
        let half_h = self.arena.h() / 2.0;
        let margin = self.params.boundary_margin;
        let strength = self.params.boundary_strength;

        for (i, state) in self.states.iter().enumerate() {
            let [x, y] = state.position;

            // (distance_to_wall, force_axis, force_sign)
            let walls: [(f32, usize, f32); 4] = [
                (x + half_w, 0, 1.0),  // left  → push right
                (half_w - x, 0, -1.0), // right → push left
                (y + half_h, 1, 1.0),  // top   → push down
                (half_h - y, 1, -1.0), // bot   → push up
            ];

            for (d, axis, sign) in walls {
                if d < margin {
                    let t = if d > 0.0 { 1.0 - d / margin } else { 1.0 };
                    forces[i][axis] += strength * t * t * sign;
                }
            }
        }
    }

    /// Semi-implicit Euler integration with damping and hard clamping.
    fn integrate(&mut self, dt: f32, forces: &[[f32; 2]]) -> UpdateResult {
        let half_w = self.arena.w() / 2.0;
        let half_h = self.arena.h() / 2.0;
        let damping = self.params.damping;

        let mut total_ke = 0.0_f32;
        let mut max_vel = 0.0_f32;
        let mut max_disp = 0.0_f32;

        for (i, state) in self.states.iter_mut().enumerate() {
            if state.anchored {
                continue;
            }

            state.velocity[0] =
                ((state.velocity[0] + forces[i][0] * dt) * damping).clamp(-MAX_SPEED, MAX_SPEED);
            state.velocity[1] =
                ((state.velocity[1] + forces[i][1] * dt) * damping).clamp(-MAX_SPEED, MAX_SPEED);

            let prev = state.position;

            state.position[0] = (state.position[0] + state.velocity[0] * dt).clamp(-half_w, half_w);
            state.position[1] = (state.position[1] + state.velocity[1] * dt).clamp(-half_h, half_h);

            let vx = state.velocity[0];
            let vy = state.velocity[1];
            let v_mag = (vx * vx + vy * vy).sqrt();
            let ddx = state.position[0] - prev[0];
            let ddy = state.position[1] - prev[1];
            let disp = (ddx * ddx + ddy * ddy).sqrt();

            total_ke += 0.5 * (vx * vx + vy * vy);
            max_vel = max_vel.max(v_mag);
            max_disp = max_disp.max(disp);
        }

        UpdateResult {
            total_kinetic_energy: total_ke,
            max_velocity: max_vel,
            max_displacement: max_disp,
        }
    }
}

// --- free helpers -------------------------------------------------------------

/// Spread nodes evenly around a circle at 40 % of the arena's half-extent.
fn circular_layout<N, E>(graph: &UnGraph<N, E>, arena: &Arena) -> Vec<NodeState> {
    let n = graph.node_count();
    let radius = arena.w().min(arena.h()) / 2.0 * 0.4;
    (0..n)
        .map(|i| {
            let angle = 2.0 * std::f32::consts::PI * i as f32 / n.max(1) as f32;
            NodeState {
                position: [radius * angle.cos(), radius * angle.sin()],
                velocity: [0.0, 0.0],
                anchored: false,
            }
        })
        .collect()
}

fn randomized_layout<N, E>(graph: &UnGraph<N, E>, arena: &Arena) -> Vec<NodeState> {
    let n = graph.node_count();
    let mut rng = rand::rng();
    let w_40 = arena.w() * 0.4;
    let h_40 = arena.h() * 0.4;

    (0..n)
        .map(|_| NodeState {
            position: [
                rng.random::<f32>() * w_40 - w_40 / 2.0,
                rng.random::<f32>() * h_40 - h_40 / 2.0,
            ],
            velocity: [0.0, 0.0],
            anchored: false,
        })
        .collect()
}

/// BFS from every node to build an all-pairs shortest-path matrix.
/// `None` means the pair is in different connected components.
fn all_pairs_distances<N, E>(graph: &UnGraph<N, E>) -> Vec<Vec<Option<u32>>> {
    let n = graph.node_count();
    let mut distances = vec![vec![None; n]; n];

    for start in graph.node_indices() {
        let si = start.index();
        distances[si][si] = Some(0);

        let mut queue = VecDeque::new();
        queue.push_back((start, 0u32));

        while let Some((node, dist)) = queue.pop_front() {
            for neighbor in graph.neighbors(node) {
                let ni = neighbor.index();
                if distances[si][ni].is_none() {
                    distances[si][ni] = Some(dist + 1);
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }
    }

    distances
}

/// Returns `(dx, dy, dist)` where `dist >= MIN_DISTANCE`.
///
/// When two nodes are nearly coincident, the raw `(dx, dy)` vector is too
/// small to produce a meaningful direction. This helper substitutes a random
/// unit vector scaled to `MIN_DISTANCE`, giving the force a well-defined
/// direction to push the pair apart.
fn safe_displacement(dx: f32, dy: f32) -> (f32, f32, f32) {
    let raw_dist = (dx * dx + dy * dy).sqrt();
    if raw_dist >= MIN_DISTANCE {
        return (dx, dy, raw_dist);
    }
    // Pick a random angle so coincident nodes get pushed in an arbitrary
    // (but nonzero) direction.
    let mut rng = rand::rng();
    let angle = rng.random::<f32>() * std::f32::consts::TAU;
    let (sin, cos) = angle.sin_cos();
    (MIN_DISTANCE * cos, MIN_DISTANCE * sin, MIN_DISTANCE)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn triangle_graph() -> UnGraph<&'static str, ()> {
        let mut g = UnGraph::new_undirected();
        let a = g.add_node("a");
        let b = g.add_node("b");
        let c = g.add_node("c");
        g.add_edge(a, b, ());
        g.add_edge(b, c, ());
        g.add_edge(c, a, ());
        g
    }

    fn path_graph() -> UnGraph<&'static str, ()> {
        let mut g = UnGraph::new_undirected();
        let a = g.add_node("a");
        let b = g.add_node("b");
        let c = g.add_node("c");
        g.add_edge(a, b, ());
        g.add_edge(b, c, ());
        g
    }

    // --- all_pairs_distances --------------------------------------------------

    #[test]
    fn distances_on_triangle_are_all_one() {
        let dists = all_pairs_distances(&triangle_graph());
        assert_eq!(dists[0][0], Some(0));
        assert_eq!(dists[0][1], Some(1));
        assert_eq!(dists[0][2], Some(1));
        assert_eq!(dists[1][2], Some(1));
    }

    #[test]
    fn distances_on_path_reflect_hop_count() {
        let dists = all_pairs_distances(&path_graph());
        assert_eq!(dists[0][1], Some(1));
        assert_eq!(dists[0][2], Some(2));
        assert_eq!(dists[1][2], Some(1));
        assert_eq!(dists[2][0], Some(2));
    }

    #[test]
    fn disconnected_nodes_have_no_distance() {
        let mut g: UnGraph<&str, ()> = UnGraph::new_undirected();
        g.add_node("a");
        g.add_node("b");
        let dists = all_pairs_distances(&g);
        assert_eq!(dists[0][1], None);
        assert_eq!(dists[1][0], None);
    }

    // --- ForceGraph -----------------------------------------------------------

    #[test]
    fn empty_graph_update_is_noop() {
        let g: UnGraph<(), ()> = UnGraph::new_undirected();
        let mut sim = ForceGraph::new(g, Arena::default(), SimulationParams::default());
        let result = sim.update(0.016);
        assert_eq!(sim.graph().node_count(), 0);
        assert!(result.is_stable(0.0));
    }

    #[test]
    fn single_node_stays_put_without_forces() {
        let mut g: UnGraph<&str, ()> = UnGraph::new_undirected();
        g.add_node("only");
        let mut sim = ForceGraph::new(g, Arena::default(), SimulationParams::default());
        let before = sim.position(NodeIndex::new(0));
        for _ in 0..100 {
            sim.update(0.016);
        }
        assert_eq!(sim.position(NodeIndex::new(0)), before);
    }

    #[test]
    fn anchored_node_does_not_move() {
        let mut sim = ForceGraph::new(
            triangle_graph(),
            Arena::default(),
            SimulationParams::default(),
        );
        sim.set_position(NodeIndex::new(0), [10.0, 20.0]);
        sim.set_anchored(NodeIndex::new(0), true);

        for _ in 0..200 {
            sim.update(0.016);
        }
        assert_eq!(sim.position(NodeIndex::new(0)), [10.0, 20.0]);
    }

    #[test]
    fn simulation_causes_movement() {
        let mut sim = ForceGraph::new(path_graph(), Arena::default(), SimulationParams::default());
        let initial: Vec<_> = sim
            .graph()
            .node_indices()
            .map(|i| sim.position(i))
            .collect();

        for _ in 0..10 {
            sim.update(0.016);
        }

        let moved = sim
            .graph()
            .node_indices()
            .any(|i| sim.position(i) != initial[i.index()]);
        assert!(moved, "at least one node should have moved");
    }

    #[test]
    fn nodes_stay_within_arena_bounds() {
        let arena = Arena::new([200.0, 200.0]);
        let mut sim = ForceGraph::new(path_graph(), arena, SimulationParams::default());

        for _ in 0..1000 {
            sim.update(0.016);
        }

        for idx in sim.graph().node_indices() {
            let [x, y] = sim.position(idx);
            assert!(
                x.abs() <= 100.0,
                "x={x} exceeds half-width for node {idx:?}"
            );
            assert!(
                y.abs() <= 100.0,
                "y={y} exceeds half-height for node {idx:?}"
            );
        }
    }

    #[test]
    fn set_position_is_reflected() {
        let mut sim = ForceGraph::new(
            triangle_graph(),
            Arena::default(),
            SimulationParams::default(),
        );
        sim.set_position(NodeIndex::new(1), [42.0, -7.0]);
        assert_eq!(sim.position(NodeIndex::new(1)), [42.0, -7.0]);
    }

    #[test]
    fn visit_nodes_yields_all_nodes() {
        let sim = ForceGraph::new(
            triangle_graph(),
            Arena::default(),
            SimulationParams::default(),
        );
        let mut count = 0;
        sim.visit_nodes(|_, _, _| count += 1);
        assert_eq!(count, 3);
    }

    #[test]
    fn visit_edges_yields_all_edges() {
        let sim = ForceGraph::new(
            triangle_graph(),
            Arena::default(),
            SimulationParams::default(),
        );
        let mut count = 0;
        sim.visit_edges(|_, _, _, _, _| count += 1);
        assert_eq!(count, 3);
    }

    // --- UpdateResult ---------------------------------------------------------

    #[test]
    fn first_update_reports_nonzero_metrics() {
        let mut sim = ForceGraph::new(path_graph(), Arena::default(), SimulationParams::default());
        let result = sim.update(0.016);
        assert!(result.max_displacement > 0.0);
        assert!(result.max_velocity > 0.0);
        assert!(result.total_kinetic_energy > 0.0);
    }

    #[test]
    fn simulation_eventually_stabilises() {
        let mut sim = ForceGraph::new(
            triangle_graph(),
            Arena::default(),
            SimulationParams::default(),
        );
        let mut stable = false;
        for _ in 0..10_000 {
            let result = sim.update(0.016);
            if result.is_stable(0.01) {
                stable = true;
                break;
            }
        }
        assert!(stable, "simulation should converge within 10 000 ticks");
    }
}
