# Progresso

## Escopo do Kikito Terminal

- Decisão do usuário: manter o aplicativo local, sem backend Kikito, login Warp, telemetria, relatórios de falha ou atualização automática. O foco é gerenciar terminais, abas e processos locais.
- Os binários OSS de GUI e TUI usam configuração local inerte. O fluxo de inicialização não exige login; credenciais locais existentes não são apagadas.
- Bloqueios no perfil OSS cobrem clientes HTTP compartilhados, assets remotos, subscriptions GraphQL, MCP, telemetria, crash reporting, tracing OTLP, credenciais AWS/SSO, OIDC/STS e abertura de links HTTP/HTTPS externos. Endereços loopback continuam permitidos.
- A interface OSS oculta menus e páginas que dependem de conta, Drive, colaboração ou serviços remotos. Identificadores técnicos herdados, como `dev.warp.WarpOss`, foram mantidos para compatibilidade com dados e integrações locais.
- Nome, ícones, metadados e instaladores do canal OSS usam Kikito Terminal e a identidade KT aprovada. A tradução pt-BR avançou em menus, configurações, paleta e busca, mas ainda não cobre toda a interface.

## Colagem de imagens em CLIs

- Ctrl+V e o botão “Colar caminho da imagem” inserem o caminho local no prompt do Rich Input ou do CLI ativo.
- Se a área de transferência contiver dados da imagem, o app salva um arquivo temporário em `%TEMP%/Kikito Terminal/clipboard-images` e insere seu caminho. Caminhos de imagens já copiados entram diretamente no prompt.
- O usuário confirmou manualmente que a colagem agora funciona como esperado.
- `cargo test -p warp --lib paste_raw_image_clipboard_in_cli_agent_inserts_local_path -- --nocapture` passou. O teste verifica que os bytes da imagem são gravados e o caminho chega ao prompt.

## Validação do modo local

- `cargo check -p warp --bin warp-oss` passou em várias rodadas anteriores, incluindo após os bloqueios de rede, mudanças de marca e localização parcial.
- `cargo check -p warp_tui --bin warp-tui-oss` passou com `PROTOC` definido para o executável instalado pelo WinGet.
- O app OSS abriu após as mudanças mais recentes.
- Uma inspeção das conexões do processo em repouso encontrou apenas um listener loopback em `127.0.0.1:9282`, sem conexão TCP externa naquele momento. Isso não comprova o comportamento de todos os fluxos acionados pelo usuário.
- `git diff --check` passou nos arquivos alterados; o Git mostra apenas avisos habituais de conversão LF/CRLF no Windows.

## Pendências

- **Isolamento de rede:** verificar conexões enquanto ações da interface são usadas; a inspeção em repouso e os bloqueios de código não cobrem cada fluxo em runtime.
- **Tradução:** completar a localização das superfícies locais visíveis em pt-BR, incluindo diálogos, tooltips, mensagens, acessibilidade e estados vazios. Conteúdo exclusivamente remoto deve permanecer fora do escopo OSS.
- **TUI:** manter `PROTOC` disponível no `PATH` ou configurado em `PROTOC` para builds locais.

**Status: EM ANDAMENTO.** A colagem de imagens está resolvida e confirmada pelo usuário. A localização e a verificação abrangente de rede continuam em andamento.

## Tradução — paleta, busca, abas e worktrees

- Traduzi rótulos e orientações de acessibilidade do seletor de arquivos, busca de comandos, estados vazios, itens recentes/sugeridos e busca do histórico.
- Traduzi os diálogos locais de configuração de abas e worktrees: títulos, instruções, seletores de repositório/branch, campos, carregamento, botões e confirmação de remoção.
- Atalhos, actions, IDs e slugs foram preservados.
- `git diff --check` passou nos arquivos dos lotes; os avisos são apenas de conversão LF/CRLF do Windows.
- A localização das superfícies locais restantes da GUI continua pendente.

## Tradução — editor local de temas

- Traduzi o modal de criação de tema por imagem, incluindo instruções, seleção da imagem, nome, cor de fundo, ações e mensagens de erro.
- Traduzi a confirmação de exclusão de tema, suas ações e a mensagem de erro local.
- Mantive IDs, actions e o atalho Escape.
- `git diff --check` passou nos arquivos deste lote; restaram apenas avisos de conversão LF/CRLF do Git no Windows.
- `progress.md` está em UTF-8 válido, sem U+FFFD nem texto corrompido.


## Tradução — salvamento de configuração

- Traduzi o modal local que salva a configuração de janelas, abas e painéis como YAML: título, botões, explicação, status, erros e acessibilidade.
- Mantive intactos o texto técnico usado para classificar arquivo existente, IDs e os atalhos Enter/Escape.
- A ligação para documentação externa permanece fora do lote por depender de um destino remoto.
- `git diff --check` passou; `progress.md` está em UTF-8 válido, U+FFFD=0 e sem sequências `??` em palavras.


## Tradução — editor da barra de ferramentas

- Traduzi o modal local de edição da barra de ferramentas do cabeçalho: título, grupos, ações e nomes dos itens configuráveis.
- Mantive enums serializados, actions e atalhos; as mudanças afetam apenas os rótulos visíveis.
- `git diff --check` passou nos arquivos deste lote; restam apenas avisos habituais de conversão LF/CRLF no Windows.


## Tradução — confirmação de alterações não salvas

- Traduzi o diálogo local de variáveis de ambiente, incluindo a mensagem e as ações para continuar editando ou descartar alterações.
- Mantive as actions e o comportamento de cada botão.
- `git diff --check` passou neste lote; o Git mostra apenas o aviso habitual de conversão LF/CRLF no Windows.


## Tradução — modal de expressões regulares de privacidade

- Traduzi os rótulos, o exemplo de nome, o estado de validação e as ações do modal local que adiciona uma regra de expressão regular.
- Preservei a sintaxe do campo de expressão regular, as actions e os atalhos de navegação/fechamento.
- `git diff --check` passou neste lote; restam apenas avisos habituais de conversão LF/CRLF no Windows.


## Tradução — seletor de cores por diretório

- Traduzi os rótulos do seletor local que adiciona diretórios à configuração de cores das abas.
- Mantive o fluxo de seleção e abertura do seletor nativo de pastas sem alterações.
- `git diff --check` passou neste lote; o Git mostra apenas o aviso habitual de conversão LF/CRLF no Windows.


## Tradução — seleção de suporte a linguagens

- Traduzi a orientação e as ações do bloco local que oferece instalação/ativação de servidores de linguagem para uma base de código.
- Mantive as actions e o atalho Enter.
- `git diff --check` passou neste lote; restam apenas avisos habituais de conversão LF/CRLF no Windows.


## Tradução — editor de comandos secretos

- Traduzi o rótulo, os campos de nome/comando e os botões do diálogo local de comandos secretos para variáveis de ambiente.
- Mantive as actions e o salvamento do comando sem alterações.
- `git diff --check` passou neste lote; o Git mostra apenas o aviso habitual de conversão LF/CRLF no Windows.


## Tradução — banner do modo Vim

- Traduzi a pergunta e a ação do banner local que oferece ativar os atalhos de teclado do Vim.
- Mantive actions, estado do banner e comportamento de ativação/dispensa.
- `git diff --check` passou neste lote; o Git mostra apenas o aviso habitual de conversão LF/CRLF no Windows.


## Tradução — confirmação de remoção de endpoint

- Traduzi o título, a explicação e os botões do diálogo local que confirma a remoção de um endpoint personalizado.
- Mantive as actions e o comportamento de cancelar/confirmar.
- `git diff --check` passou neste lote; restam apenas avisos habituais de conversão LF/CRLF no Windows.


## Tradução — seleção de modelo padrão

- Traduzi os botões do modal local que permite escolher um modelo padrão após configurar uma credencial ou endpoint personalizado.
- Mantive actions, escolhas de modelo e o fechamento pelo atalho Escape.
- A descrição dinâmica permanece fornecida pelo chamador e não foi alterada neste lote.


## Tradução — banner de encerramento do shell

- Traduzi os estados locais de encerramento/falha ao iniciar o shell, a explicação da falha de inicialização e os rótulos dos botões.
- Preservei actions, URLs e o texto de erro bruto fornecido pelo sistema operacional.
- Permanecem outras superfícies locais da GUI a localizar; este lote cobriu apenas o banner de encerramento do shell.


## Tradução — configurações de automação local

- Traduzi o título da página, estados de instalação, mensagens e descrições da CLI de controle local.
- Mantive `warpctrl` e `{command_name}` como literais técnicos, além de actions e comportamento.
- Este lote cobre apenas a página de automação; outras superfícies locais da GUI ainda precisam de varredura.


## Tradução — atalhos de visibilidade da barra de abas

- Traduzi os nomes dos três comandos de aparência que controlam quando a barra de abas fica visível.
- Mantive os bindings, actions e flags originais.
- Outras strings ligadas a recursos remotos da mesma página ficaram fora deste lote.


## Tradução — banner de arquivos no terminal

- Traduzi as mensagens e ações do banner local de visualização de Markdown e edição de arquivos de código no terminal.
- Atualizei os nomes visíveis para Kikito Terminal; actions, identificadores e destinos das URLs foram preservados.
- Outras superfícies locais da GUI ainda precisam de varredura.


## Tradução — configurações locais de editor e código

- Traduzi título, controles e descrições da página de editor, revisão de código, explorador de projetos, busca de arquivos, arquivos ocultos, formatação ao salvar, salvamento automático e numeração de linhas.
- Preservei actions, flags, termos técnicos como LSP/diff e comportamento.
- Ainda há outras superfícies locais da GUI para revisar.


## Tradução — teclas modificadoras Meta

- Traduzi os rótulos de configuração das teclas Option/Alt esquerda e direita usadas como Meta.
- Mantive os termos técnicos Option, Alt e Meta, assim como actions e armazenamento da configuração.
- A varredura da página de recursos continua; recursos remotos e nomes técnicos permanecem fora deste lote.


## Tradução — inicialização e encerramento do aplicativo

- Traduzi as preferências de restauração de janelas/abas/painéis, aviso ao sair, inicialização com o sistema e encerramento após fechar as janelas.
- Traduzi também o aviso local do Wayland; mantive flags, actions, links e comportamento.
- A varredura da página de recursos segue em andamento.


## Tradução — relatórios do terminal e sinal sonoro

- Traduzi os controles locais de relatórios de mouse, rolagem e foco, além da opção de sinal sonoro.
- Preservei actions, flags, links e o comportamento dos controles.
- A varredura da página de recursos continua em outros grupos.


## Tradução — opções de edição Vim

- Traduzi os rótulos para ativar atalhos Vim, usar o registrador sem nome como clipboard do sistema e exibir a barra de status do Vim.
- Mantive Vim como nome técnico e preservei actions, flags e comportamento.
- Outros grupos locais da página de recursos ainda estão pendentes.


## Tradução — seleção de texto e clipboard

- Traduzi opções locais de seleção inteligente, cópia ao selecionar e acesso de programas ao clipboard por OSC 52.
- Mantive OSC 52, actions, flags, links e comportamento.
- Ainda há outros grupos locais da página de recursos para revisar.


## Tradução — entrada e sugestões do terminal

- Traduzi rótulos de linha de mensagem, foco da entrada, dicas/botão de sugestões, comportamento da tecla Tab e mensagens de ajuda dos atalhos.
- Mantive os nomes internos usados para localizar bindings, as teclas, actions, flags e comportamento.
- A varredura da página de recursos continua nos grupos locais restantes.


## Tradução — rolagem e limites de exibição

- Traduzi a preferência de linhas por rolagem do mouse, os valores aceitos, o limite de linhas por bloco e a indicação do Kikito Terminal como terminal padrão.
- Mantive valores numéricos, actions, flags, links e comportamento.
- A página de recursos ainda tem outros grupos locais a revisar.


## Tradução — abertura e dicas de links

- Traduzi os controles locais para abrir links no aplicativo de desktop e exibir dicas ao clicar em links.
- Mantive actions, flags, destinos e comportamento.
- Outros grupos locais da página de recursos continuam pendentes.


## Tradução — cabeçalho fixo de comandos

- Traduzi o rótulo local da opção que mantém o cabeçalho do comando visível nos blocos do terminal.
- Mantive action, flag, link de documentação e comportamento.
- A varredura de recursos locais continua.
