# Limites de rede verificados estaticamente no canal OSS

Esta evidência reúne a revisão estática dos caminhos de rede do próprio processo `warp-oss` e uma captura runtime curta feita em 2026-10-17. Ela registra o escopo e os limites de cada verificação.

## Barreiras no código

- `crates/http_client/src/lib.rs::Client::execute_inner` rejeita, no canal OSS, qualquer requisição cujo host não seja loopback antes de executar o cliente HTTP. A política também cobre os consumidores que usam esse wrapper, inclusive GraphQL.
- `crates/asset_cache/src/lib.rs::fetch_file_to_memory` rejeita URLs remotas no OSS. Quando usa `reqwest`, desativa proxy e só segue redirecionamentos para loopback.
- `app/src/lib.rs` registra a abertura de URL da GUI e bloqueia HTTP/HTTPS fora de loopback no canal OSS.
- `crates/mcp/src/runtime.rs::spawn_server` retorna antes de criar o transporte ou iniciar um servidor MCP no canal OSS. Assim, também não executa comandos configurados para servidores MCP.
- `crates/warp_core/src/channel/state.rs` configura o canal OSS com URLs locais `127.0.0.1:0`, sem endpoint de compartilhamento, telemetria, atualizador ou configuração de crash reporting. Os caminhos de inicialização/reporte correspondentes também têm condições explícitas para ignorar o canal OSS.
- `app/src/features.rs::init_feature_flags` força para desativado no canal OSS as flags de criação/visualização de sessões compartilhadas, sessões compartilhadas por agentes, Cloud Mode, conversas cloud e outros fluxos hospedados. Isso impede o registro das actions que iniciam esses fluxos, incluindo as actions de WebSocket de sessão compartilhada registradas em `app/src/root_view.rs`.
- O crate `websocket` abre conexões TCP diretamente e não aplica a política OSS do cliente HTTP. Os caminhos identificados no app pertencem a recursos hospedados condicionados pelas flags OSS acima; não há uma barreira central de rede que bloqueie todo WebSocket externo caso um novo chamador seja adicionado sem gate.

## Limites e interpretação

- A política de HTTP permite loopback por design para componentes locais. Ela não é um firewall de processo e não confina executáveis que o usuário inicia no shell.
- Codex, Claude, `curl`, gerenciadores de pacotes e outros comandos executados no terminal podem abrir suas próprias conexões. Essas conexões pertencem aos processos filhos e não são tráfego iniciado pelo Kikito Terminal.
- Em 2026-10-17, iniciei `target/debug/warp-oss.exe` com `WARP_DATA_PROFILE=offline-interaction-audit-20261017c` e amostrei a árvore com dois processos `warp-oss.exe` na inicialização, depois de enviar `Ctrl+Shift+P` e `Ctrl+,`, e depois do pedido de fechamento. As quatro amostras não mostraram sockets externos; o único socket foi `LISTEN 127.0.0.1:9282` no processo principal.
- A porta 9282 corresponde ao servidor HTTP local OSS: `crates/http_server/src/lib.rs` liga o listener a `127.0.0.1` e monta os roteadores locais de detecção da instalação e profiling em `app/src/lib.rs`. Os atalhos foram enviados à janela, mas não houve confirmação visual do conteúdo aberto; a amostra cobre apenas esses processos e esse intervalo. Não executamos comandos de usuário.

## Captura durante interação com a janela

- Em 2026-10-17, iniciei novamente o binário com o perfil temporário `offline-ui-audit-20261017f` e validei que o handle pertencia à janela `Kikito Terminal`. Antes e depois da interação, a consulta ao PID principal encontrou somente `LISTEN 127.0.0.1:9282`, sem conexões externas naquele processo.
- O clique de teste atingiu o controle X e abriu a confirmação local de saída, com o título `Sair do Kikito Terminal?`. A captura confirmou que esse diálogo aparece em português. A paleta e as configurações não foram abertas; a janela foi fechada sem executar comandos do usuário.
- Esta amostra observou somente o processo GUI principal, não os processos filhos. Encerrei a árvore criada para a auditoria, confirmei que não restou `warp-oss.exe` ativo e removi a captura temporária.

## Auditoria estática de WebSocket

- Revisei os pontos de criação de WebSocket e os usos de `websocket::WebSocket::connect*`. No app, os fluxos encontrados pertencem a GraphQL/subscriptions, sessões compartilhadas e serviços hospedados. As actions de entrada de sessões compartilhadas dependem de `ViewingSharedSessions`; `init_feature_flags` fixa em `false` as flags OSS de sessões compartilhadas e recursos cloud antes da inicialização da interface.
- A política de `crates/http_client` não cobre WebSocket, porque o crate `websocket` conecta diretamente via TCP. A proteção atual é por disponibilidade/gating dos consumidores, e não por bloqueio de socket no nível do processo.
- Esta é uma inspeção estática dos pontos encontrados no código; não comprova, por si só, ausência de outros sockets durante todos os fluxos ou em processos filhos. A validação runtime permanece parcial.
