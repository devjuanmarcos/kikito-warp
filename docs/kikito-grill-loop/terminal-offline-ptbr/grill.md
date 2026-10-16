# Grill interno

## Onda 1 — objetivo e fronteiras

- **Confirmado:** o usuário quer que o terminal não dependa de conexão externa, quer retirar o login e fluxos que levam a servidores da Warp e quer a interface toda em português.
- **Inferido:** o idioma desejado é português brasileiro (pt-BR), pelo idioma usado e pelo contexto regional. A escolha é reversível.
- **Confirmado:** o pedido inclui a possibilidade de substituir serviços Warp por algo próprio (“deixar algo nosso mesmo”).
- **Desconhecido e material:** se o primeiro marco deve remover os recursos remotos ou incluir um serviço/backend Kikito. Não assumir criação de backend, custo ou modelo de autenticação.
- **Confirmado:** o primeiro produto será local/offline, sem backend Kikito.
- **Confirmado:** remover dependências dos servidores Warp, telemetria, relatórios de falha, atualização automática, conexões externas automáticas e autenticação Warp.
- **Confirmado:** a finalidade é gerenciar terminais e agilizar processos locais; futuras implementações precisam servir a esse produto.
- **Confirmado:** há alterações locais de identidade visual já em andamento e exclusões preexistentes de `LICENSE-AGPL` e `LICENSE-MIT`. Preservar tudo isso.

## Onda 2 — dependências e efeitos cascata

### Fato confirmado → decisão

- `app/src/bin/oss.rs` inicializa `WarpServerConfig::production()` e `OzConfig::production()`. A configuração OSS desliga telemetria, crash reporting e autoupdate, mas ainda aponta o cliente a `app.warp.dev`, `rtc.app.warp.dev`, `sessions.app.warp.dev` e `oz.warp.dev` em `crates/warp_core/src/channel/config.rs`. Portanto, “OSS” atualmente não significa offline.
- `app/src/lib.rs` inicializa `AuthStateProvider` e `AuthManager`; `app/src/auth/` implementa login, tokens, logout e handoff ao navegador. Há recursos que chamam `attempt_login_gated_feature` e várias áreas condicionam ações a `is_logged_in()`.
- `app/src/auth/mod.rs::log_out` remove a persistência SQLite. Não reutilizar esse caminho para simplesmente eliminar login: isso pode apagar dados locais.
- `app/src/server/` concentra chamadas HTTP/GraphQL, e os clientes usam `ChannelState::server_root_url()`. Remover a tela de login por si só não impede chamadas remotas de outros recursos.
- `app/src/` e `crates/` não revelaram catálogos `.ftl`, `.po`, `.pot`, `.arb` nem dependência de localização na busca feita. A interface parece estar escrita diretamente em strings de código; confirmar a busca antes de escolher o mecanismo de tradução.
- O comando `cargo` não está disponível no PATH desta sessão. A compilação e os testes não podem ser validados neste ambiente até que Rust/Cargo esteja acessível.

### Inferência → decisão reversível + risco

- Um modo offline deve ser um perfil explícito do canal Kikito, com rede bloqueada por padrão, e não apenas login opcional. Isso evita que recursos sem autenticação continuem acessando endpoints Warp. Risco: recursos remotos deixam de funcionar nesse modo.
- A primeira entrega deve priorizar terminal, abas, sessões e configurações locais. IA hospedada, Drive, colaboração, ambientes e integrações precisam ser removidos/ocultados ou substituídos; não fingir que são locais.
- O idioma pode começar em pt-BR sem impedir adicionar outros idiomas depois. Uma tradução direta e dispersa ficaria difícil de manter; avaliar uma camada de mensagens antes de converter centenas de textos.

### Decisão aplicada

A escolha é **local/offline**. Não criar backend, autenticação ou sincronização. Preservar recursos e dados locais. Desativar recursos Warp remotos que não tenham substituto local; não manter botões que iniciem login ou prometam funcionamento inexistente.

## Onda 3 — cobertura

- **Dados:** preservar sessão, abas, configurações, histórico e banco local; nenhuma migração ou exclusão sem plano de compatibilidade.
- **Segurança:** sem tokens Warp não deve haver refresh em background; controlar pedidos HTTP, WebSocket, telemetria, update, crash upload e `open_url`.
- **Estados de erro:** recursos remotos ausentes devem estar ocultos/desativados com explicação em português, sem modal de login repetido.
- **Compatibilidade:** manter URLs, nomes de bundle e protocolos existentes até um plano explícito de migração.
- **Acessibilidade:** traduzir nomes acessíveis, atalhos e mensagens de leitor de tela junto aos rótulos visíveis.
- **Rollback:** introduzir modo/configuração em alterações graduais; manter o caminho atual recuperável até comprovar restauração local.

## Lacunas em aberto

- Confirmar se o TUI entra na primeira versão offline ou fica para uma etapa posterior; o padrão atual é GUI desktop primeiro.
- A interface ainda não tem mecanismo de localização identificado. Traduzir por áreas até haver uma infraestrutura compartilhada, sem alterar nomes técnicos que não aparecem ao usuário.
- O bloqueio de HTTP do cliente comum não cobre automaticamente bibliotecas de rede diretas nem navegação externa deliberada. Continuar o inventário antes de afirmar isolamento de rede total.
