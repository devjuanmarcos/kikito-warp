use super::{
    TabActivationRoute, branch_from_head, can_activate_tab_in_selected_project,
    keycaps_from_display, preferred_open_project_tab, should_render_empty_organization_state,
    should_render_project_sidebar, tab_activation_route,
};
use crate::local_workspace::model::WorkspaceModel;

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
fn tab_activation_routes_to_its_project_or_home() {
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
        TabActivationRoute::Home
    );
    model.select_organization(organization).unwrap();
    assert_eq!(
        tab_activation_route(&model, "unassigned-tab"),
        TabActivationRoute::Home
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
