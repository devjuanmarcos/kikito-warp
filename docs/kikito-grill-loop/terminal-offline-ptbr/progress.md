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
- `git diff --check` passou; `progress.md` está em UTF-8 válido, U+FFFD=0 e sem sequências de duas interrogações em palavras.


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


## Tradução — navegação entre abas

- Traduzi os rótulos e opções de posição de novas abas, comportamento de Ctrl+Tab e comportamento da tecla Tab para sugestões.
- Mantive variantes/enums, actions, atalhos e valores de configuração.
- Outros grupos locais da página de recursos continuam pendentes.


## Tradução - renderização e sistema de janelas

- Traduzi os controles de GPU integrada e backend gráfico, incluindo o aviso de aplicação em novas janelas.
- Traduzi a descrição do uso de Wayland e seu aviso sobre atalhos globais e escala fracionária; atualizei a referência de marca para Kikito Terminal.
- Mantive os nomes técnicos dos backends, actions, flags e comportamento.
- A auditoria da página de recursos continua.


## Tradução - busca assíncrona

- Traduzi o nome da busca assíncrona e a explicação sobre manter a interface responsiva em saídas extensas.
- Mantive a flag, action, status beta e comportamento da configuração.


## Tradução - notificações locais

- Traduzi controles de notificação da área de trabalho, conclusão de tarefas de agentes, avisos de atenção e sons.
- Traduzi os textos de duração e limite das notificações temporárias, atualizando a marca para Kikito Terminal.
- Mantive actions, configurações e comportamento.


## Tradução - novas sessões

- Traduzi os rótulos de shell padrão, diretório de trabalho e modo padrão para novas sessões.
- Mantive os valores dos seletores, configurações locais e comportamento.


## Tradução - encerramento de sessões

- Traduzi a confirmação exibida antes de encerrar uma sessão compartilhada.
- Mantive a configuração local, action e comportamento.


## Tradução - notas da versão

- Traduzi a opção para mostrar as notas da versão após atualizações.
- Mantive a configuração local, action e comportamento.


## Tradução - navegação das configurações

- Traduzi os nomes locais das seções Recursos e Privacidade na navegação das configurações.
- Mantive os identificadores de seção, slugs e comportamento.


## Tradução - indexação de código

- Traduzi títulos, descrição da indexação automática e instruções para excluir arquivos.
- Mantive comandos grep/find, o nome do arquivo .warpindexingignore, termos técnicos e comportamento.


## Tradução - disponibilidade da indexação

- Traduzi avisos de ativação/desativação por administradores e de requisitos de IA, além do aviso de limite de índices.
- Mantive o nome dinâmico da equipe, mensagens de falha usadas internamente e comportamento.


## Tradução - estados locais de indexação

- Traduzi a ação para adicionar pasta, o estado vazio, o botão de regras do projeto e estados/andamento da indexação local.
- Traduzi estados e ações de servidores LSP locais; preservei placeholders, identificadores e comportamento.
- Traduzi também os estados de instalação/disponibilidade de servidores LSP e seus controles locais de reiniciar/ver registros.


## Tradução - privacidade local

- Traduzi os títulos e descrições de ocultação de segredos e regras regex.
- Mantive a sintaxe da flag `(?i)`, a semântica e o comportamento local.


## Tradução - controles de regras de segredos

- Traduzi o título da página, o modo de exibição visual, o botão de adicionar regex e a seção de regras recomendadas.
- Mantive regexes, actions e comportamento.


## Tradução - materiais de fundo da janela

- Traduzi as opções Sem material e Acrílico no seletor de fundo da janela.
- Mantive os nomes de plataforma Mica e Mica Alt, os valores e o comportamento.


## Tradução - nomes dos ícones do aplicativo

- Traduzi os nomes descritivos das variantes de ícone, incluindo Clássico, Cometas, Brilho, Holográfico e Adesivo.
- Atualizei o nome visível Warp 1 para Kikito 1 e mantive intactas as variantes e os valores internos.


## Tradução - comandos de posição da entrada

- Traduzi os comandos locais para iniciar a entrada no topo e fixar nas bordas superior e inferior.
- Mantive actions, modos e comportamento.


## Tradução - comandos da barra de ferramentas de agentes CLI

- Traduzi o título da página e os textos para configurar comandos regex que exibem a barra de ferramentas.
- Mantive `CLI`, regex, nomes de agentes, actions e comportamento.


## Tradução - controles de agentes CLI e entrada avançada

- Traduzi controles e descrições de barra de ferramentas, abertura/fechamento automáticos da Entrada avançada e envio com Ctrl+Enter.
- Mantive nomes dos comandos, tecla de atalho, actions e comportamento.


## Tradução - seletor de agente CLI

- Traduzi o item Outro e o cabeçalho do seletor de agente de programação, mantendo o rótulo de seleção consistente.
- Preservei o valor de agente desconhecido, actions e comportamento do seletor.


## Tradução - layout da barra de ferramentas

- Traduzi o cabeçalho do editor compartilhado de layout da barra de ferramentas dos agentes.
- Mantive o editor, as actions e o comportamento.


## Tradução - modos de exibição de segredos

- Traduzi os nomes exibidos das opções Asteriscos, Tachado e Sempre mostrar segredos.
- Mantive os variants, serialização e valores de configuração.

## Tradução — seletor de repositórios para worktrees

- Traduzi a busca e a ação para adicionar repositórios no seletor local de worktrees.
- Mantive os filtros, caminhos e ações existentes.

## Tradução — aviso de recuperação no Linux

- Traduzi o aviso local de recuperação após falha na inicialização e a ação “Saiba mais”.
- Mantive Xwayland e o comportamento de recuperação.

## Tradução — estado de inicialização do shell

- Traduzi a indicação de inicialização do shell e mantive o nome do shell selecionado.
- Preservei as mensagens específicas de configuração remota e seus estados.

## Tradução — busca global local

- Traduzi busca, filtros regex/case sensitivity, contagem e avisos de resultados, além dos estados locais sem resultados ou sem sessão compatível.
- Mantive `.gitignore`, Git Bash, WSL e os estados específicos de sessão remota.

## Tradução — nomes dos shells locais

- Traduzi os rótulos de shell padrão, personalizado, sistema e ambiente Docker exibidos nos seletores locais.
- Mantive nomes de comandos, distribuições WSL, caminhos e valores de telemetria.

## Tradução — erro ao continuar conversa local

- Traduzi o aviso exibido quando não é possível continuar uma conversa local de agente.
- Mantive a referência aos logs e o fluxo de recuperação.

## Tradução — divisores de sessão restaurada

- Traduzi os rótulos de conversa restaurada e sessão anterior no histórico do terminal, incluindo a data associada.
- Mantive a lógica e formatei a data como `dd/mm/aaaa às hh:mm`.

## Tradução — atalhos e terminal padrão

- Traduzi ações locais para configurar o atalho global, habilitar Wayland e definir o terminal padrão.
- Atualizei a ação para exibir Kikito Terminal, preservando suas actions e flags.

## Tradução — diretório inicial de novas sessões

- Traduzi as opções de diretório pessoal, sessão anterior e diretório personalizado.
- Mantive os modos e valores persistidos de configuração.

## Tradução — permissões de clipboard por OSC 52

- Ajustei os rótulos das opções para não permitir, permitir gravação ou permitir leitura e gravação.
- Mantive os enums e as permissões associadas a cada opção.

## Tradução — nomes gerados para configurações de worktree

- Traduzi os títulos das configurações geradas para novos worktrees.
- Mantive nomes de repositórios, branches e dados de configuração.

## Tradução — falha ao exportar logs

- Traduzi a mensagem exibida quando a criação do pacote local de logs falha.
- Mantive os detalhes do erro e o contexto técnico dos registros internos.

## Tradução — ações de IA nos blocos do terminal

- Traduzi as ações visíveis para pedir ajuda ao agente, anexar contexto e perguntar à IA do Kikito Terminal.
- Mantive as actions e o comportamento dos botões.

## Tradução — descrições das configurações de blocos

- Traduzi as descrições das preferências de notificação, foco da entrada e divisórias dos blocos.
- Mantive as chaves TOML, valores e comportamento das configurações.

## Tradução — descrições das configurações de abas

- Traduzi descrições de posicionamento, botão de fechar, visibilidade e cores das abas nos metadados das configurações.
- Preservei os campos, valores serializados e chaves TOML.

## Tradução — erros do arquivo de configurações

- Traduzi cabeçalhos, detalhes e consequências exibidos quando o TOML tem erro de sintaxe ou valores inválidos.
- Preservei as chaves das configurações e a lógica que aplica valores padrão.

## Tradução — ação de reorganizar a barra de ferramentas

- Traduzi a ação do menu de contexto do cabeçalho que abre o editor da barra de ferramentas.
- Mantive a action de abertura do editor.

## Tradução — rótulo de agente CLI desconhecido

- Traduzi o nome exibido para agentes CLI personalizados ou não reconhecidos.
- Mantive o variant `Unknown`, os dados serializados e a telemetria.

## Tradução — metadados das configurações do terminal

- Traduzi descrições de OSC 52, espaçamento e preenchimento em tela cheia, sinal sonoro, grade, bloco inicial da IA e busca assíncrona.
- Preservei chaves TOML, enums e valores técnicos `deny`, `write_only` e `read_write`.

## Tradução — mensagens de coleta de amostras

- Traduzi o sucesso e as falhas da coleta local de amostras de processo.
- Mantive o caminho do arquivo gerado e a indicação para consultar os logs.

## Tradução — filtro de saída dos blocos

- Traduzi o placeholder, tooltips e instruções de acessibilidade do filtro de saída do terminal.
- Mantive regex, ações de filtro e atalhos.

## Tradução — configurações de janela

- Traduzi as descrições de desfoque, fundo, opacidade, tamanho, painel e zoom da janela.
- Preservei as chaves TOML, os valores e as opções técnicas.

## Tradução — menu local de depuração

- Traduzi os rótulos locais de depuração do shell, gravação PTY, blocos e exportação CSV.
- Mantive atalhos, siglas e o identificador técnico `warp.pty.recording`.

## Tradução — configurações gerais

- Traduzi as descrições de encerramento, restauração de sessão, inicialização, dicas de links e revisão de código.
- Preservei as chaves TOML e o comportamento de cada configuração.
