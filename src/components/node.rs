use gpui::*;
use crate::graph::Node;

pub fn render_node<D: Clone + Send + Sync + 'static>(
    _node: &Node<D>,
    _cx: &Window,
) -> impl IntoElement {
    div()
        .size_full()
        .bg(rgb(0x303030))
        .border_1()
        .border_color(rgb(0x000000))
        .rounded_md()
        .shadow_md()
        .flex()
        .flex_col()
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
}
