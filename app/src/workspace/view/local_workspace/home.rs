use std::collections::HashMap;

use chrono::{Local, Timelike, Utc};
use warp_core::ui::Icon;
use warp_core::ui::theme::Fill as ThemeFill;
use warpui::ViewHandle;
use warpui::assets::asset_cache::AssetSource;
use warpui::elements::new_scrollable::{NewScrollable, ScrollableAppearance, SingleAxisConfig};
use warpui::elements::{
    Border, CacheOption, ClippedScrollStateHandle, ConstrainedBox, Container, CrossAxisAlignment,
    DispatchEventResult, Element, Empty, EventHandler, Expanded, Fill, Flex, Hoverable, Image,
    MainAxisAlignment, MainAxisSize, ParentElement, ScrollbarWidth, Shrinkable, Text,
};
use warpui::fonts::{Properties, Weight};
use warpui::platform::Cursor;
use warpui::text_layout::ClipConfig;
use warpui::ui_components::components::{Coords, UiComponent, UiComponentStyles};
use warpui::ui_components::text_input::TextInput;

use super::ws_style::{fg_overlay, mouse_state, neutral, radius, sized_icon};
use crate::appearance::Appearance;
use crate::editor::EditorView;
use crate::local_workspace::model::{
    LocalProject, ProjectId, ProjectPathStatus, TabId, WorkspaceModel,
};
use crate::workspace::action::WorkspaceAction;

const CONTENT_MAX_WIDTH: f32 = 960.;
/// Line box of the 14px text inside the 44px Home field.
const ASK_TEXT_HEIGHT: f32 = 17.;
const CARD_COLUMNS: usize = 4;
const RECENT_PROJECTS: usize = 6;
const RECENT_TABS: usize = 7;
const ASK_RESULTS: usize = 6;
/// Verbs the kit strips from the "O que vamos fazer?" query before matching.
const ASK_VERBS: [&str; 5] = ["abrir ", "abre ", "abra ", "ir para ", "vai para "];

/// State of the Home "O que vamos fazer?" field.
#[derive(Clone)]
pub(crate) struct HomeAsk<'a> {
    pub input: ViewHandle<EditorView>,
    pub query: &'a str,
    pub selection: usize,
    pub focused: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AskTarget {
    Project(ProjectId),
    Tab(TabId),
}

impl AskTarget {
    pub(crate) fn action(self) -> WorkspaceAction {
        match self {
            AskTarget::Project(id) => WorkspaceAction::SelectLocalWorkspaceProject(id),
            AskTarget::Tab(id) => WorkspaceAction::SelectLocalWorkspaceTab(id),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AskResult {
    pub label: String,
    pub sub: String,
    pub target: AskTarget,
}

/// Projects and project tabs whose label or location contains the query, projects first.
pub(crate) fn ask_results(
    model: &WorkspaceModel,
    query: &str,
    workspace_tab_titles: &HashMap<String, String>,
) -> Vec<AskResult> {
    let mut needle = query.trim().to_lowercase();
    if let Some(rest) = ASK_VERBS.iter().find_map(|verb| needle.strip_prefix(verb)) {
        needle = rest.trim().to_owned();
    }
    if needle.is_empty() {
        return Vec::new();
    }
    let mut projects = Vec::new();
    let mut tabs = Vec::new();
    for organization in model.organizations() {
        for project in model.projects_for_organization(organization.id) {
            projects.push(AskResult {
                label: project.name.clone(),
                sub: organization.name.clone(),
                target: AskTarget::Project(project.id),
            });
            for tab_id in &project.tabs {
                let Some(title) = model
                    .workspace_tab_id(*tab_id)
                    .and_then(|identity| workspace_tab_titles.get(identity))
                else {
                    continue;
                };
                tabs.push(AskResult {
                    label: title.clone(),
                    sub: format!("{} · {}", project.name, organization.name),
                    target: AskTarget::Tab(*tab_id),
                });
            }
        }
    }
    projects
        .into_iter()
        .chain(tabs)
        .filter(|result| {
            format!("{} {}", result.label, result.sub)
                .to_lowercase()
                .contains(&needle)
        })
        .take(ASK_RESULTS)
        .collect()
}

/// Placeholder of the Home field, using the most recent project as the example.
pub(crate) fn ask_placeholder(model: &WorkspaceModel) -> String {
    match model.recent_projects().next() {
        Some(project) => format!("O que vamos fazer? Ex.: abrir {}", project.name),
        None => "O que vamos fazer?".to_owned(),
    }
}

/// Shows `path` relative to the user's home folder when it lives inside it, as the kit does.
fn display_path(path: &str) -> String {
    let Some(home) = dirs::home_dir() else {
        return path.to_owned();
    };
    match std::path::Path::new(path).strip_prefix(&home) {
        Ok(rest) if rest.as_os_str().is_empty() => "~".to_owned(),
        Ok(rest) => format!("~{}{}", std::path::MAIN_SEPARATOR, rest.display()),
        Err(_) => path.to_owned(),
    }
}

fn dot(color: ThemeFill, size: f32) -> Box<dyn Element> {
    ConstrainedBox::new(
        Container::new(Flex::row().finish())
            .with_background(color)
            .with_corner_radius(radius(size / 2.))
            .finish(),
    )
    .with_width(size)
    .with_height(size)
    .finish()
}

fn ask_field(ask: &HomeAsk, results: &[AskResult], appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.background();
    let mut field = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(10.);
    field.add_child(sized_icon(
        Icon::AgentMode,
        if ask.focused {
            theme.accent()
        } else {
            theme.sub_text_color(background)
        },
        16.,
    ));
    field.add_child(
        Expanded::new(
            1.,
            TextInput::new(
                ask.input.clone(),
                UiComponentStyles::default()
                    .set_padding(Coords::uniform(0.))
                    .set_height(ASK_TEXT_HEIGHT),
            )
            .build()
            .finish(),
        )
        .finish(),
    );
    let field = ConstrainedBox::new(
        Container::new(field.finish())
            .with_padding_left(14.)
            .with_padding_right(14.)
            .with_background(background)
            .with_border(Border::all(1.).with_border_fill(if ask.focused {
                theme.accent()
            } else {
                neutral(theme, 20)
            }))
            .with_corner_radius(radius(8.))
            .finish(),
    )
    .with_height(44.)
    .finish();
    if results.is_empty() || !ask.focused {
        return field;
    }
    let selected = ask.selection.min(results.len() - 1);
    let mut list = Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);
    for (index, result) in results.iter().enumerate() {
        let surface = theme.surface_2();
        let mut row = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(10.);
        row.add_child(sized_icon(
            match result.target {
                AskTarget::Project(_) => Icon::Folder,
                AskTarget::Tab(_) => Icon::Terminal,
            },
            theme.sub_text_color(surface),
            14.,
        ));
        row.add_child(ellipsized(
            result.label.clone(),
            appearance.ui_font_family(),
            14.,
            theme.main_text_color(surface),
        ));
        row.add_child(
            Expanded::new(
                1.,
                ellipsized(
                    result.sub.clone(),
                    appearance.ui_font_family(),
                    12.,
                    theme.hint_text_color(surface),
                ),
            )
            .finish(),
        );
        row.add_child(
            Text::new_inline(
                match result.target {
                    AskTarget::Project(_) => "projeto",
                    AskTarget::Tab(_) => "aba",
                },
                appearance.ui_font_family(),
                10.,
            )
            .with_color(theme.hint_text_color(surface).into())
            .finish(),
        );
        let action = result.target.action();
        list.add_child(
            Hoverable::new(mouse_state("home-ask-result", index as i64), |mouse| {
                let mut row = Container::new(row.finish())
                    .with_padding_left(14.)
                    .with_padding_right(14.)
                    .with_padding_top(7.)
                    .with_padding_bottom(7.);
                if index == selected || mouse.is_mouse_over_element() {
                    row = row.with_background(theme.accent().with_opacity(20));
                }
                row.finish()
            })
            .with_cursor(Cursor::PointingHand)
            .on_click(move |ctx, _, _| ctx.dispatch_typed_action(action.clone()))
            .finish(),
        );
    }
    Flex::column()
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_spacing(6.)
        .with_child(field)
        .with_child(
            Container::new(list.finish())
                .with_padding_top(6.)
                .with_padding_bottom(6.)
                .with_background(theme.surface_2())
                .with_border(Border::all(1.).with_border_fill(theme.outline()))
                .with_corner_radius(radius(8.))
                .finish(),
        )
        .finish()
}

fn greeting() -> &'static str {
    match Local::now().hour() {
        0..=11 => "Bom dia",
        12..=17 => "Boa tarde",
        _ => "Boa noite",
    }
}

pub(super) fn relative_time(timestamp_ms: i64) -> String {
    if timestamp_ms <= 0 {
        return "—".to_owned();
    }
    let minutes = ((Utc::now().timestamp_millis() - timestamp_ms) / 60_000).max(0);
    if minutes < 1 {
        return "agora".to_owned();
    }
    if minutes < 60 {
        return format!("há {minutes} min");
    }
    let hours = (minutes as f64 / 60.).round() as i64;
    if hours < 24 {
        return format!("há {hours} h");
    }
    let days = (hours as f64 / 24.).round() as i64;
    if days == 1 {
        "ontem".to_owned()
    } else {
        format!("há {days} dias")
    }
}

fn plural(count: usize, singular: &str, plural: &str) -> String {
    format!("{count} {}", if count == 1 { singular } else { plural })
}

fn ellipsized(
    text: String,
    font: warpui::fonts::FamilyId,
    size: f32,
    color: ThemeFill,
) -> Box<dyn Element> {
    Text::new_inline(text, font, size)
        .with_color(color.into())
        .soft_wrap(false)
        .with_clip(ClipConfig::ellipsis())
        .finish()
}

fn section_title(label: &str, right: Option<String>, appearance: &Appearance) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.background();
    let mut row = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .with_cross_axis_alignment(CrossAxisAlignment::Center);
    row.add_child(
        Text::new_inline(label.to_owned(), appearance.ui_font_family(), 16.)
            .with_color(theme.main_text_color(background).into())
            .finish(),
    );
    if let Some(right) = right {
        row.add_child(
            Text::new_inline(right, appearance.ui_font_family(), 12.)
                .with_color(theme.hint_text_color(background).into())
                .finish(),
        );
    }
    Container::new(row.finish())
        .with_margin_bottom(10.)
        .finish()
}

fn project_card(
    model: &WorkspaceModel,
    project: &LocalProject,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.background();
    let sub = theme.sub_text_color(background);
    let hint = theme.hint_text_color(background);
    let action = WorkspaceAction::SelectLocalWorkspaceProject(project.id);

    let organization = model.organization(project.organization_id);
    let mut organization_row = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(6.);
    if let Some(organization) = organization {
        organization_row.add_child(dot(
            super::org_rail::organization_accent(theme, &organization.accent_color),
            8.,
        ));
        organization_row.add_child(
            Shrinkable::new(
                1.,
                ellipsized(
                    organization.name.clone(),
                    appearance.ui_font_family(),
                    11.,
                    sub,
                ),
            )
            .finish(),
        );
    }

    let mut footer = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(8.);
    let mut branch = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(8.);
    if let Some(name) = model.project_branch(project.id) {
        branch.add_child(sized_icon(Icon::GitBranch, sub, 12.));
        branch.add_child(
            Shrinkable::new(
                1.,
                ellipsized(
                    name.to_owned(),
                    appearance.monospace_font_family(),
                    11.,
                    sub,
                ),
            )
            .finish(),
        );
    }
    footer.add_child(Expanded::new(1., branch.finish()).finish());
    let (status_text, status_color) = match model.project_path_status(project.id) {
        ProjectPathStatus::Checking | ProjectPathStatus::Available => (None, sub),
        ProjectPathStatus::Missing => (Some("Pasta não encontrada"), theme.ui_error_color().into()),
        ProjectPathStatus::NotDirectory => (
            Some("O caminho não é uma pasta"),
            theme.ui_error_color().into(),
        ),
        ProjectPathStatus::Inaccessible => {
            (Some("Sem acesso à pasta"), theme.ui_error_color().into())
        }
    };
    footer.add_child(
        Text::new_inline(
            status_text.map_or_else(|| plural(project.tabs.len(), "aba", "abas"), str::to_owned),
            appearance.ui_font_family(),
            11.,
        )
        .with_color(status_color.into())
        .finish(),
    );
    footer.add_child(
        Text::new_inline("•", appearance.ui_font_family(), 11.)
            .with_color(hint.into())
            .finish(),
    );
    footer.add_child(
        Text::new_inline(
            relative_time(project.last_opened_ts),
            appearance.ui_font_family(),
            11.,
        )
        .with_color(sub.into())
        .finish(),
    );

    let mut card = Flex::column()
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_spacing(6.);
    card.add_child(organization_row.finish());
    card.add_child(
        Text::new_inline(project.name.clone(), appearance.ui_font_family(), 14.)
            .with_style(Properties::default().weight(Weight::Semibold))
            .with_color(theme.main_text_color(background).into())
            .soft_wrap(false)
            .with_clip(ClipConfig::ellipsis())
            .finish(),
    );
    card.add_child(ellipsized(
        display_path(&project.path),
        appearance.monospace_font_family(),
        12.,
        hint,
    ));
    card.add_child(Container::new(footer.finish()).with_margin_top(4.).finish());

    Hoverable::new(mouse_state("home-card", project.id as i64), |mouse| {
        let hovered = mouse.is_mouse_over_element();
        Container::new(card.finish())
            .with_uniform_padding(14.)
            .with_background(fg_overlay(theme, if hovered { 10 } else { 5 }))
            .with_border(Border::all(1.).with_border_fill(if hovered {
                fg_overlay(theme, 20)
            } else {
                theme.outline()
            }))
            .with_corner_radius(radius(8.))
            .finish()
    })
    .with_cursor(Cursor::PointingHand)
    .on_click(move |ctx, _, _| ctx.dispatch_typed_action(action.clone()))
    .finish()
}

struct RecentTab<'a> {
    tab_id: i32,
    title: String,
    project: &'a LocalProject,
    last_used: i64,
}

fn recent_tab_row(
    model: &WorkspaceModel,
    tab: &RecentTab,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.background();
    let hint = theme.hint_text_color(background);
    let action = WorkspaceAction::SelectLocalWorkspaceTab(tab.tab_id);

    let mut row = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(12.);
    row.add_child(sized_icon(
        Icon::Terminal,
        theme.sub_text_color(background),
        16.,
    ));
    row.add_child(
        Shrinkable::new(
            1.,
            ellipsized(
                tab.title.clone(),
                appearance.ui_font_family(),
                13.,
                theme.main_text_color(background),
            ),
        )
        .finish(),
    );
    let organization = model.organization(tab.project.organization_id);
    let mut location = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(6.);
    if let Some(organization) = organization {
        location.add_child(dot(
            super::org_rail::organization_accent(theme, &organization.accent_color),
            6.,
        ));
    }
    location.add_child(
        Text::new_inline(
            match organization {
                Some(organization) => format!("{} · {}", tab.project.name, organization.name),
                None => tab.project.name.clone(),
            },
            appearance.ui_font_family(),
            11.,
        )
        .with_color(hint.into())
        .finish(),
    );
    row.add_child(location.finish());
    row.add_child(
        ConstrainedBox::new(
            Flex::row()
                .with_main_axis_size(MainAxisSize::Max)
                .with_main_axis_alignment(MainAxisAlignment::End)
                .with_child(
                    Text::new_inline(
                        relative_time(tab.last_used),
                        appearance.ui_font_family(),
                        11.,
                    )
                    .with_color(hint.into())
                    .finish(),
                )
                .finish(),
        )
        .with_width(72.)
        .finish(),
    );

    Hoverable::new(mouse_state("home-tab", tab.tab_id as i64), |mouse| {
        let mut row = Container::new(row.finish())
            .with_padding_left(12.)
            .with_padding_right(12.)
            .with_corner_radius(radius(4.));
        if mouse.is_mouse_over_element() {
            row = row.with_background(fg_overlay(theme, 5));
        }
        ConstrainedBox::new(row.finish()).with_height(40.).finish()
    })
    .with_cursor(Cursor::PointingHand)
    .on_click(move |ctx, _, _| ctx.dispatch_typed_action(action.clone()))
    .finish()
}

pub(super) fn render(
    model: &WorkspaceModel,
    appearance: &Appearance,
    unassigned_tabs: &[(usize, String)],
    workspace_tab_titles: &HashMap<String, String>,
    scroll_state: ClippedScrollStateHandle,
    ask: &HomeAsk,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let background = theme.background();

    let mut header_text = Flex::column().with_spacing(4.);
    header_text.add_child(
        Text::new_inline(
            format!("{}. Tudo pronto.", greeting()),
            appearance.ui_font_family(),
            23.,
        )
        .with_style(Properties::default().weight(Weight::Semibold))
        .with_color(theme.main_text_color(background).into())
        .finish(),
    );
    header_text.add_child(
        Text::new_inline(
            "Retome de onde parou.".to_owned(),
            appearance.ui_font_family(),
            14.,
        )
        .with_color(theme.sub_text_color(background).into())
        .finish(),
    );
    let mut header = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(14.);
    header.add_child(
        ConstrainedBox::new(
            Image::new(
                AssetSource::Bundled {
                    path: "bundled/png/kikito-terminal-icon.png",
                },
                CacheOption::BySize,
            )
            .with_corner_radius(radius(10.))
            .finish(),
        )
        .with_width(44.)
        .with_height(44.)
        .finish(),
    );
    header.add_child(header_text.finish());

    let mut content = Flex::column()
        .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
        .with_spacing(32.);
    let results = ask_results(model, ask.query, workspace_tab_titles);
    content.add_child(
        Flex::column()
            .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
            .with_spacing(16.)
            .with_child(header.finish())
            .with_child(ask_field(ask, &results, appearance))
            .finish(),
    );
    if let Some(error) = model.status_error() {
        content.add_child(
            Text::new_inline(error.to_owned(), appearance.ui_font_family(), 12.)
                .with_color(theme.ui_error_color().into())
                .finish(),
        );
    }

    if model.project_count() == 0 {
        content.add_child(
            Text::new_inline(
                "Crie uma organização para começar a organizar seus projetos.".to_owned(),
                appearance.ui_font_family(),
                14.,
            )
            .with_color(theme.sub_text_color(background).into())
            .finish(),
        );
    } else {
        let projects: Vec<&LocalProject> = model.recent_projects().take(RECENT_PROJECTS).collect();
        let mut projects_section =
            Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);
        projects_section.add_child(section_title(
            "Projetos recentes",
            Some(format!(
                "{} em {}",
                plural(model.project_count(), "projeto", "projetos"),
                plural(model.organizations().count(), "organização", "organizações"),
            )),
            appearance,
        ));
        let mut grid = Flex::column()
            .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
            .with_spacing(12.);
        for chunk in projects.chunks(CARD_COLUMNS) {
            let mut row = Flex::row()
                .with_main_axis_size(MainAxisSize::Max)
                .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
                .with_spacing(12.);
            for project in chunk {
                row.add_child(Expanded::new(1., project_card(model, project, appearance)).finish());
            }
            for _ in chunk.len()..CARD_COLUMNS {
                row.add_child(Expanded::new(1., Empty::new().finish()).finish());
            }
            grid.add_child(row.finish());
        }
        projects_section.add_child(grid.finish());
        content.add_child(projects_section.finish());

        let mut recent_tabs: Vec<RecentTab> = model
            .projects()
            .flat_map(|project| {
                project.tabs.iter().filter_map(move |tab_id| {
                    let identity = model.workspace_tab_id(*tab_id)?;
                    let title = workspace_tab_titles.get(identity)?;
                    Some(RecentTab {
                        tab_id: *tab_id,
                        title: if title.trim().is_empty() {
                            "Terminal".to_owned()
                        } else {
                            title.clone()
                        },
                        project,
                        last_used: project.last_opened_ts,
                    })
                })
            })
            .collect();
        recent_tabs.sort_by(|left, right| {
            right
                .last_used
                .cmp(&left.last_used)
                .then_with(|| left.tab_id.cmp(&right.tab_id))
        });
        recent_tabs.truncate(RECENT_TABS);
        if !recent_tabs.is_empty() {
            let mut tabs_list =
                Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);
            for tab in &recent_tabs {
                tabs_list.add_child(recent_tab_row(model, tab, appearance));
            }
            let mut tabs_section =
                Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);
            tabs_section.add_child(section_title("Abas recentes", None, appearance));
            tabs_section.add_child(
                Container::new(tabs_list.finish())
                    .with_uniform_padding(4.)
                    .with_border(Border::all(1.).with_border_fill(theme.outline()))
                    .with_corner_radius(radius(8.))
                    .finish(),
            );
            content.add_child(tabs_section.finish());
        }
    }

    if !unassigned_tabs.is_empty() {
        let mut list = Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);
        for (index, title) in unassigned_tabs {
            let action = WorkspaceAction::ActivateTab(*index);
            let mut row = Flex::row()
                .with_main_axis_size(MainAxisSize::Max)
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_spacing(12.);
            row.add_child(sized_icon(
                Icon::Terminal,
                theme.sub_text_color(background),
                16.,
            ));
            row.add_child(
                Shrinkable::new(
                    1.,
                    ellipsized(
                        title.clone(),
                        appearance.ui_font_family(),
                        13.,
                        theme.main_text_color(background),
                    ),
                )
                .finish(),
            );
            list.add_child(
                EventHandler::new(
                    ConstrainedBox::new(
                        Container::new(row.finish())
                            .with_padding_left(12.)
                            .with_padding_right(12.)
                            .with_corner_radius(radius(4.))
                            .finish(),
                    )
                    .with_height(40.)
                    .finish(),
                )
                .on_left_mouse_up(move |ctx, _, _| {
                    ctx.dispatch_typed_action(action.clone());
                    DispatchEventResult::StopPropagation
                })
                .finish(),
            );
        }
        let mut section = Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);
        section.add_child(section_title("Terminais sem projeto", None, appearance));
        section.add_child(
            Container::new(list.finish())
                .with_uniform_padding(4.)
                .with_border(Border::all(1.).with_border_fill(theme.outline()))
                .with_corner_radius(radius(8.))
                .finish(),
        );
        content.add_child(section.finish());
    }

    let padded = Container::new(content.finish())
        .with_padding_top(48.)
        .with_padding_left(40.)
        .with_padding_right(40.)
        .with_padding_bottom(40.)
        .finish();
    let centered = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Shrinkable::new(1., Empty::new().finish()).finish())
        .with_child(
            ConstrainedBox::new(padded)
                .with_max_width(CONTENT_MAX_WIDTH + 80.)
                .finish(),
        )
        .with_child(Shrinkable::new(1., Empty::new().finish()).finish())
        .finish();
    let scroll = NewScrollable::vertical(
        SingleAxisConfig::Clipped {
            handle: scroll_state,
            child: centered,
        },
        theme.sub_text_color(background).into(),
        theme.main_text_color(background).into(),
        Fill::None,
    )
    .with_vertical_scrollbar(ScrollableAppearance::new(ScrollbarWidth::Auto, true))
    .finish();
    Container::new(
        Flex::column()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
            .with_child(super::project_tabs::render_title_bar(appearance, "Início"))
            .with_child(Shrinkable::new(1., scroll).finish())
            .finish(),
    )
    .with_background(background)
    .finish()
}

#[cfg(test)]
#[path = "home_tests.rs"]
mod tests;
