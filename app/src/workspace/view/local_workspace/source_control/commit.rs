use crate::appearance::Appearance;
use crate::editor::EditorView;
use crate::local_git::{HeadState, RepositoryStatus};
use crate::workspace::action::WorkspaceAction;
use warpui::ViewHandle;
use warpui::elements::{
    Container, DispatchEventResult, Element, EventHandler, Flex, ParentElement, Text,
};
use warpui::ui_components::components::{UiComponent, UiComponentStyles};
use warpui::ui_components::text_input::TextInput;

pub(super) fn render(
    status: Option<&RepositoryStatus>,
    commit_input: ViewHandle<EditorView>,
    amend_confirmation: bool,
    undo_confirmation: bool,
    revert_confirmation: bool,
    operation_pending: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let mut panel = Flex::column();
    let can_commit =
        !operation_pending && status.is_some_and(|status| status.staged_path_count() > 0);
    let can_commit_all =
        !operation_pending && status.is_some_and(|status| status.changed_path_count() > 0);
    panel.add_child(
        Container::new(
            TextInput::new(commit_input, UiComponentStyles::default().set_height(34.))
                .build()
                .finish(),
        )
        .with_padding_left(10.)
        .with_padding_right(10.)
        .with_padding_top(8.)
        .with_padding_bottom(6.)
        .finish(),
    );
    let mut commit_actions = Flex::row().with_spacing(6.);
    commit_actions.add_child(commit_button(
        "Commit preparado",
        WorkspaceAction::CommitLocalGit,
        can_commit,
        appearance,
    ));
    commit_actions.add_child(commit_button(
        "Commitar tudo",
        WorkspaceAction::CommitAllLocalGit,
        can_commit_all,
        appearance,
    ));
    let mut history_actions = Flex::row().with_spacing(6.);
    let can_amend = status.is_some_and(|status| {
        status.head_oid.is_some() && !matches!(&status.head, HeadState::Unborn)
    });
    if can_amend && amend_confirmation {
        panel.add_child(
            Container::new(
                Text::new_inline(
                    "O último commit será substituído. Confirme para continuar.",
                    appearance.ui_font_family(),
                    10.,
                )
                .with_color(appearance.theme().ui_error_color().into())
                .finish(),
            )
            .with_padding_left(10.)
            .with_padding_right(10.)
            .with_padding_bottom(6.)
            .finish(),
        );
        history_actions.add_child(commit_button(
            "Confirmar alteração",
            WorkspaceAction::AmendLocalGitCommit,
            can_commit,
            appearance,
        ));
        history_actions.add_child(commit_button(
            "Cancelar",
            WorkspaceAction::CancelLocalGitAmend,
            true,
            appearance,
        ));
    } else if can_amend {
        history_actions.add_child(commit_button(
            "Alterar último",
            WorkspaceAction::RequestLocalGitAmend,
            can_commit,
            appearance,
        ));
    }
    let has_commit = status.is_some_and(|status| status.head_oid.is_some());
    if undo_confirmation && has_commit {
        panel.add_child(
            Container::new(
                Text::new_inline(
                    "O último commit será desfeito. As alterações ficarão preparadas para novo commit.",
                    appearance.ui_font_family(),
                    10.,
                )
                .with_color(theme.ui_error_color().into())
                .finish(),
            )
            .with_padding_left(10.)
            .with_padding_right(10.)
            .with_padding_bottom(6.)
            .finish(),
        );
        history_actions.add_child(commit_button(
            "Confirmar desfazer",
            WorkspaceAction::ConfirmUndoLocalGitCommit,
            !operation_pending,
            appearance,
        ));
        history_actions.add_child(commit_button(
            "Cancelar",
            WorkspaceAction::CancelUndoLocalGitCommit,
            true,
            appearance,
        ));
    } else if has_commit && !undo_confirmation && !amend_confirmation {
        history_actions.add_child(commit_button(
            "Desfazer último",
            WorkspaceAction::RequestUndoLocalGitCommit,
            !operation_pending,
            appearance,
        ));
    }
    if revert_confirmation && has_commit {
        panel.add_child(
            Container::new(
                Text::new_inline(
                    "Será criado um novo commit que reverte o último. Conflitos podem exigir resolução manual.",
                    appearance.ui_font_family(),
                    10.,
                )
                .with_color(theme.ui_error_color().into())
                .finish(),
            )
            .with_padding_left(10.)
            .with_padding_right(10.)
            .with_padding_bottom(6.)
            .finish(),
        );
        history_actions.add_child(commit_button(
            "Confirmar reversão",
            WorkspaceAction::ConfirmRevertLocalGitCommit,
            !operation_pending,
            appearance,
        ));
        history_actions.add_child(commit_button(
            "Cancelar",
            WorkspaceAction::CancelRevertLocalGitCommit,
            true,
            appearance,
        ));
    } else if has_commit && !undo_confirmation && !amend_confirmation {
        history_actions.add_child(commit_button(
            "Reverter último",
            WorkspaceAction::RequestRevertLocalGitCommit,
            !operation_pending,
            appearance,
        ));
    }
    let mut actions = Flex::column().with_spacing(6.);
    actions.add_child(commit_actions.finish());
    actions.add_child(history_actions.finish());
    panel.add_child(
        Container::new(actions.finish())
            .with_padding_left(10.)
            .with_padding_right(10.)
            .with_padding_bottom(10.)
            .finish(),
    );
    Container::new(panel.finish()).finish()
}

fn commit_button(
    label: &'static str,
    action: WorkspaceAction,
    enabled: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let element = Text::new_inline(label, appearance.ui_font_family(), 11.)
        .with_color(if enabled {
            theme.font_color(theme.surface_1()).into()
        } else {
            theme.sub_text_color(theme.surface_1()).into()
        })
        .finish();
    let element = Container::new(element)
        .with_background(theme.surface_2())
        .with_uniform_padding(7.)
        .finish();
    if enabled {
        EventHandler::new(element)
            .on_left_mouse_up(move |ctx, _, _| {
                ctx.dispatch_typed_action(action.clone());
                DispatchEventResult::StopPropagation
            })
            .finish()
    } else {
        element
    }
}
