# Progresso

## Rodada 1 — canal local e desligamento de serviços Warp

- **Decisão do usuário:** modo local/offline, sem backend Kikito; retirar servidores Warp, telemetria, relatórios de falha, atualização automática e autenticação. Manter o foco em terminais, abas e processos locais.
- **Configuração:** os binários OSS de GUI e TUI usam endpoints locais inertes. O estado Auth local ignora credenciais salvas sem apagá-las.
- **Inicialização:** a GUI OSS pula o onboarding de login. A TUI OSS abre a sessão local diretamente. Ações de login/logout e o processamento de eventos de autenticação não fazem chamadas no canal OSS.
- **Rede e serviços:** o cliente HTTP compartilhado recusa destinos não-loopback no canal OSS; assets remotos, subscriptions GraphQL, servidores MCP e serviços Warp de MCP ficam bloqueados. A revisão dos usos de WebSocket encontrou apenas subscriptions de servidor e fluxos de compartilhamento de sessões; as flags de compartilhamento também ficam desligadas no perfil OSS. Macros de telemetria não registram eventos no canal OSS e o coletor não inicializa nem envia. Crash reporting e tracing OTLP cloud são ignorados, inclusive em subprocessos. A leitura de credenciais AWS/SSO e a emissão de credenciais OIDC/STS também ficam bloqueadas. A abertura de links HTTP/HTTPS externos é recusada no perfil OSS (endereços de loopback continuam permitidos), e links de documentação externa somem da Aparência. O menu de perfil OSS também não apresenta atalhos de documentação, feedback, Slack, conta, cobrança ou atualização. O atualizador TUI não é registrado; o atualizador GUI depende da flag desabilitada no canal.
- **Interface removida do perfil OSS:** menus de IA, Drive e ajuda externa não são construídos; atalhos para Drive, histórico de agentes e workflows também ficam ocultos; opções de Aparência para fonte de agente e abas de Drive/histórico não aparecem; páginas de conta, cloud, colaboração e serviços remotos não aparecem nas configurações; a ação de nova aba de agente fica oculta.
- **Marca e idioma:** títulos das janelas e nome do CLI TUI usam Kikito Terminal. Menus principais e alguns itens estáticos, menu de perfil local, menus de contexto de abas/grupos/painéis, descrições de atalhos de navegação e zoom, opções da paleta para criar abas/janelas/dividir painéis, estado inicial da busca de comandos, descrições e busca da página de atalhos, busca e categorias de Aparência, opções comuns de Aparência, avisos de saída e rótulos de Sobre foram passados para pt-BR.
- **Auditoria da marca:** ícones da GUI OSS e dos pacotes já apontavam para a marca KT aprovada. Foram corrigidos o nome do produto nos metadados PE do Windows, descrições do pacote OSS que ainda prometiam recursos de nuvem, o nome genérico do lançador Linux e as imagens antigas do instalador Windows (agora específicas do canal OSS). Os canais upstream não OSS mantêm a marca original. Identificadores técnicos `dev.warp.WarpOss`, `warposs`, nomes de crates, variáveis e protocolos herdados foram mantidos por compatibilidade com dados locais, integração com shells e dependências.
- **Ícone da barra de tarefas Windows:** identificado que `cargo run` usava o fallback `local` de `CARGO_BIN_NAME`, embutindo o ícone errado no executável OSS. O fallback agora é `oss`; `script/run` define o canal explicitamente. O AppUserModelID dos atalhos OSS do instalador agora corresponde ao `dev.warp.WarpOss` que o app define no processo, preservando a identidade instalada.
- **Dados locais:** nenhuma alteração de banco ou exclusão de credenciais foi feita.

## Validação

- `cargo check -p warp --bin warp-oss` — passou após os bloqueios de URLs externos e as atualizações mais recentes de menus, Aparência, abas e avisos (1m52s).
- `cargo check -p warp --bin warp-oss` — passou novamente depois da localização dos rótulos de atalhos e grupos de abas (1m24s).
- `cargo check -p warp --bin warp-oss` — passou novamente depois da tradução das descrições de navegação, opções de novas sessões na paleta e estado inicial da busca (1m11s).
- `cargo check -p warp --bin warp-oss` — passou novamente depois da tradução da página de atalhos de teclado (1m16s).
- `cargo check -p warp --bin warp-oss` — passou novamente depois de restringir o menu de perfil OSS a ações locais e localizar os menus de abas e painéis (1m27s).
- `cargo check -p warp_tui --bin warp-tui-oss` — passou apontando `PROTOC` para o executável WinGet já instalado (`Google.Protobuf`); ele não estava no `PATH` desta sessão.
- `git diff --check` — passou; restaram apenas avisos de conversão LF/CRLF do Git no Windows.
- `cargo check -p warp --bin warp-oss` — passou após os ajustes de marca e metadados (1m13s; executável Cargo chamado pelo caminho completo).
- `cargo check -p warp --bin warp-oss` — passou novamente após localizar duas mensagens visíveis de compatibilidade de shell (1m31s).
- `CARGO_BIN_NAME=oss WARP_APP_NAME="Kikito Terminal" cargo run -p warp --bin warp-oss` — abriu o app atualizado (3m03s). O `target/debug/icon.ico` corresponde ao ícone OSS por SHA-256; o recurso extraído do executável mostra o monograma KT aprovado.
- Inicialização do `warp-oss.exe` observada por 12 segundos: nenhuma conexão TCP externa foi listada; o processo foi encerrado após a observação. Isso não cobre fluxos iniciados por ações do usuário.
- Nenhum teste foi executado.
- Ainda falta executar a aplicação com monitoramento de rede enquanto são acionadas as ações de menu e outros fluxos, para comprovar o comportamento em runtime além da análise estática e da observação em repouso.

## Pendências antes de declarar encerrado

- **Isolamento:** concluir a verificação em runtime com ações do usuário; a observação em repouso e os bloqueios estáticos não provam o comportamento de cada recurso.
- **Interface:** pt-BR não está completa. Descrições e controles internos de Aparência e Atalhos, diálogos, command palette, tooltips, mensagens e estados do terminal ainda contêm textos em inglês. As strings são majoritariamente inline; não foi encontrado catálogo i18n existente.
- **Ambiente TUI:** `protoc` precisa estar no `PATH` ou ser definido na variável `PROTOC` para compilar o TUI via Cargo.

**Status: EM ANDAMENTO.** GUI e TUI OSS passam em `cargo check`; isolamento total de rede e tradução completa ainda não foram comprovados nem concluídos.

## Retomada — Aparência e identidade

- O comando da paleta para alternar o tipo de entrada agora aparece em português e usa os nomes exibidos pela interface: “Kikito Terminal/PS1”.
- O seletor macOS de ícones herdados da marca Warp não é montado no canal OSS, mantendo o ícone KT como identidade do Kikito Terminal.
- `cargo check -p warp --bin warp-oss` — passou após essas mudanças (2m33s).
- A tradução completa da GUI e a verificação runtime de isolamento continuam pendentes.

## Colagem de imagens em CLIs externos

- A primeira tentativa detectava agentes pelo comando ativo e aliases de shell registrados, mas o teste manual do usuário com `codex-keener` falhou: nada foi inserido no CLI.
- O comando `codex-keener` não correspondia ao nome exato `codex`; a detecção agora também reconhece wrappers `codex-*` e `claude-*`. O atalho Ctrl+V do Windows já estava ligado à ação de colar.
- A colagem usa a área de transferência local e envia o sinal de colagem de imagem ao CLI. Não depende da IA interna, login ou serviços externos.
- `cargo check -p warp --bin warp-oss` — passou após o ajuste de nomes dos wrappers (2m04s).
- O teste manual da correção com `codex-keener` e uma imagem real ainda está pendente.

## Retomada — ações da Aparência na paleta

- Nomes de ações da paleta para alternar configurações de Aparência foram traduzidos para pt-BR, incluindo modo compacto, cursor, divisores de blocos, painéis, abas, foco, ligaturas e espaçamento da tela alternativa.
- A dica de seleção sem cor em “Cores das abas por pasta” agora aparece como “Padrão (sem cor)”.
- Mantive fora desta rodada os rótulos associados ao tipo e ao posicionamento da entrada do terminal.
- `git diff --check` — passou; o Git reportou apenas a conversão LF/CRLF habitual no Windows.
- A tradução completa da GUI e a verificação runtime de isolamento continuam pendentes.

## Retomada — configurações gerais

- Traduzi os nomes visíveis das seções de configurações, sem alterar os slugs internos usados para restauração de sessão e compatibilidade com a CLI.
- A paleta agora usa “Ativar/Desativar” para ações de configuração; também traduzi ações de depuração, menu de fechar painel e o estado vazio da busca.
- Traduzi dicas genéricas de configurações, restauração do padrão e o rodapé para abrir o arquivo de configurações. A ação de assistência por IA foi rotulada sem a marca antiga.
- `git diff --check` — passou nos arquivos desta rodada; restaram apenas avisos de conversão LF/CRLF do Git no Windows.
- A revisão das páginas restantes da GUI, inclusive diálogos e fluxos locais, continua pendente; recursos de nuvem e agente ainda precisam ser distinguidos entre removidos e visíveis no canal OSS.

### Retomada — clipboard de imagens do navegador

- O usuário confirmou que colar arquivos copiados do Explorer funciona, mas imagens copiadas diretamente da web não.
- A leitura de clipboard do Windows via `arboard` cobre `PNG` e `CF_DIBV5`; adicionei fallback local para `CF_DIB`, convertendo o bitmap para PNG sem requisições externas.
- `cargo check -p warp --bin warp-oss` — passou após a implementação (5m11s).
- `rustfmt` foi aplicado ao arquivo alterado e `git diff --check` passou.
- O clipboard observado durante a análise continha formatos de arquivo do Explorer, então os formatos usados no caso de falha do navegador ainda precisam de confirmação pelo teste manual.
- O teste continuou falhando; o usuário observou na lista `Win+V` uma imagem representada por caminho PNG local.
- A investigação encontrou que `WindowsClipboard::write` descartava `ClipboardContent.images`; o fluxo de imagem do rich input e do arraste já chama essa escrita antes de emitir o atalho do CLI.
- A escrita do clipboard Windows agora decodifica a imagem para RGBA e publica via `arboard`. O Ctrl+V no CLI também encaminha caminhos que apontam somente para imagens pela rotina local que lê o arquivo, atualiza o clipboard e envia o sinal de colagem; aliases detectados no comando ativo são aceitos.
- O sinal de colagem no Windows agora diferencia Claude (Alt+V) dos demais CLIs (payload bracketed paste vazio), alinhado ao tratamento direto de imagens no terminal.
- `cargo check -p warp --bin warp-oss` — passou após essas mudanças (1m23s); `rustfmt` e `git diff --check` passaram.
- A validação manual com imagem originada no navegador ainda está pendente.
- `CARGO_BIN_NAME=oss WARP_APP_NAME="Kikito Terminal" cargo run -p warp --bin warp-oss` — abriu a versão atualizada para validação manual.
