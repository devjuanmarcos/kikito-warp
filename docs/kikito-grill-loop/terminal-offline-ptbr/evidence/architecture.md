# Evidência de arquitetura

- O binário `warp-oss` instancia `WarpServerConfig::production()` e `OzConfig::production()` em `app/src/bin/oss.rs`.
- `WarpServerConfig::production()` aponta para `https://app.warp.dev`, `wss://rtc.app.warp.dev/graphql/v2` e `wss://sessions.app.warp.dev`; `OzConfig::production()` aponta para `https://oz.warp.dev` em `crates/warp_core/src/channel/config.rs`.
- O canal OSS passa `None` para telemetria, crash reporting e autoupdate, mas esses `None` não anulam o `server_config`.
- `app/src/lib.rs` registra `AuthStateProvider` e `AuthManager`; autenticação participa do fluxo de inicialização e de vários recursos.
- `app/src/auth/mod.rs::log_out` remove a persistência SQLite, então desligar login não deve chamar o logout atual.
- `app/src/server/server_api.rs` e submódulos compõem diversos endpoints usando `ChannelState::server_root_url()`.
- `app/src/settings_view/` e a interface Rust não mostraram infraestrutura/catalogação i18n nas buscas feitas.
- Cargo não está disponível no PATH desta sessão; nenhum build ou teste foi executado.
