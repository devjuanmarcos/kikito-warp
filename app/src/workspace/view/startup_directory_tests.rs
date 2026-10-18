use super::existing_project_startup_directory;

#[test]
fn project_startup_directory_is_ignored_after_its_path_is_removed() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().to_path_buf();

    assert_eq!(
        existing_project_startup_directory(path.clone()),
        Some(path.clone())
    );
    drop(directory);

    assert_eq!(existing_project_startup_directory(path), None);
}

#[test]
fn different_projects_keep_distinct_startup_directories() {
    let first = tempfile::tempdir().unwrap();
    let second = tempfile::tempdir().unwrap();

    assert_eq!(
        existing_project_startup_directory(first.path().to_path_buf()),
        Some(first.path().to_path_buf())
    );
    assert_eq!(
        existing_project_startup_directory(second.path().to_path_buf()),
        Some(second.path().to_path_buf())
    );
    assert_ne!(first.path(), second.path());
}
