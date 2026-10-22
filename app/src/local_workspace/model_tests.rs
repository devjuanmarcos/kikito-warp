use super::model::{
    LocalProject, ModelError, ProjectPathStatus, WorkspaceModel, has_windows_drive_prefix,
    normalize_path_key, normalize_path_key_for_host,
};
use crate::persistence::LocalWorkspaceData;
use crate::persistence::model::{
    LocalWorkspaceOrganization, LocalWorkspaceProject, LocalWorkspaceProjectTab,
    LocalWorkspaceState,
};

#[test]
fn organization_context_menu_selection_wraps_over_its_two_actions() {
    let mut model = WorkspaceModel::new();
    let organization = model.add_organization("Equipe").unwrap();
    model.set_context_menu(Some(super::model::ContextMenuTarget::Organization(
        organization,
    )));

    model.move_context_menu_selection(1);
    assert_eq!(model.context_menu_index(), 1);

    model.move_context_menu_selection(1);
    assert_eq!(model.context_menu_index(), 0);
}

#[test]
fn project_context_menu_selection_wraps_over_its_four_actions() {
    let mut model = WorkspaceModel::new();
    let organization = model.add_organization("Equipe").unwrap();
    let project = model
        .add_project(organization, "Projeto", r"C:\projeto")
        .unwrap();
    model.set_context_menu(Some(super::model::ContextMenuTarget::Project(project)));

    model.move_context_menu_selection(-1);
    assert_eq!(model.context_menu_index(), 3);
}

#[test]
fn removing_hovered_project_clears_its_transient_hover_state() {
    let mut model = WorkspaceModel::new();
    let organization = model.add_organization("Equipe").unwrap();
    let project = model
        .add_project(organization, "Projeto", r"C:\projeto")
        .unwrap();
    model.set_hovered_project(Some(project));

    model.remove_project(project).unwrap();

    assert_eq!(model.hovered_project(), None);
}

#[test]
fn persisted_workspace_hydrates_database_ids_order_selection_and_tab_identity_ids() {
    let api_path = if cfg!(windows) {
        r"C:\repos\api"
    } else {
        "/repos/api"
    };
    let web_path = if cfg!(windows) {
        r"C:\repos\web"
    } else {
        "/repos/web"
    };
    let data = LocalWorkspaceData {
        organizations: vec![
            LocalWorkspaceOrganization {
                id: 7,
                name: "Equipe B".into(),
                sort_order: 0,
                accent_color: "cyan".into(),
                image_data: Some(vec![1, 2, 3]),
            },
            LocalWorkspaceOrganization {
                id: 3,
                name: "Equipe A".into(),
                sort_order: 1,
                accent_color: "blue".into(),
                image_data: None,
            },
        ],
        projects: vec![
            LocalWorkspaceProject {
                id: 12,
                organization_id: 7,
                name: "API".into(),
                path: api_path.into(),
                path_key: api_path.into(),
                sort_order: 0,
                last_opened_ts: 20,
            },
            LocalWorkspaceProject {
                id: 4,
                organization_id: 7,
                name: "Web".into(),
                path: web_path.into(),
                path_key: web_path.into(),
                sort_order: 1,
                last_opened_ts: 10,
            },
        ],
        tabs: vec![
            LocalWorkspaceProjectTab {
                project_id: 12,
                tab_id: 91,
                sort_order: 0,
                is_active: false,
            },
            LocalWorkspaceProjectTab {
                project_id: 12,
                tab_id: 108,
                sort_order: 1,
                is_active: true,
            },
        ],
        state: Some(LocalWorkspaceState {
            id: 1,
            selected_organization_id: Some(7),
            selected_project_id: Some(12),
        }),
        ..Default::default()
    };

    let model = WorkspaceModel::from_persisted(&data).unwrap();
    assert_eq!(
        model.organizations().map(|org| org.id).collect::<Vec<_>>(),
        vec![7, 3]
    );
    assert_eq!(model.organization(7).unwrap().projects, vec![12, 4]);
    assert_eq!(model.project(12).unwrap().tabs, vec![91, 108]);
    assert_eq!(model.project(12).unwrap().active_tab, Some(108));
    assert_eq!(model.selected_organization(), Some(7));
    assert_eq!(model.selected_project(), Some(12));
    assert_eq!(model.organization(7).unwrap().accent_color, "cyan");
    assert_eq!(
        model.organization(7).unwrap().image_data,
        Some(vec![1, 2, 3])
    );
}

#[test]
fn persisted_workspace_rejects_non_positive_database_ids() {
    let data = LocalWorkspaceData {
        organizations: vec![LocalWorkspaceOrganization {
            id: -1,
            name: "Inválida".into(),
            sort_order: 0,
            accent_color: "blue".into(),
            image_data: None,
        }],
        ..Default::default()
    };
    assert!(matches!(
        WorkspaceModel::from_persisted(&data),
        Err(ModelError::InvalidId)
    ));
}

#[test]
fn organization_visuals_accept_only_kit_accents_and_persist_image_bytes() {
    let mut model = WorkspaceModel::new();
    let organization_id = model.add_organization("Equipe").unwrap();
    model
        .set_organization_visuals(organization_id, "cyan".into(), Some(vec![1, 2, 3]))
        .unwrap();
    assert_eq!(
        model.organization(organization_id).unwrap().accent_color,
        "cyan"
    );
    assert_eq!(
        model.organization(organization_id).unwrap().image_data,
        Some(vec![1, 2, 3])
    );
    assert_eq!(
        model.set_organization_visuals(organization_id, "purple".into(), None),
        Err(ModelError::InvalidAccentColor)
    );
}

#[test]
fn windows_paths_deduplicate_case_and_separator_variants_across_organizations() {
    let mut model = WorkspaceModel::new();
    let first_org = model.add_organization("First").unwrap();
    let second_org = model.add_organization("Second").unwrap();
    let project = model
        .add_project(first_org, "Repo", r"C:\Users\Marcos\Repo")
        .unwrap();

    assert_eq!(
        model.add_project(second_org, "Duplicate", "c:/users/marcos/repo/"),
        Err(ModelError::DuplicatePath)
    );
    assert_eq!(
        model.project(project).unwrap().path,
        r"C:\Users\Marcos\Repo"
    );
}

#[cfg(windows)]
#[test]
fn unc_paths_and_unicode_are_preserved_and_normalized() {
    let a = LocalProject::new("A", r"\\server\share\Équipe\repo").unwrap();
    let b = LocalProject::new("B", "//SERVER/share/Équipe/./repo/").unwrap();

    assert_eq!(a.path, r"\\server\share\Équipe\repo");
    assert_eq!(a.path_key, b.path_key);
    assert_eq!(a.path_key, r"\\server\share\équipe\repo");
}

#[cfg(not(windows))]
#[test]
fn backslash_unc_paths_remain_windows_unc_paths() {
    let project = LocalProject::new("UNC", r"\\server\share\repo").unwrap();
    assert_eq!(project.path_key, r"\\server\share\repo");
}

#[test]
fn lexical_normalization_handles_absolute_drive_and_unc_paths() {
    assert_eq!(normalize_path_key(r"C:\\a\..\"), Ok(r"c:\".to_owned()));
    assert_eq!(normalize_path_key("C:/"), Ok(r"c:\".to_owned()));
    assert_eq!(normalize_path_key("D:/folder"), Ok(r"d:\folder".to_owned()));
    assert_eq!(
        normalize_path_key(r"\\server\share\..\folder"),
        Ok(r"\\server\share\folder".to_owned())
    );
    #[cfg(not(windows))]
    assert_eq!(normalize_path_key("/tmp/a/../b"), Ok("/tmp/b".to_owned()));
    #[cfg(windows)]
    assert_eq!(
        normalize_path_key("/tmp/a/../b"),
        Err(ModelError::RelativePath)
    );
}

#[cfg(not(windows))]
#[test]
fn unix_absolute_paths_keep_posix_separators_and_case() {
    assert_eq!(
        normalize_path_key("/Home/Team/../Repo/"),
        Ok("/Home/Repo".to_owned())
    );
    assert_eq!(
        normalize_path_key("/home/repo/./"),
        Ok("/home/repo".to_owned())
    );
    assert_ne!(
        normalize_path_key("/Home/Repo"),
        normalize_path_key("/home/repo")
    );
}

#[cfg(windows)]
#[test]
fn windows_root_relative_paths_are_rejected() {
    for path in [r"\foo", "/foo"] {
        assert_eq!(normalize_path_key(path), Err(ModelError::RelativePath));
    }
}

#[cfg(not(windows))]
#[test]
fn posix_single_leading_slash_is_an_absolute_path() {
    assert_eq!(normalize_path_key("/foo"), Ok("/foo".to_owned()));
}

#[test]
fn unicode_leading_relative_paths_do_not_use_windows_drive_parsing() {
    assert_eq!(
        normalize_path_key("\u{00e9}:/Repo/../Source"),
        Err(ModelError::RelativePath)
    );
}

#[test]
fn drive_prefix_detection_accepts_only_ascii_letters_before_colon() {
    assert!(has_windows_drive_prefix("C:\\Repo"));
    assert!(has_windows_drive_prefix("c:Repo"));
    assert!(!has_windows_drive_prefix("\u{00e9}:Repo"));
    assert!(!has_windows_drive_prefix("/Repo"));
}

#[test]
fn forward_slash_unc_interpretation_matches_the_host_path_flavor() {
    assert_eq!(
        normalize_path_key_for_host("//Foo/Repo", true),
        Ok(r"\\foo\repo".to_owned())
    );
    assert_eq!(
        normalize_path_key_for_host("//Foo/Repo", false),
        Ok("/Foo/Repo".to_owned())
    );
    assert_eq!(
        normalize_path_key_for_host("//Foo/Repo", false),
        normalize_path_key_for_host("/Foo/Repo", false)
    );
    assert_ne!(
        normalize_path_key_for_host("//Foo/Repo", false),
        normalize_path_key_for_host("/foo/repo", false)
    );
}

#[cfg(not(windows))]
#[test]
fn posix_double_slash_paths_collapse_without_case_folding() {
    assert_eq!(normalize_path_key("//Foo/Repo"), Ok("/Foo/Repo".to_owned()));
    assert_eq!(
        normalize_path_key("//Foo/Repo"),
        normalize_path_key("/Foo/Repo")
    );
    assert_ne!(
        normalize_path_key("//Foo/Repo"),
        normalize_path_key("/foo/repo")
    );
}

#[test]
fn cwd_dependent_paths_are_rejected() {
    for path in ["foo", "./foo", "C:foo", r"\foo"] {
        assert_eq!(
            normalize_path_key(path),
            Err(ModelError::RelativePath),
            "{path}"
        );
        assert_eq!(
            LocalProject::new("Relative", path),
            Err(ModelError::RelativePath),
            "{path}"
        );
    }
}

#[test]
fn organization_and_project_id_exhaustion_returns_an_error_without_partial_state() {
    let mut model = WorkspaceModel::new();
    model.seed_next_ids_for_test(u64::MAX, 0);
    assert_eq!(
        model.add_organization("Exhausted"),
        Err(ModelError::IdExhausted)
    );
    assert_eq!(model.selected_organization(), None);
    assert!(model.organization(0).is_none());

    let mut model = WorkspaceModel::new();
    let org = model.add_organization("Org").unwrap();
    model.seed_next_ids_for_test(1, u64::MAX);
    assert_eq!(
        model.add_project(org, "Exhausted", r"C:\repo"),
        Err(ModelError::IdExhausted)
    );
    assert!(model.organization(org).unwrap().projects.is_empty());
    assert_eq!(model.project_count(), 0);
}

#[test]
fn missing_project_directories_are_allowed_without_filesystem_access() {
    let project = LocalProject::new("Missing", r"Z:\does\not\exist").unwrap();
    assert_eq!(project.path, r"Z:\does\not\exist");
}

#[test]
fn empty_paths_are_rejected() {
    assert_eq!(LocalProject::new("Empty", " "), Err(ModelError::EmptyPath));
}

#[test]
fn projects_keep_insertion_order_and_selection_stays_with_its_organization() {
    let mut model = WorkspaceModel::new();
    let first_org = model.add_organization("First").unwrap();
    let second_org = model.add_organization("Second").unwrap();
    let first = model.add_project(first_org, "One", r"C:\one").unwrap();
    let other = model.add_project(second_org, "Other", r"C:\other").unwrap();
    let second = model.add_project(first_org, "Two", r"C:\two").unwrap();

    assert_eq!(
        model.organization(first_org).unwrap().projects,
        vec![first, second]
    );
    assert_eq!(model.selected_organization(), Some(first_org));
    assert_eq!(model.selected_project(), Some(first));
    model.select_project(other).unwrap();
    assert_eq!(model.selected_organization(), Some(second_org));
    assert_eq!(model.selected_project(), Some(other));
}

#[test]
fn a_tab_can_belong_to_only_one_project() {
    let mut model = WorkspaceModel::new();
    let org = model.add_organization("Org").unwrap();
    let first = model.add_project(org, "A", r"C:\a").unwrap();
    let second = model.add_project(org, "B", r"C:\b").unwrap();
    model.assign_tab(first, 10).unwrap();

    assert_eq!(
        model.assign_tab(second, 10),
        Err(ModelError::TabAlreadyAssigned)
    );
    assert_eq!(model.tab_owner(10), Some(first));
}

#[test]
fn selecting_projects_restores_each_projects_active_tab() {
    let mut model = WorkspaceModel::new();
    let org = model.add_organization("Org").unwrap();
    let first = model.add_project(org, "A", r"C:\a").unwrap();
    let second = model.add_project(org, "B", r"C:\b").unwrap();
    let first_tab = model.ensure_tab_identity("workspace-tab-a").unwrap();
    let first_active = model.ensure_tab_identity("workspace-tab-a-active").unwrap();
    let second_tab = model.ensure_tab_identity("workspace-tab-b").unwrap();
    let second_active = model.ensure_tab_identity("workspace-tab-b-active").unwrap();
    model.assign_tab(first, first_tab).unwrap();
    model.assign_tab(first, first_active).unwrap();
    model.assign_tab(second, second_tab).unwrap();
    model.assign_tab(second, second_active).unwrap();
    model.set_active_tab(first, Some(first_active)).unwrap();
    model.set_active_tab(second, Some(second_active)).unwrap();
    model.select_project(first).unwrap();

    assert_eq!(model.selected_project(), Some(first));
    assert_eq!(model.project(first).unwrap().active_tab, Some(first_active));
    assert_eq!(
        model.project(second).unwrap().active_tab,
        Some(second_active)
    );
    assert_eq!(
        model.workspace_tab_id(first_active),
        Some("workspace-tab-a-active")
    );

    model.select_project(second).unwrap();

    assert_eq!(model.selected_project(), Some(second));
    assert_eq!(model.project(first).unwrap().active_tab, Some(first_active));
    assert_eq!(
        model.project(second).unwrap().active_tab,
        Some(second_active)
    );
    assert_eq!(
        model.workspace_tab_id(second_active),
        Some("workspace-tab-b-active")
    );
}

#[test]
fn removing_organization_removes_workspace_metadata_but_not_external_data() {
    let mut model = WorkspaceModel::new();
    let org = model.add_organization("Org").unwrap();
    let project = model.add_project(org, "A", r"C:\missing").unwrap();
    model.assign_tab(project, 33).unwrap();
    model.set_context_menu(Some(super::model::ContextMenuTarget::Project(project)));
    model.remove_organization(org).unwrap();

    assert!(model.project(project).is_none());
    assert_eq!(model.tab_owner(33), None);
    assert_eq!(model.project_count(), 0);
    assert_eq!(model.context_menu(), None);
}

#[test]
fn removing_selected_organization_selects_a_remaining_organization() {
    let mut model = WorkspaceModel::new();
    let first = model.add_organization("First").unwrap();
    let second = model.add_organization("Second").unwrap();
    let project = model.add_project(first, "A", r"C:\a").unwrap();
    model.select_project(project).unwrap();

    model.remove_organization(first).unwrap();

    assert_eq!(model.selected_organization(), Some(second));
    assert_eq!(model.selected_project(), None);
    assert!(model.organization(first).is_none());
}

#[test]
fn recent_projects_are_ordered_by_last_selection_and_skip_removed_projects() {
    let mut model = WorkspaceModel::new();
    let org = model.add_organization("Org").unwrap();
    let first = model.add_project(org, "A", r"C:\a").unwrap();
    let second = model.add_project(org, "B", r"C:\b").unwrap();

    model.select_project(first).unwrap();
    model.select_project(second).unwrap();
    model.select_project(first).unwrap();

    assert_eq!(
        model
            .recent_projects()
            .map(|project| project.id)
            .collect::<Vec<_>>(),
        vec![first, second]
    );
    model.remove_project(first).unwrap();
    assert_eq!(
        model
            .recent_projects()
            .map(|project| project.id)
            .collect::<Vec<_>>(),
        vec![second]
    );
}

#[test]
fn project_path_status_is_cached_and_removed_with_project() {
    let mut model = WorkspaceModel::new();
    let organization = model.add_organization("Org").unwrap();
    let project = model
        .add_project(organization, "API", r"C:\repos\api")
        .unwrap();

    assert_eq!(
        model.project_path_status(project),
        ProjectPathStatus::Checking
    );
    model.set_project_path_status(project, ProjectPathStatus::Inaccessible);
    assert_eq!(
        model.project_path_status(project),
        ProjectPathStatus::Inaccessible
    );

    model.remove_project(project).unwrap();
    assert_eq!(
        model.project_path_status(project),
        ProjectPathStatus::Checking
    );
}

#[test]
fn project_tab_reorder_and_active_removal_are_isolated() {
    let mut model = WorkspaceModel::new();
    let organization = model.add_organization("Equipe").unwrap();
    let first = model.add_project(organization, "A", r"C:\a").unwrap();
    let second = model.add_project(organization, "B", r"C:\b").unwrap();
    for tab_id in [10, 11, 12] {
        model.assign_tab(first, tab_id).unwrap();
    }
    model.assign_tab(second, 20).unwrap();
    model.set_active_tab(first, Some(11)).unwrap();

    model.reorder_project_tabs(first, &[12, 11, 10]).unwrap();
    assert_eq!(model.project(first).unwrap().tabs, vec![12, 11, 10]);
    assert_eq!(model.project(second).unwrap().tabs, vec![20]);
    assert_eq!(model.project(first).unwrap().active_tab, Some(11));

    model.remove_tab_from_project(first, 11).unwrap();
    assert_eq!(model.project(first).unwrap().active_tab, Some(10));
    model.remove_tab_from_project(first, 10).unwrap();
    model.remove_tab_from_project(first, 12).unwrap();
    assert!(model.project(first).unwrap().tabs.is_empty());
    assert_eq!(model.project(first).unwrap().active_tab, None);
}

#[test]
fn persisted_tab_identity_resolves_workspace_tab_uuid() {
    let data = LocalWorkspaceData {
        tab_identities: vec![crate::persistence::model::LocalWorkspaceTabIdentity {
            id: 74,
            workspace_tab_id: "stable-tab".to_owned(),
            session_tab_id: Some(7),
        }],
        ..Default::default()
    };
    let model = WorkspaceModel::from_persisted(&data).unwrap();

    assert_eq!(model.workspace_tab_id(74), Some("stable-tab"));
    assert_eq!(model.tab_identity_id("stable-tab"), Some(74));
}
