use std::sync::Arc;

use crate::appearance::Appearance;
use crate::local_workspace::model::WorkspaceModel;
use crate::workspace::action::WorkspaceAction;
use bytes::Bytes;
use warp_core::ui::Icon;
use warpui::assets::asset_cache::{AssetSource, AsyncAssetId, AsyncAssetType};
use warpui::elements::Element;
use warpui::elements::{
    CacheOption, ConstrainedBox, Container, CrossAxisAlignment, DispatchEventResult, EventHandler,
    Flex, Image, MainAxisAlignment, ParentElement, Text,
};

const RAIL_WIDTH: f32 = 64.;

pub(super) fn render(model: &WorkspaceModel, appearance: &Appearance) -> Box<dyn Element> {
    let mut top_items = Flex::column()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_main_axis_alignment(MainAxisAlignment::Start)
        .with_spacing(8.);
    top_items.add_child(rail_item(
        None,
        String::new(),
        model.selected_organization().is_none(),
        false,
        None,
        true,
        None,
        WorkspaceAction::SelectLocalWorkspaceHome,
        None,
        appearance,
    ));
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
            model.hovered_organization() == Some(organization.id),
            Some(organization.accent_color.as_str()),
            false,
            organization.image_data.clone(),
            WorkspaceAction::SelectLocalWorkspaceOrganization(organization.id),
            Some(WorkspaceAction::OpenLocalWorkspaceOrganizationMenu(
                organization.id,
            )),
            appearance,
        );
        let mut organization_item = Flex::column()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(2.)
            .with_child(item);
        let menu_action = WorkspaceAction::OpenLocalWorkspaceOrganizationMenu(organization.id);
        organization_item.add_child(rail_menu_button(
            menu_action,
            model.hovered_organization() == Some(organization.id),
            appearance,
        ));
        if model.context_menu()
            == Some(crate::local_workspace::model::ContextMenuTarget::Organization(organization.id))
        {
            organization_item.add_child(super::context_menu::render(
                crate::local_workspace::model::ContextMenuTarget::Organization(organization.id),
                model.context_menu_index(),
                appearance,
            ));
        }
        top_items.add_child(organization_item.finish());
    }
    let mut footer = Flex::column()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_main_axis_alignment(MainAxisAlignment::End)
        .with_spacing(8.);
    footer.add_child(rail_action(
        Icon::Plus,
        WorkspaceAction::CreateLocalWorkspaceOrganization,
        appearance,
    ));
    footer.add_child(rail_action(
        Icon::Gear,
        WorkspaceAction::ShowSettingsPage(crate::settings_view::SettingsSection::Appearance),
        appearance,
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
            .with_padding_left(8.)
            .with_padding_right(8.)
            .with_padding_top(14.)
            .with_padding_bottom(12.)
            .finish(),
    )
    .with_width(RAIL_WIDTH)
    .finish()
}

fn rail_action(icon: Icon, action: WorkspaceAction, appearance: &Appearance) -> Box<dyn Element> {
    let icon = icon
        .to_warpui_icon(
            appearance
                .theme()
                .sub_text_color(appearance.theme().surface_1()),
        )
        .finish();
    let button = ConstrainedBox::new(Container::new(icon).finish())
        .with_width(36.)
        .with_height(36.)
        .finish();
    EventHandler::new(button)
        .on_left_mouse_up(move |ctx, _, _| {
            ctx.dispatch_typed_action(action.clone());
            DispatchEventResult::StopPropagation
        })
        .finish()
}

fn rail_item(
    icon: Option<Icon>,
    label: String,
    active: bool,
    hovered: bool,
    accent_color: Option<&str>,
    brand_home: bool,
    image_data: Option<Vec<u8>>,
    action: WorkspaceAction,
    context_action: Option<WorkspaceAction>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let child = if brand_home {
        Image::new(
            AssetSource::Bundled {
                path: "bundled/png/kikito-terminal-icon.png",
            },
            CacheOption::BySize,
        )
        .finish()
    } else if let Some(image_data) = image_data {
        Image::new(organization_image_source(image_data), CacheOption::BySize).finish()
    } else if let Some(icon) = icon {
        icon.to_warpui_icon(theme.font_color(theme.surface_1()))
            .finish()
    } else {
        Text::new_inline(label, appearance.ui_font_family(), 14.)
            .with_color(theme.font_color(theme.surface_1()).into())
            .finish()
    };
    let tile = ConstrainedBox::new(
        Container::new(child)
            .with_background(accent_color.map_or_else(
                || {
                    if active || hovered {
                        theme.surface_2()
                    } else {
                        theme.surface_1()
                    }
                },
                |accent| organization_accent(accent, theme.accent()),
            ))
            .with_border(
                warpui::elements::Border::all(if active { 2. } else { 1. }).with_border_fill(
                    if active {
                        theme.accent()
                    } else if hovered {
                        theme.outline()
                    } else {
                        theme.surface_1()
                    },
                ),
            )
            .with_corner_radius(warpui::elements::CornerRadius::with_all(
                warpui::elements::Radius::Pixels(8.),
            ))
            .finish(),
    )
    .with_width(36.)
    .with_height(36.)
    .finish();
    let hover_id = match context_action.as_ref() {
        Some(WorkspaceAction::OpenLocalWorkspaceOrganizationMenu(id)) => Some(*id),
        _ => None,
    };
    let mut handler = EventHandler::new(tile)
        .on_left_mouse_up(move |ctx, _, _| {
            ctx.dispatch_typed_action(action.clone());
            DispatchEventResult::StopPropagation
        })
        .on_mouse_in(
            move |ctx, _, _| {
                if let Some(id) = hover_id {
                    ctx.dispatch_typed_action(
                        WorkspaceAction::SetLocalWorkspaceHoveredOrganization(Some(id)),
                    );
                }
                DispatchEventResult::PropagateToParent
            },
            None,
        );
    let hover_id = match context_action.as_ref() {
        Some(WorkspaceAction::OpenLocalWorkspaceOrganizationMenu(id)) => Some(*id),
        _ => None,
    };
    handler = handler.on_mouse_out(move |ctx, _, _| {
        if hover_id.is_some() {
            ctx.dispatch_typed_action(WorkspaceAction::SetLocalWorkspaceHoveredOrganization(None));
        }
        DispatchEventResult::PropagateToParent
    });
    if let Some(context_action) = context_action {
        handler = handler.on_right_mouse_down(move |ctx, _, _, _| {
            ctx.dispatch_typed_action(context_action.clone());
            DispatchEventResult::StopPropagation
        });
    }
    handler.finish()
}

fn rail_menu_button(
    action: WorkspaceAction,
    hovered: bool,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let organization_id = match &action {
        WorkspaceAction::OpenLocalWorkspaceOrganizationMenu(id) => Some(*id),
        _ => None,
    };
    let label = Container::new(
        Text::new_inline("⋯", appearance.ui_font_family(), 14.)
            .with_color(theme.font_color(theme.surface_1()).into())
            .finish(),
    )
    .with_background(if hovered {
        theme.surface_2()
    } else {
        theme.surface_1()
    })
    .with_corner_radius(warpui::elements::CornerRadius::with_all(
        warpui::elements::Radius::Pixels(4.),
    ))
    .with_padding_left(8.)
    .with_padding_right(8.)
    .with_padding_top(1.)
    .with_padding_bottom(1.)
    .finish();
    let mut handler = EventHandler::new(label).on_left_mouse_up(move |ctx, _, _| {
        ctx.dispatch_typed_action(action.clone());
        DispatchEventResult::StopPropagation
    });
    if let Some(id) = organization_id {
        handler = handler
            .on_mouse_in(
                move |ctx, _, _| {
                    ctx.dispatch_typed_action(
                        WorkspaceAction::SetLocalWorkspaceHoveredOrganization(Some(id)),
                    );
                    DispatchEventResult::PropagateToParent
                },
                None,
            )
            .on_mouse_out(|ctx, _, _| {
                ctx.dispatch_typed_action(WorkspaceAction::SetLocalWorkspaceHoveredOrganization(
                    None,
                ));
                DispatchEventResult::PropagateToParent
            });
    }
    handler.finish()
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

fn organization_accent(
    name: &str,
    fallback: warp_core::ui::theme::Fill,
) -> warp_core::ui::theme::Fill {
    let hex = match name {
        "green" => "#20A874",
        "magenta" => "#CF5B9C",
        "yellow" => "#D8A82E",
        "cyan" => "#29A5B3",
        "red" => "#D45D5D",
        _ => "#4E83D3",
    };
    warp_core::ui::color::hex_color::coloru_from_hex_string(hex)
        .map(warp_core::ui::theme::Fill::Solid)
        .unwrap_or_else(|_| fallback.into())
}
