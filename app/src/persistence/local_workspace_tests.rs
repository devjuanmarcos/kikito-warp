use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::MigrationHarness;

use super::{LocalWorkspaceOperation as Operation, local_workspace};
use crate::persistence::model::LocalWorkspaceTabIdentity;
use crate::persistence::schema::{
    local_workspace_organizations, local_workspace_project_tabs, local_workspace_projects,
    local_workspace_state, local_workspace_tab_identities, tabs, windows,
};

fn connection() -> SqliteConnection {
    let mut connection = SqliteConnection::establish(":memory:").unwrap();
    connection
        .batch_execute("PRAGMA foreign_keys = ON;")
        .unwrap();
    connection
        .run_pending_migrations(persistence::MIGRATIONS)
        .unwrap();
    connection
}

fn create_organization(connection: &mut SqliteConnection, id: i32, name: &str) {
    local_workspace::apply(
        connection,
        Operation::CreateOrganization {
            id,
            name: name.to_string(),
            sort_order: id - 1,
        },
    )
    .unwrap();
}

fn create_project(connection: &mut SqliteConnection, id: i32, organization_id: i32, path: &str) {
    local_workspace::apply(
        connection,
        Operation::CreateProject {
            id,
            organization_id,
            name: format!("Projeto {id}"),
            path: path.to_string(),
            sort_order: id - 1,
        },
    )
    .unwrap();
}

fn seed_session_tab(connection: &mut SqliteConnection, tab_id: i32) -> i32 {
    diesel::insert_into(windows::table)
        .values((windows::id.eq(tab_id), windows::active_tab_index.eq(0)))
        .execute(connection)
        .unwrap();
    diesel::insert_into(tabs::table)
        .values((tabs::id.eq(tab_id), tabs::window_id.eq(tab_id)))
        .execute(connection)
        .unwrap();
    diesel::insert_into(local_workspace_tab_identities::table)
        .values((
            local_workspace_tab_identities::workspace_tab_id.eq(format!("stable-tab-{tab_id}")),
            local_workspace_tab_identities::session_tab_id.eq(Some(tab_id)),
        ))
        .execute(connection)
        .unwrap();
    local_workspace_tab_identities::table
        .filter(local_workspace_tab_identities::session_tab_id.eq(Some(tab_id)))
        .select(local_workspace_tab_identities::id)
        .first(connection)
        .unwrap()
}

#[test]
fn duplicate_normalized_paths_and_orphan_tabs_are_rejected() {
    let mut connection = connection();
    create_organization(&mut connection, 1, "Equipe");
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("Projeto");
    let display_path = path.display().to_string();
    create_project(&mut connection, 1, 1, &display_path);

    let duplicate_path = format!("{}{}.", display_path, std::path::MAIN_SEPARATOR);
    let duplicate = local_workspace::apply(
        &mut connection,
        Operation::CreateProject {
            id: 2,
            organization_id: 1,
            name: "Duplicado".to_string(),
            path: duplicate_path,
            sort_order: 1,
        },
    );
    assert!(duplicate.is_err());
    assert_eq!(
        local_workspace::read(&mut connection)
            .unwrap()
            .projects
            .len(),
        1
    );

    let orphan = local_workspace::apply(
        &mut connection,
        Operation::AssignTab {
            project_id: 1,
            tab_id: 999,
            sort_order: 0,
        },
    );
    assert!(orphan.is_err());
    assert_eq!(
        local_workspace::read(&mut connection).unwrap().tabs.len(),
        0
    );
}

#[test]
fn local_workspace_selection_order_and_tab_membership_restore_from_sqlite() {
    let mut connection = connection();
    create_organization(&mut connection, 1, "Equipe A");
    create_organization(&mut connection, 2, "Equipe B");
    let root = tempfile::tempdir().unwrap();
    create_project(
        &mut connection,
        1,
        1,
        &root.path().join("Projeto A").display().to_string(),
    );
    create_project(
        &mut connection,
        2,
        2,
        &root.path().join("Projeto B").display().to_string(),
    );
    local_workspace::apply(&mut connection, Operation::ReorderOrganizations(vec![2, 1])).unwrap();
    local_workspace::apply(
        &mut connection,
        Operation::ReorderProjects {
            organization_id: 2,
            project_ids: vec![2],
        },
    )
    .unwrap();
    local_workspace::apply(
        &mut connection,
        Operation::RenameOrganization {
            id: 2,
            name: "Equipe Renomeada".to_string(),
        },
    )
    .unwrap();
    local_workspace::apply(
        &mut connection,
        Operation::RenameProject {
            id: 2,
            name: "Projeto Renomeado".to_string(),
        },
    )
    .unwrap();

    let workspace_tab_id = seed_session_tab(&mut connection, 72);
    local_workspace::apply(
        &mut connection,
        Operation::AssignTab {
            project_id: 2,
            tab_id: workspace_tab_id,
            sort_order: 0,
        },
    )
    .unwrap();
    local_workspace::apply(
        &mut connection,
        Operation::SetActiveTab {
            project_id: 2,
            tab_id: Some(workspace_tab_id),
        },
    )
    .unwrap();
    local_workspace::apply(
        &mut connection,
        Operation::Select {
            organization_id: Some(2),
            project_id: Some(2),
        },
    )
    .unwrap();

    let restored = local_workspace::read(&mut connection).unwrap();
    assert_eq!(restored.organizations[0].id, 2);
    assert_eq!(restored.organizations[0].name, "Equipe Renomeada");
    assert_eq!(restored.projects[0].id, 2);
    assert_eq!(restored.projects[0].name, "Projeto Renomeado");
    assert_eq!(restored.tabs[0].tab_id, workspace_tab_id);
    assert!(restored.tabs[0].is_active);
    assert_eq!(restored.tab_identities[0].workspace_tab_id, "stable-tab-72");
    assert_eq!(restored.tab_identities[0].id, workspace_tab_id);
    let state = restored.state.unwrap();
    assert_eq!(state.selected_organization_id, Some(2));
    assert_eq!(state.selected_project_id, Some(2));
}

#[test]
fn reorder_rejects_sort_orders_that_cannot_be_safely_offset() {
    let mut connection = connection();
    create_organization(&mut connection, 1, "Equipe A");
    create_organization(&mut connection, 2, "Equipe B");
    diesel::update(local_workspace_organizations::table.find(2))
        .set(local_workspace_organizations::sort_order.eq(i32::MAX - 3))
        .execute(&mut connection)
        .unwrap();

    let result =
        local_workspace::apply(&mut connection, Operation::ReorderOrganizations(vec![2, 1]));

    assert!(result.is_err());
    let orders = local_workspace_organizations::table
        .order(local_workspace_organizations::id.asc())
        .select(local_workspace_organizations::sort_order)
        .load::<i32>(&mut connection)
        .unwrap();
    assert_eq!(orders, vec![0, i32::MAX - 3]);
}

#[test]
fn failed_selection_rolls_back_and_metadata_delete_preserves_session_tab() {
    let mut connection = connection();
    create_organization(&mut connection, 1, "Equipe");
    let root = tempfile::tempdir().unwrap();
    create_project(
        &mut connection,
        1,
        1,
        &root.path().join("Projeto").display().to_string(),
    );
    let identity_id = seed_session_tab(&mut connection, 82);
    local_workspace::apply(
        &mut connection,
        Operation::AssignTab {
            project_id: 1,
            tab_id: identity_id,
            sort_order: 0,
        },
    )
    .unwrap();
    local_workspace::apply(
        &mut connection,
        Operation::Select {
            organization_id: Some(1),
            project_id: Some(1),
        },
    )
    .unwrap();

    assert!(
        local_workspace::apply(
            &mut connection,
            Operation::Select {
                organization_id: Some(1),
                project_id: Some(999),
            },
        )
        .is_err()
    );
    assert_eq!(
        local_workspace::read(&mut connection)
            .unwrap()
            .state
            .unwrap()
            .selected_project_id,
        Some(1)
    );

    local_workspace::apply(&mut connection, Operation::DeleteOrganization(1)).unwrap();
    assert_eq!(
        tabs::table
            .select(diesel::dsl::count_star())
            .first::<i64>(&mut connection)
            .unwrap(),
        1
    );
    assert_eq!(
        local_workspace_tab_identities::table
            .find(identity_id)
            .first::<LocalWorkspaceTabIdentity>(&mut connection)
            .unwrap()
            .session_tab_id,
        Some(82)
    );
    assert_eq!(
        local_workspace_project_tabs::table
            .select(diesel::dsl::count_star())
            .first::<i64>(&mut connection)
            .unwrap(),
        0
    );
    assert_eq!(
        local_workspace_projects::table
            .select(diesel::dsl::count_star())
            .first::<i64>(&mut connection)
            .unwrap(),
        0
    );
    assert_eq!(
        local_workspace_organizations::table
            .select(diesel::dsl::count_star())
            .first::<i64>(&mut connection)
            .unwrap(),
        0
    );
    assert_eq!(
        local_workspace_state::table
            .find(1)
            .select(local_workspace_state::selected_organization_id)
            .first::<Option<i32>>(&mut connection)
            .unwrap(),
        None
    );
}
