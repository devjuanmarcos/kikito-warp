CREATE TABLE local_workspace_organizations (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    sort_order INTEGER NOT NULL UNIQUE
);

CREATE TABLE local_workspace_projects (
    id INTEGER PRIMARY KEY NOT NULL,
    organization_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    path_key TEXT NOT NULL COLLATE BINARY UNIQUE,
    sort_order INTEGER NOT NULL,
    FOREIGN KEY (organization_id) REFERENCES local_workspace_organizations(id) ON DELETE CASCADE,
    UNIQUE (organization_id, sort_order)
);

CREATE INDEX local_workspace_projects_organization_order_idx
    ON local_workspace_projects (organization_id, sort_order);

CREATE TABLE local_workspace_tab_identities (
    id INTEGER PRIMARY KEY NOT NULL,
    workspace_tab_id TEXT NOT NULL UNIQUE,
    session_tab_id INTEGER UNIQUE,
    FOREIGN KEY (session_tab_id) REFERENCES tabs(id) ON DELETE SET NULL
);

INSERT INTO local_workspace_tab_identities (id, workspace_tab_id, session_tab_id)
SELECT id, 'legacy-tab-' || id, id FROM tabs;

CREATE TABLE local_workspace_project_tabs (
    project_id INTEGER NOT NULL,
    tab_id INTEGER NOT NULL UNIQUE,
    sort_order INTEGER NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (project_id, tab_id),
    FOREIGN KEY (project_id) REFERENCES local_workspace_projects(id) ON DELETE CASCADE,
    FOREIGN KEY (tab_id) REFERENCES local_workspace_tab_identities(id) ON DELETE CASCADE,
    UNIQUE (project_id, sort_order)
);

CREATE UNIQUE INDEX local_workspace_project_tabs_active_idx
    ON local_workspace_project_tabs (project_id)
    WHERE is_active = TRUE;

CREATE TABLE local_workspace_state (
    id INTEGER PRIMARY KEY NOT NULL CHECK (id = 1),
    selected_organization_id INTEGER,
    selected_project_id INTEGER,
    FOREIGN KEY (selected_organization_id)
        REFERENCES local_workspace_organizations(id) ON DELETE SET NULL,
    FOREIGN KEY (selected_project_id)
        REFERENCES local_workspace_projects(id) ON DELETE SET NULL
);

INSERT INTO local_workspace_state (id, selected_organization_id, selected_project_id)
VALUES (1, NULL, NULL);
