use gpui::*;
use crate::graph::Node;

pub fn render_node<D: Clone + Send + Sync + 'static>(
    node: &Node<D>,
    _cx: &Window,
) -> impl IntoElement {
    div()
        .size_full()
        .bg(rgb(0x303030))
        .border_1()
        .border_color(rgb(0x000000))
        .rounded_md()
        .shadow_md()
        .relative() // Make sure handles can be positioned absolutely relative to this
        .children(
            node.handles.iter().map(|handle| {
                let (t, b, l, r) = match handle.position {
                    crate::graph::HandlePosition::Top => (Some(px(-5.0)), None, Some(px(75.0 - 5.0)), None), // Center top
                    crate::graph::HandlePosition::Bottom => (None, Some(px(-5.0)), Some(px(75.0 - 5.0)), None), // Center bottom
                    crate::graph::HandlePosition::Left => (Some(px(40.0 - 5.0)), None, Some(px(-5.0)), None), // Center left
                    crate::graph::HandlePosition::Right => (Some(px(40.0 - 5.0)), None, None, Some(px(-5.0))), // Center right
                };

                let mut div = div()
                    .absolute()
                    .w(px(10.0))
                    .h(px(10.0))
                    .bg(rgb(0xffffff))
                    .rounded_full()
                    .border_1()
                    .border_color(rgb(0x000000));

                if let Some(val) = t { div = div.top(val); }
                if let Some(val) = b { div = div.bottom(val); }
                if let Some(val) = l { div = div.left(val); }
                if let Some(val) = r { div = div.right(val); }

                div
            })
        )
        .child(
            div()
                .flex()
                .flex_col()
                .size_full()
                .child(
                    div()
                        .w_full()
                        .h(px(24.0))
                        .bg(rgb(0x404040))
                        .rounded_t_md()
                        .flex()
                        .items_center()
                        .px_2()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .child("Node Title")
                        )
                )
                .child(
                    div()
                        .flex_1()
                        .p_2()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child("Content")
                )
        )
}
