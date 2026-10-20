use super::{
    expand_home, next_image_generation, project_path_placeholder, submit_blocking_status,
    validate_project_path_metadata,
};
use std::io::ErrorKind;

#[test]
fn submission_waits_for_every_image_operation() {
    for (job, image_path, crop) in [
        (true, false, false),
        (false, true, false),
        (false, false, true),
    ] {
        assert_eq!(
            submit_blocking_status(job, image_path, crop, false, false),
            Some("Aguarde o processamento da imagem terminar.")
        );
    }
}

#[test]
fn choosing_an_invalid_image_invalidates_older_image_jobs() {
    let older_job_generation = 7;
    let invalid_selection_generation = next_image_generation(older_job_generation);

    assert_ne!(invalid_selection_generation, older_job_generation);
    assert_eq!(
        next_image_generation(u64::MAX),
        0,
        "generation must wrap without panicking"
    );
}

#[test]
fn submission_waits_for_project_path_validation() {
    assert_eq!(
        submit_blocking_status(false, false, false, true, false),
        Some("Aguarde a verificação da pasta terminar.")
    );
    assert_eq!(
        submit_blocking_status(false, false, false, false, false),
        None
    );
    assert_eq!(
        submit_blocking_status(false, false, false, false, true),
        Some("Remova a imagem com erro ou escolha outra antes de salvar.")
    );
}

#[test]
fn project_path_metadata_requires_an_accessible_directory() {
    assert_eq!(validate_project_path_metadata(Ok(true)), Ok(()));
    assert_eq!(
        validate_project_path_metadata(Ok(false)),
        Err("O caminho selecionado não é uma pasta.")
    );
    assert_eq!(
        validate_project_path_metadata(Err(ErrorKind::NotFound)),
        Err("A pasta selecionada não foi encontrada.")
    );
    assert_eq!(
        validate_project_path_metadata(Err(ErrorKind::PermissionDenied)),
        Err("Sem acesso à pasta selecionada.")
    );
    assert_eq!(
        validate_project_path_metadata(Err(ErrorKind::Other)),
        Err("Não foi possível verificar a pasta selecionada.")
    );
}

#[test]
fn project_path_placeholder_slugs_the_name_like_the_kit() {
    assert_eq!(project_path_placeholder(""), "~/projeto");
    assert_eq!(project_path_placeholder("  Keener API  "), "~/keener-api");
    assert_eq!(project_path_placeholder("Gestão de Ação!"), "~/gestao-de-acao");
}

#[test]
fn expand_home_resolves_only_a_leading_tilde() {
    let home = dirs::home_dir().expect("home dir");
    assert_eq!(expand_home("~/keener-api"), home.join("keener-api"));
    assert_eq!(expand_home("~"), home);
    assert_eq!(expand_home("D:/code/~x"), std::path::PathBuf::from("D:/code/~x"));
}
