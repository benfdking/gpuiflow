use crate::graph::Edge;
use gpui::PathBuilder;
use gpui::*;

pub fn render_edge(
    _edge: &Edge,
    source_pos: Point<f32>,
    target_pos: Point<f32>,
    _cx: &Window,
) -> impl IntoElement {
    // Draw edge directly using absolute coordinates.
    let start_px = source_pos.map(px);
    let end_px = target_pos.map(px);
    let control_1 = point(
        source_pos.x,
        source_pos.y + (target_pos.y - source_pos.y) / 2.0,
    );
    let control_2 = point(
        target_pos.x,
        target_pos.y - (target_pos.y - source_pos.y) / 2.0,
    );
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
                move |bounds, _, window, _cx| {
                    let start_px = bounds.origin + start_px;
                    let end_px = bounds.origin + end_px;
                    let control_1_px = bounds.origin + control_1_px;
                    let control_2_px = bounds.origin + control_2_px;

                    // Manually subdivide bezier curve to ensure correct rendering
                    let steps = 20;
                    let mut builder = PathBuilder::stroke(px(2.0));
                    builder.move_to(start_px);

                    for i in 1..=steps {
                        let t = i as f32 / steps as f32;
                        let t_inv = 1.0 - t;

                        // Cubic bezier formula: (1-t)^3 P0 + 3(1-t)^2 t P1 + 3(1-t) t^2 P2 + t^3 P3
                        let p0 = start_px;
                        let p1 = control_1_px;
                        let p2 = control_2_px;
                        let p3 = end_px;

                        let x = t_inv.powi(3) * f32::from(p0.x)
                            + 3.0 * t_inv.powi(2) * t * f32::from(p1.x)
                            + 3.0 * t_inv * t.powi(2) * f32::from(p2.x)
                            + t.powi(3) * f32::from(p3.x);

                        let y = t_inv.powi(3) * f32::from(p0.y)
                            + 3.0 * t_inv.powi(2) * t * f32::from(p1.y)
                            + 3.0 * t_inv * t.powi(2) * f32::from(p2.y)
                            + t.powi(3) * f32::from(p3.y);

                        builder.line_to(point(px(x), px(y)));
                    }

                    window.paint_path(builder.build().unwrap(), rgb(0xaaaaaa));
                },
            )
            .size_full(),
        )
}
