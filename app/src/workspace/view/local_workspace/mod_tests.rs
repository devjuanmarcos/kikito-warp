use super::{
    TabActivationRoute, branch_from_head, can_activate_tab_in_selected_project, drag_target_index,
    keycaps_from_display, preferred_open_project_tab, should_render_empty_organization_state,
    should_render_project_sidebar, tab_activation_route, tab_after_close,
};
use crate::local_workspace::model::WorkspaceModel;
use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::vector::vec2f;

#[test]
fn empty_project_does_not_expose_an_unassigned_open_terminal() {
    let mut model = WorkspaceModel::new();
    let organization = model.add_organization("Equipe").unwrap();
    let empty_project = model
        .add_project(organization, "Vazio", r"C:\vazio")
        .unwrap();
    let other_project = model
        .add_project(organization, "Outro", r"C:\outro")
        .unwrap();
    let other_tab = model.ensure_tab_identity("other-tab").unwrap();
    model.assign_tab(other_project, other_tab).unwrap();

    assert_eq!(
        preferred_open_project_tab(&model, empty_project, &["other-tab".to_owned()]),
        None
    );
    assert_eq!(
        preferred_open_project_tab(&model, other_project, &["other-tab".to_owned()]),
        Some(other_tab)
    );
    model.set_active_tab(other_project, None).unwrap();
    assert_eq!(
        preferred_open_project_tab(&model, other_project, &["other-tab".to_owned()]),
        Some(other_tab)
    );
    assert_eq!(preferred_open_project_tab(&model, other_project, &[]), None);
}

#[test]
fn project_navigation_rejects_foreign_and_unassigned_tabs() {
    let mut model = WorkspaceModel::new();
    let organization = model.add_organization("Equipe").unwrap();
    let first = model
        .add_project(organization, "Primeiro", r"C:\primeiro")
        .unwrap();
    let second = model
        .add_project(organization, "Segundo", r"C:\segundo")
        .unwrap();
    let first_tab = model.ensure_tab_identity("first-tab").unwrap();
    let second_tab = model.ensure_tab_identity("second-tab").unwrap();
    model.ensure_tab_identity("unassigned-tab").unwrap();
    model.assign_tab(first, first_tab).unwrap();
    model.assign_tab(second, second_tab).unwrap();

    assert!(can_activate_tab_in_selected_project(
        &model,
        Some(first),
        "first-tab"
    ));
    assert!(!can_activate_tab_in_selected_project(
        &model,
        Some(first),
        "second-tab"
    ));
    assert!(!can_activate_tab_in_selected_project(
        &model,
        Some(first),
        "unassigned-tab"
    ));
    assert!(can_activate_tab_in_selected_project(
        &model,
        None,
        "unassigned-tab"
    ));
}

#[test]
fn tab_activation_routes_to_its_project_and_keeps_the_view_for_unassigned_tabs() {
    let mut model = WorkspaceModel::new();
    let organization = model.add_organization("Equipe").unwrap();
    let first = model
        .add_project(organization, "Primeiro", r"C:\primeiro")
        .unwrap();
    let second = model
        .add_project(organization, "Segundo", r"C:\segundo")
        .unwrap();
    let first_tab = model.ensure_tab_identity("first-tab").unwrap();
    let second_tab = model.ensure_tab_identity("second-tab").unwrap();
    model.ensure_tab_identity("unassigned-tab").unwrap();
    model.assign_tab(first, first_tab).unwrap();
    model.assign_tab(second, second_tab).unwrap();
    model.select_project(first).unwrap();

    assert_eq!(
        tab_activation_route(&model, "first-tab"),
        TabActivationRoute::KeepSelection
    );
    assert_eq!(
        tab_activation_route(&model, "second-tab"),
        TabActivationRoute::Project {
            organization_id: organization,
            project_id: second,
        }
    );
    assert_eq!(
        tab_activation_route(&model, "unassigned-tab"),
        TabActivationRoute::KeepSelection
    );
    model.select_organization(organization).unwrap();
    assert_eq!(
        tab_activation_route(&model, "unassigned-tab"),
        TabActivationRoute::KeepSelection
    );
}

#[test]
fn project_sidebar_is_visible_only_after_selecting_an_organization() {
    let mut model = WorkspaceModel::new();
    assert!(!should_render_project_sidebar(&model));

    let organization = model.add_organization("Equipe").unwrap();
    model.select_organization(organization).unwrap();

    assert!(should_render_project_sidebar(&model));
}

#[test]
fn empty_organization_uses_project_empty_state_instead_of_home() {
    let mut model = WorkspaceModel::new();
    assert!(!should_render_empty_organization_state(&model));

    let organization = model.add_organization("Equipe").unwrap();
    model.select_organization(organization).unwrap();
    assert!(should_render_empty_organization_state(&model));

    let project = model
        .add_project(organization, "Projeto", r"C:\\projeto")
        .unwrap();
    model.select_project(project).unwrap();
    assert!(!should_render_empty_organization_state(&model));
}

#[test]
fn branch_from_head_reads_symbolic_and_detached_heads() {
    assert_eq!(
        branch_from_head("ref: refs/heads/feat/checkout\n").as_deref(),
        Some("feat/checkout")
    );
    assert_eq!(
        branch_from_head("4f2c9e1d0b7a6c5e4d3c2b1a0f9e8d7c6b5a4f3e\n").as_deref(),
        Some("4f2c9e1")
    );
    assert_eq!(branch_from_head("  \n"), None);
}

#[test]
fn keycaps_split_mac_symbols_and_spaced_labels() {
    assert_eq!(keycaps_from_display("⌘T"), vec!["⌘", "T"]);
    assert_eq!(keycaps_from_display("⇧⌘T"), vec!["⇧", "⌘", "T"]);
    assert_eq!(
        keycaps_from_display("Ctrl Shift T"),
        vec!["Ctrl", "Shift", "T"]
    );
}

fn row(y: f32) -> Option<RectF> {
    Some(RectF::new(vec2f(0., y), vec2f(100., 40.)))
}

#[test]
fn dragged_item_moves_once_its_center_crosses_a_neighbor_midpoint() {
    let rects = [row(0.), row(40.), row(80.), row(120.)];

    // Center at 55: short of the next row's midpoint (100).
    assert_eq!(
        drag_target_index(
            1,
            RectF::new(vec2f(0., 35.), vec2f(100., 40.)),
            &rects,
            true
        ),
        1
    );
    // Center at 105: past row 2's midpoint.
    assert_eq!(
        drag_target_index(
            1,
            RectF::new(vec2f(0., 85.), vec2f(100., 40.)),
            &rects,
            true
        ),
        2
    );
    // A fast drag past several rows jumps straight to the last one crossed.
    assert_eq!(
        drag_target_index(
            0,
            RectF::new(vec2f(0., 125.), vec2f(100., 40.)),
            &rects,
            true
        ),
        3
    );
    assert_eq!(
        drag_target_index(
            3,
            RectF::new(vec2f(0., -5.), vec2f(100., 40.)),
            &rects,
            true
        ),
        0
    );
}

#[test]
fn drag_target_skips_items_that_are_not_on_screen() {
    let rects = [
        Some(RectF::new(vec2f(0., 0.), vec2f(80., 26.))),
        None,
        Some(RectF::new(vec2f(84., 0.), vec2f(80., 26.))),
    ];

    assert_eq!(
        drag_target_index(
            0,
            RectF::new(vec2f(90., 0.), vec2f(80., 26.)),
            &rects,
            false
        ),
        2
    );
    assert_eq!(
        drag_target_index(
            0,
            RectF::new(vec2f(20., 0.), vec2f(80., 26.)),
            &rects,
            false
        ),
        0
    );
}

#[test]
fn closing_a_tab_lands_on_a_neighbor_of_the_same_project() {
    // Tabs left after closing index 1: [project A, project B, project A].
    let in_project_a = |index: usize| index == 0 || index == 2;

    // The tab now at the closed index belongs to another project, so the next own tab wins.
    assert_eq!(tab_after_close(1, 3, in_project_a), Some(2));
    // Closing the last tab falls back to the nearest own tab before it.
    assert_eq!(tab_after_close(3, 3, |index| index == 0), Some(0));
    assert_eq!(tab_after_close(1, 3, |_| false), None);
}
