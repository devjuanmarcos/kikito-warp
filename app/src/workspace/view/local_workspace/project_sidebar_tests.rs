use super::empty_project_message;

#[test]
fn empty_organization_keeps_its_empty_state_during_search() {
    assert_eq!(
        empty_project_message(0, "api"),
        "Ainda não há projetos nesta organização."
    );
}

#[test]
fn search_with_no_matching_project_reports_no_results() {
    assert_eq!(
        empty_project_message(2, "api"),
        "Nenhum projeto corresponde à busca."
    );
}

#[test]
fn organization_without_search_uses_its_empty_state() {
    assert_eq!(
        empty_project_message(0, ""),
        "Ainda não há projetos nesta organização."
    );
}
