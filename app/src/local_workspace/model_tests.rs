use super::model::{
    LocalProject, ModelError, WorkspaceModel, has_windows_drive_prefix, normalize_path_key,
};

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

#[test]
fn unc_paths_and_unicode_are_preserved_and_normalized() {
    let a = LocalProject::new("A", r"\\server\share\Équipe\repo").unwrap();
    let b = LocalProject::new("B", "//SERVER/share/Équipe/./repo/").unwrap();

    assert_eq!(a.path, r"\\server\share\Équipe\repo");
    assert_eq!(a.path_key, b.path_key);
    assert_eq!(a.path_key, r"\\server\share\équipe\repo");
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
    assert_eq!(normalize_path_key("/tmp/a/../b"), Ok("/tmp/b".to_owned()));
}

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
fn selecting_a_project_does_not_change_another_projects_active_tab() {
    let mut model = WorkspaceModel::new();
    let org = model.add_organization("Org").unwrap();
    let first = model.add_project(org, "A", r"C:\a").unwrap();
    let second = model.add_project(org, "B", r"C:\b").unwrap();
    model.assign_tab(first, 10).unwrap();
    model.assign_tab(first, 11).unwrap();
    model.assign_tab(second, 20).unwrap();
    model.assign_tab(second, 21).unwrap();
    model.set_active_tab(first, Some(11)).unwrap();
    model.set_active_tab(second, Some(21)).unwrap();
    model.select_project(first).unwrap();

    assert_eq!(model.project(first).unwrap().active_tab, Some(11));
    assert_eq!(model.project(second).unwrap().active_tab, Some(21));
}

#[test]
fn removing_organization_removes_workspace_metadata_but_not_external_data() {
    let mut model = WorkspaceModel::new();
    let org = model.add_organization("Org").unwrap();
    let project = model.add_project(org, "A", r"C:\missing").unwrap();
    model.assign_tab(project, 33).unwrap();
    model.remove_organization(org).unwrap();

    assert!(model.project(project).is_none());
    assert_eq!(model.tab_owner(33), None);
    assert_eq!(model.project_count(), 0);
}
