use serde::{Deserialize, Serialize};
use settings::macros::define_settings_group;
use settings::{RespectUserSyncSetting, SupportedPlatforms, SyncToCloud};
use warp_core::features::FeatureFlag;
use warpui::units::Pixels;
use warpui::{AppContext, SingletonEntity};

use crate::settings::{AISettings, InputSettings, TerminalSpacing};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(
    description = "Controla se programas do terminal podem acessar a área de transferência do sistema por sequências de escape OSC 52.",
    rename_all = "snake_case"
)]
pub enum Osc52ClipboardAccess {
    #[default]
    #[schemars(description = "Impede todo acesso à área de transferência por OSC 52.")]
    Deny,
    #[schemars(
        description = "Permite que programas do terminal gravem na área de transferência, mas não leiam seu conteúdo."
    )]
    WriteOnly,
    #[schemars(
        description = "Permite que programas do terminal leiam e gravem na área de transferência."
    )]
    ReadWrite,
}

impl Osc52ClipboardAccess {
    pub fn allows_write(self) -> bool {
        matches!(self, Self::WriteOnly | Self::ReadWrite)
    }

    pub fn allows_read(self) -> bool {
        matches!(self, Self::ReadWrite)
    }

    pub fn as_dropdown_label(self) -> &'static str {
        match self {
            Self::Deny => "Não permitir",
            Self::WriteOnly => "Somente gravação",
            Self::ReadWrite => "Leitura e gravação",
        }
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(
    description = "Espaçamento entre blocos do terminal.",
    rename_all = "snake_case"
)]
pub enum SpacingMode {
    #[default]
    #[schemars(description = "Normal")]
    Normal,
    #[schemars(description = "Compacto")]
    Compact,
}

impl SpacingMode {
    pub fn other_mode(&self) -> SpacingMode {
        match *self {
            SpacingMode::Normal => SpacingMode::Compact,
            SpacingMode::Compact => SpacingMode::Normal,
        }
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(
    description = "Como o preenchimento é aplicado em aplicativos de terminal em tela cheia.",
    rename_all = "snake_case"
)]
pub enum AltScreenPaddingMode {
    #[schemars(description = "Usa o mesmo preenchimento da lista de blocos.")]
    MatchBlocklist,
    #[schemars(description = "Usa um valor uniforme de preenchimento personalizado.")]
    Custom { uniform_padding: Pixels },
}

impl Default for AltScreenPaddingMode {
    fn default() -> Self {
        Self::Custom {
            uniform_padding: Pixels::zero(),
        }
    }
}

impl AltScreenPaddingMode {
    pub fn toggled(&self) -> Self {
        match self {
            Self::MatchBlocklist => Self::Custom {
                uniform_padding: Pixels::zero(),
            },
            Self::Custom { .. } => Self::MatchBlocklist,
        }
    }

    pub fn telemetry_string(&self) -> String {
        match self {
            Self::MatchBlocklist => "MatchBlocklist",
            Self::Custom { .. } => "Custom",
        }
        .to_string()
    }
}

define_settings_group!(TerminalSettings, settings: [
    use_audible_bell: UseAudibleBell {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::DESKTOP, /* Audible bell is not supported on web */
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "terminal.use_audible_bell",
        description: "Se um som deve ser reproduzido quando o terminal emitir um sinal sonoro.",
    },
    spacing_mode: Spacing {
        type: SpacingMode,
        default: SpacingMode::default(),
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "appearance.spacing",
        description: "Controla o espaçamento entre blocos do terminal.",
    }
    maximum_grid_size: MaximumGridSize {
        type: usize,
        default: 50_000,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "terminal.maximum_grid_size",
        description: "Número máximo de linhas na grade do terminal.",
    },
    alt_screen_padding: AltScreenPadding {
        type: AltScreenPaddingMode,
        default: AltScreenPaddingMode::default(),
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "appearance.full_screen_apps.alt_screen_padding",
        max_table_depth: 0,
        description: "Controla o preenchimento ao redor de aplicativos de terminal em tela cheia.",
    },
    // This field should not be referenced directly to check zero state block visibility -- use
    // the `should_show_zero_state_block()` getter, which also considers global AI enablement.
    show_terminal_zero_state_block: ShowTerminalZeroStateBlock {
        type: bool,
        default: true,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "terminal.show_terminal_zero_state_block",
        description: "Se o bloco inicial da IA deve aparecer em novas sessões do terminal.",
    },
    osc52_clipboard_access: Osc52ClipboardAccessSetting {
        type: Osc52ClipboardAccess,
        default: Osc52ClipboardAccess::default(),
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "terminal.osc52_clipboard_access",
        description: "Controla se programas do terminal podem acessar a área de transferência do sistema por sequências de escape OSC 52. Opções: deny (padrão), write_only, read_write.",
    },
    // Opt-in toggle for running terminal find on a background thread. Only consulted on
    // channels where `FeatureFlag::AsyncFind` is off; channels with the flag on force the
    // feature on and hide this toggle. See `is_async_find_enabled` for the composite check.
    async_find_enabled: AsyncFindEnabled {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "experimental.async_find_enabled",
        description: "Usa uma implementação aprimorada da busca para manter a interface responsiva ao procurar correspondências em saídas extensas.",
    },
]);

impl TerminalSettings {
    /// Spacing for the terminal blocks.
    pub fn terminal_spacing(&self, line_height_ratio: f32, ctx: &AppContext) -> TerminalSpacing {
        match *self.spacing_mode {
            SpacingMode::Normal => TerminalSpacing::normal(line_height_ratio, ctx),
            SpacingMode::Compact => TerminalSpacing::compact(line_height_ratio, ctx),
        }
    }

    /// Whether the terminal zero state block should be shown.
    /// Checks both the user setting and the global AI enablement.
    pub fn should_show_zero_state_block(&self, ctx: &AppContext) -> bool {
        *self.show_terminal_zero_state_block && AISettings::as_ref(ctx).is_any_ai_enabled(ctx)
    }

    /// Whether asynchronous terminal find should be used. On channels where
    /// `FeatureFlag::AsyncFind` is on, the feature is force-enabled (no toggle shown).
    /// On other channels, users opt in via the `async_find_enabled` setting.
    pub fn is_async_find_enabled(&self) -> bool {
        FeatureFlag::AsyncFind.is_enabled() || *self.async_find_enabled
    }

    /// Spacing for the input box.
    pub fn terminal_input_spacing(
        &self,
        line_height_ratio: f32,
        ctx: &AppContext,
    ) -> TerminalSpacing {
        let should_force_normal_spacing =
            InputSettings::as_ref(ctx).is_universal_developer_input_enabled(ctx);
        if should_force_normal_spacing {
            return TerminalSpacing::normal(line_height_ratio, ctx);
        }
        match *self.spacing_mode {
            SpacingMode::Normal => TerminalSpacing::normal(line_height_ratio, ctx),
            SpacingMode::Compact => TerminalSpacing::compact(line_height_ratio, ctx),
        }
    }
}

#[cfg(test)]
#[path = "settings_tests.rs"]
mod tests;
