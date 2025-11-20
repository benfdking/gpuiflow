use gpui::*;
use crate::graph::{Graph, Node};
use crate::components::node::render_node;


use crate::components::edge::render_edge;
use uuid::Uuid;

struct DragState {
    node_id: Uuid,
    offset: Point<f32>,
}

pub struct GraphView<D: 'static> {
    graph: Graph<D>,
    drag_state: Option<DragState>,
    pan_offset: Point<f32>,
    zoom_level: f32,
    is_panning: bool,
    last_mouse_pos: Point<f32>,
    is_locked: bool,
}

impl<D: Clone + Send + Sync + 'static> GraphView<D> {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            graph: Graph::new(),
            drag_state: None,
            pan_offset: Point::default(),
            zoom_level: 1.0,
            is_panning: false,
            last_mouse_pos: Point::default(),
            is_locked: false,
        }
    }

    pub fn add_node(&mut self, node: Node<D>) {
        self.graph.add_node(node);
    }

    pub fn add_edge(&mut self, edge: crate::graph::Edge) {
        self.graph.add_edge(edge);
    }

    fn handle_scroll_wheel(&mut self, event: &ScrollWheelEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let delta = event.delta.pixel_delta(px(20.0));
        if delta.y != px(0.0) {
            let zoom_factor = 1.1f32;
            let new_zoom = if delta.y > px(0.0) {
                self.zoom_level * zoom_factor
            } else {
                self.zoom_level / zoom_factor
            };
            // Clamp zoom
            self.zoom_level = new_zoom.clamp(0.1, 5.0);
            cx.notify();
        } else {
            // Pan with scroll wheel (touchpad)
            // Convert delta to f32
            let delta_f32 = point(f32::from(delta.x), f32::from(delta.y));
            self.pan_offset = self.pan_offset + delta_f32;
            cx.notify();
        }
    }

    fn handle_mouse_down(&mut self, event: &MouseDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let position = event.position.map(|p| f32::from(p));
        // Transform click position to graph coordinates
        let graph_pos = (position - self.pan_offset) / self.zoom_level;

        // Check if we clicked on a node
        let mut clicked_node = false;
        for node in self.graph.nodes.iter().rev() {
            let node_rect = Bounds::new(node.position, size(150.0, 80.0));
            if node_rect.contains(&graph_pos) {
                self.drag_state = Some(DragState {
                    node_id: node.id,
                    offset: graph_pos - node.position,
                });
                clicked_node = true;
                cx.notify();
                break;
            }
        }

        if !clicked_node {
            self.is_panning = true;
            self.last_mouse_pos = position;
        }
    }

    fn handle_mouse_up(&mut self, _event: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if self.drag_state.is_some() {
            self.drag_state = None;
            cx.notify();
        }
        if self.is_panning {
            self.is_panning = false;
            cx.notify();
        }
    }

    fn handle_mouse_move(&mut self, event: &MouseMoveEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let position = event.position.map(|p| f32::from(p));
        if let Some(drag_state) = &self.drag_state {
            let graph_pos = (position - self.pan_offset) / self.zoom_level;
            if let Some(node) = self.graph.nodes.iter_mut().find(|n| n.id == drag_state.node_id) {
                node.position = graph_pos - drag_state.offset;
                cx.notify();
            }
        } else if self.is_panning {
            let delta = position - self.last_mouse_pos;
            self.pan_offset = self.pan_offset + delta;
            self.last_mouse_pos = position;
            cx.notify();
        }
    }

    pub fn zoom_in(&mut self, _cx: &mut Context<Self>) {
        if self.is_locked { return; }
        self.zoom_level = (self.zoom_level * 1.2).clamp(0.1, 5.0);
    }

    pub fn zoom_out(&mut self, _cx: &mut Context<Self>) {
        if self.is_locked { return; }
        self.zoom_level = (self.zoom_level / 1.2).clamp(0.1, 5.0);
    }

    pub fn fit_view(&mut self, _cx: &mut Context<Self>) {
        if self.is_locked { return; }
        if self.graph.nodes.is_empty() { return; }

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for node in &self.graph.nodes {
            min_x = min_x.min(node.position.x);
            min_y = min_y.min(node.position.y);
            max_x = max_x.max(node.position.x + 150.0); // 150 is node width
            max_y = max_y.max(node.position.y + 80.0);  // 80 is node height
        }

        let width = max_x - min_x;
        let height = max_y - min_y;
        let center_x = min_x + width / 2.0;
        let center_y = min_y + height / 2.0;

        // Assume window size is somewhat fixed or we can get it?
        // For now, let's assume a viewport of 800x600 for calculation or just center it.
        // Actually, without knowing viewport size, "fit view" is hard.
        // But we can just center the graph at 0,0 offset and set zoom to 1.0 or calculate based on some assumption.
        // Let's just reset to default for now as a "Home" button.
        self.pan_offset = point(-center_x + 400.0, -center_y + 300.0); // Center in 800x600
        self.zoom_level = 1.0;
    }

    pub fn toggle_lock(&mut self, _cx: &mut Context<Self>) {
        self.is_locked = !self.is_locked;
    }
}


impl<D: Clone + Send + Sync + 'static> Render for GraphView<D> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .bg(rgb(0x202020))
            .text_color(rgb(0xffffff))
            .relative()
            .on_scroll_wheel(cx.listener(Self::handle_scroll_wheel))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_mouse_down))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .on_mouse_move(cx.listener(Self::handle_mouse_move))
            .child(
                div()
                    .absolute()
                    .left(px(self.pan_offset.x))
                    .top(px(self.pan_offset.y))
                    .size_full()
                    .children(
                        self.graph.edges.iter().filter_map(|edge| {
                            let source = self.graph.get_node(edge.source_id)?;
                            let target = self.graph.get_node(edge.target_id)?;

                            // Helper to get handle position relative to node
                            let get_handle_pos = |node: &Node<D>, handle_id: &Option<String>, default_pos: Point<f32>| -> Point<f32> {
                                if let Some(h_id) = handle_id {
                                    if let Some(handle) = node.handles.iter().find(|h| &h.id == h_id) {
                                        let (x, y) = match handle.position {
                                            crate::graph::HandlePosition::Top => (75.0, 0.0),
                                            crate::graph::HandlePosition::Bottom => (75.0, 80.0),
                                            crate::graph::HandlePosition::Left => (0.0, 40.0),
                                            crate::graph::HandlePosition::Right => (150.0, 40.0),
                                        };
                                        return (node.position * self.zoom_level) + point(x * self.zoom_level, y * self.zoom_level);
                                    }
                                }
                                default_pos
                            };

                            let source_default = (source.position * self.zoom_level) + point(150.0 * self.zoom_level / 2.0, 80.0 * self.zoom_level);
                            let target_default = (target.position * self.zoom_level) + point(150.0 * self.zoom_level / 2.0, 0.0);

                            let source_pos = get_handle_pos(source, &edge.source_handle_id, source_default);
                            let target_pos = get_handle_pos(target, &edge.target_handle_id, target_default);

                            Some(render_edge(edge, source_pos, target_pos, _window))
                        })
                    )
                    .children(
                        self.graph.nodes.iter().map(|node| {
                            div()
                                .absolute()
                                .left(px(node.position.x * self.zoom_level))
                                .top(px(node.position.y * self.zoom_level))
                                .child(
                                    div()
                                        .w(px(150.0 * self.zoom_level))
                                        .h(px(80.0 * self.zoom_level))
                                        .child(render_node(node, _window))
                                )
                        })
                    )
            )
            .child(
                div()
                    .absolute()
                    .bottom(px(10.0))
                    .left(px(10.0))
                    .flex()
                    .flex_col()
                    .bg(rgb(0xffffff))
                    .border_1()
                    .border_color(rgb(0xeeeeee))
                    .shadow_md()
                    .rounded_md()
                    .child(
                        div()
                            .w(px(24.0))
                            .h(px(24.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|s| s.bg(rgb(0xf0f0f0)))
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|view, _, _, cx| {
                                view.zoom_in(cx);
                                cx.notify();
                            }))
                            .child("+")
                            .text_color(rgb(0x000000))
                            .text_sm()
                            .border_b_1()
                            .border_color(rgb(0xeeeeee))
                    )
                    .child(
                        div()
                            .w(px(24.0))
                            .h(px(24.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|s| s.bg(rgb(0xf0f0f0)))
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|view, _, _, cx| {
                                view.zoom_out(cx);
                                cx.notify();
                            }))
                            .child("-")
                            .text_color(rgb(0x000000))
                            .text_sm()
                            .border_b_1()
                            .border_color(rgb(0xeeeeee))
                    )
                    .child(
                        div()
                            .w(px(24.0))
                            .h(px(24.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|s| s.bg(rgb(0xf0f0f0)))
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|view, _, _, cx| {
                                view.fit_view(cx);
                                cx.notify();
                            }))
                            .child("[]")
                            .text_color(rgb(0x000000))
                            .text_sm()
                            .border_b_1()
                            .border_color(rgb(0xeeeeee))
                    )
                    .child(
                        div()
                            .w(px(24.0))
                            .h(px(24.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|s| s.bg(rgb(0xf0f0f0)))
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|view, _, _, cx| {
                                view.toggle_lock(cx);
                                cx.notify();
                            }))
                            .child(if self.is_locked { "L" } else { "U" })
                            .text_color(rgb(0x000000))
                            .text_sm()
                    )
            )
    }
}
