use std::sync::LazyLock;

use async_trait::async_trait;

use super::{CliAgentPluginManager, PluginInstructionStep, PluginInstructions};

// Keep in sync with the opencode-warp npm package version.
// This version is also hardcoded into UPDATE_INSTRUCTIONS below (so the update
// instructions tell users to pin to this specific version to force OpenCode's
// plugin cache to re-fetch). Update both together.
const MINIMUM_PLUGIN_VERSION: &str = "0.1.5";

pub(super) struct OpenCodePluginManager;

#[async_trait]
impl CliAgentPluginManager for OpenCodePluginManager {
    fn minimum_plugin_version(&self) -> &'static str {
        MINIMUM_PLUGIN_VERSION
    }

    fn can_auto_install(&self) -> bool {
        false
    }

    fn install_instructions(&self) -> &'static PluginInstructions {
        &INSTALL_INSTRUCTIONS
    }

    fn update_instructions(&self) -> &'static PluginInstructions {
        &UPDATE_INSTRUCTIONS
    }
}

static INSTALL_INSTRUCTIONS: LazyLock<PluginInstructions> = LazyLock::new(|| PluginInstructions {
    title: "Instalar o plugin Warp para OpenCode",
    subtitle: "Adicione o plugin Warp à configuração do OpenCode e reinicie o OpenCode.",
    steps: &[
        PluginInstructionStep {
            description: "Abra ou crie o arquivo opencode.json. Ele pode ficar na raiz do projeto ou no caminho da configuração global:",
            command: "~/.config/opencode/opencode.json",
            executable: false,
            link: None,
        },
        PluginInstructionStep {
            description: "Adicione \"@warp-dot-dev/opencode-warp\" ao array \"plugin\" do objeto JSON principal:",
            command: "\"plugin\": [\"@warp-dot-dev/opencode-warp\"]",
            executable: false,
            link: None,
        },
    ],
    post_install_notes: &["Reinicie o OpenCode para ativar o plugin."],
});

static UPDATE_INSTRUCTIONS: LazyLock<PluginInstructions> = LazyLock::new(|| PluginInstructions {
    title: "Atualizar o plugin Warp para OpenCode",
    subtitle: "Fixe a versão mais recente do plugin no arquivo opencode.json. O OpenCode armazena os plugins em cache por especificação de versão; alterar essa especificação faz com que o plugin seja baixado novamente ao reiniciar.",
    steps: &[
        PluginInstructionStep {
            description: "Abra ou crie o arquivo opencode.json. Ele pode ficar na raiz do projeto ou no caminho da configuração global:",
            command: "~/.config/opencode/opencode.json",
            executable: false,
            link: None,
        },
        PluginInstructionStep {
            description: "Substitua a entrada atual de \"@warp-dot-dev/opencode-warp\" no array \"plugin\" pela versão explícita:",
            command: "\"plugin\": [\"@warp-dot-dev/opencode-warp@0.1.5\"]",
            executable: false,
            link: None,
        },
    ],
    post_install_notes: &["Reinicie o OpenCode para carregar o plugin atualizado."],
});

#[cfg(test)]
#[path = "opencode_tests.rs"]
mod tests;
