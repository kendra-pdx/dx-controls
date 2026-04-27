use dioxus::prelude::*;
use spacial::prelude::*;

pub trait AsLocation {
    fn as_xy(&self) -> impl TwizzleXY<Output = f32> + Copy;
}

pub trait IntoSvgEdgeElement {
    type RenderState;
    type Node;
    fn to_edge_element(
        &self,
        render_state: &Self::RenderState,
        a: (&Point2, &Self::Node),
        b: (&Point2, &Self::Node),
    ) -> Element;
}

pub trait IntoSvgNodeElement {
    type RenderState;
    fn to_node_element(&self, render_state: &Self::RenderState, at: &Point2) -> Element;
}

#[derive(Clone, Copy, PartialEq)]
pub struct Viewbox {
    bounds: ReadSignal<Rect2<[f32; 2], [f32; 2]>>,
    padding: ReadSignal<f32>,
}

impl Viewbox {
    pub fn new_use(
        bounds: ReadSignal<Rect2<[f32; 2], [f32; 2]>>,
        padding: ReadSignal<f32>,
    ) -> ReadSignal<Viewbox> {
        use_memo(move || Viewbox { bounds, padding }).into()
    }

    pub fn width(&self) -> ReadSignal<f32> {
        let bounds = self.bounds;
        let padding = self.padding;

        use_memo(move || bounds().size.w() + padding()).into()
    }

    pub fn height(&self) -> ReadSignal<f32> {
        let bounds = self.bounds;
        let padding = self.padding;

        use_memo(move || bounds().size.h() + padding()).into()
    }

    pub fn to_attribute(&self) -> String {
        let bounds = self.bounds.read();
        let padding = *self.padding.read();

        let x = bounds.at.x() - padding;
        let y = bounds.at.y() - padding;
        let w = bounds.size.w() + padding;
        let h = bounds.size.h() + padding;

        format!("{} {} {} {}", x, y, w, h)
    }
}

pub fn use_bounds<Point: AsLocation + 'static>(points: ReadSignal<Vec<Point>>) -> Memo<Rect2> {
    use_memo(move || {
        let points = points.read();
        let points = points.iter().map(|p| p.as_xy()).collect::<Vec<_>>();
        Rect2::from_points(points.iter())
    })
}
