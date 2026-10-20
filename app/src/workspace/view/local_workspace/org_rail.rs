use std::sync::Arc;

use super::ws_style::{TooltipSide, mouse_state, neutral, tooltip, with_tooltip};
use crate::appearance::Appearance;
use warpui::platform::Cursor;
use crate::local_workspace::model::WorkspaceModel;
use crate::workspace::action::WorkspaceAction;
use bytes::Bytes;
use pathfinder_color::ColorU;
use warp_core::ui::Icon;
use warp_core::ui::theme::WarpTheme;
use warpui::assets::asset_cache::{AssetSource, AsyncAssetId, AsyncAssetType};
use warpui::elements::Element;
use warpui::fonts::{Properties, Weight};
use warpui::elements::{
    Border, CacheOption, ChildAnchor, ConstrainedBox, Container, CrossAxisAlignment,
    Dash, Flex, Hoverable, Image, MainAxisAlignment, OffsetPositioning, ParentAnchor,
    ParentElement, ParentOffsetBounds, Rect, Stack, Text,
};
use pathfinder_geometry::vector::vec2f;

const RAIL_WIDTH: f32 = 64.;
const RAIL_HEADER_HEIGHT: f32 = 34.;
const ORGANIZATION_TILE_SIZE: f32 = 36.;
const ACTIVE_MARKER_HEIGHT: f32 = 28.;
const HOVER_MARKER_HEIGHT: f32 = 12.;

fn organization_marker_height(active: bool, hovered: bool) -> f32 {
    if active {
        ACTIVE_MARKER_HEIGHT
    } else if hovered {
        HOVER_MARKER_HEIGHT
    } else {
        0.
    }
}

pub(super) fn render(model: &WorkspaceModel, appearance: &Appearance) -> Box<dyn Element> {
    let mut top_items = Flex::column()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_main_axis_alignment(MainAxisAlignment::Start)
        .with_spacing(8.);
    top_items.add_child(
        ConstrainedBox::new(Rect::new().finish())
            .with_width(RAIL_WIDTH)
            .with_height(RAIL_HEADER_HEIGHT)
            .finish(),
    );
    top_items.add_child(rail_item(
        None,
        String::new(),
        model.selected_organization().is_none(),
        ("home-tile", 0),
        Some("Início".to_owned()),
        None,
        true,
        None,
        WorkspaceAction::SelectLocalWorkspaceHome,
        None,
        appearance,
    ));
    let divider = ConstrainedBox::new(
        Flex::column()
            .with_main_axis_size(warpui::elements::MainAxisSize::Max)
            .with_main_axis_alignment(MainAxisAlignment::Center)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_child(
                ConstrainedBox::new(
                    Rect::new()
                        .with_background(appearance.theme().outline())
                        .finish(),
                )
                .with_width(24.)
                .with_height(1.)
                .finish(),
            )
            .finish(),
    )
    .with_width(RAIL_WIDTH)
    .with_height(5.)
    .finish();
    top_items.add_child(divider);
    for organization in model.organizations() {
        let item = rail_item(
            None,
            organization
                .name
                .chars()
                .next()
                .unwrap_or('?')
                .to_uppercase()
                .to_string(),
            model.selected_organization() == Some(organization.id),
            ("org-tile", organization.id as i64),
            (model.context_menu()
                != Some(crate::local_workspace::model::ContextMenuTarget::Organization(
                    organization.id,
                )))
            .then(|| organization.name.clone()),
            Some(organization.accent_color.as_str()),
            false,
            organization.image_data.clone(),
            WorkspaceAction::SelectLocalWorkspaceOrganization(organization.id),
            Some(WorkspaceAction::OpenLocalWorkspaceOrganizationMenu(
                organization.id,
            )),
            appearance,
        );
        let mut organization_item = Stack::new().with_child(item);
        if model.context_menu()
            == Some(crate::local_workspace::model::ContextMenuTarget::Organization(organization.id))
        {
            organization_item.add_positioned_overlay_child(
                super::context_menu::render(
                    crate::local_workspace::model::ContextMenuTarget::Organization(organization.id),
                    model.context_menu_index(),
                    model.context_menu_keyboard_active(),
                    appearance,
                ),
                OffsetPositioning::offset_from_parent(
                    vec2f(RAIL_WIDTH / 2., ORGANIZATION_TILE_SIZE / 2.),
                    ParentOffsetBounds::WindowByPosition,
                    ParentAnchor::TopLeft,
                    ChildAnchor::TopLeft,
                ),
            );
        }
        top_items.add_child(organization_item.finish());
    }
    top_items.add_child(rail_action(
        Icon::Plus,
        WorkspaceAction::CreateLocalWorkspaceOrganization,
        appearance,
        ("rail-add", 0),
        "Nova organização",
    ));
    let mut footer = Flex::column()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_main_axis_alignment(MainAxisAlignment::End)
        .with_spacing(8.);
    footer.add_child(rail_action(
        Icon::Gear,
        WorkspaceAction::ShowSettingsPage(crate::settings_view::SettingsSection::Appearance),
        appearance,
        ("rail-settings", 0),
        "Configurações",
    ));
    let rail = Flex::column()
        .with_main_axis_size(warpui::elements::MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .with_child(top_items.finish())
        .with_child(footer.finish())
        .finish();
    ConstrainedBox::new(
        Container::new(rail)
            .with_background(appearance.theme().surface_1())
            .with_border(Border::right(1.).with_border_fill(appearance.theme().outline()))
            .with_padding_top(0.)
            .with_padding_bottom(12.)
            .finish(),
    )
    .with_width(RAIL_WIDTH + 1.)
    .finish()
}

fn rail_action(
    icon: Icon,
    action: WorkspaceAction,
    appearance: &Appearance,
    key: (&'static str, i64),
    label: &str,
) -> Box<dyn Element> {
    let is_add = icon == Icon::Plus;
    let state = mouse_state(key.0, key.1);
    let hovered = state.lock().unwrap().is_mouse_over_element();
    let icon = icon
        .to_warpui_icon(
            appearance
                .theme()
                .sub_text_color(appearance.theme().surface_1()),
        )
        .finish();
    let icon = ConstrainedBox::new(icon)
        .with_width(16.)
        .with_height(16.)
        .finish();
    let content = Flex::row()
        .with_main_axis_size(warpui::elements::MainAxisSize::Max)
        .with_main_axis_alignment(MainAxisAlignment::Center)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(icon)
        .finish();
    let content = ConstrainedBox::new(content)
        .with_width(36.)
        .with_height(36.)
        .finish();
    let mut button = Container::new(content);
    if is_add {
        button = button
            .with_border(
                Border::all(1.)
                    .with_border_fill(neutral(appearance.theme(), 20))
                    .with_dashed_border(Dash {
                        dash_length: 4.,
                        gap_length: 4.,
                        ..Default::default()
                    }),
            )
            .with_corner_radius(warpui::elements::CornerRadius::with_all(
                warpui::elements::Radius::Pixels(8.),
            ));
    }
    let button = ConstrainedBox::new(button.finish())
        .with_width(36.)
        .with_height(36.)
        .finish();
    let button = Hoverable::new(state, |_| button)
        .with_cursor(Cursor::PointingHand)
        .on_click(move |ctx, _, _| ctx.dispatch_typed_action(action.clone()))
        .finish();
    if hovered {
        with_tooltip(button, tooltip(appearance, label), TooltipSide::Right { gap: 10. })
    } else {
        button
    }
}

fn rail_item(
    icon: Option<Icon>,
    label: String,
    active: bool,
    hover_key: (&'static str, i64),
    tooltip_label: Option<String>,
    accent_color: Option<&str>,
    brand_home: bool,
    image_data: Option<Vec<u8>>,
    action: WorkspaceAction,
    context_action: Option<WorkspaceAction>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let hover_state = mouse_state(hover_key.0, hover_key.1);
    let hovered = hover_state.lock().unwrap().is_mouse_over_element();
    let child = if brand_home {
        ConstrainedBox::new(
            Image::new(
                AssetSource::Bundled {
                    path: "bundled/png/kikito-terminal-icon.png",
                },
                CacheOption::BySize,
            )
            .with_opacity(if active { 1. } else { 0.85 })
            .with_corner_radius(warpui::elements::CornerRadius::with_all(
                warpui::elements::Radius::Pixels(8.),
            ))
            .finish(),
        )
        .with_width(ORGANIZATION_TILE_SIZE)
        .with_height(ORGANIZATION_TILE_SIZE)
        .finish()
    } else if let Some(image_data) = image_data {
        Image::new(organization_image_source(image_data), CacheOption::BySize).finish()
    } else if let Some(icon) = icon {
        icon.to_warpui_icon(theme.font_color(theme.surface_2()))
            .finish()
    } else {
        Text::new_inline(label, appearance.ui_font_family(), 15.)
            .with_style(Properties::default().weight(Weight::Bold))
            .with_color(
                accent_color.map_or_else(
                    || theme.font_color(theme.surface_2()).into(),
                    |accent| organization_accent_with_alpha(theme, accent, 255).into(),
                ),
            )
            .finish()
    };
    let tile_contents = Flex::row()
        .with_main_axis_size(warpui::elements::MainAxisSize::Max)
        .with_main_axis_alignment(MainAxisAlignment::Center)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(child)
        .finish();
    let tile_contents = ConstrainedBox::new(tile_contents)
        .with_width(ORGANIZATION_TILE_SIZE)
        .with_height(ORGANIZATION_TILE_SIZE)
        .finish();
    let tile = if brand_home {
        Container::new(tile_contents).finish()
    } else {
        Container::new(tile_contents)
            .with_background(accent_color.map_or_else(
                || theme.surface_2(),
                |accent| organization_accent_with_alpha(theme, accent, if active { 82 } else { 46 }),
            ))
            .with_border(
                warpui::elements::Border::all(1.).with_border_fill(if active {
                    accent_color.map_or_else(
                        || theme.accent(),
                        |accent| organization_accent_with_alpha(theme, accent, 255),
                    )
                } else {
                    organization_accent_with_alpha(theme, accent_color.unwrap_or_default(), 76)
                }),
            )
            .with_corner_radius(warpui::elements::CornerRadius::with_all(
                warpui::elements::Radius::Pixels(if active { 10. } else { 8. }),
            ))
            .finish()
    };
    let tile = ConstrainedBox::new(tile)
        .with_width(ORGANIZATION_TILE_SIZE)
        .with_height(ORGANIZATION_TILE_SIZE)
        .finish();
    let marker_height = organization_marker_height(active, hovered);
    let content = Flex::row()
        .with_main_axis_size(warpui::elements::MainAxisSize::Max)
        .with_main_axis_alignment(MainAxisAlignment::Center)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(tile);
    let content = ConstrainedBox::new(content.finish())
        .with_width(RAIL_WIDTH)
        .with_height(ORGANIZATION_TILE_SIZE)
        .finish();
    let mut tile_stack = Stack::new().with_child(content);
    if marker_height > 0. {
        let marker = ConstrainedBox::new(
            Rect::new()
                .with_background(theme.font_color(theme.surface_1()))
                .finish(),
        )
        .with_width(3.)
        .with_height(marker_height)
        .finish();
        tile_stack.add_positioned_overlay_child(
            marker,
            OffsetPositioning::offset_from_parent(
                vec2f(0., 0.),
                ParentOffsetBounds::Unbounded,
                ParentAnchor::MiddleLeft,
                ChildAnchor::MiddleLeft,
            ),
        );
    }
    let mut handler = Hoverable::new(hover_state, |_| tile_stack.finish())
        .with_cursor(Cursor::PointingHand)
        .on_click(move |ctx, _, _| ctx.dispatch_typed_action(action.clone()));
    if let Some(context_action) = context_action {
        handler = handler
            .on_right_click(move |ctx, _, _| ctx.dispatch_typed_action(context_action.clone()));
    }
    let handler = handler.finish();
    match tooltip_label {
        Some(label) if hovered => with_tooltip(
            handler,
            tooltip(appearance, &label),
            TooltipSide::Right { gap: 10. - (RAIL_WIDTH - ORGANIZATION_TILE_SIZE) / 2. },
        ),
        _ => handler,
    }
}

struct OrganizationImageAsset;

impl AsyncAssetType for OrganizationImageAsset {}

fn organization_image_source(image_data: Vec<u8>) -> AssetSource {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(&image_data, &mut hasher);
    use std::hash::Hasher;
    let key = format!("organization-image-{}", hasher.finish());
    AssetSource::Async {
        id: AsyncAssetId::new::<OrganizationImageAsset>(key),
        fetch: Arc::new(move || {
            let bytes = Bytes::from(image_data.clone());
            Box::pin(async move { Ok(bytes) })
        }),
    }
}

pub(super) fn organization_accent(theme: &WarpTheme, name: &str) -> warp_core::ui::theme::Fill {
    organization_accent_with_alpha(theme, name, 255)
}

pub(super) fn organization_accent_with_alpha(
    theme: &WarpTheme,
    name: &str,
    alpha: u8,
) -> warp_core::ui::theme::Fill {
    let colors = &theme.terminal_colors().normal;
    let base: ColorU = match name {
        "green" => colors.green,
        "magenta" => colors.magenta,
        "yellow" => colors.yellow,
        "cyan" => colors.cyan,
        "red" => colors.red,
        _ => colors.blue,
    }
    .into();
    warp_core::ui::theme::Fill::Solid(ColorU::new(base.r, base.g, base.b, alpha))
}

#[cfg(test)]
#[path = "org_rail_tests.rs"]
mod tests;
