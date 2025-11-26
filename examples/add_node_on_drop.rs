use gpui::*;
use gpuiflow::{Edge, GraphEditor, GraphView, Handle, HandleType, Node};
use gpuiflow::types::position::Position;

fn main() {
    gpui::Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(None, size(px(800.0), px(600.0)), cx))),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|cx| {
                    let mut view = GraphView::<String>::new(cx);

                    let node1 = Node::new("Node 1".to_string(), Point::new(100.0, 100.0)).with_handles(vec![
                        Handle::new("right", HandleType::Source, Position::Right),
                    ]);

                    view.add_node(node1);
                    
                    let view_handle = cx.new_view(|_| view);

                    let editor = GraphEditor::new(view_handle, cx)
                        .on_connect_end(|source_id, source_handle, pos, graph, _window, cx| {
                            println!("Connect end at {:?}", pos);
                            
                            // Create a new node at the drop position
                            let new_node_id = uuid::Uuid::new_v4();
                            let new_node = Node::new(format!("Node {}", new_node_id), pos).with_handles(vec![
                                Handle::new("left", HandleType::Target, Position::Left),
                                Handle::new("right", HandleType::Source, Position::Right),
                            ]);
                            
                            let new_node_id = new_node.id;
                            graph.add_node(new_node);

                            // Create an edge from the source to the new node
                            let edge = Edge::new(*source_id, new_node_id).with_handles(source_handle.clone(), "left");
                            graph.add_edge(edge);
                            
                            cx.notify();
                        });
                        
                    editor
                })
            },
        );
    });
}
