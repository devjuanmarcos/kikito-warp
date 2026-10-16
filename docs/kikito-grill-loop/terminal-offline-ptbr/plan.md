# Plano de execução

## Checkpoint 0 — decisão de produto (concluído)

- **Escolha:** local/offline, sem backend Kikito.
- **Escopo:** servidor Warp, telemetria, crash reporting, autoupdate, conexões externas automáticas e login Warp deixam o produto; terminal, sessões, abas e dados locais permanecem.
- **Evidência:** `briefing.md` e solicitação explícita do usuário.

## Checkpoint 1 — inventário executável

- **Arquivos:** `app/src/auth/`, `app/src/server/`, `app/src/lib.rs`, `app/src/root_view.rs`, `app/src/bin/oss.rs`, `crates/warp_core/src/channel/`, clientes de AI/Drive/collaboration, autoupdate/telemetria.
- **Ação:** mapear cada origem de rede para feature/ação; distinguir serviço obrigatório, opcional e local.
- **Teste/evidência:** tabela de chamadas e execução baseline sob monitoramento de rede.
- **Parada:** qualquer pedido que possa apagar/alterar dados ou exigir credencial própria.
- **Rollback:** inventário/documentação apenas.

## Checkpoint 2 — modo local sem autenticação remota

- **Arquivos prováveis:** configuração do canal, inicialização em `app/src/lib.rs`/`root_view.rs`, estado de auth e pontos de login-gating.
- **Ação:** representar estado local sem login remoto; retirar browser handoff e refresh; manter persistência e identidade de usuário local necessárias aos modelos.
- **Teste antes:** testes de inicialização/sessão sem login e persistência após reinício.
- **Evidência:** abertura direta em workspace sem tráfego de autenticação.
- **Parada:** se a inicialização precisar chamar API ou destruir a base local.
- **Rollback:** manter perfil anterior selecionável até verificar migração.

## Checkpoint 3 — recursos e conexões

- **Arquivos prováveis:** `app/src/server/`, `app/src/ai/`, `app/src/drive/`, colaboração, update, crash reporting e configuração OSS.
- **Ação:** desligar ou ocultar serviços remotos no perfil selecionado; substituir apenas integrações que tenham escopo/servidor próprio definido.
- **Teste antes:** testes de contrato/mocks e bloqueio de rede por host; inventário de `open_url`.
- **Evidência:** sessão offline sem requisições externas inesperadas; recursos remotos claramente indisponíveis.
- **Parada:** endpoint externo descoberto sem classificação; não o permitir silenciosamente.
- **Rollback:** opção de configuração por perfil, sem apagar código ou dados de serviço.

## Checkpoint 4 — localização pt-BR

- **Arquivos prováveis:** camada compartilhada de mensagens, strings de UI em `app/src/` e `crates/warpui*`, TUI se incluída no escopo.
- **Ação:** escolher catálogo/mecanismo após o inventário; traduzir interface por áreas, preservando placeholders e atalhos.
- **Teste antes:** checagem de placeholders/plurais e snapshots/renderização das superfícies tocadas.
- **Evidência:** varredura de textos visíveis restantes e UI renderizada em pt-BR.
- **Parada:** texto remoto/servidor que não possa ser traduzido pelo cliente; registrar como limitação.
- **Rollback:** catálogo de idioma independente da lógica.

## Checkpoint 5 — validação e entrega

- Executar testes direcionados, build, lint/typecheck e formatter segundo `AGENTS.md` após Cargo estar disponível.
- Validar GUI/TUI conforme o escopo escolhido, dados locais e bloqueio de rede.
- Comparar `git diff` e preservar as alterações preexistentes.
- Não publicar nem empurrar alterações sem pedido explícito.

## Checkpoint 6 — descoberta incremental pós-decisão

- Atualizar `progress.md` com os limites verificados em cada entrega.
- Só declarar offline total após inventariar HTTP direto, WebSocket, telemetria/crash, atualizador, MCP, links externos e demais sockets iniciados pelo app.
- Manter fora recursos que exigiriam login, conta, Drive, colaboração ou IA hospedada; retirar suas entradas da interface em uma etapa própria.
- Localizar para pt-BR as superfícies locais preservadas, incluindo acessibilidade e mensagens de erro.
