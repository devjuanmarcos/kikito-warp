use std::collections::{HashMap, HashSet};

use anyhow::{Result, bail};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::model::{
    LocalWorkspaceOrganization, LocalWorkspaceProject, LocalWorkspaceProjectTab,
    LocalWorkspaceState, LocalWorkspaceTabIdentity, NewLocalWorkspaceOrganization,
    NewLocalWorkspaceProject, NewLocalWorkspaceProjectTab,
};
use super::schema::{
    local_workspace_organizations, local_workspace_project_tabs, local_workspace_projects,
    local_workspace_state, local_workspace_tab_identities,
};
use super::{LocalWorkspaceData, LocalWorkspaceOperation};

pub(super) fn apply(
    connection: &mut SqliteConnection,
    operation: LocalWorkspaceOperation,
) -> Result<()> {
    match operation {
        LocalWorkspaceOperation::CreateOrganization {
            id,
            name,
            sort_order,
            accent_color,
            image_data,
        } => connection.transaction(|connection| {
            validate_organization_visuals(&accent_color, image_data.as_deref())?;
            let next_sort_order = local_workspace_organizations::table
                .select(diesel::dsl::max(local_workspace_organizations::sort_order))
                .first::<Option<i32>>(connection)?
                .map(|last_order| {
                    last_order
                        .checked_add(1)
                        .ok_or_else(|| anyhow::anyhow!("organization order is exhausted"))
                })
                .transpose()?
                .unwrap_or(sort_order)
                .max(sort_order);
            diesel::insert_into(local_workspace_organizations::table)
                .values(NewLocalWorkspaceOrganization {
                    id,
                    name,
                    sort_order: next_sort_order,
                    accent_color,
                    image_data,
                })
                .execute(connection)?;
            let selected = local_workspace_state::table
                .find(1)
                .select(local_workspace_state::selected_organization_id)
                .first::<Option<i32>>(connection)?;
            if selected.is_none() {
                diesel::update(local_workspace_state::table.find(1))
                    .set(local_workspace_state::selected_organization_id.eq(Some(id)))
                    .execute(connection)?;
            }
            Ok(())
        }),
        LocalWorkspaceOperation::CreateProject {
            id,
            organization_id,
            name,
            path,
            sort_order,
        } => {
            let path_key = crate::local_workspace::model::normalize_path_key(&path)
                .map_err(|error| anyhow::anyhow!("invalid local project path: {error:?}"))?;
            connection.transaction(|connection| {
                let next_sort_order = local_workspace_projects::table
                    .filter(local_workspace_projects::organization_id.eq(organization_id))
                    .select(diesel::dsl::max(local_workspace_projects::sort_order))
                    .first::<Option<i32>>(connection)?
                    .map(|last_order| {
                        last_order
                            .checked_add(1)
                            .ok_or_else(|| anyhow::anyhow!("project order is exhausted"))
                    })
                    .transpose()?
                    .unwrap_or(sort_order)
                    .max(sort_order);
                diesel::insert_into(local_workspace_projects::table)
                    .values(NewLocalWorkspaceProject {
                        id,
                        organization_id,
                        name,
                        path,
                        path_key,
                        sort_order: next_sort_order,
                        last_opened_ts: 0,
                    })
                    .execute(connection)?;
                diesel::update(
                    local_workspace_state::table
                        .filter(local_workspace_state::id.eq(1))
                        .filter(local_workspace_state::selected_organization_id.eq(organization_id))
                        .filter(local_workspace_state::selected_project_id.is_null()),
                )
                .set(local_workspace_state::selected_project_id.eq(Some(id)))
                .execute(connection)?;
                Ok(())
            })
        }
        LocalWorkspaceOperation::RenameOrganization { id, name } => {
            let updated = diesel::update(local_workspace_organizations::table.find(id))
                .set(local_workspace_organizations::name.eq(name))
                .execute(connection)?;
            require_one_row(updated, "organization", id)
        }
        LocalWorkspaceOperation::UpdateOrganization {
            id,
            name,
            accent_color,
            image_data,
        } => {
            validate_organization_visuals(&accent_color, image_data.as_deref())?;
            let updated = diesel::update(local_workspace_organizations::table.find(id))
                .set((
                    local_workspace_organizations::name.eq(name),
                    local_workspace_organizations::accent_color.eq(accent_color),
                    local_workspace_organizations::image_data.eq(image_data),
                ))
                .execute(connection)?;
            require_one_row(updated, "organization", id)
        }
        LocalWorkspaceOperation::SetOrganizationVisuals {
            id,
            accent_color,
            image_data,
        } => {
            validate_organization_visuals(&accent_color, image_data.as_deref())?;
            let updated = diesel::update(local_workspace_organizations::table.find(id))
                .set((
                    local_workspace_organizations::accent_color.eq(accent_color),
                    local_workspace_organizations::image_data.eq(image_data),
                ))
                .execute(connection)?;
            require_one_row(updated, "organization", id)
        }
        LocalWorkspaceOperation::RenameProject { id, name } => {
            let updated = diesel::update(local_workspace_projects::table.find(id))
                .set(local_workspace_projects::name.eq(name))
                .execute(connection)?;
            require_one_row(updated, "project", id)
        }
        LocalWorkspaceOperation::ReorderOrganizations(ids) => {
            connection.transaction(|connection| {
                let current = local_workspace_organizations::table
                    .select(local_workspace_organizations::id)
                    .load::<i32>(connection)?;
                validate_permutation(&current, &ids)?;
                let max_order = local_workspace_organizations::table
                    .select(diesel::dsl::max(local_workspace_organizations::sort_order))
                    .first::<Option<i32>>(connection)?
                    .unwrap_or(-1);
                for (index, id) in ids.iter().enumerate() {
                    let temporary_order = temporary_sort_order(max_order, ids.len(), index)?;
                    diesel::update(local_workspace_organizations::table.find(id))
                        .set(local_workspace_organizations::sort_order.eq(temporary_order))
                        .execute(connection)?;
                }
                for (index, id) in ids.iter().enumerate() {
                    diesel::update(local_workspace_organizations::table.find(id))
                        .set(local_workspace_organizations::sort_order.eq(index as i32))
                        .execute(connection)?;
                }
                Ok(())
            })
        }
        LocalWorkspaceOperation::ReorderProjects {
            organization_id,
            project_ids,
        } => connection.transaction(|connection| {
            let current = local_workspace_projects::table
                .filter(local_workspace_projects::organization_id.eq(organization_id))
                .select(local_workspace_projects::id)
                .load::<i32>(connection)?;
            validate_permutation(&current, &project_ids)?;
            let max_order = local_workspace_projects::table
                .filter(local_workspace_projects::organization_id.eq(organization_id))
                .select(diesel::dsl::max(local_workspace_projects::sort_order))
                .first::<Option<i32>>(connection)?
                .unwrap_or(-1);
            for (index, id) in project_ids.iter().enumerate() {
                let temporary_order = temporary_sort_order(max_order, project_ids.len(), index)?;
                diesel::update(
                    local_workspace_projects::table
                        .filter(local_workspace_projects::id.eq(id))
                        .filter(local_workspace_projects::organization_id.eq(organization_id)),
                )
                .set(local_workspace_projects::sort_order.eq(temporary_order))
                .execute(connection)?;
            }
            for (index, id) in project_ids.iter().enumerate() {
                diesel::update(
                    local_workspace_projects::table
                        .filter(local_workspace_projects::id.eq(id))
                        .filter(local_workspace_projects::organization_id.eq(organization_id)),
                )
                .set(local_workspace_projects::sort_order.eq(index as i32))
                .execute(connection)?;
            }
            Ok(())
        }),
        LocalWorkspaceOperation::AssignTab {
            project_id,
            tab_id,
            sort_order,
        } => connection.transaction(|connection| {
            ensure_tab_identity_exists(connection, tab_id)?;
            let is_active = local_workspace_project_tabs::table
                .filter(local_workspace_project_tabs::project_id.eq(project_id))
                .select(diesel::dsl::count_star())
                .first::<i64>(connection)?
                == 0;
            diesel::insert_into(local_workspace_project_tabs::table)
                .values(NewLocalWorkspaceProjectTab {
                    project_id,
                    tab_id,
                    sort_order,
                    is_active,
                })
                .execute(connection)?;
            Ok(())
        }),
        LocalWorkspaceOperation::ReplaceProjectTabs {
            project_id,
            tabs,
            active_tab_id,
            new_tab_identities,
        } => connection.transaction(|connection| {
            ensure_project_exists(connection, project_id)?;
            for (id, workspace_tab_id) in new_tab_identities {
                claim_tab_identity(connection, id, &workspace_tab_id)?;
            }
            let mut seen = HashSet::new();
            for (tab_id, _) in &tabs {
                if !seen.insert(*tab_id) {
                    bail!("tab {tab_id} appears more than once in project membership");
                }
                ensure_tab_identity_exists(connection, *tab_id)?;
                let other_project = local_workspace_project_tabs::table
                    .filter(local_workspace_project_tabs::tab_id.eq(tab_id))
                    .select(local_workspace_project_tabs::project_id)
                    .first::<i32>(connection)
                    .optional()?;
                if other_project.is_some_and(|owner| owner != project_id) {
                    bail!("tab {tab_id} is already assigned to project {other_project:?}");
                }
            }
            if active_tab_id.is_some_and(|active| !seen.contains(&active)) {
                bail!("active tab must belong to project {project_id}");
            }

            diesel::delete(
                local_workspace_project_tabs::table
                    .filter(local_workspace_project_tabs::project_id.eq(project_id)),
            )
            .execute(connection)?;
            for (tab_id, sort_order) in tabs {
                diesel::insert_into(local_workspace_project_tabs::table)
                    .values(NewLocalWorkspaceProjectTab {
                        project_id,
                        tab_id,
                        sort_order,
                        is_active: active_tab_id == Some(tab_id),
                    })
                    .execute(connection)?;
            }
            Ok(())
        }),
        LocalWorkspaceOperation::SetActiveTab { project_id, tab_id } => {
            connection.transaction(|connection| {
                if let Some(tab_id) = tab_id {
                    let affected = diesel::update(
                        local_workspace_project_tabs::table
                            .filter(local_workspace_project_tabs::project_id.eq(project_id)),
                    )
                    .set(local_workspace_project_tabs::is_active.eq(false))
                    .execute(connection)?;
                    if affected == 0 {
                        ensure_project_exists(connection, project_id)?;
                    }
                    let selected = diesel::update(
                        local_workspace_project_tabs::table
                            .filter(local_workspace_project_tabs::project_id.eq(project_id))
                            .filter(local_workspace_project_tabs::tab_id.eq(tab_id)),
                    )
                    .set(local_workspace_project_tabs::is_active.eq(true))
                    .execute(connection)?;
                    if selected != 1 {
                        bail!("tab {tab_id} is not assigned to project {project_id}");
                    }
                } else {
                    diesel::update(
                        local_workspace_project_tabs::table
                            .filter(local_workspace_project_tabs::project_id.eq(project_id)),
                    )
                    .set(local_workspace_project_tabs::is_active.eq(false))
                    .execute(connection)?;
                    ensure_project_exists(connection, project_id)?;
                }
                Ok(())
            })
        }
        LocalWorkspaceOperation::Select {
            organization_id,
            project_id,
        } => connection.transaction(|connection| {
            match (organization_id, project_id) {
                (None, None) => {}
                (Some(organization_id), None) => {
                    ensure_organization_exists(connection, organization_id)?;
                }
                (Some(organization_id), Some(project_id)) => {
                    let owner = local_workspace_projects::table
                        .find(project_id)
                        .select(local_workspace_projects::organization_id)
                        .first::<i32>(connection)?;
                    if owner != organization_id {
                        bail!(
                            "project {project_id} does not belong to organization {organization_id}"
                        );
                    }
                }
                (None, Some(_)) => bail!("a selected project requires a selected organization"),
            }
            diesel::update(local_workspace_state::table.find(1))
                .set((
                    local_workspace_state::selected_organization_id.eq(organization_id),
                    local_workspace_state::selected_project_id.eq(project_id),
                ))
                .execute(connection)?;
            if let Some(project_id) = project_id {
                let last_timestamp = local_workspace_projects::table
                    .select(diesel::dsl::max(local_workspace_projects::last_opened_ts))
                    .first::<Option<i64>>(connection)?
                    .unwrap_or_default();
                let timestamp = chrono::Utc::now()
                    .timestamp_millis()
                    .max(last_timestamp.saturating_add(1));
                diesel::update(local_workspace_projects::table.find(project_id))
                    .set(local_workspace_projects::last_opened_ts.eq(timestamp))
                    .execute(connection)?;
            }
            Ok(())
        }),
        LocalWorkspaceOperation::DeleteOrganization(id) => connection.transaction(|connection| {
            let selected_organization = local_workspace_state::table
                .find(1)
                .select(local_workspace_state::selected_organization_id)
                .first::<Option<i32>>(connection)?;
            if selected_organization == Some(id) {
                let fallback_organization = local_workspace_organizations::table
                    .filter(local_workspace_organizations::id.ne(id))
                    .order(local_workspace_organizations::sort_order.asc())
                    .select(local_workspace_organizations::id)
                    .first::<i32>(connection)
                    .optional()?;
                let fallback_project = if let Some(organization_id) = fallback_organization {
                    local_workspace_projects::table
                        .filter(local_workspace_projects::organization_id.eq(organization_id))
                        .order(local_workspace_projects::sort_order.asc())
                        .select(local_workspace_projects::id)
                        .first::<i32>(connection)
                        .optional()?
                } else {
                    None
                };
                diesel::update(local_workspace_state::table.find(1))
                    .set((
                        local_workspace_state::selected_organization_id.eq(fallback_organization),
                        local_workspace_state::selected_project_id.eq(fallback_project),
                    ))
                    .execute(connection)?;
            }
            let deleted = diesel::delete(local_workspace_organizations::table.find(id))
                .execute(connection)?;
            if deleted != 1 {
                bail!("organization {id} does not exist");
            }
            Ok(())
        }),
        LocalWorkspaceOperation::DeleteProject(id) => connection.transaction(|connection| {
            let deleted =
                diesel::delete(local_workspace_projects::table.find(id)).execute(connection)?;
            if deleted != 1 {
                bail!("project {id} does not exist");
            }
            Ok(())
        }),
    }
}

fn validate_organization_visuals(accent_color: &str, image_data: Option<&[u8]>) -> Result<()> {
    if !matches!(
        accent_color,
        "blue" | "green" | "magenta" | "yellow" | "cyan" | "red"
    ) {
        bail!("invalid organization accent color");
    }
    if image_data.is_some_and(|bytes| bytes.len() > 128 * 1024) {
        bail!("organization image exceeds the storage limit");
    }
    Ok(())
}

pub(super) fn read(connection: &mut SqliteConnection) -> diesel::QueryResult<LocalWorkspaceData> {
    let organizations = local_workspace_organizations::table
        .order(local_workspace_organizations::sort_order.asc())
        .load::<LocalWorkspaceOrganization>(connection)?;
    let organization_order = organizations
        .iter()
        .enumerate()
        .map(|(index, organization)| (organization.id, index))
        .collect::<HashMap<_, _>>();
    let mut projects = local_workspace_projects::table
        .order((
            local_workspace_projects::organization_id.asc(),
            local_workspace_projects::sort_order.asc(),
        ))
        .load::<LocalWorkspaceProject>(connection)?;
    projects.sort_by_key(|project| {
        (
            organization_order
                .get(&project.organization_id)
                .copied()
                .unwrap_or(usize::MAX),
            project.sort_order,
            project.id,
        )
    });
    Ok(LocalWorkspaceData {
        organizations,
        projects,
        tabs: local_workspace_project_tabs::table
            .order((
                local_workspace_project_tabs::project_id.asc(),
                local_workspace_project_tabs::sort_order.asc(),
            ))
            .load::<LocalWorkspaceProjectTab>(connection)?,
        tab_identities: local_workspace_tab_identities::table
            .order(local_workspace_tab_identities::id.asc())
            .load::<LocalWorkspaceTabIdentity>(connection)?,
        state: local_workspace_state::table
            .find(1)
            .first::<LocalWorkspaceState>(connection)
            .optional()?,
    })
}

fn validate_permutation(current: &[i32], reordered: &[i32]) -> Result<()> {
    if current.len() != reordered.len()
        || current.iter().copied().collect::<HashSet<_>>().len() != current.len()
        || reordered.iter().copied().collect::<HashSet<_>>().len() != reordered.len()
        || current.iter().any(|id| !reordered.contains(id))
    {
        bail!("reorder must contain every current item exactly once");
    }
    Ok(())
}

fn temporary_sort_order(max_order: i32, item_count: usize, index: usize) -> Result<i32> {
    let item_count = i32::try_from(item_count).map_err(|_| anyhow::anyhow!("too many items"))?;
    let index = i32::try_from(index).map_err(|_| anyhow::anyhow!("too many items"))?;
    max_order
        .checked_add(item_count)
        .and_then(|order| order.checked_add(1))
        .and_then(|order| order.checked_add(index))
        .ok_or_else(|| anyhow::anyhow!("sort order overflow"))
}

fn require_one_row(updated: usize, entity: &str, id: i32) -> Result<()> {
    if updated != 1 {
        bail!("{entity} {id} does not exist");
    }
    Ok(())
}

fn ensure_organization_exists(connection: &mut SqliteConnection, id: i32) -> Result<()> {
    let exists = diesel::select(diesel::dsl::exists(
        local_workspace_organizations::table.find(id),
    ))
    .get_result::<bool>(connection)?;
    if !exists {
        bail!("organization {id} does not exist");
    }
    Ok(())
}

/// Gives `workspace_tab_id` the identity row `id` chosen by the workspace model.
///
/// App-state snapshots create identity rows for every open tab with database-assigned ids the
/// model never loaded, so the model's next id can already belong to such a row, and the tab
/// itself may already have one under another id. Those snapshot rows are never part of a
/// project, so they are renumbered instead of rejecting the model's id.
fn claim_tab_identity(
    connection: &mut SqliteConnection,
    id: i32,
    workspace_tab_id: &str,
) -> Result<()> {
    let holder = local_workspace_tab_identities::table
        .find(id)
        .select(local_workspace_tab_identities::workspace_tab_id)
        .first::<String>(connection)
        .optional()?;
    match holder {
        Some(holder) if holder == workspace_tab_id => return Ok(()),
        Some(_) => {
            ensure_identity_unassigned(connection, id)?;
            let free_id = next_free_identity_id(connection)?;
            diesel::update(local_workspace_tab_identities::table.find(id))
                .set(local_workspace_tab_identities::id.eq(free_id))
                .execute(connection)?;
        }
        None => {}
    }
    let existing = local_workspace_tab_identities::table
        .filter(local_workspace_tab_identities::workspace_tab_id.eq(workspace_tab_id))
        .select(local_workspace_tab_identities::id)
        .first::<i32>(connection)
        .optional()?;
    match existing {
        Some(existing) => {
            ensure_identity_unassigned(connection, existing)?;
            diesel::update(local_workspace_tab_identities::table.find(existing))
                .set(local_workspace_tab_identities::id.eq(id))
                .execute(connection)?;
        }
        None => {
            diesel::insert_into(local_workspace_tab_identities::table)
                .values((
                    local_workspace_tab_identities::id.eq(id),
                    local_workspace_tab_identities::workspace_tab_id.eq(workspace_tab_id),
                    local_workspace_tab_identities::session_tab_id.eq(None::<i32>),
                ))
                .execute(connection)?;
        }
    }
    Ok(())
}

fn ensure_identity_unassigned(connection: &mut SqliteConnection, id: i32) -> Result<()> {
    let assigned = diesel::select(diesel::dsl::exists(
        local_workspace_project_tabs::table.filter(local_workspace_project_tabs::tab_id.eq(id)),
    ))
    .get_result::<bool>(connection)?;
    if assigned {
        bail!("tab identity {id} already belongs to a project");
    }
    Ok(())
}

fn next_free_identity_id(connection: &mut SqliteConnection) -> Result<i32> {
    let max = local_workspace_tab_identities::table
        .select(diesel::dsl::max(local_workspace_tab_identities::id))
        .first::<Option<i32>>(connection)?
        .unwrap_or(0);
    Ok(max + 1)
}

/// Fails when `tab_id` has no identity row. An identity whose session tab is still unsaved is
/// accepted: the next app-state snapshot attaches it, and the snapshot drops project memberships
/// of identities that are still detached by then.
fn ensure_tab_identity_exists(connection: &mut SqliteConnection, tab_id: i32) -> Result<()> {
    let exists = diesel::select(diesel::dsl::exists(
        local_workspace_tab_identities::table.find(tab_id),
    ))
    .get_result::<bool>(connection)?;
    if !exists {
        bail!("tab identity {tab_id} does not exist");
    }
    Ok(())
}

fn ensure_project_exists(connection: &mut SqliteConnection, id: i32) -> Result<()> {
    let exists = diesel::select(diesel::dsl::exists(
        local_workspace_projects::table.find(id),
    ))
    .get_result::<bool>(connection)?;
    if !exists {
        bail!("project {id} does not exist");
    }
    Ok(())
}
