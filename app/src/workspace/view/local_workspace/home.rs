use std::collections::HashMap;

use chrono::{Local, Timelike, Utc};
use warp_core::ui::Icon;
use warp_core::ui::theme::Fill as ThemeFill;
use warpui::assets::asset_cache::AssetSource;
use warpui::elements::new_scrollable::{NewScrollable, ScrollableAppearance, SingleAxisConfig};
use warpui::elements::{
    Border, CacheOption, ClippedScrollStateHandle, ConstrainedBox, Container, CrossAxisAlignment,
    DispatchEventResult, Element, Empty, EventHandler, Expanded, Fill, Flex, Image, MainAxisAlignment,
    MainAxisSize, ParentElement, ScrollbarWidth, Shrinkable, Text,
};
use warpui::fonts::{Properties, Weight};
use warpui::text_layout::ClipConfig;

use super::ws_style::{fg_overlay, radius, sized_icon};
use crate::appearance::Appearance;
use crate::local_workspace::model::{LocalProject, ProjectPathStatus, WorkspaceModel};
use crate::workspace::action::WorkspaceAction;

const CONTENT_MAX_WIDTH: f32 = 960.;
const CARD_COLUMNS: usize = 4;
const RECENT_PROJECTS: usize = 6;
const RECENT_TABS: usize = 7;

fn greeting() -> &'static str {
    match Local::now().hour() {
        0..=11 => "Bom dia",
        12..=17 => "Boa tarde",
        _ => "Boa noite",
    }
}

fn relative_time(timestamp_ms: i64) -> String {
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

fn section_title(
    label: &str,
    right: Option<String>,
    appearance: &Appearance,
) -> Box<dyn Element> {
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
    Container::new(row.finish()).with_margin_bottom(10.).finish()
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
        organization_row.add_child(sized_icon(
            Icon::CircleFilled,
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
    let (status_text, status_color) = match model.project_path_status(project.id) {
        ProjectPathStatus::Checking | ProjectPathStatus::Available => (None, sub),
        ProjectPathStatus::Missing => (
            Some("Pasta não encontrada"),
            theme.ui_error_color().into(),
        ),
        ProjectPathStatus::NotDirectory => (
            Some("O caminho não é uma pasta"),
            theme.ui_error_color().into(),
        ),
        ProjectPathStatus::Inaccessible => {
            (Some("Sem acesso à pasta"), theme.ui_error_color().into())
        }
    };
    footer.add_child(
        Shrinkable::new(
            1.,
            ellipsized(
                status_text.map_or_else(
                    || plural(project.tabs.len(), "aba", "abas"),
                    str::to_owned,
                ),
                appearance.ui_font_family(),
                11.,
                status_color,
            ),
        )
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
        project.path.clone(),
        appearance.monospace_font_family(),
        12.,
        hint,
    ));
    card.add_child(Container::new(footer.finish()).with_margin_top(4.).finish());

    EventHandler::new(
        Container::new(card.finish())
            .with_uniform_padding(14.)
            .with_background(fg_overlay(theme, 5))
            .with_border(Border::all(1.).with_border_fill(theme.outline()))
            .with_corner_radius(radius(8.))
            .finish(),
    )
    .on_left_mouse_up(move |ctx, _, _| {
        ctx.dispatch_typed_action(action.clone());
        DispatchEventResult::StopPropagation
    })
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
        location.add_child(sized_icon(
            Icon::CircleFilled,
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
    .finish()
}

pub(super) fn render(
    model: &WorkspaceModel,
    appearance: &Appearance,
    unassigned_tabs: &[(usize, String)],
    workspace_tab_titles: &HashMap<String, String>,
    scroll_state: ClippedScrollStateHandle,
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
            Container::new(
                Image::new(
                    AssetSource::Bundled {
                        path: "bundled/png/kikito-terminal-icon.png",
                    },
                    CacheOption::BySize,
                )
                .finish(),
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
    content.add_child(header.finish());
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
        let projects: Vec<&LocalProject> =
            model.recent_projects().take(RECENT_PROJECTS).collect();
        let mut projects_section = Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);
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
            let mut tabs_list = Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);
            for tab in &recent_tabs {
                tabs_list.add_child(recent_tab_row(model, tab, appearance));
            }
            let mut tabs_section = Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);
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
            .with_child(Shrinkable::new(1., scroll).finish())
            .finish(),
    )
    .with_background(background)
    .finish()
}
