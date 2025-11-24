use gpui::*;
use gpuiflow::{Edge, GraphView, Handle, HandleType, Node};
use gpuiflow::types::position::Position;

fn main() {
    gpui::Application::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|cx| {
                let mut view = GraphView::<()>::new(cx);

                let node1 = Node::new((), Point::new(100.0, 100.0)).with_handles(vec![
                    Handle::new("right", HandleType::Source, Position::Right),
                    Handle::new("bottom", HandleType::Source, Position::Bottom),
                ]);

                let node2 =
                    Node::new((), Point::new(400.0, 100.0)).with_handles(vec![Handle::new(
                        "left",
                        HandleType::Target,
                        Position::Left,
                    )]);

                let node3 =
                    Node::new((), Point::new(100.0, 400.0)).with_handles(vec![Handle::new(
                        "top",
                        HandleType::Target,
                        Position::Top,
                    )]);

                let edge1 = Edge::new(node1.id, node2.id).with_handles("right", "left");

                let edge2 = Edge::new(node1.id, node3.id).with_handles("bottom", "top");

                view.add_node(node1);
                view.add_node(node2);
                view.add_node(node3);
                view.add_edge(edge1);
                view.add_edge(edge2);
                view
            })
        })
        .unwrap();
    });
}
