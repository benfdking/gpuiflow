use gpui::*;
use gpuiflow::components::background::{BackgroundProps, BackgroundVariant};
use gpuiflow::{Edge, GraphView, Handle, HandleType, Node, Position};

struct BackgroundExample {
    graph: Entity<GraphView<String>>,
    background: BackgroundProps,
}

impl BackgroundExample {
    fn new(cx: &mut Context<Self>) -> Self {
        let graph = cx.new(|cx| {
            let mut view = GraphView::<String>::new(cx);

            // Create nodes with custom data and handles
            let node1 =
                Node::new("Input Node".to_string(), Point::new(100.0, 150.0)).with_handles(vec![
                    Handle::new("out", HandleType::Source, Position::Right),
                ]);

            let node2 =
                Node::new("Process".to_string(), Point::new(350.0, 100.0)).with_handles(vec![
                    Handle::new("in", HandleType::Target, Position::Left),
                    Handle::new("out", HandleType::Source, Position::Right),
                ]);

            let node3 = Node::new("Output".to_string(), Point::new(350.0, 250.0))
                .with_handles(vec![Handle::new("in", HandleType::Target, Position::Left)]);

            let node4 =
                Node::new("Result".to_string(), Point::new(600.0, 175.0)).with_handles(vec![
                    Handle::new("in1", HandleType::Target, Position::Left),
                    Handle::new("in2", HandleType::Target, Position::Left),
                ]);

            // Create edges with handles
            let edge1 = Edge::new(node1.id, node2.id).with_handles("out", "in");
            let edge2 = Edge::new(node1.id, node3.id).with_handles("out", "in");
            let edge3 = Edge::new(node2.id, node4.id).with_handles("out", "in1");
            let edge4 = Edge::new(node3.id, node4.id).with_handles("in", "in2");

            view.add_node(node1);
            view.add_node(node2);
            view.add_node(node3);
            view.add_node(node4);
            view.add_edge(edge1);
            view.add_edge(edge2);
            view.add_edge(edge3);
            view.add_edge(edge4);

            view
        });

        Self {
            graph,
            background: BackgroundProps::default(),
        }
    }

    fn update_background(&mut self, cx: &mut Context<Self>) {
        let props = self.background;
        self.graph.update(cx, |view, cx| {
            view.set_background(props, cx);
        });
        cx.notify();
    }

    fn render_variant_button(
        &self,
        variant: BackgroundVariant,
        label: &str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_active = self.background.variant == variant;
        let bg_color = if is_active {
            rgb(0x4a90e2)
        } else {
            rgb(0x444444)
        };

        div()
            .flex()
            .items_center()
            .justify_center()
            .bg(bg_color)
            .text_color(rgb(0xffffff))
            .py(px(4.0))
            .px(px(8.0))
            .rounded_md()
            .cursor_pointer()
            .child(label.to_string())
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _window, cx| {
                    this.background.variant = variant;
                    this.update_background(cx);
                }),
            )
    }

    fn render_slider(
        &self,
        label: &str,
        min: f32,
        max: f32,
        value: f32,
        on_change: impl Fn(&mut Self, f32, &mut Context<Self>) + 'static + Copy,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let steps = 20;
        let current_step = ((value - min) / (max - min) * steps as f32).round() as i32;

        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .child(label.to_string())
                    .child(format!("{:.1}", value)),
            )
            .child(
                div()
                    .flex()
                    .h(px(20.0))
                    .w(px(200.0))
                    .bg(rgb(0x444444))
                    .rounded_md()
                    .overflow_hidden()
                    .cursor_pointer()
                    .children((0..steps).map(|i| {
                        let is_active = i < current_step;
                        let val_at_step = min + ((i + 1) as f32 / steps as f32) * (max - min);
                        div()
                            .flex_grow()
                            .bg(if is_active {
                                rgb(0x4a90e2)
                            } else {
                                rgb(0x444444)
                            })
                            .hover(|s| s.bg(rgb(0x5a90f2)))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _window, cx| {
                                    on_change(this, val_at_step, cx);
                                }),
                            )
                    })),
            )
    }
}

impl Render for BackgroundExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.graph.clone()).child(
            div()
                .absolute()
                .top(px(20.0))
                .left(px(20.0))
                .w(px(220.0))
                .bg(rgba(0x000000aa))
                .text_color(rgb(0xffffff))
                .p(px(10.0))
                .rounded_md()
                .flex()
                .flex_col()
                .gap(px(10.0))
                .child("Background Settings")
                // Variant Buttons
                .child(
                    div()
                        .flex()
                        .gap(px(5.0))
                        .child(self.render_variant_button(BackgroundVariant::Dots, "Dots", cx))
                        .child(self.render_variant_button(BackgroundVariant::Lines, "Lines", cx))
                        .child(self.render_variant_button(BackgroundVariant::Cross, "Cross", cx)),
                )
                // Gap Slider
                .child(self.render_slider(
                    "Gap",
                    10.0,
                    100.0,
                    self.background.gap,
                    |this, val, cx| {
                        this.background.gap = val;
                        this.update_background(cx);
                    },
                    cx,
                ))
                // Size Slider
                .child(self.render_slider(
                    "Size",
                    0.5,
                    5.0,
                    self.background.size,
                    |this, val, cx| {
                        this.background.size = val;
                        this.update_background(cx);
                    },
                    cx,
                )),
        )
    }
}

fn main() {
    gpui::Application::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|cx| BackgroundExample::new(cx))
        })
        .unwrap();
    });
}
