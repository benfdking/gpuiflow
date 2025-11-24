use crate::graph::Graph;
use crate::types::position::Position;
use crate::view::{GraphEvent, GraphView};
use gpui::*;
use gpui::*;
use gpui::PathBuilder;
use uuid::Uuid;

struct ConnectingState {
    source_node_id: Uuid,
    source_handle_id: String,
    current_pos: Point<f32>,
}

pub struct GraphEditor<D: 'static> {
    graph_view: gpui::View<GraphView<D>>,
    connecting_state: Option<ConnectingState>,
    on_connect_end: Option<Box<dyn Fn(&Uuid, &String, Point<f32>, &mut Graph<D>, &mut Window, &mut Context<Self>)>>,
}

impl<D: Clone + Send + Sync + 'static> GraphEditor<D> {
    pub fn new(graph_view: gpui::View<GraphView<D>>, cx: &mut Context<Self>) -> Self {
        cx.subscribe(&graph_view, Self::handle_graph_event).detach();
        Self {
            graph_view,
            connecting_state: None,
            on_connect_end: None,
        }
    }

    pub fn on_connect_end(
        mut self,
        callback: impl Fn(&Uuid, &String, Point<f32>, &mut Graph<D>, &mut Window, &mut Context<Self>) + 'static,
    ) -> Self {
        self.on_connect_end = Some(Box::new(callback));
        self
    }

    fn handle_graph_event(
        &mut self,
        _view: gpui::View<GraphView<D>>,
        event: &GraphEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            GraphEvent::HandleClicked {
                node_id,
                handle_id,
                position,
            } => {
                self.connecting_state = Some(ConnectingState {
                    source_node_id: *node_id,
                    source_handle_id: handle_id.clone(),
                    current_pos: *position,
                });
                cx.notify();
            }
        }
    }

    fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(connecting) = &mut self.connecting_state {
            let position = event.position.map(|p| f32::from(p));
            // We need to account for pan/zoom from the view if we want to be precise,
            // but for now let's assume the editor overlay is in window coordinates or similar?
            // Actually, the graph view handles pan/zoom internally for its content.
            // If we draw the connection line on top, we need to know where the source handle is in screen coordinates.
            // The handle click event gave us the position in graph coordinates?
            // Wait, `handle_pos` in `GraphView` was `node.position + point(x, y)`. That is graph coordinates.
            // We need to transform it to screen coordinates to draw the line if we are drawing in the editor overlay.
            // OR we can ask the graph view to convert?
            // Or we can just pass the screen position in the event?
            // Let's assume for now we can get the graph view's transform.
            // But `GraphView` state is internal.
            
            // Alternative: The `GraphEditor` wraps `GraphView`.
            // The `GraphView` renders the graph.
            // The `GraphEditor` renders the `GraphView` and then the pending connection line on top.
            // To draw the line correctly, we need the source handle position in screen coordinates.
            // The `GraphEvent` provided `position` which was `handle_pos` (graph coords).
            
            // Let's update `GraphView` to expose `pan_offset` and `zoom_level` or a helper to map coordinates.
            // For now, let's just use the mouse position as the target.
            
            // Wait, if `GraphView` handles pan/zoom, then `GraphEditor` needs to know about it to draw the line correctly relative to the graph?
            // Or `GraphEditor` draws in screen space.
            // The source handle position is in graph space. We need to project it to screen space.
            
            // Let's add `get_screen_pos` to `GraphView`? Or just expose public fields?
            // Let's expose public fields for now or accessors.
            
            connecting.current_pos = position; // This is screen coords (roughly, relative to view)
            cx.notify();
        }
    }

    fn handle_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(connecting) = self.connecting_state.take() {
            if let Some(callback) = &self.on_connect_end {
                 self.graph_view.update(cx, |view, cx| {
                     // We need to pass the drop position in graph coordinates to the callback
                     // so the user can create a node there.
                     // `connecting.current_pos` is in screen coordinates (from mouse move).
                     // We need to unproject it.
                     
                     let graph_pos = (connecting.current_pos - view.pan_offset) / view.zoom_level;

                    (callback)(
                        &connecting.source_node_id,
                        &connecting.source_handle_id,
                        graph_pos,
                        &mut view.graph,
                        window,
                        cx,
                    );
                 });
            }
            cx.notify();
        }
    }
}

impl<D: Clone + Send + Sync + 'static> Render for GraphEditor<D> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .on_mouse_move(cx.listener(Self::handle_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .child(self.graph_view.clone())
            .children(self.connecting_state.as_ref().map(|connecting| {
                let view = self.graph_view.read(cx);
                let source_node = view.graph.get_node(connecting.source_node_id).unwrap();
                let (x, y) = if let Some(handle) = source_node.handles.iter().find(|h| h.id == connecting.source_handle_id) {
                     match handle.position {
                        Position::Top => (75.0, 0.0),
                        Position::Bottom => (75.0, 80.0),
                        Position::Left => (0.0, 40.0),
                        Position::Right => (150.0, 40.0),
                    }
                } else {
                    (0.0, 0.0)
                };
                
                let source_pos = (source_node.position * view.zoom_level) + point(x * view.zoom_level, y * view.zoom_level) + point(view.pan_offset.x, view.pan_offset.y);
                let target_pos = connecting.current_pos;

                div()
                    .absolute()
                    .size_full()
                    .child(
                        canvas(
                            move |_, _, _| {},
                            move |bounds, _, window, _cx| {
                                let start_px = bounds.origin + point(px(source_pos.x), px(source_pos.y));
                                let end_px = bounds.origin + point(px(target_pos.x), px(target_pos.y));
                                
                                let mut builder = PathBuilder::stroke(px(2.0));
                                builder.move_to(start_px);
                                builder.line_to(end_px);
                                
                                window.paint_path(builder.build().unwrap(), rgb(0xffffff));
                            }
                        )
                        .size_full()
                    )
            }))
    }
}
