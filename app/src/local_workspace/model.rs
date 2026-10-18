use std::collections::{BTreeMap, HashMap};

pub type OrganizationId = u64;
pub type ProjectId = u64;
pub type TabId = i32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Organization {
    pub id: OrganizationId,
    pub name: String,
    pub projects: Vec<ProjectId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalProject {
    pub id: ProjectId,
    pub organization_id: OrganizationId,
    pub name: String,
    pub path: String,
    pub path_key: String,
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
            tabs: Vec::new(),
            active_tab: None,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModelError {
    EmptyPath,
    DuplicatePath,
    MissingOrganization,
    MissingProject,
    TabAlreadyAssigned,
    TabNotAssigned,
}

#[derive(Default)]
pub struct WorkspaceModel {
    organizations: BTreeMap<OrganizationId, Organization>,
    projects: BTreeMap<ProjectId, LocalProject>,
    path_index: HashMap<String, ProjectId>,
    tab_index: HashMap<TabId, ProjectId>,
    pub selected_organization: Option<OrganizationId>,
    pub selected_project: Option<ProjectId>,
    next_organization_id: OrganizationId,
    next_project_id: ProjectId,
}

impl WorkspaceModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_organization(&mut self, name: impl Into<String>) -> OrganizationId {
        self.next_organization_id += 1;
        let id = self.next_organization_id;
        self.organizations.insert(
            id,
            Organization {
                id,
                name: name.into(),
                projects: Vec::new(),
            },
        );
        self.selected_organization.get_or_insert(id);
        id
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

        self.next_project_id += 1;
        let id = self.next_project_id;
        project.id = id;
        project.organization_id = organization_id;
        self.organizations
            .get_mut(&organization_id)
            .expect("organization was checked above")
            .projects
            .push(id);
        self.path_index.insert(project.path_key.clone(), id);
        self.projects.insert(id, project);
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

    pub fn select_project(&mut self, project_id: ProjectId) -> Result<(), ModelError> {
        let project = self
            .projects
            .get(&project_id)
            .ok_or(ModelError::MissingProject)?;
        self.selected_organization = Some(project.organization_id);
        self.selected_project = Some(project_id);
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
        for id in organization.projects {
            if let Some(project) = self.projects.remove(&id) {
                self.path_index.remove(&project.path_key);
                for tab in project.tabs {
                    self.tab_index.remove(&tab);
                }
            }
        }

        if self.selected_organization == Some(organization_id) {
            self.selected_organization = self.organizations.keys().next().copied();
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
}

pub fn normalize_path_key(path: &str) -> Result<String, ModelError> {
    if path.trim().is_empty() {
        return Err(ModelError::EmptyPath);
    }

    let input = path.replace('/', "\\");
    let (prefix, rest, rooted, unc) = if let Some(rest) = input.strip_prefix("\\\\") {
        ("\\\\".to_owned(), rest, true, true)
    } else if input.as_bytes().get(1) == Some(&b':') {
        let drive = input.get(..2).expect("ASCII drive prefix");
        let rest = &input[2..];
        if let Some(rest) = rest.strip_prefix('\\') {
            (format!("{}\\", drive), rest, true, false)
        } else {
            (drive.to_owned(), rest, false, false)
        }
    } else if let Some(rest) = input.strip_prefix('\\') {
        ("\\".to_owned(), rest, true, false)
    } else {
        (String::new(), input.as_str(), false, false)
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
            ".." if !rooted => parts.push(".."),
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

    if input.as_bytes().get(1) == Some(&b':') || unc {
        key = key.to_lowercase();
    }
    Ok(key)
}
