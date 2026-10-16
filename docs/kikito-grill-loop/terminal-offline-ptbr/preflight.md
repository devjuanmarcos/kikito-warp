# Pré-verificação

## Repositório e estado

- Repositório: checkout atual `D:\DEVJUANMARCOS\PROJETOS\KIKITO WARP`.
- Canal executável inicial: `warp-oss` (`app/src/bin/oss.rs`).
- O checkout tem mudanças locais anteriores de branding/ícone e as exclusões preexistentes `LICENSE-AGPL` e `LICENSE-MIT`; não descartar nem reformatar essas alterações.
- Criada pasta de estado em `docs/kikito-grill-loop/terminal-offline-ptbr/`.

## Referências inspecionadas

- `app/src/bin/oss.rs`
- `crates/warp_core/src/channel/config.rs` e `crates/warp_core/src/channel/state.rs`
- `app/src/auth/`, `app/src/lib.rs`, `app/src/root_view.rs`
- `app/src/server/`
- `app/src/settings/` e `app/src/settings_view/`
- Busca de arquivos/catálogos de localização e dependências i18n em `app/`, `crates/` e manifests Cargo.

## Dependências e capacidades

- Há endpoints de produção no cliente OSS, incluindo autenticação, API/GraphQL, RTC, compartilhamento de sessão e Oz.
- Telemetria, crash reporting e autoupdate estão configurados como `None` no binário OSS; ainda é necessário auditar chamadas de rede independentes e bibliotecas.
- Login é compartilhado com autorização de vários recursos. A remoção exige separar inicialização de identidade local da autenticação remota.
- A rotina atual de logout apaga a persistência local, então não é mecanismo seguro para remover o login.
- Busca não encontrou infraestrutura/catálogos de localização conhecidos.

## Validação disponível

- `git diff --check` é executável.
- `cargo check` não iniciou: o comando `cargo` não foi encontrado no PATH.
- Não há evidência de execução visual, proxy de rede ou verificação offline nesta etapa.
- Na implementação, validar com inicialização sem rede e captura/negação de tráfego; também verificar restauração de abas e persistência sem login.

## Riscos

- Mudanças amplas em autenticação podem afetar inicialização, gating de recursos, recuperação de sessão, telemetria e banco local.
- “Sem conexão externa” precisa significar ausência verificável de tráfego, inclusive DNS, atualização, crash reports, APIs indiretas e navegador.
- “Algo nosso” pode significar só aplicação desktop local ou uma infraestrutura de servidor própria; isso não deve ser inferido.
- A tradução exige cobrir strings dinâmicas, plurais, atalhos, tooltips, mensagens de erro e nomes acessíveis, não apenas os rótulos principais.
