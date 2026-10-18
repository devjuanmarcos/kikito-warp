use super::{
    TabActivationRoute, can_activate_tab_in_selected_project, preferred_open_project_tab,
    tab_activation_route,
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
    let unassigned_tab = model.ensure_tab_identity("unassigned-tab").unwrap();
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
    let unassigned_tab = model.ensure_tab_identity("unassigned-tab").unwrap();
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
