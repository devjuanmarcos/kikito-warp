use crate::local_workspace::model::ProjectPathStatus;

use super::{empty_project_message, path_status_message, project_matches_search};

#[test]
fn project_search_matches_names_and_paths_without_case_sensitivity() {
    assert!(project_matches_search(
        "Aplicativo",
        "D:\\Projetos\\App",
        "APLICATIVO"
    ));
    assert!(project_matches_search(
        "Aplicativo",
        "D:\\Projetos\\App",
        "projetos"
    ));
    assert!(!project_matches_search(
        "Aplicativo",
        "D:\\Projetos\\App",
        "terminal"
    ));
}

#[test]
fn empty_project_message_distinguishes_empty_organization_from_no_results() {
    assert_eq!(
        empty_project_message(0),
        "Nada aqui ainda. Quer que eu crie um projeto?"
    );
    assert_eq!(empty_project_message(1), "Nenhum resultado encontrado");
}

#[test]
fn path_status_message_hides_available_paths_and_explains_failures() {
    assert_eq!(path_status_message(ProjectPathStatus::Available), None);
    assert_eq!(
        path_status_message(ProjectPathStatus::Checking),
        Some("Verificando pasta…")
    );
    assert_eq!(
        path_status_message(ProjectPathStatus::Missing),
        Some("Pasta não encontrada")
    );
    assert_eq!(
        path_status_message(ProjectPathStatus::NotDirectory),
        Some("O caminho não é uma pasta")
    );
    assert_eq!(
        path_status_message(ProjectPathStatus::Inaccessible),
        Some("Sem acesso à pasta")
    );
}
