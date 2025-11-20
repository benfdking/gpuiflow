use gpui::*;

pub fn render_controls(
    on_zoom_in: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    on_zoom_out: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    on_fit_view: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    on_lock: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    is_locked: bool,
) -> impl IntoElement {
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
            control_button("+", on_zoom_in)
                .border_b_1()
                .border_color(rgb(0xeeeeee))
        )
        .child(
            control_button("-", on_zoom_out)
                .border_b_1()
                .border_color(rgb(0xeeeeee))
        )
        .child(
            control_button("[]", on_fit_view)
                .border_b_1()
                .border_color(rgb(0xeeeeee))
        )
        .child(
            control_button(if is_locked { "L" } else { "U" }, on_lock)
        )
}

fn control_button(
    text: &str,
    on_click: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
) -> Div {
    div()
        .w(px(24.0))
        .h(px(24.0))
        .flex()
        .items_center()
        .justify_center()
        .hover(|s| s.bg(rgb(0xf0f0f0)))
        .cursor_pointer()
        .on_mouse_down(MouseButton::Left, on_click)
        .child(text)
        .text_color(rgb(0x000000))
        .text_sm()
}
