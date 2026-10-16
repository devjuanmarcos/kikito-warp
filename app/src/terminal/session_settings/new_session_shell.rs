use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use warp_util::path::ShellFamily;
use warpui::platform::OperatingSystem;

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(
    description = "Shell usado ao abrir novas sessões.",
    rename_all = "snake_case"
)]
pub enum NewSessionShell {
    #[default]
    #[schemars(description = "Usar o shell padrão do sistema operacional.")]
    SystemDefault,
    #[schemars(description = "Caminho para um executável de shell.")]
    Executable(String),
    #[schemars(description = "Ambiente de shell do MSYS2.")]
    MSYS2(String),
    #[schemars(description = "Distribuição do Subsistema do Windows para Linux (WSL).")]
    WSL(String),
    #[schemars(description = "Comando de shell personalizado.")]
    Custom(String),
}

impl NewSessionShell {
    pub fn shell_family(&self) -> ShellFamily {
        let shell = match self {
            NewSessionShell::SystemDefault => return OperatingSystem::get().default_shell_family(),
            NewSessionShell::WSL(_) => return ShellFamily::Posix,
            NewSessionShell::Executable(shell) => shell,
            NewSessionShell::MSYS2(shell) => shell,
            NewSessionShell::Custom(shell) => shell,
        };

        let path = PathBuf::from(shell);
        if let Some(file_stem) = path
            .file_stem()
            .and_then(|s| s.to_str().map(|s| s.to_lowercase()))
            && (file_stem.contains("powershell") || file_stem.contains("pwsh"))
        {
            return ShellFamily::PowerShell;
        }
        ShellFamily::Posix
    }
}
