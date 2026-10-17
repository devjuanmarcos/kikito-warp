# Limites de rede verificados estaticamente no canal OSS

Esta evidência descreve os caminhos de rede do próprio processo `warp-oss` encontrados na revisão de 2026-10-17. Ela não substitui uma captura de tráfego durante interação real.

## Barreiras no código

- `crates/http_client/src/lib.rs::Client::execute_inner` rejeita, no canal OSS, qualquer requisição cujo host não seja loopback antes de executar o cliente HTTP. A política também cobre os consumidores que usam esse wrapper, inclusive GraphQL.
- `crates/asset_cache/src/lib.rs::fetch_file_to_memory` rejeita URLs remotas no OSS. Quando usa `reqwest`, desativa proxy e só segue redirecionamentos para loopback.
- `app/src/lib.rs` registra a abertura de URL da GUI e bloqueia HTTP/HTTPS fora de loopback no canal OSS.
- `crates/mcp/src/runtime.rs::spawn_server` retorna antes de criar o transporte ou iniciar um servidor MCP no canal OSS. Assim, também não executa comandos configurados para servidores MCP.
- `crates/warp_core/src/channel/state.rs` configura o canal OSS com URLs locais `127.0.0.1:0`, sem endpoint de compartilhamento, telemetria, atualizador ou configuração de crash reporting. Os caminhos de inicialização/reporte correspondentes também têm condições explícitas para ignorar o canal OSS.

## Limites e interpretação

- A política de HTTP permite loopback por design para componentes locais. Ela não é um firewall de processo e não confina executáveis que o usuário inicia no shell.
- Codex, Claude, `curl`, gerenciadores de pacotes e outros comandos executados no terminal podem abrir suas próprias conexões. Essas conexões pertencem aos processos filhos e não são tráfego iniciado pelo Kikito Terminal.
- A observação runtime já registrada cobriu somente a inicialização ociosa do binário OSS, sem conexões TCP observadas nas amostras. Ainda falta medir ações reais da GUI e atribuir sockets aos processos pai/filhos durante esse uso.
- Portanto, a conclusão desta evidência é limitada: os caminhos revisados do cliente HTTP, carregamento de assets, abertura de URL e MCP bloqueiam ou desativam destinos remotos no OSS; não é uma alegação de zero sockets em qualquer cenário ou de isolamento dos comandos executados pelo usuário.