use gpui::*;
use crate::graph::Edge;
use gpui::PathBuilder;

pub fn render_edge(
    _edge: &Edge,
    source_pos: Point<f32>,
    target_pos: Point<f32>,
    _cx: &Window,
) -> impl IntoElement {
    // Draw edge directly using absolute coordinates.
    let start_px = source_pos.map(px);
    let end_px = target_pos.map(px);
    let control_1 = point(source_pos.x + (target_pos.x - source_pos.x) / 2.0, source_pos.y);
    let control_2 = point(target_pos.x - (target_pos.x - source_pos.x) / 2.0, target_pos.y);
    let control_1_px = control_1.map(px);
    let control_2_px = control_2.map(px);

    div()
        .absolute()
        .left(px(0.0))
        .top(px(0.0))
        .size_full()
        .child(
            canvas(
                |_, _, _| {},
                move |_bounds, _, window, _cx| {
                    let mut builder = PathBuilder::stroke(px(2.0));
                    builder.move_to(start_px);
                    builder.cubic_bezier_to(control_1_px, control_2_px, end_px);
                    let path = builder.build().unwrap();
                    window.paint_path(path, rgb(0xaaaaaa));
                },
            )
            .size_full()
        )
}
