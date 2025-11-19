use gpui::*;
use gpuiflow::{GraphView, Node, Edge};

fn main() {
    gpui::Application::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|cx| {
                let mut view = GraphView::<()>::new(cx);
                let node1 = Node::new((), Point::new(100.0, 100.0));
                let node2 = Node::new((), Point::new(400.0, 300.0));
                let edge = Edge::new(node1.id, node2.id);
                
                view.add_node(node1);
                view.add_node(node2);
                view.add_edge(edge);
                view
            })
        }).unwrap();
    });
}
