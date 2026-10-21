use warp_core::ui::Icon;
use warp_core::ui::theme::Fill;
use warpui::elements::Point;
use warpui::elements::{
    Border, ConstrainedBox, Container, CrossAxisAlignment, Element, Flex, Hoverable,
    MainAxisAlignment, MainAxisSize, ParentElement, Text,
};
use warpui::event::DispatchedEvent;
use warpui::fonts::{Properties, Weight};
use warpui::geometry::vector::Vector2F;
use warpui::platform::Cursor;
use warpui::{
    AfterLayoutContext, AppContext, EventContext, LayoutContext, PaintContext, SizeConstraint,
};

use super::ws_style::{mouse_state, neutral, radius, sized_icon};
use crate::appearance::Appearance;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ButtonVariant {
    Secondary,
    Primary,
    Error,
}

/// Key legend pill: `size`px text on a rounded cap with `padding` on every side, inside the kit's
/// transparent 1px border. Buttons render their legend in their own semibold weight.
pub(super) fn keycap(
    appearance: &Appearance,
    label: &str,
    size: f32,
    padding: f32,
    weight: Weight,
    color: Fill,
    background: Fill,
) -> Box<dyn Element> {
    let min = size + 2. * (padding + 1.);
    // The UI font has no return-key glyph; the kit's browser fallback draws a corner-down-left arrow.
    let legend = if label == "⏎" {
        sized_icon(Icon::CornerDownLeft, color, size)
    } else {
        Text::new_inline(label.to_owned(), appearance.ui_font_family(), size)
            .with_style(Properties::default().weight(weight))
            .with_color(color.into())
            .finish()
    };
    ConstrainedBox::new(
        Container::new(
            Flex::row()
                .with_main_axis_size(MainAxisSize::Min)
                .with_main_axis_alignment(MainAxisAlignment::Center)
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_child(legend)
                .finish(),
        )
        .with_uniform_padding(padding + 1.)
        .with_background(background)
        .with_corner_radius(radius(3.))
        .finish(),
    )
    .with_min_width(min)
    .with_min_height(min)
    .finish()
}

/// Close affordance of dialogs: an X glyph followed by the ESC legend. The legend extends past
/// the dialog's right padding, as in the design kit.
pub(super) fn dialog_close(
    appearance: &Appearance,
    key: (&'static str, i64),
    on_click: impl Fn(&mut warpui::EventContext) + 'static,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let content = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(4.)
        .with_child(sized_icon(Icon::X, theme.foreground(), 16.))
        .with_child(keycap(
            appearance,
            "ESC",
            12.,
            2.,
            Weight::Semibold,
            theme.foreground(),
            neutral(theme, 15),
        ))
        .finish();
    Hoverable::new(mouse_state(key.0, key.1), |_| {
        Container::new(content).with_margin_right(-7.).finish()
    })
    .with_cursor(Cursor::PointingHand)
    .on_click(move |ctx, _, _| on_click(ctx))
    .finish()
}

/// 32px kit button with an optional leading 16px icon and trailing keycaps. A disabled primary
/// button drops its fill and ignores clicks.
#[allow(clippy::too_many_arguments)]
pub(super) fn dialog_button(
    appearance: &Appearance,
    icon: Option<Icon>,
    label: &str,
    keys: &[&str],
    variant: ButtonVariant,
    enabled: bool,
    key: (&'static str, i64),
    on_click: impl Fn(&mut warpui::EventContext, Vector2F) + 'static,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let state = mouse_state(key.0, key.1);
    let (over, pressed) = {
        let state = state.lock().unwrap();
        (state.is_mouse_over_element(), state.is_clicked())
    };
    let disabled = !enabled;
    let text_color: Fill = if disabled {
        neutral(theme, 40)
    } else {
        match variant {
            ButtonVariant::Secondary => theme.foreground(),
            ButtonVariant::Primary => theme.font_color(theme.accent().into_solid()),
            ButtonVariant::Error => Fill::Solid(pathfinder_color::ColorU::white()),
        }
    };
    let background: Option<Fill> = if disabled {
        None
    } else {
        match variant {
            ButtonVariant::Secondary => {
                if pressed && over {
                    Some(theme.foreground().with_opacity(15))
                } else if over {
                    Some(theme.foreground().with_opacity(10))
                } else {
                    None
                }
            }
            ButtonVariant::Primary => Some(if pressed && over {
                theme.accent().with_opacity(40)
            } else if over {
                theme.accent().with_opacity(60)
            } else {
                theme.accent()
            }),
            ButtonVariant::Error => {
                let base = theme.ui_error_color();
                Some(Fill::Solid(pathfinder_color::ColorU::new(
                    base.r,
                    base.g,
                    base.b,
                    if over { 204 } else { 255 },
                )))
            }
        }
    };
    let mut row = Flex::row()
        .with_main_axis_size(MainAxisSize::Min)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(4.);
    if let Some(icon) = icon {
        row.add_child(sized_icon(icon, text_color, 16.));
    }
    row.add_child(
        Text::new_inline(label.to_owned(), appearance.ui_font_family(), 14.)
            .with_style(Properties::default().weight(Weight::Semibold))
            .with_color(text_color.into())
            .finish(),
    );
    if !keys.is_empty() {
        let (cap_color, cap_background) = match (variant, disabled) {
            (ButtonVariant::Secondary, _) | (_, true) => (text_color, neutral(theme, 15)),
            _ => (
                text_color,
                Fill::Solid(pathfinder_color::ColorU::transparent_black()),
            ),
        };
        let mut caps = Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(4.);
        for cap in keys {
            caps.add_child(keycap(
                appearance,
                cap,
                12.,
                2.,
                Weight::Semibold,
                cap_color,
                cap_background,
            ));
        }
        row.add_child(Container::new(caps.finish()).with_margin_left(2.).finish());
    }
    let mut body = Container::new(
        ConstrainedBox::new(
            Flex::row()
                .with_main_axis_size(MainAxisSize::Min)
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_child(row.finish())
                .finish(),
        )
        .with_height(if variant == ButtonVariant::Secondary {
            30.
        } else {
            32.
        })
        .finish(),
    )
    .with_padding_left(12.)
    .with_padding_right(12.)
    .with_corner_radius(radius(4.));
    if variant == ButtonVariant::Secondary && !disabled {
        body = body.with_border(Border::all(1.).with_border_fill(neutral(theme, 20)));
    }
    if let Some(background) = background {
        body = body.with_background(background);
    }
    let element = body.finish();
    let mut hoverable = Hoverable::new(state, |_| element).with_cursor(if disabled {
        Cursor::Arrow
    } else {
        Cursor::PointingHand
    });
    if enabled {
        hoverable = hoverable.on_click(move |ctx, _, position| on_click(ctx, position));
    }
    hoverable.finish()
}

/// Colour swatch: 16px dot with a 2px ring that is transparent unless selected.
pub(super) fn color_dot(
    fill: Fill,
    selected: bool,
    ring: Fill,
    key: (&'static str, i64),
    on_click: impl Fn(&mut warpui::EventContext) + 'static,
) -> Box<dyn Element> {
    let dot = ConstrainedBox::new(
        Container::new(Flex::row().finish())
            .with_background(fill)
            .with_corner_radius(radius(8.))
            .finish(),
    )
    .with_width(16.)
    .with_height(16.)
    .finish();
    let ringed = Container::new(dot)
        .with_corner_radius(radius(10.))
        .with_border(if selected {
            Border::all(2.).with_border_fill(ring)
        } else {
            Border::all(2.)
        });
    Hoverable::new(mouse_state(key.0, key.1), |_| ringed.finish())
        .with_cursor(Cursor::PointingHand)
        .on_click(move |ctx, _, _| on_click(ctx))
        .finish()
}

/// 24px outlined button around caller-provided content.
pub(super) fn small_secondary(
    theme: &warp_core::ui::theme::WarpTheme,
    key: (&'static str, i64),
    on_click: impl Fn(&mut warpui::EventContext) + 'static,
    content: Box<dyn Element>,
) -> Box<dyn Element> {
    small_button(theme, true, key, on_click, content)
}

/// 24px borderless button around caller-provided content.
pub(super) fn small_naked(
    theme: &warp_core::ui::theme::WarpTheme,
    key: (&'static str, i64),
    on_click: impl Fn(&mut warpui::EventContext) + 'static,
    content: Box<dyn Element>,
) -> Box<dyn Element> {
    small_button(theme, false, key, on_click, content)
}

fn small_button(
    theme: &warp_core::ui::theme::WarpTheme,
    outlined: bool,
    key: (&'static str, i64),
    on_click: impl Fn(&mut warpui::EventContext) + 'static,
    content: Box<dyn Element>,
) -> Box<dyn Element> {
    let state = mouse_state(key.0, key.1);
    Hoverable::new(state, |mouse| {
        let mut button = Container::new(ConstrainedBox::new(content).with_height(22.).finish())
            .with_border(if outlined {
                Border::all(1.).with_border_fill(neutral(theme, 20))
            } else {
                Border::all(1.)
            })
            .with_corner_radius(radius(4.));
        if mouse.is_mouse_over_element() {
            button =
                button.with_background(theme.foreground().with_opacity(if mouse.is_clicked() {
                    15
                } else {
                    10
                }));
        }
        button.finish()
    })
    .with_cursor(Cursor::PointingHand)
    .on_click(move |ctx, _, _| on_click(ctx))
    .finish()
}

/// Paints its child at the nearest whole device pixel. Centering an odd-sized dialog in an
/// even-sized window lands on a half pixel, which blurs every border while text stays snapped.
pub(super) struct PixelAligned {
    child: Box<dyn Element>,
}

impl PixelAligned {
    pub(super) fn new(child: Box<dyn Element>) -> Self {
        Self { child }
    }
}

impl Element for PixelAligned {
    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
        app: &AppContext,
    ) -> Vector2F {
        self.child.layout(constraint, ctx, app)
    }

    fn after_layout(&mut self, ctx: &mut AfterLayoutContext, app: &AppContext) {
        self.child.after_layout(ctx, app);
    }

    fn paint(&mut self, origin: Vector2F, ctx: &mut PaintContext, app: &AppContext) {
        let scale = ctx.scene.scale_factor();
        let snapped = Vector2F::new(
            (origin.x() * scale).round() / scale,
            (origin.y() * scale).round() / scale,
        );
        self.child.paint(snapped, ctx, app);
    }

    fn dispatch_event(
        &mut self,
        event: &DispatchedEvent,
        ctx: &mut EventContext,
        app: &AppContext,
    ) -> bool {
        self.child.dispatch_event(event, ctx, app)
    }

    fn size(&self) -> Option<Vector2F> {
        self.child.size()
    }

    fn origin(&self) -> Option<Point> {
        self.child.origin()
    }
}
