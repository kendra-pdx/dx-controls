pub mod force;

#[cfg(feature = "ui")]
pub mod svg;

#[cfg(feature = "ui")]
use dioxus::prelude::*;
#[cfg(feature = "ui")]
use dx_web_animate::use_request_animation_frame;

#[cfg(feature = "ui")]
pub use dx_web_animate::AnimationTime;
#[cfg(feature = "ui")]
use petgraph::prelude::*;
#[cfg(feature = "ui")]
use spacial::prelude::*;

#[cfg(feature = "ui")]
use crate::{
    force::{Arena, ForceGraph, GraphInfoMut, SimulationParams},
    svg::{IntoSvgEdgeElement, IntoSvgNodeElement, Viewbox},
};

#[cfg(feature = "ui")]
pub trait GraphData: Into<UnGraph<Self::Node, Self::Edge>> + Clone + PartialEq + 'static {
    type RenderState: Default;
    type Node: PartialEq + IntoSvgNodeElement<RenderState = Self::RenderState>;
    type Edge: PartialEq + IntoSvgEdgeElement<RenderState = Self::RenderState, Node = Self::Node>;

    #[allow(unused_variables)]
    fn on_init(
        graph: impl GraphInfoMut<Node = Self::Node, Edge = Self::Edge>,
        render_state: &mut Self::RenderState,
    ) {
        // by default do nothing
    }

    #[allow(unused_variables)]
    fn on_before_update(
        graph: impl GraphInfoMut<Node = Self::Node, Edge = Self::Edge>,
        time: AnimationTime,
        render_state: &mut Self::RenderState,
    ) {
        // by default do nothing.
    }

    #[allow(unused_variables)]
    fn on_after_update(
        graph: impl GraphInfoMut<Node = Self::Node, Edge = Self::Edge>,
        time: AnimationTime,
        render_state: &mut Self::RenderState,
    ) {
        // by default do nothing.
    }

    fn new_render_state(&self) -> Self::RenderState {
        Self::RenderState::default()
    }
}

#[cfg(feature = "ui")]
#[component]
pub fn Graph<G: GraphData>(
    /// attributes to apply to the root svg element
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// the simulation's parameters
    #[props(default)]
    sim_params: SimulationParams,

    /// the arena defines the space where all nodes are bound within with the center at (0,0)
    #[props(default)]
    arena: ReadSignal<Arena>,

    /// the viewbox defines the display space, should be relative to the arena. defaults to the arena size (plus some padding)
    #[props(default)]
    view_box: Option<ReadSignal<Viewbox>>,

    /// optionally stop the animation once the relative change in graph shape drops below this threshold
    #[props(default)]
    stablize_threshold: Option<f32>,

    /// the graph data
    graph: G,
) -> Element {
    let padding = use_signal(|| 10.0);
    let arena_bounds = use_memo(move || arena.read().bounds());

    let view_box: ReadSignal<Viewbox> = if let Some(view_box) = view_box {
        view_box
    } else {
        Viewbox::new_use(arena_bounds.into(), padding.into())
    };

    let mut render_state = use_signal(|| graph.new_render_state());

    let mut graph = use_signal(|| {
        let graph: UnGraph<_, _> = graph.into();
        ForceGraph::new(graph, *arena.read(), sim_params)
    });

    use_hook(|| {
        // the hook makes this just happen once
        G::on_init(graph.write().graph_mut(), &mut render_state.write());
    });

    use_request_animation_frame({
        // let mut render_state = render_state.clone();
        move |t| {
            let dt = t.delta_t.as_secs_f32();
            let mut g = graph.write();

            let mut render_state = render_state.write();
            G::on_before_update(g.graph_mut(), t, &mut render_state);
            let delta = g.update(dt);
            G::on_after_update(g.graph_mut(), t, &mut render_state);

            if let Some(stablize_threshold) = stablize_threshold {
                !delta.is_stable(stablize_threshold)
            } else {
                true
            }
        }
    });

    let nodes = use_memo({
        // let mut render_state = render_state.clone();
        move || {
            let mut nodes = Vec::new();
            graph.read().visit_nodes(|_, node, [x, y]| {
                let node = node.to_node_element(&render_state.read(), &Point2::from([x, y]));
                nodes.push(node);
            });
            nodes
        }
    });

    let edges = use_memo({
        // let mut render_state = render_state.clone();
        move || {
            let mut edges = Vec::new();
            graph.read().visit_edges(|a, [ax, ay], b, [bx, by], edge| {
                let from_point = Point2::from([ax, ay]);
                let from = a;

                let to_point = Point2::from([bx, by]);
                let to = b;

                let edge = edge.to_edge_element(
                    &render_state.read(),
                    (&from_point, from),
                    (&to_point, to),
                );
                edges.push(edge)
            });
            edges
        }
    });

    rsx! {
        svg { view_box: view_box().to_attribute(),
            width: view_box().width(),
            height: view_box().height(),
            ..attributes,
            for e in edges.iter() {
                {e.clone()}
            }

            for i in nodes.iter() {
                {i.clone()}
            }
        }
    }
}
