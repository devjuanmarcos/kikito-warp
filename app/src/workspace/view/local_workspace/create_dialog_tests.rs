use super::{next_image_generation, submit_blocking_status, validate_project_path_metadata};
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
        Some("Aguarde a verificacao da pasta terminar.")
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
        Err("O caminho selecionado nao e uma pasta.")
    );
    assert_eq!(
        validate_project_path_metadata(Err(ErrorKind::NotFound)),
        Err("A pasta selecionada nao foi encontrada.")
    );
    assert_eq!(
        validate_project_path_metadata(Err(ErrorKind::PermissionDenied)),
        Err("Sem acesso a pasta selecionada.")
    );
    assert_eq!(
        validate_project_path_metadata(Err(ErrorKind::Other)),
        Err("Nao foi possivel verificar a pasta selecionada.")
    );
}
