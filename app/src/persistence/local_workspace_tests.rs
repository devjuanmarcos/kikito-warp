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
            accent_color: "blue".to_string(),
            image_data: None,
        },
    )
    .unwrap();
}

#[test]
fn organization_visual_metadata_survives_reopening_the_database() {
    let root = tempfile::tempdir().unwrap();
    let database_path = root.path().join("workspace.sqlite");
    let mut connection = SqliteConnection::establish(database_path.to_str().unwrap()).unwrap();
    connection
        .batch_execute("PRAGMA foreign_keys = ON;")
        .unwrap();
    connection
        .run_pending_migrations(persistence::MIGRATIONS)
        .unwrap();
    create_organization(&mut connection, 1, "Equipe");
    local_workspace::apply(
        &mut connection,
        Operation::SetOrganizationVisuals {
            id: 1,
            accent_color: "cyan".to_string(),
            image_data: Some(vec![1, 2, 3, 4]),
        },
    )
    .unwrap();
    drop(connection);

    let mut reopened = SqliteConnection::establish(database_path.to_str().unwrap()).unwrap();
    reopened.batch_execute("PRAGMA foreign_keys = ON;").unwrap();
    reopened
        .run_pending_migrations(persistence::MIGRATIONS)
        .unwrap();
    let organization = local_workspace::read(&mut reopened)
        .unwrap()
        .organizations
        .remove(0);
    assert_eq!(organization.accent_color, "cyan");
    assert_eq!(organization.image_data, Some(vec![1, 2, 3, 4]));
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
        Operation::SetOrganizationVisuals {
            id: 1,
            accent_color: "magenta".to_string(),
            image_data: Some(vec![137, 80, 78, 71]),
        },
    )
    .unwrap();
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
fn project_tab_membership_and_active_tab_update_atomically() {
    let mut connection = connection();
    create_organization(&mut connection, 1, "Equipe");
    let root = tempfile::tempdir().unwrap();
    create_project(
        &mut connection,
        1,
        1,
        &root.path().join("Projeto").display().to_string(),
    );
    let first_tab = seed_session_tab(&mut connection, 91);
    let second_tab = seed_session_tab(&mut connection, 92);

    local_workspace::apply(
        &mut connection,
        Operation::ReplaceProjectTabs {
            project_id: 1,
            tabs: vec![(first_tab, 0), (second_tab, 1)],
            active_tab_id: Some(second_tab),
            new_tab_identities: Vec::new(),
        },
    )
    .unwrap();

    let restored = local_workspace::read(&mut connection).unwrap();
    assert_eq!(
        restored
            .tabs
            .iter()
            .map(|tab| (tab.tab_id, tab.is_active))
            .collect::<Vec<_>>(),
        vec![(first_tab, false), (second_tab, true)]
    );

    local_workspace::apply(
        &mut connection,
        Operation::ReplaceProjectTabs {
            project_id: 1,
            tabs: vec![(first_tab, 0)],
            active_tab_id: Some(first_tab),
            new_tab_identities: Vec::new(),
        },
    )
    .unwrap();

    let restored = local_workspace::read(&mut connection).unwrap();
    assert_eq!(restored.tabs.len(), 1);
    assert_eq!(restored.tabs[0].tab_id, first_tab);
    assert!(restored.tabs[0].is_active);

    let invalid_update = local_workspace::apply(
        &mut connection,
        Operation::ReplaceProjectTabs {
            project_id: 1,
            tabs: vec![(first_tab, 0)],
            active_tab_id: Some(second_tab),
            new_tab_identities: Vec::new(),
        },
    );
    assert!(invalid_update.is_err());
    let restored = local_workspace::read(&mut connection).unwrap();
    assert_eq!(restored.tabs.len(), 1);
    assert_eq!(restored.tabs[0].tab_id, first_tab);
    assert!(restored.tabs[0].is_active);
}

#[test]
fn restarted_workspace_restores_selected_project_active_tab_and_identity() {
    let root = tempfile::tempdir().unwrap();
    let database_path = root.path().join("workspace.sqlite");
    let mut connection = SqliteConnection::establish(database_path.to_str().unwrap()).unwrap();
    connection
        .batch_execute("PRAGMA foreign_keys = ON;")
        .unwrap();
    connection
        .run_pending_migrations(persistence::MIGRATIONS)
        .unwrap();
    create_organization(&mut connection, 1, "Equipe");
    create_project(&mut connection, 1, 1, &root.path().display().to_string());
    let first_tab = seed_session_tab(&mut connection, 121);
    let second_tab = seed_session_tab(&mut connection, 122);
    local_workspace::apply(
        &mut connection,
        Operation::ReplaceProjectTabs {
            project_id: 1,
            tabs: vec![(first_tab, 0), (second_tab, 1)],
            active_tab_id: Some(second_tab),
            new_tab_identities: Vec::new(),
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
    drop(connection);

    let mut reopened = SqliteConnection::establish(database_path.to_str().unwrap()).unwrap();
    reopened.batch_execute("PRAGMA foreign_keys = ON;").unwrap();
    let model = crate::local_workspace::model::WorkspaceModel::from_persisted(
        &local_workspace::read(&mut reopened).unwrap(),
    )
    .unwrap();
    assert_eq!(model.selected_project(), Some(1));
    assert_eq!(model.project(1).unwrap().active_tab, Some(second_tab));
    assert_eq!(model.workspace_tab_id(second_tab), Some("stable-tab-122"));
}

#[test]
fn deleting_selected_organization_persists_remaining_selection() {
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
    local_workspace::apply(
        &mut connection,
        Operation::Select {
            organization_id: Some(1),
            project_id: Some(1),
        },
    )
    .unwrap();

    local_workspace::apply(&mut connection, Operation::DeleteOrganization(1)).unwrap();

    let restored = local_workspace::read(&mut connection).unwrap();
    let state = restored.state.unwrap();
    assert_eq!(state.selected_organization_id, Some(2));
    assert_eq!(state.selected_project_id, Some(2));
}

#[test]
fn project_recency_is_persisted_and_monotonically_advanced() {
    let mut connection = connection();
    create_organization(&mut connection, 1, "Equipe");
    let root = tempfile::tempdir().unwrap();
    create_project(
        &mut connection,
        1,
        1,
        &root.path().join("A").display().to_string(),
    );
    create_project(
        &mut connection,
        2,
        1,
        &root.path().join("B").display().to_string(),
    );
    for project_id in [2, 1] {
        local_workspace::apply(
            &mut connection,
            Operation::Select {
                organization_id: Some(1),
                project_id: Some(project_id),
            },
        )
        .unwrap();
    }

    let projects = local_workspace::read(&mut connection).unwrap().projects;
    assert!(projects[0].last_opened_ts > projects[1].last_opened_ts);
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
    let restored_organization = local_workspace::read(&mut connection)
        .unwrap()
        .organizations
        .into_iter()
        .next()
        .unwrap();
    assert_eq!(restored_organization.accent_color, "magenta");
    assert_eq!(
        restored_organization.image_data,
        Some(vec![137, 80, 78, 71])
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
