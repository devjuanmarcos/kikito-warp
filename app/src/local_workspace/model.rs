use std::collections::{BTreeMap, HashMap};

use warpui::{Entity, SingletonEntity};

use crate::persistence::LocalWorkspaceData;

pub type OrganizationId = u64;
pub type ProjectId = u64;
pub type TabId = i32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectPathStatus {
    Checking,
    Available,
    Missing,
    NotDirectory,
    Inaccessible,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextMenuTarget {
    Organization(OrganizationId),
    Project(ProjectId),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Organization {
    pub id: OrganizationId,
    pub name: String,
    pub projects: Vec<ProjectId>,
    pub accent_color: String,
    pub image_data: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalProject {
    pub id: ProjectId,
    pub organization_id: OrganizationId,
    pub name: String,
    pub path: String,
    pub path_key: String,
    pub last_opened_ts: i64,
    pub tabs: Vec<TabId>,
    pub active_tab: Option<TabId>,
}

impl LocalProject {
    pub fn new(name: impl Into<String>, path: impl AsRef<str>) -> Result<Self, ModelError> {
        let path = path.as_ref();
        let path_key = normalize_path_key(path)?;
        Ok(Self {
            id: 0,
            organization_id: 0,
            name: name.into(),
            path: path.to_owned(),
            path_key,
            last_opened_ts: 0,
            tabs: Vec::new(),
            active_tab: None,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModelError {
    EmptyPath,
    RelativePath,
    InvalidPath,
    InvalidId,
    InvalidAccentColor,
    InvalidImageData,
    DuplicatePath,
    IdExhausted,
    MissingOrganization,
    MissingProject,
    TabAlreadyAssigned,
    TabNotAssigned,
    InvalidTabOrder,
}

#[derive(Default)]
pub struct WorkspaceModel {
    organizations: BTreeMap<OrganizationId, Organization>,
    organization_order: Vec<OrganizationId>,
    projects: BTreeMap<ProjectId, LocalProject>,
    path_index: HashMap<String, ProjectId>,
    tab_index: HashMap<TabId, ProjectId>,
    tab_identity_by_id: HashMap<TabId, String>,
    unpersisted_tab_identities: HashMap<TabId, String>,
    selected_organization: Option<OrganizationId>,
    selected_project: Option<ProjectId>,
    context_menu: Option<ContextMenuTarget>,
    context_menu_index: usize,
    context_menu_keyboard_active: bool,
    hovered_organization: Option<OrganizationId>,
    hovered_project: Option<ProjectId>,
    hovered_tab: Option<TabId>,
    status_error: Option<String>,
    project_path_statuses: HashMap<ProjectId, ProjectPathStatus>,
    /// Checked-out branch of each project folder that is a Git work tree.
    project_branches: HashMap<ProjectId, String>,
    next_organization_id: OrganizationId,
    next_project_id: ProjectId,
}

impl WorkspaceModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_persisted(data: &LocalWorkspaceData) -> Result<Self, ModelError> {
        let mut model = Self::default();
        for saved in &data.organizations {
            if saved.id <= 0 || model.organizations.contains_key(&(saved.id as u64)) {
                return Err(ModelError::InvalidId);
            }
            let id = OrganizationId::try_from(saved.id).map_err(|_| ModelError::InvalidId)?;
            model.organizations.insert(
                id,
                Organization {
                    id,
                    name: saved.name.clone(),
                    projects: Vec::new(),
                    accent_color: saved.accent_color.clone(),
                    image_data: saved.image_data.clone(),
                },
            );
            model.organization_order.push(id);
            model.next_organization_id = model.next_organization_id.max(id);
        }

        for saved in &data.projects {
            if saved.id <= 0 || model.projects.contains_key(&(saved.id as u64)) {
                return Err(ModelError::InvalidId);
            }
            let id = ProjectId::try_from(saved.id).map_err(|_| ModelError::InvalidId)?;
            let organization_id = OrganizationId::try_from(saved.organization_id)
                .map_err(|_| ModelError::InvalidId)?;
            let mut project = LocalProject::new(saved.name.clone(), &saved.path)?;
            if !model.organizations.contains_key(&organization_id) {
                return Err(ModelError::MissingOrganization);
            }
            if model.path_index.contains_key(&project.path_key) {
                return Err(ModelError::DuplicatePath);
            }
            project.id = id;
            project.organization_id = organization_id;
            project.last_opened_ts = saved.last_opened_ts;
            model
                .organizations
                .get_mut(&organization_id)
                .expect("organization was checked above")
                .projects
                .push(id);
            model.path_index.insert(project.path_key.clone(), id);
            model.projects.insert(id, project);
            model
                .project_path_statuses
                .insert(id, ProjectPathStatus::Checking);
            model.next_project_id = model.next_project_id.max(id);
        }

        for saved in &data.tabs {
            if saved.project_id <= 0 || saved.tab_id <= 0 {
                return Err(ModelError::InvalidId);
            }
            let project_id =
                ProjectId::try_from(saved.project_id).map_err(|_| ModelError::InvalidId)?;
            let tab_id = saved.tab_id;
            let project = model
                .projects
                .get_mut(&project_id)
                .ok_or(ModelError::MissingProject)?;
            if model.tab_index.insert(tab_id, project_id).is_some() {
                return Err(ModelError::TabAlreadyAssigned);
            }
            project.tabs.push(tab_id);
            if saved.is_active {
                project.active_tab = Some(tab_id);
            }
        }

        for identity in &data.tab_identities {
            if identity.id <= 0 || model.tab_identity_by_id.contains_key(&identity.id) {
                return Err(ModelError::InvalidId);
            }
            model
                .tab_identity_by_id
                .insert(identity.id, identity.workspace_tab_id.clone());
        }

        if let Some(state) = &data.state {
            let organization_id = state
                .selected_organization_id
                .map(OrganizationId::try_from)
                .transpose()
                .map_err(|_| ModelError::InvalidId)?;
            let project_id = state
                .selected_project_id
                .map(ProjectId::try_from)
                .transpose()
                .map_err(|_| ModelError::InvalidId)?;
            if organization_id.is_some_and(|id| !model.organizations.contains_key(&id)) {
                return Err(ModelError::MissingOrganization);
            }
            if project_id.is_some_and(|id| {
                model
                    .projects
                    .get(&id)
                    .map(|project| project.organization_id)
                    != organization_id
            }) {
                return Err(ModelError::MissingProject);
            }
            model.selected_organization = organization_id;
            model.selected_project = project_id;
        }
        Ok(model)
    }

    pub fn organizations(&self) -> impl Iterator<Item = &Organization> {
        self.organization_order
            .iter()
            .filter_map(|id| self.organizations.get(id))
    }

    pub fn projects(&self) -> impl Iterator<Item = &LocalProject> {
        self.projects.values()
    }

    pub fn recent_projects(&self) -> impl Iterator<Item = &LocalProject> {
        let mut projects = self.projects.values().collect::<Vec<_>>();
        projects.sort_by(|left, right| {
            right
                .last_opened_ts
                .cmp(&left.last_opened_ts)
                .then_with(|| left.id.cmp(&right.id))
        });
        projects.into_iter()
    }

    pub fn projects_for_organization(
        &self,
        organization_id: OrganizationId,
    ) -> impl Iterator<Item = &LocalProject> {
        self.organizations
            .get(&organization_id)
            .into_iter()
            .flat_map(|organization| organization.projects.iter())
            .filter_map(|id| self.projects.get(id))
    }

    pub fn project_path_status(&self, project_id: ProjectId) -> ProjectPathStatus {
        self.project_path_statuses
            .get(&project_id)
            .copied()
            .unwrap_or(ProjectPathStatus::Checking)
    }

    pub fn all_project_paths(&self) -> Vec<(ProjectId, String)> {
        self.projects
            .values()
            .map(|project| (project.id, project.path.clone()))
            .collect()
    }

    pub fn project_branch(&self, project_id: ProjectId) -> Option<&str> {
        self.project_branches.get(&project_id).map(String::as_str)
    }

    pub fn set_project_branch(&mut self, project_id: ProjectId, branch: Option<String>) {
        match branch {
            Some(branch) if self.projects.contains_key(&project_id) => {
                self.project_branches.insert(project_id, branch);
            }
            _ => {
                self.project_branches.remove(&project_id);
            }
        }
    }

    pub fn set_project_path_status(&mut self, project_id: ProjectId, status: ProjectPathStatus) {
        if self.projects.contains_key(&project_id) {
            self.project_path_statuses.insert(project_id, status);
        }
    }

    pub fn select_organization(&mut self, id: OrganizationId) -> Result<(), ModelError> {
        if !self.organizations.contains_key(&id) {
            return Err(ModelError::MissingOrganization);
        }
        self.selected_organization = Some(id);
        self.selected_project = None;
        Ok(())
    }

    pub fn select_home(&mut self) {
        self.selected_organization = None;
        self.selected_project = None;
    }

    pub fn rename_organization(
        &mut self,
        id: OrganizationId,
        name: String,
    ) -> Result<(), ModelError> {
        self.organizations
            .get_mut(&id)
            .ok_or(ModelError::MissingOrganization)?
            .name = name;
        Ok(())
    }

    pub fn set_organization_visuals(
        &mut self,
        id: OrganizationId,
        accent_color: String,
        image_data: Option<Vec<u8>>,
    ) -> Result<(), ModelError> {
        if !matches!(
            accent_color.as_str(),
            "blue" | "green" | "magenta" | "yellow" | "cyan" | "red"
        ) {
            return Err(ModelError::InvalidAccentColor);
        }
        if image_data
            .as_ref()
            .is_some_and(|bytes| bytes.len() > 128 * 1024)
        {
            return Err(ModelError::InvalidImageData);
        }
        let organization = self
            .organizations
            .get_mut(&id)
            .ok_or(ModelError::MissingOrganization)?;
        organization.accent_color = accent_color;
        organization.image_data = image_data;
        Ok(())
    }

    pub fn rename_project(&mut self, id: ProjectId, name: String) -> Result<(), ModelError> {
        self.projects
            .get_mut(&id)
            .ok_or(ModelError::MissingProject)?
            .name = name;
        Ok(())
    }

    pub fn remove_project(&mut self, id: ProjectId) -> Result<(), ModelError> {
        let project = self
            .projects
            .remove(&id)
            .ok_or(ModelError::MissingProject)?;
        self.project_path_statuses.remove(&id);
        self.project_branches.remove(&id);
        self.path_index.remove(&project.path_key);
        for tab_id in project.tabs {
            self.tab_index.remove(&tab_id);
        }
        if let Some(organization) = self.organizations.get_mut(&project.organization_id) {
            organization.projects.retain(|project_id| *project_id != id);
        }
        if self.selected_project == Some(id) {
            self.selected_project = None;
        }
        if self.hovered_project == Some(id) {
            self.hovered_project = None;
        }
        Ok(())
    }

    pub fn next_organization_id(&self) -> Result<OrganizationId, ModelError> {
        self.next_organization_id
            .checked_add(1)
            .ok_or(ModelError::IdExhausted)
    }

    pub fn next_project_id(&self) -> Result<ProjectId, ModelError> {
        self.next_project_id
            .checked_add(1)
            .ok_or(ModelError::IdExhausted)
    }

    pub fn add_organization(
        &mut self,
        name: impl Into<String>,
    ) -> Result<OrganizationId, ModelError> {
        let id = self
            .next_organization_id
            .checked_add(1)
            .ok_or(ModelError::IdExhausted)?;
        self.next_organization_id = id;
        self.organizations.insert(
            id,
            Organization {
                id,
                name: name.into(),
                projects: Vec::new(),
                accent_color: "blue".to_owned(),
                image_data: None,
            },
        );
        self.organization_order.push(id);
        self.selected_organization.get_or_insert(id);
        Ok(id)
    }

    pub fn add_project(
        &mut self,
        organization_id: OrganizationId,
        name: impl Into<String>,
        path: impl AsRef<str>,
    ) -> Result<ProjectId, ModelError> {
        let mut project = LocalProject::new(name, path)?;
        if !self.organizations.contains_key(&organization_id) {
            return Err(ModelError::MissingOrganization);
        }
        if self.path_index.contains_key(&project.path_key) {
            return Err(ModelError::DuplicatePath);
        }

        let id = self
            .next_project_id
            .checked_add(1)
            .ok_or(ModelError::IdExhausted)?;
        self.next_project_id = id;
        project.id = id;
        project.organization_id = organization_id;
        self.organizations
            .get_mut(&organization_id)
            .expect("organization was checked above")
            .projects
            .push(id);
        self.path_index.insert(project.path_key.clone(), id);
        self.projects.insert(id, project);
        self.project_path_statuses
            .insert(id, ProjectPathStatus::Checking);
        if self.selected_organization == Some(organization_id) {
            self.selected_project.get_or_insert(id);
        }
        Ok(id)
    }

    pub fn organization(&self, id: OrganizationId) -> Option<&Organization> {
        self.organizations.get(&id)
    }

    pub fn project(&self, id: ProjectId) -> Option<&LocalProject> {
        self.projects.get(&id)
    }

    pub fn project_count(&self) -> usize {
        self.projects.len()
    }

    pub fn selected_organization(&self) -> Option<OrganizationId> {
        self.selected_organization
    }

    pub fn selected_project(&self) -> Option<ProjectId> {
        self.selected_project
    }

    pub fn context_menu(&self) -> Option<ContextMenuTarget> {
        self.context_menu
    }

    pub fn status_error(&self) -> Option<&str> {
        self.status_error.as_deref()
    }

    pub fn hovered_organization(&self) -> Option<OrganizationId> {
        self.hovered_organization
    }

    pub fn set_hovered_organization(&mut self, id: Option<OrganizationId>) {
        self.hovered_organization = id.filter(|id| self.organizations.contains_key(id));
    }

    pub fn hovered_project(&self) -> Option<ProjectId> {
        self.hovered_project
    }

    pub fn set_hovered_project(&mut self, id: Option<ProjectId>) {
        self.hovered_project = id.filter(|id| self.projects.contains_key(id));
    }

    pub fn hovered_tab(&self) -> Option<TabId> {
        self.hovered_tab
    }

    pub fn set_hovered_tab(&mut self, id: Option<TabId>) {
        self.hovered_tab = id.filter(|id| self.tab_index.contains_key(id));
    }

    pub fn set_status_error(&mut self, error: Option<String>) {
        self.status_error = error;
    }

    pub fn set_context_menu(&mut self, target: Option<ContextMenuTarget>) {
        self.context_menu = target;
        self.context_menu_index = 0;
        self.context_menu_keyboard_active = false;
    }

    pub fn context_menu_index(&self) -> usize {
        self.context_menu_index
    }

    pub fn context_menu_keyboard_active(&self) -> bool {
        self.context_menu_keyboard_active
    }

    pub fn move_context_menu_selection(&mut self, delta: i32) {
        let Some(target) = self.context_menu else {
            return;
        };
        let item_count = match target {
            ContextMenuTarget::Organization(_) => 2,
            ContextMenuTarget::Project(_) => 3,
        };
        self.context_menu_keyboard_active = true;
        self.context_menu_index =
            (self.context_menu_index as i32 + delta).rem_euclid(item_count as i32) as usize;
    }

    pub fn assign_tab(&mut self, project_id: ProjectId, tab_id: TabId) -> Result<(), ModelError> {
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or(ModelError::MissingProject)?;
        if self.tab_index.contains_key(&tab_id) {
            return Err(ModelError::TabAlreadyAssigned);
        }
        project.tabs.push(tab_id);
        project.active_tab.get_or_insert(tab_id);
        self.tab_index.insert(tab_id, project_id);
        Ok(())
    }

    pub fn set_active_tab(
        &mut self,
        project_id: ProjectId,
        tab_id: Option<TabId>,
    ) -> Result<(), ModelError> {
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or(ModelError::MissingProject)?;
        if tab_id.is_some_and(|tab| !project.tabs.contains(&tab)) {
            return Err(ModelError::TabNotAssigned);
        }
        project.active_tab = tab_id;
        Ok(())
    }

    pub fn reorder_project_tabs(
        &mut self,
        project_id: ProjectId,
        tab_ids: &[TabId],
    ) -> Result<(), ModelError> {
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or(ModelError::MissingProject)?;
        if tab_ids.len() != project.tabs.len()
            || tab_ids
                .iter()
                .copied()
                .collect::<std::collections::HashSet<_>>()
                != project.tabs.iter().copied().collect()
        {
            return Err(ModelError::InvalidTabOrder);
        }
        project.tabs = tab_ids.to_vec();
        Ok(())
    }

    pub fn replace_project_tabs(
        &mut self,
        project_id: ProjectId,
        tab_ids: &[TabId],
        active_tab_id: Option<TabId>,
    ) -> Result<(), ModelError> {
        if tab_ids
            .iter()
            .copied()
            .collect::<std::collections::HashSet<_>>()
            .len()
            != tab_ids.len()
            || active_tab_id.is_some_and(|active| !tab_ids.contains(&active))
        {
            return Err(ModelError::InvalidTabOrder);
        }
        if tab_ids.iter().any(|tab_id| {
            self.tab_index
                .get(tab_id)
                .is_some_and(|owner| *owner != project_id)
        }) {
            return Err(ModelError::TabAlreadyAssigned);
        }
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or(ModelError::MissingProject)?;
        for tab_id in &project.tabs {
            self.tab_index.remove(tab_id);
        }
        project.tabs = tab_ids.to_vec();
        project.active_tab = active_tab_id;
        self.tab_index
            .extend(tab_ids.iter().map(|tab_id| (*tab_id, project_id)));
        Ok(())
    }

    pub fn remove_tab_from_project(
        &mut self,
        project_id: ProjectId,
        tab_id: TabId,
    ) -> Result<Option<TabId>, ModelError> {
        let project = self
            .projects
            .get_mut(&project_id)
            .ok_or(ModelError::MissingProject)?;
        let index = project
            .tabs
            .iter()
            .position(|candidate| *candidate == tab_id)
            .ok_or(ModelError::TabNotAssigned)?;
        project.tabs.remove(index);
        if project.active_tab == Some(tab_id) {
            project.active_tab = project
                .tabs
                .get(index.min(project.tabs.len().saturating_sub(1)))
                .copied();
        }
        self.tab_index.remove(&tab_id);
        Ok(project.active_tab)
    }

    pub fn select_project(&mut self, project_id: ProjectId) -> Result<(), ModelError> {
        let project = self
            .projects
            .get(&project_id)
            .ok_or(ModelError::MissingProject)?;
        self.selected_organization = Some(project.organization_id);
        self.selected_project = Some(project_id);
        let newest = self
            .projects
            .values()
            .map(|project| project.last_opened_ts)
            .max()
            .unwrap_or_default();
        let timestamp = chrono::Utc::now()
            .timestamp_millis()
            .max(newest.saturating_add(1));
        self.projects
            .get_mut(&project_id)
            .expect("project was checked above")
            .last_opened_ts = timestamp;
        Ok(())
    }

    pub fn remove_organization(
        &mut self,
        organization_id: OrganizationId,
    ) -> Result<(), ModelError> {
        let organization = self
            .organizations
            .remove(&organization_id)
            .ok_or(ModelError::MissingOrganization)?;
        let removed_project_ids = organization.projects;
        for id in &removed_project_ids {
            if let Some(project) = self.projects.remove(id) {
                self.project_path_statuses.remove(id);
                self.path_index.remove(&project.path_key);
                for tab in project.tabs {
                    self.tab_index.remove(&tab);
                }
            }
        }

        self.organization_order.retain(|id| *id != organization_id);
        if self.selected_organization == Some(organization_id) {
            self.selected_organization = self.organization_order.first().copied();
        }
        if self.hovered_organization == Some(organization_id) {
            self.hovered_organization = None;
        }
        if self
            .hovered_project
            .is_some_and(|id| removed_project_ids.contains(&id))
        {
            self.hovered_project = None;
        }
        if self.context_menu.is_some_and(|target| match target {
            ContextMenuTarget::Organization(id) => id == organization_id,
            ContextMenuTarget::Project(id) => removed_project_ids.contains(&id),
        }) {
            self.set_context_menu(None);
        }
        if self.selected_project.is_some_and(|id| {
            self.projects
                .get(&id)
                .map(|project| project.organization_id)
                != self.selected_organization
        }) {
            self.selected_project = self
                .selected_organization
                .and_then(|id| self.organizations.get(&id))
                .and_then(|organization| organization.projects.first().copied());
        }
        Ok(())
    }

    pub fn tab_owner(&self, tab_id: TabId) -> Option<ProjectId> {
        self.tab_index.get(&tab_id).copied()
    }

    pub fn workspace_tab_id(&self, tab_id: TabId) -> Option<&str> {
        self.tab_identity_by_id.get(&tab_id).map(String::as_str)
    }

    pub fn tab_identity_id(&self, workspace_tab_id: &str) -> Option<TabId> {
        self.tab_identity_by_id
            .iter()
            .find_map(|(id, identity)| (identity == workspace_tab_id).then_some(*id))
    }

    pub fn ensure_tab_identity(&mut self, workspace_tab_id: &str) -> Result<TabId, ModelError> {
        if let Some(id) = self.tab_identity_id(workspace_tab_id) {
            return Ok(id);
        }
        let id = self
            .tab_identity_by_id
            .keys()
            .copied()
            .max()
            .unwrap_or_default()
            .checked_add(1)
            .ok_or(ModelError::IdExhausted)?;
        self.tab_identity_by_id
            .insert(id, workspace_tab_id.to_owned());
        self.unpersisted_tab_identities
            .insert(id, workspace_tab_id.to_owned());
        Ok(id)
    }

    pub fn pending_tab_identities(&self) -> Vec<(TabId, String)> {
        self.unpersisted_tab_identities
            .iter()
            .map(|(id, identity)| (*id, identity.clone()))
            .collect()
    }

    pub fn mark_tab_identities_persisted(&mut self, identities: &[TabId]) {
        for id in identities {
            self.unpersisted_tab_identities.remove(id);
        }
    }

    #[cfg(test)]
    pub(super) fn seed_next_ids_for_test(
        &mut self,
        organization_id: OrganizationId,
        project_id: ProjectId,
    ) {
        self.next_organization_id = organization_id;
        self.next_project_id = project_id;
    }
}

impl Entity for WorkspaceModel {
    type Event = ();
}

impl SingletonEntity for WorkspaceModel {}

pub fn normalize_path_key(path: &str) -> Result<String, ModelError> {
    normalize_path_key_for_host(path, cfg!(windows))
}

pub(super) fn normalize_path_key_for_host(
    path: &str,
    is_windows_host: bool,
) -> Result<String, ModelError> {
    if path.trim().is_empty() {
        return Err(ModelError::EmptyPath);
    }

    if path.starts_with('/') {
        if is_windows_host {
            if !path.starts_with("//") {
                return Err(ModelError::RelativePath);
            }
        } else {
            return Ok(normalize_posix_absolute_path(path));
        }
    }

    let input = path.replace('/', "\\");
    let has_drive_prefix = has_windows_drive_prefix(&input);
    let (prefix, rest, unc) = if let Some(rest) = input.strip_prefix("\\\\") {
        let mut components = rest.split('\\').filter(|part| !part.is_empty());
        let has_server = components
            .next()
            .is_some_and(|part| part != "." && part != "..");
        let has_share = components
            .next()
            .is_some_and(|part| part != "." && part != "..");
        if !has_server || !has_share {
            return Err(ModelError::InvalidPath);
        }
        ("\\\\".to_owned(), rest, true)
    } else if has_drive_prefix {
        let drive = input.get(..2).expect("ASCII drive prefix");
        let rest = &input[2..];
        let rest = rest.strip_prefix('\\').ok_or(ModelError::RelativePath)?;
        (format!("{}\\", drive), rest, false)
    } else {
        return Err(ModelError::RelativePath);
    };

    let mut parts = Vec::new();
    for component in rest.split('\\') {
        match component {
            "" | "." => {}
            ".." if unc && parts.len() > 2 => {
                parts.pop();
            }
            ".." if unc => {}
            ".." if !parts.is_empty() => {
                parts.pop();
            }
            ".." => {}
            _ => parts.push(component),
        }
    }

    let mut key = if prefix.ends_with('\\') || prefix == "\\\\" {
        format!("{prefix}{}", parts.join("\\"))
    } else if prefix.is_empty() {
        parts.join("\\")
    } else if parts.is_empty() {
        prefix.clone()
    } else {
        format!("{prefix}{}", parts.join("\\"))
    };
    if key.len() > prefix.len() && key.ends_with('\\') {
        key.pop();
    }

    if has_drive_prefix || unc {
        key = key.to_lowercase();
    }
    Ok(key)
}

fn normalize_posix_absolute_path(path: &str) -> String {
    let mut parts = Vec::new();
    for component in path.split('/') {
        match component {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(component),
        }
    }
    format!("/{}", parts.join("/"))
}

pub(super) fn has_windows_drive_prefix(path: &str) -> bool {
    let bytes = path.as_bytes();
    bytes
        .first()
        .is_some_and(|first| first.is_ascii_alphabetic())
        && bytes.get(1) == Some(&b':')
}
