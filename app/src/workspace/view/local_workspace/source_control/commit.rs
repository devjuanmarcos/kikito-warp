use pathfinder_geometry::vector::vec2f;
use warp_core::ui::Icon;
use warp_core::ui::theme::Fill;
use warpui::elements::{
    ChildAnchor, ConstrainedBox, Container, CornerRadius, CrossAxisAlignment, Element, Expanded,
    Flex, Hoverable, MainAxisAlignment, MainAxisSize, OffsetPositioning, ParentAnchor,
    ParentElement, ParentOffsetBounds, Radius, Stack, Text,
};
use warpui::fonts::{Properties, Weight};
use warpui::platform::Cursor;
use warpui::ui_components::components::{Coords, UiComponent, UiComponentStyles};
use warpui::ui_components::text_input::TextInput;

use super::SourceControl;
use super::changes::small_secondary_button;
use crate::appearance::Appearance;
use crate::local_git::{HeadState, RepositoryStatus};
use crate::workspace::action::WorkspaceAction;
use crate::workspace::view::local_workspace::context_menu::{MenuItem, menu};
use crate::workspace::view::local_workspace::ws_style::{mouse_state, neutral, radius, sized_icon};

/// Two 16px lines plus padding and border, the kit textarea's resting height.
const MESSAGE_MIN_HEIGHT: f32 = 48.;
const BUTTON_HEIGHT: f32 = 24.;
const MENU_TOGGLE_WIDTH: f32 = 28.;
const COMMIT_MENU_WIDTH: f32 = 200.;

/// Message box, split commit button with its menu, history confirmations and the last note.
pub(super) fn render(
    source_control: &SourceControl<'_>,
    status: &RepositoryStatus,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.surface_1();
    let pending = source_control.operation_pending;
    let staged = status.staged_path_count();
    let changed = status.changed_path_count();
    let can_commit = !pending && !source_control.commit_message_empty && changed > 0;
    let commit_action = if staged > 0 {
        WorkspaceAction::CommitLocalGit
    } else {
        WorkspaceAction::CommitAllLocalGit
    };
    let mut body = Flex::column()
        .with_main_axis_size(MainAxisSize::Min)
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_spacing(8.);
    body.add_child(
        ConstrainedBox::new(
            TextInput::new(
                source_control.commit_input.clone(),
                UiComponentStyles::default()
                    .set_border_radius(radius(4.))
                    .set_border_width(1.)
                    .set_border_color(
                        if source_control.commit_focused {
                            theme.accent()
                        } else {
                            theme.foreground().with_opacity(20)
                        }
                        .into(),
                    )
                    .set_background(theme.background().into())
                    .set_padding(Coords {
                        top: 7.,
                        bottom: 7.,
                        left: 8.,
                        right: 8.,
                    }),
            )
            .build()
            .finish(),
        )
        .with_min_height(MESSAGE_MIN_HEIGHT)
        .finish(),
    );

    let label = if staged > 0 || changed == 0 {
        "Commit"
    } else {
        "Commit de tudo"
    };
    let split = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(1.)
        .with_child(
            Expanded::new(
                1.,
                primary_button(
                    Some(Icon::Check),
                    Some(label),
                    can_commit,
                    CornerRadius::with_left(Radius::Pixels(4.)),
                    commit_action,
                    ("git-commit", 0),
                    appearance,
                ),
            )
            .finish(),
        )
        .with_child(
            ConstrainedBox::new(primary_button(
                Some(Icon::ChevronDown),
                None,
                can_commit,
                CornerRadius::with_right(Radius::Pixels(4.)),
                WorkspaceAction::ToggleLocalGitCommitMenu,
                ("git-commit-menu", 0),
                appearance,
            ))
            .with_width(MENU_TOGGLE_WIDTH)
            .finish(),
        )
        .finish();
    let mut split_with_menu = Stack::new().with_child(split);
    if source_control.panel.commit_menu_open && can_commit {
        split_with_menu.add_positioned_overlay_child(
            menu(
                commit_menu_items(status, staged > 0),
                None,
                COMMIT_MENU_WIDTH,
                WorkspaceAction::ToggleLocalGitCommitMenu,
                appearance,
            ),
            OffsetPositioning::offset_from_parent(
                vec2f(0., 4.),
                ParentOffsetBounds::WindowByPosition,
                ParentAnchor::BottomRight,
                ChildAnchor::TopRight,
            ),
        );
    }
    body.add_child(split_with_menu.finish());

    for (shown, text, confirm, cancel) in [
        (
            source_control.amend_confirmation,
            "O último commit será substituído pela mensagem e pelas alterações preparadas.",
            WorkspaceAction::AmendLocalGitCommit,
            WorkspaceAction::CancelLocalGitAmend,
        ),
        (
            source_control.undo_confirmation,
            "O último commit será desfeito. As alterações ficarão preparadas para novo commit.",
            WorkspaceAction::ConfirmUndoLocalGitCommit,
            WorkspaceAction::CancelUndoLocalGitCommit,
        ),
        (
            source_control.revert_confirmation,
            "Será criado um novo commit que reverte o último. Conflitos podem exigir resolução manual.",
            WorkspaceAction::ConfirmRevertLocalGitCommit,
            WorkspaceAction::CancelRevertLocalGitCommit,
        ),
    ] {
        if !shown {
            continue;
        }
        let mut actions = Flex::row().with_spacing(6.);
        if !pending {
            actions.add_child(small_secondary_button(
                Icon::Check,
                "Confirmar",
                confirm,
                ("git-history-confirm", 0),
                appearance,
            ));
        }
        actions.add_child(small_secondary_button(
            Icon::X,
            "Cancelar",
            cancel,
            ("git-history-cancel", 0),
            appearance,
        ));
        body.add_child(
            Flex::column()
                .with_spacing(6.)
                .with_child(
                    Text::new(text, appearance.ui_font_family(), 11.)
                        .with_color(theme.ui_error_color())
                        .finish(),
                )
                .with_child(actions.finish())
                .finish(),
        );
    }

    if let Some(note) = &source_control.panel.note {
        let green = Fill::Solid(theme.ui_green_color());
        body.add_child(
            Flex::row()
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_spacing(6.)
                .with_child(sized_icon(Icon::Check, green, 12.))
                .with_child(
                    Text::new_inline(note.clone(), appearance.ui_font_family(), 11.)
                        .with_color(green.into())
                        .finish(),
                )
                .finish(),
        );
    }
    if pending {
        body.add_child(
            Text::new_inline(
                "Aplicando operação do Git…",
                appearance.ui_font_family(),
                11.,
            )
            .with_color(theme.hint_text_color(background).into())
            .finish(),
        );
    }
    Container::new(body.finish())
        .with_padding_left(10.)
        .with_padding_right(10.)
        .with_padding_top(2.)
        .with_padding_bottom(10.)
        .finish()
}

/// The kit's commit menu, followed by the native history actions the kit has no room for.
fn commit_menu_items(status: &RepositoryStatus, has_staged: bool) -> Vec<MenuItem> {
    let mut items = vec![
        MenuItem {
            label: "Commit",
            icon: Icon::Check,
            action: if has_staged {
                WorkspaceAction::CommitLocalGit
            } else {
                WorkspaceAction::CommitAllLocalGit
            },
            danger: false,
            separator_before: false,
        },
        MenuItem {
            label: "Commit e push",
            icon: Icon::Upload,
            action: WorkspaceAction::CommitAndPushLocalGit,
            danger: false,
            separator_before: false,
        },
        MenuItem {
            label: "Preparar tudo",
            icon: Icon::Plus,
            action: WorkspaceAction::StageAllLocalGit,
            danger: false,
            separator_before: true,
        },
        MenuItem {
            label: "Remover tudo da preparação",
            icon: Icon::Minus,
            action: WorkspaceAction::UnstageAllLocalGit,
            danger: false,
            separator_before: false,
        },
    ];
    let has_commit = status.head_oid.is_some() && !matches!(status.head, HeadState::Unborn);
    if has_commit {
        items.extend([
            MenuItem {
                label: "Alterar último commit",
                icon: Icon::Pencil,
                action: WorkspaceAction::RequestLocalGitAmend,
                danger: false,
                separator_before: true,
            },
            MenuItem {
                label: "Desfazer último commit",
                icon: Icon::Retry,
                action: WorkspaceAction::RequestUndoLocalGitCommit,
                danger: false,
                separator_before: false,
            },
            MenuItem {
                label: "Reverter último commit",
                icon: Icon::Retry,
                action: WorkspaceAction::RequestRevertLocalGitCommit,
                danger: true,
                separator_before: false,
            },
        ]);
    }
    items
}

/// Kit small primary `Button`; disabled it drops the fill and dims the label.
fn primary_button(
    icon: Option<Icon>,
    label: Option<&'static str>,
    enabled: bool,
    corners: CornerRadius,
    action: WorkspaceAction,
    key: (&'static str, i64),
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let state = mouse_state(key.0, key.1);
    let hoverable = Hoverable::new(state, move |mouse| {
        let color = if enabled {
            theme.font_color(theme.accent().into_solid())
        } else {
            neutral(theme, 40)
        };
        let mut row = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_main_axis_alignment(MainAxisAlignment::Center)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(2.);
        if let Some(icon) = icon {
            row.add_child(sized_icon(icon, color, 14.));
        }
        if let Some(label) = label {
            row.add_child(
                Text::new_inline(label, appearance.ui_font_family(), 12.)
                    .with_style(Properties::default().weight(Weight::Semibold))
                    .with_color(color.into())
                    .finish(),
            );
        }
        let mut button = Container::new(
            ConstrainedBox::new(row.finish())
                .with_height(BUTTON_HEIGHT)
                .finish(),
        )
        .with_corner_radius(corners);
        if enabled {
            button = button.with_background(if mouse.is_clicked() {
                theme.accent().with_opacity(40)
            } else if mouse.is_mouse_over_element() {
                theme.accent().with_opacity(60)
            } else {
                theme.accent()
            });
        }
        button.finish()
    })
    .with_cursor(if enabled {
        Cursor::PointingHand
    } else {
        Cursor::Arrow
    });
    if enabled {
        hoverable
            .on_click(move |ctx, _, _| ctx.dispatch_typed_action(action.clone()))
            .finish()
    } else {
        hoverable.finish()
    }
}
