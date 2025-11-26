use gpui::*;
use gpuiflow::{Edge, GraphView, Node};

fn main() {
    gpui::Application::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|cx| {
                let mut view = GraphView::<()>::new(cx);

                // Register a custom node type "custom"
                view.register_node_type("custom", |_node, _cx| {
                    div()
                        .size_full()
                        .bg(rgb(0x505080)) // Different background color
                        .border_2()
                        .border_color(rgb(0x8080ff))
                        .rounded_xl()
                        .shadow_lg()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .text_lg()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff))
                                .child("Custom Node"),
                        )
                });

                let node1 = Node::new((), Point::new(100.0, 100.0));
                let node2 = Node::new((), Point::new(400.0, 300.0)).with_type("custom");
                let edge = Edge::new(node1.id, node2.id);

                view.add_node(node1);
                view.add_node(node2);
                view.add_edge(edge);
                view
            })
        })
        .unwrap();
    });
}
