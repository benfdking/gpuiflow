use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundVariant {
    Lines,
    Dots,
    Cross,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BackgroundProps {
    pub variant: BackgroundVariant,
    pub gap: f32,
    pub color: Hsla,
    pub size: f32,
}

impl Default for BackgroundProps {
    fn default() -> Self {
        Self {
            variant: BackgroundVariant::Dots,
            gap: 20.0,
            color: rgb(0x666666).into(),
            size: 1.0,
        }
    }
}

pub fn render_background(props: BackgroundProps) -> impl IntoElement {
    div()
        .absolute()
        .left(px(0.0))
        .top(px(0.0))
        .size_full()
        .child(
            canvas(
                |_, _, _| {},
                move |bounds, _, window, _cx| match props.variant {
                    BackgroundVariant::Lines => {
                        render_lines(&bounds, &props, window);
                    }
                    BackgroundVariant::Dots => {
                        render_dots(&bounds, &props, window);
                    }
                    BackgroundVariant::Cross => {
                        render_cross(&bounds, &props, window);
                    }
                },
            )
            .size_full(),
        )
}

fn render_lines(bounds: &Bounds<Pixels>, props: &BackgroundProps, window: &mut Window) {
    let gap = props.gap;
    let width = f32::from(bounds.size.width);
    let height = f32::from(bounds.size.height);

    let mut builder = PathBuilder::stroke(px(props.size));

    // Vertical lines
    let mut x = 0.0;
    while x <= width {
        let screen_x = bounds.origin.x + px(x);
        builder.move_to(point(screen_x, bounds.origin.y));
        builder.line_to(point(screen_x, bounds.origin.y + bounds.size.height));
        x += gap;
    }

    // Horizontal lines
    let mut y = 0.0;
    while y <= height {
        let screen_y = bounds.origin.y + px(y);
        builder.move_to(point(bounds.origin.x, screen_y));
        builder.line_to(point(bounds.origin.x + bounds.size.width, screen_y));
        y += gap;
    }

    if let Ok(path) = builder.build() {
        window.paint_path(path, props.color);
    }
}

fn render_dots(bounds: &Bounds<Pixels>, props: &BackgroundProps, window: &mut Window) {
    let gap = props.gap;
    let width = f32::from(bounds.size.width);
    let height = f32::from(bounds.size.height);
    let size = props.size.max(1.0);

    let mut builder = PathBuilder::fill();

    let mut y = 0.0;
    while y <= height {
        let mut x = 0.0;
        while x <= width {
            let center = bounds.origin + point(px(x), px(y));

            // Draw each dot as a small square for better performance
            // Draw each dot as a circle using polygon approximation
            let steps = 12;
            for i in 0..=steps {
                let theta = (i as f32 / steps as f32) * 2.0 * std::f32::consts::PI;
                let px_x = center.x + px(size * theta.cos());
                let px_y = center.y + px(size * theta.sin());
                let p = point(px_x, px_y);
                if i == 0 {
                    builder.move_to(p);
                } else {
                    builder.line_to(p);
                }
            }

            x += gap;
        }
        y += gap;
    }

    if let Ok(path) = builder.build() {
        window.paint_path(path, props.color);
    }
}

fn render_cross(bounds: &Bounds<Pixels>, props: &BackgroundProps, window: &mut Window) {
    let gap = props.gap;
    let width = f32::from(bounds.size.width);
    let height = f32::from(bounds.size.height);
    let size = props.size * 3.0;

    let mut builder = PathBuilder::stroke(px(props.size));

    let mut y = 0.0;
    while y <= height {
        let mut x = 0.0;
        while x <= width {
            let center = bounds.origin + point(px(x), px(y));

            // Draw horizontal line of the cross
            builder.move_to(point(center.x - px(size), center.y));
            builder.line_to(point(center.x + px(size), center.y));

            // Draw vertical line of the cross
            builder.move_to(point(center.x, center.y - px(size)));
            builder.line_to(point(center.x, center.y + px(size)));

            x += gap;
        }
        y += gap;
    }

    if let Ok(path) = builder.build() {
        window.paint_path(path, props.color);
    }
}
