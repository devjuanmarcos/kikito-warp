use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum GitError {
    NotRepository,
    GitUnavailable,
    InvalidReference,
    InvalidBranchName,
    EmptyCommitMessage,
    InvalidPatch,
    MissingReference(String),
    InvalidOutput,
    CommandFailed {
        operation: &'static str,
        code: Option<i32>,
        failure: CommandFailure,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CommandFailure {
    NotRepository,
    MissingReference,
    Conflict,
    DirtyWorktree,
    PermissionDenied,
    NoCommonHistory,
    RemoteUnavailable,
    Other,
}

impl fmt::Display for GitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRepository => formatter.write_str("Esta pasta não contém um repositório Git."),
            Self::GitUnavailable => {
                formatter.write_str("Não foi possível iniciar o Git instalado.")
            }
            Self::InvalidReference => formatter.write_str("A referência Git informada é inválida."),
            Self::InvalidBranchName => formatter.write_str("Informe um nome válido para a branch."),
            Self::EmptyCommitMessage => formatter.write_str("Informe uma mensagem para o commit."),
            Self::InvalidPatch => formatter.write_str(
                "O bloco mudou desde a comparação. Atualize a visualização e tente novamente.",
            ),
            Self::MissingReference(reference) => write!(
                formatter,
                "A referência '{reference}' não está disponível localmente. Busque as referências manualmente e tente novamente."
            ),
            Self::InvalidOutput => {
                formatter.write_str("O Git retornou dados em um formato inválido.")
            }
            Self::CommandFailed {
                operation,
                code,
                failure,
            } => {
                let reason = match failure {
                    CommandFailure::MissingReference => {
                        " A refer\u{00ea}ncia local n\u{00e3}o foi encontrada."
                    }
                    CommandFailure::NotRepository => " Esta pasta não contém um repositório Git.",
                    CommandFailure::Conflict => " Resolva os conflitos e tente novamente.",
                    CommandFailure::DirtyWorktree => {
                        " Há alterações locais que impedem essa operação."
                    }
                    CommandFailure::PermissionDenied => {
                        " Verifique as permissões da pasta e do repositório."
                    }
                    CommandFailure::NoCommonHistory => {
                        " As referências locais não têm um histórico em comum."
                    }
                    CommandFailure::RemoteUnavailable => {
                        " O repositório não tem um remoto configurado ou ele está inacessível."
                    }
                    CommandFailure::Other => "",
                };
                match code {
                    Some(code) => write!(
                        formatter,
                        "O Git não conseguiu {operation} (código {code}).{reason}"
                    ),
                    None => write!(formatter, "O Git não conseguiu {operation}.{reason}"),
                }
            }
        }
    }
}

impl std::error::Error for GitError {}
