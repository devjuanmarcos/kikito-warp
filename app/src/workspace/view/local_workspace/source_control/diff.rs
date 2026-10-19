use crate::appearance::Appearance;
use crate::local_git::{DiffLine, DiffLineKind};
use crate::workspace::action::WorkspaceAction;
use warpui::elements::{
    Container, CrossAxisAlignment, DispatchEventResult, Element, EventHandler, Flex, MainAxisSize,
    ParentElement, Shrinkable, Text,
};

use super::DiffPreview;

pub(super) fn render(
    preview: &DiffPreview,
    operation_pending: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let mut panel = Flex::column()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch);
    if let Some(path) = &preview.path {
        panel.add_child(label(path, appearance));
    }
    if preview.loading {
        panel.add_child(label("Carregando comparação…", appearance));
    } else if let Some(error) = &preview.error {
        panel.add_child(
            Text::new_inline(error.clone(), appearance.ui_font_family(), 10.)
                .with_color(theme.ui_error_color().into())
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
        .with_padding_top(8.)
        .finish()
}

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
    let button = Container::new(
        Text::new_inline(text, appearance.ui_font_family(), 9.)
            .with_color(theme.font_color(theme.surface_1()).into())
            .finish(),
    )
    .with_background(theme.surface_2())
    .with_uniform_padding(4.)
    .finish();
    if !operation_pending {
        row.add_child(
            EventHandler::new(button)
                .on_left_mouse_up(move |ctx, _, _| {
                    ctx.dispatch_typed_action(action.clone());
                    DispatchEventResult::StopPropagation
                })
                .finish(),
        );
    } else {
        row.add_child(button);
    }
    Container::new(row.finish())
        .with_background(theme.surface_1())
        .with_padding_top(if index == 0 { 5. } else { 2. })
        .with_padding_bottom(2.)
        .finish()
}

fn label(value: &str, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    Text::new_inline(value.to_owned(), appearance.ui_font_family(), 10.)
        .with_color(theme.sub_text_color(theme.surface_1()).into())
        .finish()
}

fn diff_row(line: &DiffLine, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
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
    let mut row = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_spacing(2.);
    row.add_child(Shrinkable::new(1., old).finish());
    row.add_child(Shrinkable::new(1., new).finish());
    Container::new(row.finish())
        .with_background(theme.surface_1())
        .finish()
}

fn diff_cell(
    line_number: Option<usize>,
    text: Option<&str>,
    kind: DiffLineKind,
    is_old: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let content = match (line_number, text) {
        (Some(number), Some(text)) => format!("{number:>4} {text}"),
        _ => String::new(),
    };
    let color: pathfinder_color::ColorU = match (kind, is_old) {
        (DiffLineKind::Changed, true) => theme.ui_error_color().into(),
        (DiffLineKind::Changed, false) => theme.font_color(theme.surface_1()).into(),
        (DiffLineKind::Context, _) => theme.sub_text_color(theme.surface_1()).into(),
    };
    Container::new(
        Text::new_inline(content, appearance.ui_font_family(), 9.)
            .with_color(color.into())
            .finish(),
    )
    .with_background(if kind == DiffLineKind::Changed {
        theme.surface_2()
    } else {
        theme.surface_1()
    })
    .with_padding_left(2.)
    .with_padding_right(2.)
    .finish()
}
