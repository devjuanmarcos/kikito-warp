use crate::appearance::Appearance;
use crate::local_git::{DiffLine, DiffLineKind};
use crate::workspace::action::WorkspaceAction;
use pathfinder_color::ColorU;
use warp_core::ui::theme::Fill;
use warpui::elements::{
    Border, ConstrainedBox, Container, CrossAxisAlignment, Element, Expanded, Flex, Hoverable,
    MainAxisSize, ParentElement, Shrinkable, Text,
};
use warpui::platform::Cursor;
use warpui::text_layout::ClipConfig;

use super::super::ws_style::{fg_overlay, mouse_state, radius};

use super::DiffPreview;

pub(super) fn render(
    preview: &DiffPreview,
    operation_pending: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    // Lives inside a vertical scrollable, whose unbounded height a max-sized column can't fill.
    let mut panel = Flex::column()
        .with_main_axis_size(MainAxisSize::Min)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch);
    if let Some(path) = &preview.path {
        panel.add_child(label(path, appearance));
    }
    if preview.loading {
        panel.add_child(label("Carregando comparação…", appearance));
    } else if let Some(error) = &preview.error {
        panel.add_child(
            Text::new_inline(error.clone(), appearance.ui_font_family(), 10.)
                .with_color(theme.ui_error_color())
                .finish(),
        );
    } else if preview.binary {
        panel.add_child(label(
            "O arquivo é binário e não possui diff textual.",
            appearance,
        ));
    } else if let Some(lines) = &preview.lines {
        for (index, hunk) in preview.hunks.iter().enumerate() {
            let header = hunk
                .lines()
                .find(|line| line.starts_with("@@ "))
                .unwrap_or("Bloco de alterações");
            if let Some(path) = preview.path.as_deref() {
                panel.add_child(hunk_action(
                    header,
                    index,
                    path,
                    hunk,
                    preview.staged,
                    preview.untracked,
                    operation_pending,
                    appearance,
                ));
            }
        }
        if lines.is_empty() {
            panel.add_child(label("Nenhuma diferença textual.", appearance));
        }
        for line in lines {
            panel.add_child(diff_row(line, appearance));
        }
    }
    Container::new(panel.finish())
        .with_background(theme.surface_1())
        .with_padding_left(8.)
        .with_padding_right(8.)
        .with_padding_top(6.)
        .with_padding_bottom(8.)
        .finish()
}

#[allow(clippy::too_many_arguments)]
fn hunk_action(
    header: &str,
    index: usize,
    path: &str,
    patch: &str,
    staged: bool,
    untracked: bool,
    operation_pending: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let mut row = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(6.);
    row.add_child(Shrinkable::new(1., label(header, appearance)).finish());
    let text = if staged {
        "Retirar bloco"
    } else {
        "Preparar bloco"
    };
    let action = WorkspaceAction::ApplyLocalGitHunk {
        path: path.to_owned(),
        patch: patch.to_owned(),
        staged,
        untracked,
    };
    let enabled = !operation_pending;
    let button = Hoverable::new(mouse_state("git-hunk", index as i64), move |mouse| {
        let mut button = Container::new(
            Text::new_inline(text, appearance.ui_font_family(), 11.)
                .with_color(if enabled {
                    theme.font_color(theme.surface_1()).into()
                } else {
                    theme.disabled_text_color(theme.surface_1()).into()
                })
                .finish(),
        )
        .with_padding_left(6.)
        .with_padding_right(6.)
        .with_padding_top(2.)
        .with_padding_bottom(2.)
        .with_border(Border::all(1.).with_border_fill(theme.outline()))
        .with_corner_radius(radius(4.));
        if enabled && mouse.is_mouse_over_element() {
            button = button.with_background(fg_overlay(theme, 10));
        }
        button.finish()
    });
    let button = if enabled {
        button
            .with_cursor(Cursor::PointingHand)
            .on_click(move |ctx, _, _| ctx.dispatch_typed_action(action.clone()))
            .finish()
    } else {
        button.finish()
    };
    row.add_child(button);
    Container::new(row.finish())
        .with_background(theme.surface_1())
        .with_padding_top(if index == 0 { 4. } else { 6. })
        .with_padding_bottom(4.)
        .finish()
}

fn label(value: &str, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    Text::new_inline(value.to_owned(), appearance.ui_font_family(), 11.)
        .with_color(theme.sub_text_color(theme.surface_1()).into())
        .soft_wrap(false)
        .with_clip(ClipConfig::ellipsis())
        .finish()
}

fn diff_row(line: &DiffLine, appearance: &Appearance) -> Box<dyn Element> {
    let old = diff_cell(
        line.old_line,
        line.old_text.as_deref(),
        line.kind,
        true,
        appearance,
    );
    let new = diff_cell(
        line.new_line,
        line.new_text.as_deref(),
        line.kind,
        false,
        appearance,
    );
    Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_spacing(1.)
        .with_child(Expanded::new(1., old).finish())
        .with_child(Expanded::new(1., new).finish())
        .finish()
}

/// One side of a side-by-side diff row: a fixed line-number gutter and the line's text, tinted
/// red (removed) or green (added) when the line changed.
fn diff_cell(
    line_number: Option<usize>,
    text: Option<&str>,
    kind: DiffLineKind,
    is_old: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let changed = kind == DiffLineKind::Changed && text.is_some();
    let tint: Option<Fill> = changed.then(|| {
        let base: ColorU = if is_old {
            theme.terminal_colors().normal.red.into()
        } else {
            theme.terminal_colors().normal.green.into()
        };
        Fill::Solid(ColorU::new(base.r, base.g, base.b, 38))
    });
    let text_color: ColorU = if changed {
        theme.font_color(theme.surface_1()).into()
    } else {
        theme.sub_text_color(theme.surface_1()).into()
    };
    let gutter = ConstrainedBox::new(
        Text::new_inline(
            line_number
                .map(|number| number.to_string())
                .unwrap_or_default(),
            appearance.monospace_font_family(),
            11.,
        )
        .with_color(theme.hint_text_color(theme.surface_1()).into())
        .soft_wrap(false)
        .finish(),
    )
    .with_width(LINE_NUMBER_WIDTH)
    .finish();
    let row = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(4.)
        .with_child(gutter)
        .with_child(
            Expanded::new(
                1.,
                Text::new_inline(
                    text.unwrap_or_default().to_owned(),
                    appearance.monospace_font_family(),
                    11.,
                )
                .with_color(text_color)
                .soft_wrap(false)
                .with_clip(ClipConfig::ellipsis())
                .finish(),
            )
            .finish(),
        )
        .finish();
    let mut cell = Container::new(row)
        .with_padding_left(4.)
        .with_padding_right(4.)
        .with_padding_top(1.)
        .with_padding_bottom(1.);
    if let Some(tint) = tint {
        cell = cell.with_background(tint);
    }
    cell.finish()
}

const LINE_NUMBER_WIDTH: f32 = 22.;
