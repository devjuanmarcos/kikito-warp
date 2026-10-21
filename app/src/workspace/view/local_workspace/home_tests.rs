use std::collections::HashMap;

use super::{AskTarget, ask_results};
use crate::local_workspace::model::WorkspaceModel;

fn model_with_tab() -> (WorkspaceModel, HashMap<String, String>) {
    let mut model = WorkspaceModel::new();
    let organization = model.add_organization("Keener").unwrap();
    let api = model
        .add_project(organization, "keener-api", r"C:\keener\api")
        .unwrap();
    model
        .add_project(organization, "keener-web", r"C:\keener\web")
        .unwrap();
    let tab = model.ensure_tab_identity("tab-1").unwrap();
    model.assign_tab(api, tab).unwrap();
    let titles = HashMap::from([("tab-1".to_owned(), "cargo watch".to_owned())]);
    (model, titles)
}

#[test]
fn ask_results_strip_the_open_verb_and_list_projects_before_tabs() {
    let (model, titles) = model_with_tab();
    let results = ask_results(&model, "abrir keener-api", &titles);
    assert_eq!(results.len(), 2);
    assert!(matches!(results[0].target, AskTarget::Project(_)));
    assert_eq!(results[0].label, "keener-api");
    assert_eq!(results[1].label, "cargo watch");
    assert_eq!(results[1].sub, "keener-api · Keener");
}

#[test]
fn ask_results_are_empty_for_a_bare_verb() {
    let (model, titles) = model_with_tab();
    assert!(ask_results(&model, "  abrir  ", &titles).is_empty());
    assert!(ask_results(&model, "", &titles).is_empty());
}
