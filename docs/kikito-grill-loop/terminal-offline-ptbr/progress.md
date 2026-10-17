# Progresso

## Escopo do Kikito Terminal

- Decisão do usuário: manter o aplicativo local, sem backend Kikito, login Warp, telemetria, relatórios de falha ou atualização automática. O foco é gerenciar terminais, abas e processos locais.
- Os binários OSS de GUI e TUI usam configuração local inerte. O fluxo de inicialização não exige login; credenciais locais existentes não são apagadas.
- Bloqueios no perfil OSS cobrem clientes HTTP compartilhados, assets remotos, subscriptions GraphQL, MCP, telemetria, crash reporting, tracing OTLP, credenciais AWS/SSO, OIDC/STS e abertura de links HTTP/HTTPS externos. Endereços loopback continuam permitidos.
- Fluxos de autorização de provedores integrados (SuperGrok, assinatura ChatGPT e Gemini Enterprise) também ficam desativados no perfil OSS, inclusive quando havia preferências antigas persistidas.
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

## Tradução — eventos em tela cheia

- Traduzi as descrições de encaminhamento de eventos do mouse, rolagem e foco para aplicativos de terminal em tela cheia.
- Mantive as chaves TOML e o comportamento das opções.

## Tradução — direção dos blocos

- Traduzi a descrição do modo que define como os blocos são organizados na área visível do terminal.
- Preservei os valores serializados do enum.

## Tradução — ligaduras tipográficas

- Traduzi a descrição da configuração que controla a exibição de ligaduras no terminal.
- Mantive a chave TOML e a feature flag.

## Tradução — configurações de teclas de atalho

- Traduzi descrições de Quake Mode, atalhos globais, teclas meta e o comportamento de Ctrl+Tab.
- Preservei os caminhos de configuração, exemplos de atalhos e os nomes das teclas.

## Tradução — ocultação de segredos

- Traduzi as descrições dos modos de exibição e ocultação de segredos detectados na saída do terminal.
- Preservei os enums, a compatibilidade com a configuração legada e os caminhos TOML.

## Tradução — preferências de notificação

- Traduzi os estados e as descrições de notificações para eventos do terminal, como comandos
  demorados, solicitações de senha e tarefas do agente.
- Preservei enums, campos, valores e o limite em segundos.

## Tradução — layout das barras de ferramentas de agentes

- Traduzi descrições das opções de layout padrão ou personalizado para as barras do Agent Mode e de
  agentes CLI.
- Preservei os enums e as listas serializadas de itens à esquerda e à direita.

## Tradução — preferências de shell e sessão

- Traduzi as descrições de shell inicial, shell para novas sessões, prompt PS1 e confirmação ao fechar
  sessão.
- Preservei os campos e caminhos TOML.

## Tradução — opções de shell

- Traduzi descrições dos tipos de shell para novas sessões e das opções de shell de inicialização.
- Preservei os valores `bash`, `zsh`, `fish`, `pwsh`, `null` e a estrutura serializada.

## Tradução — menus de contexto do terminal

- Traduzi ações locais de copiar/colar, selecionar texto, copiar caminhos e URLs, abrir links e
  inserir texto na entrada.
- Preservei as ações internas, os atalhos e o comportamento de abertura de arquivos.

## Tradução — agrupamento de abas

- Traduzi as ações de criar grupo e remover abas de um grupo, além do nome padrão de grupo sem
  título.
- Preservei as ações internas e a lógica de agrupamento.

## Tradução — tooltips do painel lateral

- Traduzi os tooltips locais do explorador de projetos, busca global, conversas do agente e botão de
  fechar painel.
- Preservei os atalhos e as ações associados.

## Tradução — estados e rótulos das abas verticais

- Traduzi tooltips de opções/configuração, o estado vazio sem abas e os rótulos padrão de aba e
  sessão.
- Preservei atalhos, ações e valores internos.

## Tradução — busca e criação de abas

- Traduzi os textos de orientação da busca de repositórios e abas, além dos tooltips para nova aba e
  configurações de abas.
- Preservei atalhos e ações.

## Tradução — painel direito do workspace

- Traduzi tooltips de maximização, inicialização de repositório e fechamento do painel, além do título
  de revisão de código.
- Preservei atalhos, ações e navegação do painel.

## Tradução — lista local de conversas

- Traduzi busca, estado vazio, explicação de conversas, criação, exclusão e ramificação local em nova
  aba ou painel.
- Preservei as ações e deixei o item de compartilhamento condicionado ao serviço remoto sem alteração.

## Tradução — nomes padrão de grupos e painéis

- Traduzi os nomes de preenchimento usados quando grupos ou painéis ainda não têm um título definido.
- Preservei os nomes personalizados e o comportamento de edição.

## Tradução — mensagens de arquivos locais do workspace

- Traduzi notificações de falha ao carregar ou remover configurações locais e os links para abrir os
  arquivos.
- Traduzi mensagens de sucesso ou falha ao salvar um perfil de heap e o atalho para exibi-lo no
  explorador.
- Preservei os caminhos, IDs das notificações e ações.

## Tradução — mensagens do ciclo de conversas

- Traduzi notificações locais de falha ao carregar, copiar e excluir conversas, incluindo o título
  padrão de conversa.
- Preservei as ações e os IDs; as mensagens não alteram o fluxo de dados.

## Tradução — falha ao abrir sessão

- Traduzi o aviso exibido quando o Kikito Terminal não pode criar uma nova sessão de terminal.
- Preservei a condição, o fluxo e a ação que apresentam a notificação.

## Tradução — menu de contexto do explorador de arquivos

- Traduzi ações para abrir, criar, renomear e excluir arquivos, navegar até diretórios, anexar
  contexto e copiar caminhos.
- Traduzi os rótulos para exibir itens no Finder, Explorador de Arquivos ou gerenciador de arquivos.
- Preservei as ações internas e as condições de disponibilidade.

## Tradução — inicialização de projeto e ambiente

- Traduzi o selo de opção sugerida, a ação de gerar novamente `AGENTS.md` e o botão para ver o status
  da indexação.
- Preservei o nome do arquivo e as ações internas.

## Tradução — menu de contexto dos blocos do terminal

- Traduzi ações locais para copiar saídas, comandos e prompts; buscar, rolar, filtrar, marcar e limpar
  blocos; e editar prompts e barras de ferramentas.
- Traduzi ações de cópia e reversão de conversas exibidas no menu local.
- Preservei as ações e os atalhos; mantive intactos os itens remotos de fluxo de trabalho e
  compartilhamento.

## Tradução — conversas na paleta de comandos

- Traduzi seções, ações de criar, retomar e copiar conversas, dicas e instruções de acessibilidade.
- Traduzi avisos quando o agente está monitorando um comando e impede a troca ou o início de conversa.
- Preservei ações, navegação e condições de bloqueio.

## Tradução — controles do editor de código

- Traduzi ações de navegação por símbolos, gerenciamento de alterações, cópia de caminho, acesso à
  pasta e prévia Markdown.
- Preservei as ações internas e o comportamento de edição.

## Tradução — navegação de sessões na paleta

- Traduzi estados de sessão vazia, horário ausente e tempo desde a conclusão, além da dica para abrir
  uma sessão.
- Preservei a lógica de navegação e os dados de data/hora.

## Tradução — controles de sugestões do editor

- Traduzi o item de atalho personalizado e as dicas para alterar o atalho ou ignorar uma sugestão.
- Preservei as ações e os atalhos disponíveis.

## Tradução — navegação e revisão no editor

- Traduzi o botão de rejeitar alterações e o placeholder do campo de navegação para linha/coluna.
- Preservei a ação de reversão e o separador `:` do formato.

## Tradução — menu de contexto dos blocos de IA

- Traduzi ações para copiar prompts, saídas, comandos, branches, horários e texto de conversa, além
  de criar cópias e reverter até um ponto.
- Mantive intactos os itens de compartilhamento que dependem do serviço remoto.
- Preservei as ações internas e as condições de disponibilidade.

## Tradução — acessibilidade da paleta de comandos

- Traduzi os rótulos acessíveis e as instruções para abrir configurações de inicialização e iniciar
  sessões.
- Preservei os itens selecionados, as ações e as configurações carregadas.

## Tradução — ações de links em notebooks

- Traduzi as ações acessíveis para iniciar uma sessão no diretório vinculado e abrir arquivos
  Markdown no editor.
- Preservei os destinos e o comportamento de abertura dos links.

## Tradução — erros de resolução de links

- Traduzi as mensagens exibidas quando o arquivo vinculado não existe, falta o diretório base ou o
  link está inválido.
- Preservei as variantes e a lógica de resolução.

## Tradução — busca e substituição no editor

- Traduzi rótulos, dicas, estados sem resultados ou em busca e mensagens acessíveis das barras de
  busca em notebooks e no editor de código.
- Preservei IDs de ações, atalhos, estado da busca e comportamento de substituição.

## Tradução — seletor de versões do Node.js

- Traduzi os estados vazios, as instruções e o título da lista de versões instaladas.
- Mantive os nomes `Node.js` e `nvm` e o comando exibido para instalar a versão mais recente.

## Tradução — comandos de notebook

- Traduzi descrições dos comandos para aumentar, diminuir e restaurar o tamanho da fonte e focar a
  entrada do terminal.
- Preservei IDs, ações, contextos e atalhos registrados.

## Tradução — ações de configurações de aba

- Traduzi os botões para definir uma configuração como padrão, editá-la e removê-la, além da dica
  que informa o estado padrão.
- Preservei os eventos e as condições de disponibilidade das ações.

## Tradução — seletor de tipo de sessão

- Traduzi o rótulo do seletor e substituí a marca antiga no nome da opção de agente por “Agente”.
- Preservei os tipos de sessão e a seleção correspondente.

## Tradução — estados de espera de agentes locais

- Traduzi as mensagens padrão exibidas quando um agente CLI aguarda resposta ou precisa de interação.
- Preservei os estados, eventos e a mensagem fornecida pelo próprio agente quando disponível.

## Tradução — descrições das configurações de abas verticais

- Traduzi descrições de layout, conteúdo e comportamento das abas verticais e da barra de ferramentas.
- Mantive sem alteração as descrições relacionadas a revisão de código e links de pull request.
- Preservei chaves, enums e valores das configurações.

## Tradução — aviso de recursos do cabeçalho de painéis

- Traduzi o aviso sobre abrir arquivos e revisar diferenças de código exibido no cabeçalho do painel.
- Preservei o fluxo de exibição e dispensa do aviso.

## Tradução — tela inicial

- Traduzi o título, a descrição e as ações para abrir uma sessão de terminal na tela de boas-vindas.
- Atualizei a marca exibida para Kikito Terminal e preservei os atalhos e ações.

## Tradução — expansão de notificações

- Traduzi os botões e o rótulo acessível que expandem ou recolhem mensagens longas de notificações.
- Preservei o estado e a ação de expansão.

## Tradução — selo de novos recursos

- Traduzi o texto do selo exibido em avisos de novos recursos.
- Preservei a variante visual e o fluxo de dispensa.

## Tradução — confirmação de exclusão de conversa

- Traduzi o título, a explicação e os botões do diálogo de confirmação.
- Mantive os atalhos e as ações de confirmar e cancelar.

## Tradução — estado indisponível do explorador de projetos

- Traduzi o título exibido quando o explorador de projetos não pode ser mostrado.
- Preservei o conteúdo dinâmico e o estado do explorador.

## Tradução — confirmação de reversão do agente

- Traduzi o título, a explicação, a observação e os botões do diálogo de reversão.
- Preservei os atalhos e as ações de confirmação e cancelamento.

## Tradução — opção de modal nativo

- Traduzi o rótulo da opção para não exibir novamente em modais nativos.
- Preservei o estado persistido da opção e as ações dos botões.

## Tradução — busca da barra de título

- Traduzi o texto de orientação para buscar sessões, agentes e arquivos.
- Preservei o controle e o comportamento de busca existentes.

## Tradução — confirmação de cancelamento do resumo

- Traduzi o aviso e as opções para cancelar ou continuar a geração do resumo.
- Preservei as ações e a tecla Escape para continuar.

## Tradução — dica das configurações de abas

- Traduzi a dica que orienta onde acessar as configurações das abas.
- Preservei o identificador do chip e o comportamento de exibição.

## Tradução — carregamento de sessão

- Traduzi a mensagem exibida enquanto uma sessão está sendo carregada.
- Mantive o estado e a renderização de carregamento.

## Tradução — conclusão das dicas de boas-vindas

- Traduzi o título, a mensagem e o botão do estado final das dicas.
- Mantive a ação de fechar e um rótulo curto para caber no botão.

## Tradução — opção de pular dicas

- Traduzi o controle que permite pular as dicas de boas-vindas.
- Preservei a ação de dispensar as dicas.

## Tradução — argumentos de fluxos de trabalho

- Traduzi rótulos, placeholders e a orientação para preencher argumentos e variáveis de ambiente.
- Preservei a chave interna do prefixo, as ações e a estrutura do editor.

## Tradução — importação de configurações

- Traduzi os botões de importação e restauração das configurações.
- Preservei as ações e os limites de largura dos botões.

## Tradução — editor de links dos cadernos

- Traduzi os placeholders e o botão para inserir links em cadernos.
- Preservei o modelo do editor e a ação que aplica o link selecionado.

## Tradução — visualização de diagramas Mermaid

- Traduzi as dicas dos modos de código-fonte e visualização, além do comando de tela cheia.
- Preservei os modos Mermaid e a ação de abrir o diagrama ampliado.

## Tradução — estados de arquivos em notebooks

- Traduzi as mensagens de carregamento, ausência e falha de leitura de arquivos.
- Traduzi a ação de tentar novamente e preservei o nome dinâmico do arquivo.

## Tradução — dicas dos chips de contexto

- Traduzi dicas para alterar a branch do Git e o diretório de trabalho, além do rótulo desse
  diretório.
- Preservei as ações dos chips e os valores exibidos.

## Tradução — estado do comando editado

- Traduzi o aviso de comando editado e o controle para restaurar o estado original.
- Preservei a ação existente de redefinição do comando.

## Tradução — seleção de temas

- Traduzi as orientações dos modos claro, escuro e geral, além do estado sem resultados.
- Preservei os modos do sistema e o filtro de temas.

## Tradução — inserção de blocos em cadernos

- Traduzi a dica do controle que abre o menu de inserção de blocos.
- Preservei o identificador do botão e o comportamento do menu.

## Tradução — título inicial do caderno

- Traduzi o placeholder do título vazio do caderno para Sem título.
- Mantive o valor do título e a persistência sem alteração.

## Tradução — filtro de menus

- Traduzi o placeholder de busca e o estado sem resultados dos menus filtráveis.
- Preservei a lógica de filtragem e seleção.

## Tradução — estados da busca de comandos

- Traduzi os estados de carregamento e de busca sem resultados da paleta de comandos.
- Preservei a coleta, ordenação e seleção dos resultados.

## Tradução — indicador da sessão atual

- Traduzi o selo que identifica a sessão atual nos resultados da paleta de navegação.
- Preservei o contexto da sessão e a navegação existentes.

## Tradução — caminhos de shell e diretório

- Traduzi os placeholders de caminho do executável do shell e do diretório de trabalho.
- Preservei os valores personalizados e as configurações de sessão.

## Tradução — criação de perfil de agente

- Traduzi o botão que inicia a criação de um perfil de agente.
- Preservei o acionamento e a habilitação conforme a disponibilidade de agentes.

## Tradução — cabeçalhos de indexação local

- Traduzi os cabeçalhos de pastas indexadas e servidores LSP.
- Mantive o acrônimo técnico e os dados de indexação.

## Tradução — atalhos globais nas configurações

- Traduzi o rótulo do atalho de teclado e a instrução para definir o atalho global.
- Preservei a captura e a gravação dos atalhos.

## Tradução — ocultar janela Quake Mode

- Traduzi a opção de ocultar a janela ao perder o foco do teclado.
- Preservei o controle e o estado configurável do Quake Mode.

## Tradução — seletor de modelo no prompt

- Traduzi o rótulo da opção para exibir o seletor de modelos no prompt.
- Preservei a configuração e a interface de seleção.

## Tradução — título e acessibilidade do seletor de temas

- Traduzi o título da janela de temas e suas instruções de acessibilidade.
- Preservei o papel da janela e a instrução para fechá-la com Esc.

## Tradução — atalho global no Wayland

- Traduzi o título, o aviso de compatibilidade e o rótulo do link de documentação.
- Mantive a URL, o estado desativado e a ação do atalho.

## Tradução — agrupamento de arquivos no editor

- Traduzi o rótulo e a descrição da preferência para agrupar arquivos abertos na mesma aba em um único painel do editor.
- Preservei a opção e seu comportamento.

## Tradução — estados de permissão do perfil de execução

- Traduzi os quatro estados de permissão exibidos no resumo do perfil: decisão do agente, permitir sempre, perguntar sempre e estado desconhecido.
- Mantive os valores internos e a lógica de permissão.

## Tradução — demais estados de permissão

- Padronizei em português os estados exibidos para escrita, uso do computador, perguntas, execução de agentes e permissões booleanas.
- Preservei as variantes e a lógica internas do perfil.

## Tradução — nomes dos modelos no perfil de execução

- Traduzi o cabeçalho da seção de modelos e os rótulos de modelo base, uso completo do terminal e uso do computador.
- Mantive nomes de modelos, permissões e comportamento.

## Tradução — opções de visualização das abas verticais

- Traduzi os títulos e as opções da seção de visualização no menu das abas verticais.
- Preservei os modos, as ações e os valores internos.

## Tradução — títulos e metadados das abas verticais

- Traduzi os rótulos para escolher informação do painel e configurar metadados, link da PR, estatísticas do diff e detalhes ao passar o cursor.
- Mantive o termo técnico Branch, os nomes de ferramentas e as ações.

## Tradução — confirmação ao fechar sessão

- Traduzi a confirmação de fechamento, a explicação sobre o compartilhamento e os rótulos dos botões e da opção para não exibir novamente.
- Mantive as ações de fechar, cancelar e lembrar a preferência.

## Tradução — controles de maximização do painel de revisão

- Traduzi o nome exibido do atalho e as dicas de ferramenta para maximizar e minimizar o painel de revisão de código.
- Preservei a ação, o ID do atalho e os ícones.

## Tradução — expansão da lista de conversas

- Traduzi os rótulos alternados do botão da lista de conversas: ver todas e mostrar menos.
- Mantive a ação que expande e recolhe a lista.

## Tradução — estados da lista de conversas

- Traduzi a mensagem sem resultados, o aviso ao tentar excluir uma conversa em andamento e o título genérico de conversa.
- Preservei os critérios de exibição e exclusão.

## Tradução — busca e grupos de abas verticais

- Traduzi o estado de busca sem resultados, o contador de abas nos grupos e o nome padrão de grupo sem título.
- Preservei a lógica de busca, agrupamento e contagem.

## Tradução — avisos de inicialização do terminal

- Traduzi os avisos sobre demora ao iniciar o shell e falha no preenchimento automático, incluindo botões e links de ajuda.
- Preservei as URLs, o atalho para abrir configurações e a ação para mostrar o bloco de inicialização.

## Tradução — aviso de atalhos Emacs no terminal

- Traduzi a pergunta sobre o uso de Ctrl+A/Ctrl+E e os botões para escolher entre atalhos Emacs e IDE.
- Preservei as teclas exibidas e as ações de dispensa temporária ou permanente.

## Tradução — sugestões de pergunta ao assistente no terminal

- Traduzi as sugestões exibidas ao pedir explicação sobre uma seleção, um bloco ou um comando.
- Preservei o contexto enviado ao assistente e o fluxo de preenchimento do campo.

## Tradução — notificações de comandos e sessões

- Traduzi os textos de apresentação das notificações e os trechos de status, duração, saída e erro.
- Preservei os dados dinâmicos, o comando exibido e a estrutura dos avisos.

## Tradução — compatibilidade dos prompts de shell

- Traduzi os avisos sobre compatibilidade de Powerlevel10k e Pure, incluindo os links de atualização e ajuda.
- Atualizei a referência de marca para Kikito Terminal e preservei nomes técnicos e URLs.

## Tradução — sugestão para executar um plano

- Traduzi o rótulo e o texto de entrada da sugestão exibida quando um plano é criado.
- Preservei o nome estável da sugestão e o fluxo de execução.

## Tradução — ciclo de vida dos plugins de agentes CLI

- Traduzi mensagens de instalação e atualização bem-sucedidas, erros e instruções de ativação/reinício para Claude Code, Codex e Gemini CLI.
- Traduzi os textos compartilhados dos botões e erros de comando; preservei comandos, nomes dos agentes, caminhos, URLs e configurações literais.
- Substituí as referências visíveis de marca Warp por Kikito Terminal.

## Tradução — aviso da AWS CLI e identidade do plugin externo

- Traduzi o título, a descrição e o botão do aviso local de instalação da AWS CLI.
- Preservei “Warp” nas referências ao plugin e marketplace de terceiros, evitando atribuir essa integração ao Kikito Terminal; comandos, URLs, IDs e ações permanecem intactos.

## Tradução — avisos de notificação no terminal

- Traduzi os estados, explicações e ações do banner local de notificações.
- Atualizei a referência ao aplicativo para Kikito Terminal; nomes de ações, eventos e modos de notificação foram preservados.

## Tradução — aviso de encerramento do shell

- Traduzi os títulos de encerramento normal e prematuro do processo do shell, além da orientação de depuração.
- Atualizei a referência ao script de inicialização para Kikito Terminal; lógica e valores técnicos permanecem iguais.

## Tradução — banner de expansão de aliases

- Traduzi o título e a ação do banner que oferece expansão automática de aliases.
- Atualizei a referência de marca para Kikito Terminal e mantive os valores e a renderização do alias intactos.

## Tradução — configuração do modo de agente

- Traduzi o título, a explicação e o botão do banner de configuração do Agent Mode.
- Atualizei a referência de marca para Kikito Terminal e preservei o comando `/init`, as ações e os identificadores técnicos.

## Revisão — banner do modo Vim

- Revisei o banner local do modo Vim: o título e a ação já estavam em português do Brasil. Mantive o nome Vim e as ações existentes sem alterações no código.

## Revisão — banner para abrir arquivos no terminal

- Revisei o banner de abertura de arquivos: títulos e botões já estavam em pt-BR e identificavam o aplicativo como Kikito Terminal.
- Mantive extensões, nomes de linguagens dinâmicos, URLs, ações e identificadores internos sem alterações.

## Tradução — erros ao enviar notificações

- Traduzi as ações e os títulos locais do banner de erro de notificações, incluindo os títulos de erro e os textos alternativos.
- Atualizei a marca do aplicativo para Kikito Terminal; preservei a lógica e as ações. O banner do modo Vim foi auditado e já estava em pt-BR.

## Tradução — instruções do plugin Warp para OpenCode

- Traduzi títulos, descrições, passos e notas de instalação/atualização do plugin externo.
- Mantive as identidades Warp e OpenCode, o pacote `@warp-dot-dev/opencode-warp`, as chaves JSON, caminhos, versão e comandos intactos.

## Tradução — acessibilidade e dicas do campo de comando

- Traduzi o rótulo de acessibilidade, a instrução para leitores de tela e as dicas do campo de comando.
- Mantive os atalhos Cmd+seta para cima/Cmd+L e o gatilho `#`; não alterei a lógica de entrada nem os fluxos da área de transferência.

## Tradução — dicas e exemplos do modo de agente

- Traduzi o texto de orientação do campo para o agente, os exemplos rotativos, as dicas de orientação/fila e os estados do menu de sugestões dinâmicas de comandos.
- Preservei nomes de tecnologias, serviços, comandos e atalhos citados nos exemplos; substituí a expressão "Warp anything" por uma instrução natural em português.

## Tradução — nomes de ações do campo de comando

- Traduzi rótulos de atalhos e ações locais para histórico, tela, rolagem, buscas, sugestões, fluxos de trabalho e modo de agente.
- Atualizei a referência de marca no rótulo do registro de rede; mantive IDs de ações e atalhos sem alterações.

## Tradução — estados e histórico do menu inline

- Traduzi os estados de carregamento/sem resultados dos menus inline e o cabeçalho, o botão de configuração e os rótulos de acessibilidade do histórico.
- Mantive a consulta interna de busca, os dados apresentados, as ações e a compatibilidade sem alterações.

## Tradução — instruções de seleção de resultados

- Traduzi o separador de atalho nos resultados de comandos slash e as instruções para executar/enviar comandos e abrir conversas selecionadas no histórico.
- Mantive comandos, atalhos, títulos dinâmicos e ações sem alterações.

## Tradução — abas do histórico inline

- Traduzi os rótulos das abas para Tudo e Comandos; mantive Prompts como termo técnico.
- Preservei os enums de filtro e o comportamento de seleção do histórico.

## Tradução — seletor de modelos do campo de entrada

- Traduzi o rótulo de categoria, o botão de configuração e os avisos de compatibilidade entre o agente básico e o agente de uso completo do terminal.
- Mantive o comando `/model`, as variantes, filtros e lógica de seleção intactos.

## Tradução — seletor de perfis de execução

- Traduzi a ação para gerenciar perfis, o indicador de perfil selecionado e o prefixo do rótulo de acessibilidade.
- Mantive IDs de perfil, resultados de busca e ações intactos.

## Tradução — menu de conversas do campo de entrada

- Traduzi as abas de escopo, o indicador de conversa aberta em outro painel e o prefixo do rótulo de acessibilidade.
- Mantive IDs, filtros, títulos de conversa e a lógica de seleção sem alterações.

## Tradução — rótulos acessíveis de planos e prompts

- Traduzi os prefixos de acessibilidade dos resultados de planos e prompts salvos.
- Mantive títulos, nomes dos itens, comandos slash, IDs e ações sem alterações.

## Tradução — estado vazio da sessão de terminal

- Traduzi o título de nova sessão, as instruções de conversa/histórico, a dica de detecção automática e a ação para dispensar o aviso.
- Mantive atalhos e ações intactos e deixei o item de conversa na nuvem fora deste lote.

## Tradução — controle de detalhes dos painéis

- Traduzi as dicas de ferramenta para mostrar/ocultar detalhes nos controles do painel da conversa, tanto no app para desktop quanto no WASM.
- Preservei a ação, a condição de estado e os identificadores internos.

## Tradução — ações de maximizar painéis

- Traduzi os rótulos compartilhados para maximizar e minimizar painéis.
- Como são criados por um helper comum, os rótulos atualizados aparecem nos menus locais de painéis; a lógica e atalhos permanecem iguais.

## Tradução — menus de divisão de painéis

- Traduzi os itens de divisão nas quatro direções e os itens de maximizar/minimizar/fechar painel nos menus locais de configurações e variáveis de ambiente.
- Preservei as ações, os atalhos e as condições de exibição.

## Tradução — ações para abrir no desktop

- Substituí os rótulos visíveis “Open in Warp” por “Abrir no Kikito Terminal” e traduzi a explicação para abrir uma conversa no aplicativo para desktop.
- Traduzi também o botão para ver execuções na nuvem; ações, identificadores e destinos foram preservados.
## Abas e grupos — menus contextuais

- Revisei os menus de abas individuais e seleção múltipla: ações de fixar/desafixar, renomear, fechar e agrupar já estavam em português; os rótulos de mover para grupo e remover do grupo também foram mantidos.
- Traduzi os resumos de abas ocultas para “+ N mais” e “e mais N”, preservando contagens e comportamento.

## Controles de workspace e sessão

- Traduzi o título do diálogo de abertura de configuração de aba de “Open: {nome}” para “Abrir: {nome}”, sem alterar o fluxo.
- Revisei menus locais de abas, grupos e sessões; as ações restantes já estavam em português.


## Perfis do Agente Kikito

- Traduzi descrições de limites, perfis, listas de comandos, modelo principal, contexto da base de código e permissões MCP em `agent_profiles_page.rs`; atualizei as menções visíveis do agente próprio para Agente Kikito.
- Preservei ações, modelos, siglas, links e configurações.


## Preferências de editor externo

- Traduzi rótulos e descrições para abrir arquivos, escolher o layout, usar o visualizador Markdown e agrupar arquivos do editor.
- Mantive a opção de editor externo “Warp” e os valores/IDs internos; as referências ao aplicativo foram atualizadas para Kikito Terminal.

## Aparência: painel de ferramentas

- Traduzi a descrição visível do controle do painel de ferramentas e atualizei o nome apresentado de Warp Drive para Kikito Drive.
- Mantive nomes de recursos e identificadores internos sem alteração.


## Recursos do terminal

- Traduzi as categorias da página, controles de posição da janela Quake, preferências de terminal, sugestões, edição de texto, atalhos e área de transferência.
- Rebatizei o rótulo visível de Warp completions para Sugestões do Kikito Terminal; preservei identificadores, ações, nomes internos e atalhos.

### Complementos da página de recursos

- Traduzi controles adicionais de Vim, preenchimento automático, sugestões do shell, correções de comandos, expansão de aliases e captura de atalhos.
- O rótulo do seletor de tela ativa e a dica da área de transferência primária do Linux também foram localizados.

### Revisão adicional de Aparência

- Auditei categorias, opções de tema e janela, tooltips e controles de fonte, cursor, abas e entrada; não encontrei outros rótulos ou descrições locais em inglês para traduzir.
- Mantive nomes de temas e materiais visuais, termos técnicos e identificadores internos.


## Indexação da base de código

- Traduzi o cabeçalho visível "Code" para "Código" e os estados de indexação de repositórios: sem índice, limite atingido, indisponível, desativado, na fila, indexando, sincronizado, desatualizado e falha.
- Preservei nomes de arquivos de configuração, termos técnicos, siglas, IDs e mensagens internas usadas para comparar falhas remotas.

## Menus do aplicativo

- Traduzi o menu de ajuda, o controle manual de status de rede em depuração e os itens locais de reabertura de sessão e configurações de inicialização.
- Mantive sem alteração os itens de conta, feedback/documentação externos e menus ligados a serviços remotos.

## Filtro de blocos

- Traduzi a descrição do atalho para alternar o filtro e o rótulo de acessibilidade ao abrir o editor do filtro.
- O placeholder, as dicas de regex, distinção entre maiúsculas/minúsculas, inversão e linhas de contexto já estavam em português; consulta e comandos não foram alterados.

## Banners de SSH remoto

- Auditei os avisos de descontinuação do tmux e de falha da extensão SSH. Ambos pertencem a fluxos de sessão SSH avançada/remota e ficam fora da localização das superfícies locais do OSS.
- Não mantive alterações nos banners nem na lógica de exibição.

## Tooltips do terminal

- Traduzi as ações de revelar, ocultar e copiar segredos, além das opções para mostrar um arquivo no Finder ou a pasta correspondente.
- Confirmei que são tooltips locais do terminal: as ações de segredo são renderizadas para conteúdo local; as opções de arquivo usam `local_fs`, habilitado para builds desktop compatíveis.

## Acessibilidade do seletor de argumentos

- Traduzi as mensagens acessíveis de inserção de um fluxo de trabalho, seleção do próximo argumento por Shift+Tab e identificação do argumento selecionado.
- Essas mensagens são emitidas pelo fluxo local do seletor de argumentos; mantive o atalho e o conteúdo dinâmico do comando/nome do argumento.

## Estado do editor de linha

- Auditei `line_editor_status.rs`: o módulo apenas acompanha eventos internos de atividade do editor do shell e não contém textos de interface ou acessibilidade.
- Conferi os rótulos e a ajuda acessível do campo de entrada associados em `input.rs`; já estão em português, sem outras strings locais em inglês nesse fluxo.

## Identificação de painéis nas abas verticais

- Localizei os rótulos de tipo de painel exibidos nas abas e resumos; traduzi os nomes restantes de código, arquivo, fluxo de trabalho, configurações, ambientes, regras, plano e perfil de execução, além do estado de alterações não salvas.
- Atualizei o rótulo visível do agente do próprio app para Agente Kikito; nomes dos tipos/IDs e lógica permaneceram intactos.

## Auditoria de tooltips em pane_group

- Os tooltips em inglês de `pane_group/pane/view/header/sharing.rs` pertencem aos controles de compartilhamento e permissão de conteúdo compartilhado; não são superfícies locais do perfil OSS offline e ficaram fora do lote.

## Tooltips e acessibilidade do workspace

- Traduzi o tooltip do painel de gerenciamento de agentes, o rótulo de Configurações e o aviso de recursos indisponíveis offline. Os controles aparecem no workspace desktop; o painel de agentes está entre os recursos padrão do app.
- Localizei o anúncio acessível de alteração da verbosidade do leitor de tela, com rótulos detalhado/conciso sem alterar os valores persistidos. Ignorei mensagens de transferência, conta e nuvem.

## Revisão de tooltips e acessibilidade de abas

- Revisei `vertical_tabs.rs` e os controles de abas em `workspace/view.rs`: tooltips, busca e estado sem resultados estão em português; as strings restantes são nomes de produtos/dados dinâmicos, identificadores ou comentários.
- O caminho `app/src/workspace/tab.rs` não existe nesta estrutura; `tab_group.rs` não contém copy visível. Não encontrei novos textos locais para alterar neste lote.

## Falha ao localizar shell alternativo

- Traduzi a mensagem exibida quando nenhum shell alternativo pode ser encontrado; mantive PowerShell e WSL como referências e indiquei selecioná-los nas configurações de shell. Removi a instrução para abrir uma issue externa.

## Banner de encerramento do shell

- Confirmei que "Comunicar problema" abria o formulário de issues do GitHub e "Mais informações" abria a documentação externa do Warp. Removi esses dois controles e as ações que abriam URLs.
- Mantive a cópia do erro de inicialização do PTY e o texto explicativo do encerramento prematuro; não havia controle de fechar neste banner.

## Seletor de shell

- Traduzi as opções fixas `Default` e `Custom` do seletor de shell nas configurações para Padrão e Personalizado. O placeholder do caminho, os nomes dos shells, os detalhes e as opções da paleta já estavam localizados.
- Preservei nomes próprios (PowerShell, Git Bash e WSL), comandos, caminhos e valores persistidos.

## Banner local de subshell

- Traduzi o título do banner de inicialização de subshell e a opção para não exibi-lo novamente.
- Os demais banners locais desta pasta já estavam em português; excluí login AWS/Bedrock, sessões compartilhadas e sugestões de prompt associadas ao agente/cloud.

## Perfis de agentes: seleção MCP e uso

- Traduzi os dois cabeçalhos dos seletores de servidores MCP e o aviso de quando o limite de uso reinicia, incluindo os nomes dos meses em português e a data no fuso local.
- As strings aparecem como cabeçalho de dropdown na página local de perfis e no widget de uso quando esse widget está ativo.

## Privacidade — redação de segredos

- Traduzi os rótulos das abas pessoal/organização, o aviso de configuração somente pela organização e o estado vazio dos padrões corporativos. Fluxos de conta, telemetria e armazenamento remoto ficaram fora deste lote.
## Integração de shell e configurações SSH

- Traduzi descrições, placeholders e controles locais de integração de shells e SSH, incluindo as opções de instalação. Substituí o rótulo da funcionalidade por descrições em português e removi o link para a documentação externa. `main_page.rs` e `show_blocks_view.rs` ficaram fora deste lote por tratarem de conta, atualização e blocos compartilhados remotos.

## Diretório inicial de sessões

- Traduzi os rótulos de diretório inicial para janelas, abas e painéis divididos, além das opções de diretório pessoal, sessão anterior e diretório personalizado.

## Configuração de endpoint de inferência

- Traduzi os campos, instruções, exemplos e botões do modal local para endpoints configurados pelo usuário; mantive URLs, prefixos de chave e identificadores de modelo como valores técnicos.

## Reabertura de sessões fechadas

- Traduzi o controle para permitir reabrir sessões fechadas e o rótulo do período de tolerância, incluindo a unidade em segundos.

## Bloqueio de recursos remotos no canal OSS

- Forcei como desativados no canal OSS o changelog remoto, a gravação de sessões de computer use e o servidor auxiliar SSH remoto. O modo OSS já configura telemetria, crash reporting e autoupdate como ausentes e mantém URLs de serviço em loopback.
- Esta é uma verificação estática de configuração e flags; a auditoria de conexões acionadas em runtime continua pendente.


## Revisão de preferências de terminal

- Auditei as preferências locais de shell/terminal e comportamento em `features_page.rs`. Os textos visíveis desse recorte já estão em português; os literais ingleses restantes são termos de busca, nomes usados para localizar atalhos ou conteúdo de links externos, não rótulos exibidos. Nenhuma alteração de código neste lote.

## Revisão de Aparência

- Auditei rótulos, descrições, opções e campos da página local de Aparência. Os textos de interface encontrados neste recorte já estão em português; os nomes em inglês restantes são identificadores, nomes técnicos ou conteúdo remoto. Nenhuma alteração de código neste lote.

## Painel local de revisão de código

- Traduzi o fallback `Unknown` para `Desconhecido` no seletor local de repositórios do painel lateral. Os demais controles visíveis revisados já estavam em português; mensagens de diagnóstico e fluxos remotos foram excluídos.

## Agrupamento de abas no workspace

- Revisei os menus locais de agrupamento e seleção de abas em `tab_grouping.rs`; os rótulos e o fallback de grupo sem título já estão em português. Nenhum texto de interface precisou de alteração.

## Seções da lista de conversas

- Traduzi os títulos das seções da lista para `ATIVAS` e `ANTERIORES`. Mantive fora deste lote a ação de compartilhamento e suas superfícies remotas.

## Remoção de login no painel lateral OSS

- O canal OSS não inclui mais Drive nem histórico de conversas nas ferramentas do painel lateral. Ações antigas ou disparadas por atalhos não abrem o fluxo de autenticação; se não houver ferramenta local disponível, o painel fica vazio.

## Painel lateral do workspace

- Auditei os rótulos e estados do painel lateral. O controle de fechamento e os tooltips dos painéis locais já estão em português; as strings inglesas restantes pertencem a estados que exigem conta/IA ou ao Warp Drive remoto, mantidos fora do escopo. Nenhuma alteração de código neste lote.

## Estados do Explorador de projetos

- Traduzi as mensagens locais para sessão sem workspace habilitado e para sessões WSL incompatíveis. Mantive a mensagem específica de sessão remota fora deste lote.

## Busca global local

- Auditei rótulos, placeholders, controles de busca, estados vazios e sessões incompatíveis em `global_search/view.rs`; os textos locais já estão em português. Mantive em inglês apenas estados exclusivos de conexão/sessão remota, fora do escopo. Nenhuma alteração de código neste lote.

## Menus locais do aplicativo

- Auditei os menus de janela, aba, depuração e arquivo em `app_menus.rs`; os rótulos locais visíveis já estão em português. Excluí privacidade/documentação externas, logout e itens de conta/IA não disponíveis no OSS. Nenhuma alteração de código neste lote.

## Opções de worktree em configurações de aba

- Traduzi a dica que explica a dependência da opção de nome automático de branch em relação à criação automática de worktree. Os demais textos visíveis do modal de configuração de sessão já estavam em português.

## Ciclo de vida do shell local

- Revisei erros de inicialização/fallback em `terminal_manager.rs` e avisos de encerramento do shell. As mensagens visíveis de OSS já estão em português; erros internos e avisos de sessão compartilhada/SSH não foram tratados como copy local. Nenhuma alteração de código neste lote.

## Atalhos da sessão de terminal

- Traduzi a dica de atalho local para abrir o painel de revisão de código no bloco inicial do terminal. A dica de nova conversa com agente na nuvem ficou fora do escopo.


## Menu de estado do TUI

- Traduzi a seção e os campos de versão, sessão, ID da conversa, diretório de trabalho, organização e e-mail.

## Contexto local do projeto no TUI

- Traduzi o estado de descoberta do contexto, o carregamento dos arquivos de regras locais e a contagem de habilidades do projeto na tela inicial do TUI.

## Indicador de inicialização do shell no TUI

- Traduzi o estado local exibido enquanto o terminal inicia: `Starting terminal...` para `Iniciando o terminal...`. Mantive fora desta revisão as telas de autenticação e recuperação de conversas.

## Preferências locais no TUI

- Traduzi as mensagens de alteração e persistência do tema e do modo Vim no ciclo de sessão do TUI. Mantive `auto`, `light` e `dark` como valores aceitos pelo comando.

## Estados e controles locais da sessão TUI

- Traduzi dicas de shell/saída, erros locais de configuração, estado sem título, rótulos de modo shell/cópia, feedback da linha de status e confirmações do comando `/theme`. Atualizei a referência visível ao logotipo do app para Kikito Terminal. Mantive fora mensagens de autenticação, conta, agente e conversa remota.
## Paleta local de comandos do TUI

- Traduzi descrições de `/theme`, `/vim-mode`, `/statusline`, `/reset-statusline` e `/exit`, além dos estados de carregamento/lista vazia da paleta e da dica de completar comandos do shell. Mantive comandos e argumentos intactos; deixei de fora `/status` por incluir informações de conta.

## Primeiro uso e marca do TUI

- Atualizei títulos e apresentação inicial para Kikito Terminal em português. Removi da tela de primeiro uso as promessas de agentes, modelos e orquestração hospedados; agora ela apresenta abas, painéis, shells, CLIs e contexto local do projeto.

## Marca do TUI

- Substituí a silhueta animada antiga por um monograma KT adaptado da marca aprovada, ajustando a proporção para a grade de caracteres do terminal. Traduzi também as mensagens de falha ao carregar arte ASCII personalizada.

## Ajuda de edição do TUI

- Traduzi descrições dos atalhos locais para inserir/apagar texto, mover o cursor, ampliar seleções, copiar, recortar, colar e desfazer/refazer. Os nomes e atalhos não mudaram.

## Configuração local da linha de status no TUI

- Traduzi as instruções de teclado/rodapé da tela de configuração e os rótulos locais de diretório, Git, Vim, data e hora. Mantive opções de modelo, equipe, créditos, agente e voz fora do escopo.

## Navegação da seleção no TUI

- Traduzi o título da configuração da linha de status e os textos genéricos do seletor usados nessa tela: busca, paginação, estado sem resultados e navegação entre opções.

## Navegação local do editor

- Traduzi o comando `Go to line` e o título da janela de navegação para `Ir para a linha`, além de `Find in code editor` para `Localizar no editor de código`.
- Traduzi os estados locais de carregamento e contagem da janela de referências, assim como nomes de atalhos de navegação, seleção, edição, Vim e área de transferência. Atalhos, IDs e ações permanecem intactos.

## Navegação local do workspace

- Traduzi os rótulos `Branch` para `Ramificação` nas opções compactas das abas verticais.
- Traduzi os nomes de fallback `File` e `Folder` na árvore de arquivos para `Arquivo` e `Pasta`.
- Traduzi o estado local `Enable {server}` para `Ativar {server}` no rodapé de suporte a linguagens; as mensagens dinâmicas do servidor permanecem como recebidas.
- Traduzi descrições dos campos de configuração do diretório inicial das novas sessões, incluindo configuração global e opções separadas para painel, aba e janela.

## Continuação da localização local

- Traduzi para português o título da aba de configurações e a ação para abrir o arquivo local de preferências.
- Traduzi “Advanced” para “Avançado” na configuração de diretório de trabalho e atualizei o rótulo do editor interno para “Kikito Terminal”.
- As demais superfícies locais adjacentes de aparência, atalhos, automação, editor e revisão foram auditadas; os rótulos visíveis já estavam em português. Itens de conta, nuvem, agentes e serviços externos ficaram fora do escopo.

## Proteção contra redirecionamentos externos

- A auditoria encontrou que uma requisição permitida para loopback poderia seguir um redirecionamento HTTP até um host público. O cliente compartilhado e o cache de assets agora interrompem redirecionamentos não loopback no canal OSS; o cliente de controle local não segue redirecionamentos.
- A busca por conexões diretas identificou usos locais no controle do app e na ponte ACP. Conexões MCP, sessões compartilhadas/remotas e subscriptions GraphQL permanecem desativadas no OSS.
- `cargo check -p http_client -p asset_cache -p local_control`, `cargo clippy -p http_client -p asset_cache -p local_control --all-targets -- -D warnings` e `cargo check -p warp --bin warp-oss` passaram após a correção. Não executei testes.
- A verificação de rede enquanto ações da interface são usadas continua pendente; a auditoria estática e a observação em repouso não cobrem todos os fluxos em runtime.
- Os clientes OSS também ignoram proxies de ambiente nesse tráfego local; isso impede que uma URL loopback seja enviada a um proxy externo.
- Após a correção, executei o `warp-oss.exe` por 20 segundos em repouso: só apareceu o listener `127.0.0.1:9282`, sem conexões TCP de saída. Encerrei o processo e o helper iniciado junto dele; fluxos acionados na interface continuam sem captura.

## Tradução da linha de status e seletor do TUI

- Traduzi os rótulos locais da linha de status para português, incluindo aprovação automática, modelo, equipe, uso, GitHub, agente e entrada de voz; a dica do atalho de voz também foi localizada.
- Traduzi os badges genéricos do seletor, a ação de tentar novamente e o estado de carregamento.
- `cargo fmt --all`, `cargo check -p warp --bin warp-oss` e `cargo check -p warp_tui --bin warp-tui-oss` concluíram com sucesso. Não executei testes.

## Remoção do atalho para documentação hospedada

- Removi da configuração local do visualizador Markdown o ícone de ajuda que tentava abrir a documentação em `docs.warp.dev`, além da action e URL associadas. A preferência local continua disponível.
- `cargo fmt --all`, `cargo check -p warp --bin warp-oss`, `cargo clippy -p warp --bin warp-oss -- -D warnings` e `git diff --check` passaram. Não executei testes.

## Busca e comandos da tela de código

- Traduzi os seis nomes de ações de configuração da tela de editor/revisão exibidos na Paleta de Comandos.
- Acrescentei termos equivalentes em português aos índices de busca das configurações locais, mantendo os termos técnicos existentes como aliases.
- `cargo fmt --all`, `cargo check -p warp --bin warp-oss`, `cargo clippy -p warp --bin warp-oss -- -D warnings` e `git diff --check` passaram. Não executei testes.

## Configurações locais e menus do workspace

- Traduzi ações da Paleta de Comandos, opções locais de clique direito e atalho global, além dos modos padrão de sessão.
- Acrescentei termos em português à busca das preferências locais e traduzi ações de menu/atalhos do workspace, preservando identificadores, ações e teclas.
- Removi o atalho de documentação hospedada da opção local de fluxos de trabalho globais.
- `cargo fmt --all`, `cargo check -p warp --bin warp-oss`, `cargo clippy -p warp --bin warp-oss -- -D warnings` e `git diff --check` passaram. Não executei testes.

## Ações locais do workspace e IA

- Traduzi rótulos da Paleta de Comandos para regras de IA, servidores MCP, navegação de tarefas/agentes, notificações, blocos compartilhados e configurações de IA.
- Traduzi o nome da seção local Warpify como Personalização para evitar a marca antiga na interface.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.

## Links de privacidade no OSS

- Ocultei no canal OSS os cartões de gerenciamento de dados e política de privacidade que direcionavam para páginas hospedadas pela Warp. Os canais que usam esses serviços mantêm os cartões.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.

## Ações externas e páginas de conta no OSS

- Ocultei da Paleta de Comandos no canal OSS as entradas de conta, equipes, blocos compartilhados, cobrança, indicações, ambientes na nuvem, servidores MCP, convite, Slack, documentação, feedback e política de privacidade.
- Mantive disponível a ação local de visualizar logs e traduzi seu rótulo para Kikito Terminal.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.

## Bloqueio de retornos de autenticação

- O `AuthManager` agora descarta redirecionamentos de autenticação no canal OSS antes de iniciar qualquer consulta ao usuário. Isso cobre links de login antigos que ainda possam chegar ao aplicativo.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. O `cargo check` descartou alguns artefatos incrementais corrompidos pelo cache e concluiu com sucesso. Não executei testes.

## Ajuda de notificações e recuperação gráfica no OSS

- Traduzi a notificação local de permissão de notificações. O atalho para instruções hospedadas só aparece fora do OSS.
- O banner de recuperação gráfica no Linux mantém a explicação local, mas oculta o botão que abriria a documentação externa no OSS.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. O primeiro check apontou imports exclusivos de Linux não usados no Windows; deixei-os condicionais à plataforma e a segunda verificação passou. Não executei testes.

## Tooltips e ações locais de arquivo

- Traduzi os tooltips de links detectados no terminal para abrir arquivos, pastas e links.
- Traduzi os botões do editor local de roteadores de modelos e o tooltip para abrir arquivos na revisão de código.
- A revisão da tela de Aparência não encontrou mais textos visíveis em inglês no canal OSS; não foram necessárias alterações.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.

## Ações hospedadas restantes na Paleta de Comandos

- Ocultei no canal OSS as ações de busca/exportação do Warp Drive, logout e instalação global das CLIs Oz e Warp Control.
- Traduzi o comando local “Toggle Warp AI” para “Alternar assistente de IA”.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.

## Toasts de objetos ausentes

- Traduzi para pt-BR os avisos locais exibidos quando um objeto sincronizado não é encontrado ou o acesso é negado, incluindo os caminhos de abertura por link.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.

## Importação de configurações e dicas do agente

- Traduzi os estados e resumos do importador de configurações locais, incluindo carregamento, perfil selecionado, opções e aviso de sessão nova.
- Traduzi as dicas do agente, os rótulos de ações e o atalho “Saiba mais”. No OSS, as dicas do Warp Drive são omitidas e nenhuma dica oferece links para documentação hospedada.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.

## Bloqueio central de autenticação no OSS

- Impedi tentativas de login, refresh de credenciais, API key, autorização por dispositivo, criação/vinculação de usuário anônimo e geração de links de autenticação no perfil OSS.
- O caminho de atualização do estado local de onboarding continua local, mas não envia a atualização ao servidor.
- O banner de reautenticação também fica oculto no OSS. Os controles da interface já não iniciavam o login nesse canal; estas proteções cobrem chamadas internas e credenciais antigas.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.

## Mensagens de contexto e correção de configurações

- Traduzi avisos para anexar planos como contexto e o prompt enviado ao assistente para corrigir um `settings.toml` local.
- Traduzi o botão para “Corrigir com o assistente de IA” e mantive “Abrir arquivo” como ação local.
- `git diff --check` passou; não executei testes.

## Tradução do TUI, fila de prompts e onboarding

- Traduzi para pt-BR os rótulos e mensagens locais da sessão TUI: conversas, ações, atalhos, anexos de imagem, entrada por voz, exportação, MCP e estados do agente. Mantive comandos, IDs e conteúdo dinâmico intactos; textos específicos de sessões cloud continuam em inglês porque esse recurso não está disponível no OSS.
- Traduzi a fila local de prompts e o onboarding do prompt do terminal. Links para documentação e reporte hospedados são omitidos no OSS.
- Atualizei o modelo de licenças de terceiros para identificar a distribuição como Kikito Terminal.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.
## Chaves de API e criação de ambientes locais

- Traduzi a interface local de seleção e gerenciamento de chaves de API de agentes e o modal de seleção de repositórios para criação de ambientes: busca, ações, estados vazios/carregamento, acessibilidade, mensagens e botões.
- Atualizei as asserções existentes do modal para refletir os textos traduzidos; não adicionei testes nem os executei.
- A revisão de rede confirmou no código que o perfil OSS neutraliza a abertura de URLs HTTP(S) externas e rejeita requisições do cliente HTTP fora de endereços loopback. Essa inspeção não substitui a captura de conexões durante todos os fluxos em execução.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram.
## Perfis locais de agentes e auditoria de configurações

- Traduzi para pt-BR os rótulos dos perfis de agente: níveis de permissão/autonomia, campos de exemplo, ações e regras de diretórios permitidos. A página de perfis não entra na navegação OSS atual, que mantém Aparência, Atalhos e Sobre, mas os textos locais ficaram consistentes no código.
- A página de ambientes e seu botão de criação são exclusivos do gerenciamento de ambientes cloud e não aparecem na navegação OSS; mantive esse conteúdo sem mudanças.
- `cli_agents_page.rs` já estava traduzido nos textos locais; não precisou de alterações.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss`, `cargo clippy -p warp --bin warp-oss -- -D warnings` e `PROTOC=(WinGet protoc) cargo check -p warp_tui --bin warp-tui-oss` passaram. Não executei testes.
## Entrada do terminal e seletores de modelo

- Traduzi tooltips do campo de entrada para anexar contexto/arquivos, entrada por voz, comandos de barra e seleção do modo agente/terminal, além dos avisos locais de indisponibilidade.
- Traduzi a seleção local de perfis e modelos, a ação de gerenciar perfis e o cabeçalho de modelos personalizados. Mantive textos relacionados a ChatGPT, sessões compartilhadas, pedidos de acesso e dados hospedados sem alterações.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.
## Onboarding de novidades e configuração de abas

- Traduzi CTAs, etapas de configuração de abas, banner e cartões de novidades do onboarding para pt-BR, preservando os nomes Claude Code, Codex e OpenCode.
- O fluxo depende de flags que não são habilitadas no OSS por padrão, embora exista uma ação de demonstração em builds de depuração. O link para documentação hospedada agora é omitido no OSS.
- A auditoria da janela principal não encontrou novos rótulos locais de abas, painéis ou navegação que precisassem de tradução.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.
## Variáveis de ambiente e revisão de código local

- Traduzi os rótulos, placeholders, ações e avisos locais para criar e executar variáveis de ambiente; a dica sobre segredos agora usa a marca Kikito Terminal. Ações de cópia de link, duplicação, exclusão e exportação continuam sem alteração porque aparecem apenas em coleções sincronizadas no servidor.
- Traduzi os controles locais de revisão de código: navegação por arquivos, commit e descarte local, estados vazios/erros, diferenças, comentários e ações de contexto. Push, publicação, PR e integrações GitHub permanecem fora da interface OSS/local.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.
## Banners locais do terminal

- Os banners de shell encerrado, Vim, aliases, notificações e agente já tinham cópia local em pt-BR.
- O banner local de arquivos mantém a ação de abrir/editar e oculta no OSS o botão que levava à documentação Warp hospedada. O link para a documentação oficial da AWS foi mantido.
- Prompts enviados aos agentes e banners de sessão compartilhada ou login de equipe ficaram fora da localização da interface local.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.
## Chips de contexto e menus de busca

- Traduzi ações locais de copiar chips, placeholders de busca de diretórios e branches e o estado vazio de menus locais.
- A busca por ambientes continua em inglês porque pertence ao fluxo cloud; comandos, versões e atalhos do popup do Node foram preservados. As descrições das configurações de blocos já estavam traduzidas.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.
## Título de conversa no painel local

- Traduzi o título padrão “New agent conversation” para “Nova conversa com agente”. Os controles do painel já estavam em português; títulos cloud e ações de sessões compartilhadas não foram alterados.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.
## Descrições dos comandos de barra

- Traduzi descrições de comandos locais para conversas, terminal, MCP, chaves, logs, voz, Docker local, skills, regras, editor, abas, revisão, perfis, planejamento, histórico, ramificação e exportação.
- Mantive a sintaxe e os argumentos dos comandos, prompts enviados aos agentes e entradas ligadas a conta, cobrança, feedback, controle remoto ou execução cloud.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss`, `cargo clippy -p warp --bin warp-oss -- -D warnings` e `PROTOC=(WinGet protoc) cargo check -p warp_tui --bin warp-tui-oss` passaram. Não executei testes.
## Avisos de anexos no campo de entrada

- Traduzi os estados do prompt que informam quando comandos ou texto selecionado foram anexados como contexto, incluindo as quantidades e a indicação de remoção pelo atalho Escape.
- A view compartilhada da barra de entrada não tinha rótulos estáticos locais pendentes. Os seletores de prompts da Warp Drive são cloud-only; as views locais de prompts e skills não definem rótulos próprios.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.
## Bloqueio dos fluxos de créditos hospedados

- O banner de compra de créditos e o modal de recarga automática agora ficam indisponíveis no canal OSS: guarda na inserção, renderização, ações e encaminhamento/abertura dos eventos. Canais não OSS mantêm o comportamento existente.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.

## Menu local de histórico inline

- Traduzi o cabeçalho do menu de histórico e o título padrão das conversas sem nome.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. O Clippy foi repetido e concluído depois da interrupção inicial. Não executei testes.

## Status de envio de arquivos por SSH

- Traduzi os status e controles locais do painel de envio de arquivos: aguardando senha, envio/conclusão/falha, destino, ações para ver/fechar a sessão, remoção e cabeçalho.
- Mantive nomes de comandos, caminhos, host remoto e detalhes dinâmicos sem alteração.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.

## Status da navegação na paleta de comandos

- Traduzi os status de execução/conclusão e os rótulos de acessibilidade da navegação entre sessões. Os comandos e prompts dinâmicos continuam intactos.
- As descrições dos comandos de barra e as ações de conta, autenticação, cobrança e cloud ficaram fora deste lote.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.

## Filtros locais da busca e paleta

- Traduzi nomes e placeholders dos filtros locais de histórico, ações, sessões, abas, configurações de abertura, arquivos, comandos, blocos, código, regras, repositórios, comandos de barra e conversas; também as categorias de sugestões por IA, habilidades e modelos.
- Workflows, Agent Mode Workflows, notebooks, planos, Warp Drive e variáveis de ambiente são conectados ao Warp Drive e não entram na paleta OSS quando `WarpDriveSettings` está desativado. Mantive seus rótulos. Prompt History é compartilhado com o menu de histórico do modo cloud, então também ficou sem alteração.
- “Skills” virou “habilidades” na interface; o tipo/filtro técnico e demais identificadores permanecem inalterados.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.

## Seletor local de habilidades

- Traduzi o estado vazio do seletor inline de habilidades, a dica de fechar com Escape, o selo “Habilidade do projeto”, o rótulo de acessibilidade e o aviso quando uma habilidade local é acionada em uma sessão remota.
- Mantive o nome da habilidade, os comandos e o atalho Escape intactos.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.

## Menus locais de contexto do terminal

- Traduzi ações de busca no menu de contexto da entrada, busca de comandos com IA, alternância da dica da entrada e cópia de seleção no menu da tela alternativa. Os itens antigos “Ask Warp AI” agora usam o rótulo já localizado “Perguntar à IA do Kikito Terminal”.
- Confirmei os gates OSS: `init_feature_flags` desativa `CreatingSharedSessions`; itens de compartilhamento também exigem `ContextFlag::CreateSharedSession`. `Save as workflow` só aparece quando `WarpDriveSettings::is_warp_drive_enabled` retorna true, condição que exige estado autenticado e Drive habilitado; o perfil OSS inicia desconectado. Esses itens cloud não foram alterados.
- A busca com IA só aparece quando existe um provedor habilitado em `AISettings`; os textos mudaram sem alterar o fluxo, IDs ou atalhos.
- Também traduzi os três nomes de atalhos registrados em `terminal/view/init.rs` para a IA sobre seleção, último bloco e texto selecionado; IDs e eventos de telemetry não mudaram. `ASK_AI_ASSISTANT_TEXT` já estava em pt-BR como “Perguntar à IA do Kikito Terminal”.
- `cargo fmt --all`, `git diff --check`, `cargo check -p warp --bin warp-oss` e `cargo clippy -p warp --bin warp-oss -- -D warnings` passaram. Não executei testes.

## Cabeçalhos dos blocos do agente no TUI

- Traduzi `Thinking...`, `Thought for {duration}`, `Conversation summary` e o prefixo `Completed` para os cabeçalhos locais dos blocos do agente. Mantive a duração, os títulos dinâmicos das tarefas e seus índices.
- Não alterei textos de menus de conversas associados a equipes, mensagens de execução cloud, saída do shell ou comandos.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp_tui --bin warp-tui-oss`. Não executei testes nem Clippy.

## Seletor local de habilidades no TUI

- Traduzi o título `Skills` para `Habilidades` e o estado vazio `No skills found` para `Nenhuma habilidade encontrada`. Esse menu consulta habilidades locais do projeto; nomes de habilidades e a sintaxe `/` permanecem intactos.
- Mantive menus de equipes, conversas, modelos e outros fluxos cloud fora deste lote.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp_tui --bin warp-tui-oss`. Não executei testes nem Clippy.

## Rótulos locais de anexos no TUI

- Traduzi as descrições de atalhos para navegar/remover anexos e devolver o foco ao campo de entrada. Também localizei os marcadores `[image]`/`[file]` e o status `loading…` no painel de anexos.
- Mantive nomes dinâmicos de arquivos, conteúdo da área de transferência e comandos sem alteração. Atualizei as expectativas de renderização associadas, sem executar testes.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp_tui --bin warp-tui-oss`. Não executei testes nem Clippy.

## Auditoria da configuração local da linha de status no TUI

- Auditei o seletor aberto por `/statusline`, as descrições dos atalhos, o rodapé, as confirmações de salvar/restaurar e os nomes dos itens da linha de status. A copy estática local já está em português; nenhum texto foi alterado neste lote.
- Itens `Git`, `Vim` e `Pull request` foram mantidos como termos técnicos. A lista inclui também metadados de modelo/equipe/créditos, que não fazem parte da copy local tratada nesta auditoria.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp_tui --bin warp-tui-oss`. Não executei testes nem Clippy.

## Histórico da sessão e atalhos do TUI

- Traduzi o título e os estados vazios do histórico aberto com seta para cima. A consulta reúne o histórico local da sessão; mantive os comandos e prompts armazenados como conteúdo dinâmico.
- Traduzi as dicas locais do campo de entrada e os rótulos de atalhos para comandos, modo shell, conversas, histórico de entrada, aprovação automática, planos e controle do terminal. Atualizei as expectativas relacionadas sem executar testes.
- Mantive o atalho e a dica de orquestração de agentes fora do lote por pertencerem ao fluxo de agentes; identificadores, comandos e o termo técnico `Terminal` não mudaram.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp_tui --bin warp-tui-oss`. Não executei testes nem Clippy.

## Aviso local de uso em falhas do agente no TUI

- Traduzi o aviso estático `This response won't count towards your usage.` para `Esta resposta não será contabilizada no seu uso.`. Atualizei as expectativas correspondentes sem executar testes.
- Mantive mensagens encaminhadas por provedores/servidores e textos de autenticação, créditos e assinaturas. O pedido de desculpas compartilhado chega combinado com mensagem dinâmica, então também ficou intacto.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp_tui --bin warp-tui-oss`. Não executei testes nem Clippy.

## Observação de rede do processo ativo no Windows

- A inspeção de processo/socket identificou o processo ativo como `C:\Program Files\Warp\warp.exe`, a instalação oficial já existente, não o binário Kikito Terminal OSS. As duas conexões HTTPS a `34.117.41.85:443` pertenciam somente a esse processo e não foram usadas como evidência sobre o Kikito.
- Não havia processo OSS ativo para observar; portanto, a captura runtime do OSS enquanto ações da interface são usadas continua pendente.

## Auditoria de preferências locais no TUI OSS

- Auditei as telas locais de aparência/tema, atalhos e configuração da linha de status. Os títulos, descrições de ações, rótulos de itens, rodapés e confirmações verificados já estão em pt-BR; não alterei código neste lote.
- Mantive `auto`, `light` e `dark` como valores aceitos pelo comando `/theme` e pelos arquivos de configuração. A tela de atalhos não tem um menu de edição dedicado; ela lista os atalhos ativos, com descrições localizadas. Não encontrei uma tela geral de preferências no TUI OSS neste escopo.
- Uma superfície próxima que ainda pode merecer auditoria separada é a documentação em inglês embutida nos metadados de configurações em `app/src/settings`, que não é texto destas telas TUI.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp_tui --bin warp-tui-oss`. Não executei testes nem Clippy.

## Auditoria de metadados de configurações no OSS

- Confirmei que a janela de Settings no canal OSS mantém apenas as páginas Appearance, Keybindings e About (`settings_view/mod.rs`). As chamadas que renderizam diretamente `Setting::description()` ficam na página de perfis de agentes, excluída dessa navegação local; não encontrei esses metadados nas telas locais da GUI nem do TUI.
- O TUI pode exportar um JSON Schema por um comando de linha de comando de execução única. As descrições `schemars`/`SettingSchemaEntry` vão para esse artefato técnico; não são copy de menu, help ou status renderizada durante a sessão. Mantive essas descrições, os nomes e valores serializados sem alteração neste lote.
- Não encontrei descrições locais elegíveis para traduzir neste escopo. Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`. Não executei testes nem Clippy.

## Zero-state local do TUI OSS

- O título e a orientação de primeira execução, além do contexto local de projeto e habilidades, já estavam em pt-BR. Traduzi o fallback `dev build` para `compilação de desenvolvimento` e os estados locais de endpoints personalizados; mantive `/api-keys` e `agents.custom_endpoints` como comandos/caminho de configuração.
- O updater não é registrado no canal OSS e o changelog é desativado. A linha de conta e a seção MCP ainda eram permitidas pela visibilidade padrão apesar de não se aplicarem ao OSS; agora ficam ocultas nesse canal. A linha de versão OSS também não consulta o modelo de updater, que não é registrado nesse canal. Os textos dinâmicos de changelog, conta, atualizador e MCP foram mantidos sem tradução.
- As descrições em `app/src/settings/tui_zero_state.rs` são metadados do schema, não copy exibida pelo zero-state.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp_tui --bin warp-tui-oss`. Não executei testes nem Clippy.

## Auditoria de menções locais à marca Warp

- Traduzi os títulos e textos das cinco dicas locais em `app/src/tips/tip_view.rs`, incluindo “Paleta de comandos” e “Seletor de temas”; as referências à marca agora dizem Kikito Terminal. Essas dicas são montadas diretamente pela janela local, sem gate cloud.
- Traduzi o título e a descrição da notificação local de nova aba em `app/src/uri/mod.rs` (“Nova aba criada” / “Abra o Kikito Terminal para ver sua nova aba”). Esse aviso é usado no caminho de `NewTab` em plataformas que exibem a notificação do sistema; não depende de autenticação ou serviço remoto.
- Mantive as ocorrências restantes ligadas a créditos/planos, autenticação, Warp Drive, colaboração, cloud agents, atualização, integração com host/protocolo OSC e identificadores/configuração internos. Não são copy local elegível nesta auditoria OSS.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`. Não executei testes nem Clippy.

## Segunda varredura de menções da marca em UI OSS

- Traduzi a copy da saudação local da IA para “Peça à IA do Kikito Terminal para explicar erros, sugerir comandos ou escrever scripts.” A view só aparece quando há provedor de IA habilitado localmente, o Agent Mode está desativado e a aba está em modo compatível; não exige conta/Drive.
- Troquei “Warp Essentials” pelo título local “Recursos do Kikito Terminal” no cabeçalho do Resource Center e no tooltip da ação correspondente.
- Classificação de ocorrências mantidas: `Log in with Warp`/`Copy login URL` pertencem ao fluxo de autenticação hospedado e não aparecem no OSS. Verifiquei que `app/src/tui/mod.rs::init` inicia o modelo em `LoggedIn` no canal `Oss`, sem depender de `AuthState`; `session::init` então cria a sessão local e troca a raiz para `Terminal`. A ação de login também retorna sem iniciar rede no OSS, e o handler de eventos de auth ignora esse canal. Os textos de créditos/planos, Drive, equipe/colaboração, CLI/update, avisos de login e status de serviço continuam em fluxos cloud ou remotos. Nomes como `WarpTheme`, `WarpDrive`, `WarpClient`, `warp-oss`, eventos de telemetry e mensagens de log/path são identificadores, protocolo ou diagnóstico interno.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`. Não executei testes nem Clippy.

## Confirmação do estado de autenticação no TUI OSS

- Confirmei o bootstrap antes de classificar os rótulos `Log in with Warp` e `Copy login URL`: no canal `Oss`, `TuiLoginModel` é criado em `LoggedIn` mesmo sem identidade autenticada; a sessão inicial é local e a raiz muda para `Terminal`. Os handlers de auth não iniciam o login via browser no OSS. Portanto, esses rótulos hospedados não aparecem no binário OSS e nenhum ajuste de auth foi necessário.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp_tui --bin warp-tui-oss`. Não executei testes nem Clippy.

## Menções de marca no Resource Center e menus nativos

- Traduzi a copy local “Access all of Warp” e “Make Warp your own” na seção Getting Started, o título Maximize Warp e os rótulos de atalhos nativos “Hide Warp”/“Quit Warp” para Kikito Terminal em pt-BR. Os IDs dos atalhos permaneceram intactos.
- A ação do Resource Center é exposta quando `ContextFlag::WarpEssentials` está habilitada (padrão global habilitado); o cabeçalho da ação e a seção de recursos são superfícies locais. `About Warp` já usa a descrição nativa localizada “Sobre o Kikito Terminal”.
- Classifiquei como não elegíveis neste escopo os artigos do bloco Advanced Setup que abrem documentação/blog hospedados em `warp.dev`, o convite “Invite a friend to Warp” (só renderiza com identidade autenticada, ausente no OSS), e a seção de changelog (FeatureFlag::Changelog desabilitada no OSS). Mantive protocolos, IDs de configuração e telemetry sem alteração.
- Após revisão, traduzi todos os títulos e descrições locais de Getting Started/Maximize Warp e a seção foi nomeada “Primeiros passos”. Ajustei a dica de ações em blocos para não prometer compartilhamento e limitei a descrição da busca a comandos anteriores, sem workflows cloud. O Resource Center agora omite a seção de changelog no canal OSS, onde a feature está desabilitada, e omite Advanced Setup nesse canal porque seus três itens levam a docs/blog hospedados em Warp; ambos permanecem com o comportamento anterior nos demais canais. O convite continua oculto no OSS pela condição de identidade autenticada.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`. Não executei testes nem Clippy.

## Tooltip de cancelamento de agente local

- Traduzi a dica `Cancel` do botão que interrompe uma tarefa de agente para `Cancelar` em `app/src/terminal/view/pane_impl.rs`. O botão é uma ação de interface local e não depende de serviço hospedado.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss` passaram. O check emitiu somente avisos de artefatos incrementais antigos que o Cargo ignorou. Não executei testes nem Clippy.
## Modal hospedado do Warp Agent CLI no canal OSS

- A auditoria encontrou `AgentCliLaunchModal`: onboarding do Warp Agent CLI com link para `docs.warp.dev`, controlado por feature flag. Desativei `FeatureFlag::AgentCliLaunchModal` no conjunto de flags locais do canal OSS para evitar esse fluxo hospedado em builds OSS.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss` passaram. O check emitiu somente avisos de artefatos incrementais antigos que o Cargo ignorou. Não executei testes nem Clippy.
## Outros modais de lançamento hospedados no OSS

- A mesma varredura identificou modais de Oz, orquestração multiagente e anúncio de código aberto do Warp. Eles promovem agentes em cloud, serviços hosted ou links para o repositório oficial do Warp; desativei `OzLaunchModal`, `OrchestrationLaunchModal` e `OpenWarpLaunchModal` no canal OSS junto ao bloqueio anterior de `AgentCliLaunchModal`.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss` passaram. O check emitiu somente avisos de artefatos incrementais antigos que o Cargo ignorou. Não executei testes nem Clippy.
## Auditoria estática de chamadas externas na UI

- Revisei chamadas `ctx.open_url` em `app/src`: suporte do Agent Mode, docs de launch configs, Wayland, links de download WebAssembly, citações de documentação/memória e páginas administrativas.
- As chamadas de suporte/citações dependem de Agent Mode ou conteúdo de agentes/cloud desativado no OSS; o fluxo de download é `wasm` e não compõe a GUI desktop OSS; a ação de docs de launch config não tem ponto de construção de item na busca atual; e o botão de documentação do Wayland já é ocultado no canal OSS. O Resource Center e modais de lançamento com links Warp também foram ocultados em etapas anteriores.
- A abertura global de URLs mantém o bloqueio OSS para destinos HTTP(S) externos, permitindo somente intenções locais de loopback. Esta revisão é estática; captura de sockets com o binário OSS em execução continua pendente porque não havia processo OSS ativo durante a inspeção anterior.
## Revisão estática do bootstrap OSS e serviços

- Confirmei em `crates/warp_core/src/channel/state.rs` que a configuração do canal OSS usa `WarpServerConfig::local()` (`127.0.0.1:0`), `OzConfig::local()`, sem endpoint de compartilhamento, `telemetry_config: None`, `autoupdate_config: None` e `crash_reporting_config: None`.
- Em `app/src/lib.rs`, o OSS inicializa `AuthState` local, não inicia refresh de autenticação/segredos e não inicializa nem envia erros para crash reporting. O handler central bloqueia abertura de HTTP(S) fora de loopback, e `crates/http_client` aplica a restrição de host nas requisições.
- Validação desta etapa foi por inspeção estática; o build `cargo check -p warp --bin warp-oss` passou nas mudanças de feature flags anteriores. Captura de tráfego do executável em execução ainda não foi realizada.
## Captura runtime ociosa do binário OSS

- Iniciei `target/debug/warp-oss.exe` com `WARP_DATA_PROFILE=offline-audit-20261017`, perfil separado do uso normal. O processo PID 3360 permaneceu ativo por cerca de 22 segundos; duas consultas de conexões TCP para esse PID não retornaram conexões. Depois encerrei o processo.
- O resultado cobre somente a inicialização ociosa do executável e não avalia subprocessos ou tráfego provocado por comandos/aliases que o usuário execute no terminal. Não é uma afirmação sobre conexões iniciadas por Codex, Claude ou outros programas externos.
## Logs do atualizador no bootstrap OSS

- O bootstrap ainda chamava `autoupdate::check_and_report_update_errors` para toda execução da GUI. No Windows, essa função lê logs do instalador antigo e pode encaminhar diagnósticos. Condicionei a chamada ao canal diferente de OSS, para que o Kikito Terminal local não processe logs nem execute o fluxo de relatório de falhas do atualizador.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss` passaram. O check emitiu somente avisos de artefatos incrementais antigos que o Cargo ignorou. Não executei testes nem Clippy.
## Telemetria de inicialização e origem do download no OSS

- `AuthState::initialize_local` não autentica usuário; por isso, a GUI caía no ramo de startup deslogado e emitia `LoggedOutStartup`, além de iniciar `download_method::determine_and_report` para telemetria. Condicionei esse ramo a canais diferentes de OSS. O Kikito OSS agora não emite esse evento nem executa a checagem local de origem do download.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss` passaram. O check emitiu somente avisos de artefatos incrementais antigos que o Cargo ignorou. Não executei testes nem Clippy.
## Bloqueio central de macros de telemetria no OSS

- Os macros de telemetria assíncronos já filtravam `Channel::Oss`, mas os macros síncronos e o caminho por executor não. Adicionei a mesma condição de canal aos três macros em `app/src/server/telemetry/macros.rs`, impedindo que handlers/eventos no OSS criem tarefas de envio síncrono ou registrem eventos pelo executor.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss` passaram. O check emitiu somente avisos de artefatos incrementais antigos que o Cargo ignorou. Não executei testes nem Clippy.
## Revisão estática dos limites de rede do processo OSS

- Registrei as barreiras encontradas no cliente HTTP, no carregamento de assets, na abertura de links da GUI e no início de servidores MCP. Os caminhos OSS bloqueiam destinos HTTP externos, restringem redirecionamentos de assets a loopback e não iniciam MCP. A configuração de canal mantém endpoints locais e desliga configuração de telemetria, atualização e crash reporting.
- Esclareci o limite do produto: comandos que o usuário inicia no terminal, como Codex, Claude ou ferramentas de linha de comando, são processos externos e podem abrir conexões próprias; isso não é tráfego do processo do Kikito Terminal.
- A captura anterior continua sendo ociosa. Não medi sockets durante ações reais da GUI; portanto, a revisão estática não prova ausência absoluta de conexões em todo cenário.
## Tradução — confirmação de reenvio ao agente

- Traduzi a mensagem temporária `again to send to agent` para `novamente para enviar ao agente` em `app/src/terminal/view/agent_view.rs`. Ela aparece quando o usuário precisa pressionar Enter novamente para confirmar o envio do prompt ao agente.
- Mantive o atalho Enter, a janela de confirmação e a lógica de envio sem alterações.
- `git diff --check` passou. `cargo check -p warp --bin warp-oss` passou; nenhum teste foi executado.
## Tradução — atalhos de anúncios de acessibilidade

- Na varredura de superfícies locais de terminal e workspace, encontrei duas descrições em inglês no registro dos atalhos macOS para ajustar o nível de anúncios de acessibilidade. Traduzi para `Definir anúncios de acessibilidade concisos` e `Definir anúncios de acessibilidade detalhados` em `app/src/workspace/mod.rs`.
- Preservei os IDs, as ações e os atalhos existentes. Mantive fora deste lote os resultados ligados a Warp Drive, cobrança, agentes na nuvem e instalação de CLIs hospedados; também não alterei descrições usadas apenas por schema e não renderizadas na interface local do OSS.
- Validação: `git diff --check` e `cargo check -p warp --bin warp-oss` passou. Nenhum teste foi executado.

## Auditoria — Settings OSS e atalhos do editor

- Revisei as páginas Appearance, Keybindings e About expostas na navegação OSS, além da busca geral e das dicas/diálogos alcançados nessas páginas. Títulos, rótulos, controles, busca, estados vazios, confirmação de atalhos, tooltip de configuração local e conteúdo da página Sobre já estavam em pt-BR. Os seletores de tema, fonte, ícone e material mantêm nomes próprios/técnicos; os modais locais de tema e cores por diretório já haviam sido traduzidos em lotes anteriores.
- A página Keybindings mostra também descrições de ações registradas por outras áreas. Traduzi o conjunto de atalhos do `EditorView` em `app/src/editor/view/mod.rs` (seleção, navegação, edição de texto, cursor múltiplo, sugestão automática e Vim), preservando IDs, condições e teclas.
- Mantive os slugs e chaves `SettingsSection::slug/from_slug`, pois são contratos persistidos e argumentos da CLI; os termos em inglês de busca interna são índices, não texto exibido. Os rótulos do painel Cloud platform e outras páginas foram excluídos por pertencerem à navegação não OSS. URLs de documentação não foram alteradas; a copy de apoio adjacente já está em português.
- A página Keybindings ainda pode exibir descrições cadastradas por outros módulos (por exemplo, ações específicas de editor de código, terminal e painéis); esses registros não faziam parte do conjunto `EditorView` deste lote e permanecem como área de auditoria futura. Conteúdo dinâmico de provedores também fica fora da tradução local.
- Validação: `cargo fmt --all` e `git diff --check`; `cargo check -p warp --bin warp-oss` passou. Nenhum teste foi executado.

## Tentativa de captura durante atalhos da GUI OSS

- Tentei abrir o binário com um `WARP_DATA_PROFILE` isolado, enviar atalhos para a paleta e Settings e amostrar conexões TCP do processo. A automação não devolveu saída de captura verificável, então descartei a tentativa como evidência. Encerrei os processos `warp-oss.exe` criados para essa tentativa; não havia instância deles ao conferir depois.
- A captura interativa permanece pendente. A observação ociosa anterior e a revisão estática são as únicas evidências runtime/estáticas válidas até aqui.
## Tradução — atalhos locais do terminal

- Traduzi descrições dos atalhos locais de `EditableBinding` em `app/src/terminal/view/init.rs`: colar/copiar, foco e localização, seleção e navegação de blocos, marcadores, rolagem, edição durante comandos em execução, ajustes do campo de entrada da CLI e ações locais de contexto/projeto. Preservei IDs, teclas, ações e condições.
- Mantive em inglês descrições de compartilhamento de sessão/bloco, fluxos de equipe e comandos de depuração ou onboarding, por serem funções hospedadas ou destinadas apenas a depuração. A ação `Warpify subshell` aparece no footer de subshells detectados e ativa uma integração local de shell; ela não depende de serviço hospedado, então também traduzi seu rótulo para `Ativar integração de shell no subshell`. Excluí nomes/descrições construídos dinamicamente para comandos de provedores.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss` passaram. O check emitiu avisos de artefatos incrementais antigos, ignorados pelo Cargo. Nenhum teste foi executado.

## Tradução — atalhos do editor de código

- Traduzi em `app/src/code/view.rs` os atalhos de salvar arquivo, salvar como e fechar abas; em `app/src/notebooks/file/mod.rs`, os atalhos de focar a entrada do terminal a partir do arquivo, recarregar arquivo e alternar a visualização do Markdown. Preservei nomes, teclas, ações e condições.
- Revisei os registros do editor de código e da barra de busca: as demais descrições já estavam em português. O Explorador de Projetos (`app/src/code/file_tree/view.rs`) registra atalhos fixos sem descrição configurável, portanto não há rótulos de atalho para traduzir nesse painel. Mantive identificadores e nomes de teclas em inglês por serem chaves internas/combinações.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss` passaram. O check emitiu avisos de artefatos incrementais antigos, ignorados pelo Cargo. Nenhum teste foi executado.

## Captura de rede do OSS após atalhos locais

- Iniciei `target/debug/warp-oss.exe` com o perfil isolado `offline-interaction-audit-20261017c`. Em quatro amostras da árvore contendo dois processos `warp-oss.exe` — inicialização, após envio de `Ctrl+Shift+P`, após envio de `Ctrl+,` e após pedido de fechamento — não havia conexões externas. O único socket foi o listener `127.0.0.1:9282`, que corresponde ao servidor HTTP local OSS, ligado a loopback e usado pelos roteadores locais de detecção da instalação/profiling.
- Os atalhos foram enviados à janela `Kikito Terminal`, mas não confirmei visualmente que as telas correspondentes abriram; portanto, a evidência demonstra somente que o processo permaneceu sem sockets externos observáveis durante as tentativas. Nenhum comando foi executado no terminal. Forcei o encerramento da árvore de teste e confirmei que não restaram processos `warp-oss.exe` dela.
- Esta amostra curta não demonstra isolamento de todo cenário ou de processos que o usuário inicia no terminal. Ela complementa a observação ociosa e as barreiras estáticas documentadas em `evidence/network-boundaries.md`.
## Auditoria — atalhos de painéis e abas

- Revisei os `EditableBinding` de `app/src/pane_group`: os rótulos de fechar sessão, dividir painéis, mover foco, redimensionar painéis, maximizar o painel e iniciar uma sessão de terminal já estavam em português. Nenhuma alteração de código foi necessária.
- Mantive `Share pane` em inglês porque o atalho chama o diálogo de compartilhamento do conteúdo do painel e só é registrado no contexto de objeto compartilhado, uma função hospedada fora do escopo OSS local. Os demais atalhos de navegação de painel são `FixedBinding`, sem descrição editável no Settings.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss` passaram. O check emitiu avisos de artefatos incrementais antigos, ignorados pelo Cargo. Nenhum teste foi executado.

## Tradução — paleta e busca locais

- Traduzi mensagens acessíveis da busca (carregamento, seleção, erro e confirmação de atalhos), o título e estado vazio da busca de comandos e a orientação de navegação das abas.
- Traduzi rótulos locais do menu de contexto de arquivos, comandos, blocos, código e diferenças, além de estados vazios/carregamento/indexação e descrições acessíveis de arquivos, comandos, blocos, símbolos e diferenças Git. O seletor de referência local também recebeu placeholder e estado vazio em português. Mantive nomes dinâmicos de comandos, arquivos e branches intactos.
- Mantive sem alteração categorias e itens hospedados, como workflows, cadernos, planos, conversas, prompts salvos e segredos externos; também deixei em inglês comandos de cobrança/conta, Grok, handoff, execução remota, créditos e feedback externo. Descrições dos comandos slash locais já estavam em português. Identificadores e teclas continuam sendo chaves internas.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss` passaram. O check emitiu avisos de artefatos incrementais antigos, descartados automaticamente pelo Cargo. Nenhum teste foi executado.
## Auditoria — comandos no terminal

- O caminho `app/src/commands` não existe nesta checkout. Auditei a superfície equivalente `app/src/terminal/input/slash_commands`: os comandos locais usam os nomes e descrições do registro estático já traduzidos em português no lote anterior; a renderização inline não acrescenta rótulos próprios em inglês.
- Mantive intactos nomes e descrições dinâmicos de skills/provedores e prompts salvos do Warp Drive. A vista `cloud_mode_v2_view.rs`, incluindo cabeçalhos Commands/Skills/Prompts, estados Loading/No results e “Show more”, é a apresentação cloud hospedada e ficou fora do escopo OSS local.
- Não foi necessária alteração de código neste lote; IDs, sintaxe `/comando` e descrições dinâmicas permanecem como estavam.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss` passaram. Nenhum teste foi executado.

## Auditoria — mensagens locais da vista do terminal

- Revisei rótulos, tooltips, menus de contexto, avisos de encerramento do shell e estados da sessão local. As ações locais visíveis já estão em português; não encontrei strings locais em inglês para alterar neste lote. Mensagens do shell e saídas de comandos continuam dinâmicas e preservam o texto produzido pelo sistema.
- Mantive sem alteração os fluxos de Agent/IA e infraestrutura hospedada da inicialização de projeto (indexação associada ao agente, geração assistida de AGENTS.md, criação de ambiente para cloud agents), instalação remota de servidores de linguagem, compartilhamento de sessão, SSH remoto e cópia de links/IDs de depuração. Esses caminhos dependem de serviços ou conteúdo externo e estão fora deste recorte OSS local.
- Não houve mudança de código neste lote. `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss` passaram; nenhum teste foi executado.

## Auditoria — menus e chrome da janela

- Revisei a barra de menus do aplicativo, o menu de janela/dock e a origem do título da janela. No canal OSS, os menus visíveis e suas ações de janela/arquivo/edição usam rótulos já traduzidos; o título da janela acompanha o nome dinâmico da aba/sessão, sem rótulo fixo em inglês.
- Itens em inglês encontrados ficam condicionados a canais não OSS (política de privacidade, sair, ajuda/feedback, menus AI/Drive e nova aba de agente). Mantive-os fora do escopo. Também não alterei rótulos de ações nativas de menu do sistema, identificadores internos, nomes de arquivo de configuração nem entradas de depuração que criam usuário/autenticam em serviço externo.
- Não há integração de ícone de bandeja nesta camada do app. Nenhum código foi alterado neste lote; `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss` passaram. Nenhum teste foi executado.

## Auditoria — explorador e entrada de projetos locais

- Traduzi os botões de abertura de pasta/repositório e criação de projeto, seus tooltips e descrições de atalhos na tela de entrada local. IDs e atalhos foram preservados.
- Os menus do Explorador de projetos, rótulos de arquivo/pasta e estados locais já estavam em português. Mantive nomes de arquivos e diretórios intactos.
- Mantive em inglês o texto de indisponibilidade para sessões remotas, pois descreve especificamente o modo remoto. Traduzi “Clone repository” e a explicação do botão; a operação de clonagem continua podendo acessar o host Git escolhido pelo usuário. Logs de diagnóstico não são copy da interface.
- `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss` passaram. Nenhum teste foi executado.



## Auditoria — boas-vindas e configuração inicial

- O caminho `app/src/onboarding` não existe nesta checkout. Revisei as telas equivalentes em `workspace/view/onboarding.rs`, `workspace/hoa_onboarding` e a lógica de preferências em `settings/onboarding.rs`. O copy visível de boas-vindas/configuração local já está em português; não foi necessária alteração de código.
- Mantive fora do escopo o onboarding de login/conta e as partes sobre caixa de entrada de agentes, execução hospedada e links para documentação externa. Os textos restantes em inglês nesta superfície são comentários, logs ou nomes internos, não copy exibido ao usuário.
- `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss` passaram. Nenhum teste foi executado.

## Auditoria — componentes compartilhados

- Traduzi os estados visíveis do lightbox de imagens: vazio, falha ao carregar e carregamento. O componente é compartilhado e usado por conteúdo local da interface OSS.
- Revisei os componentes comuns de botão, diálogo, tooltip e atalhos, além das camadas de janela/plataforma do WarpUI. Não encontrei copy fixo adicional em inglês para interfaces locais; textos de diálogo vêm do chamador, nomes de teclas permanecem como abreviações convencionais, e diagnósticos/APIs nativas foram preservados.
- `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss` passaram. Nenhum teste foi executado.

### Verificação adicional — metadados de settings exibidos no OSS

- Reconfirmei as referências no código de UI: `Setting::description()` aparece apenas em `settings_view/agent_profiles_page.rs`, página que não é registrada na navegação OSS. As descrições de `app/src/settings/*.rs` alimentam o schema/configuração e não são apresentadas como ajuda nas páginas locais Appearance, Keybindings e About.
- A tela Appearance usa rótulos próprios para o seletor de ícones em `appearance_page.rs`; eles já estão localizados. Descrições de ícone como “The app icon displayed in the dock” não são renderizadas nessa tela. Preservei IDs, slugs, caminhos e valores técnicos.
- Não encontrei copy local em inglês para alterar neste follow-up. Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`; nenhum teste foi executado.

## Tradução — editor local de código

- Traduzi em `app/src/code/local_code_editor.rs` o tooltip para adicionar a seleção como contexto e o aviso de alterações salvas que ainda não aparecem na versão exibida.
- Traduzi em `app/src/code/view.rs` o título padrão de arquivos sem nome (`Sem título`) e a ação para fechar arquivos salvos. Mantive nomes de arquivo, caminhos, conteúdo de código e saída do LSP como dados do usuário/sistema.
- O Explorador de Projetos foi auditado separadamente. Comentários importados do GitHub e ações de comentário foram mantidos fora deste lote por pertencerem ao fluxo hospedado de revisão; sugestões de correção da última execução pertencem ao fluxo assistido por IA; o aviso de host desconectado descreve uma sessão remota. Rótulos locais da barra de status do LSP já estavam em português.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`; nenhum teste foi executado.


## Auditoria — barra de entrada e colagem de imagem

- Revisei `app/src/terminal/input/message_bar`, `terminal_message_bar.rs` e o fluxo local da ação de colar imagem pela área de transferência. O rótulo `Colar caminho da imagem` e sua dica já estavam traduzidos; corrigi a acentuação de dois avisos de imagem/clipboard em `app/src/terminal/view/use_agent_footer/mod.rs`.
- Mantive intactos os prompts e ações de Agent/IA, mensagens internas de depuração/invariantes, conteúdo digitado, caminhos e nomes de arquivos. O restante da barra de mensagens inspecionada não revelou copy local elegível em inglês.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`; nenhum teste foi executado.





## Auditoria — confirmação local de saída

- Revisei `app/src/quit_warning/mod.rs`, que apresenta confirmações ao fechar abas, painéis, janelas ou o aplicativo e avisa sobre processos em execução e alterações de arquivos não salvas. Os textos exibidos já estão em português; não foi necessária alteração de código.
- A referência a sessões compartilhadas é conteúdo dinâmico de um recurso hospedado e não foi alterada. IDs, contagens, nomes de arquivos e demais dados dinâmicos permanecem intactos.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`; nenhum teste foi executado.

## Auditoria — footer do editor e menu LSP

- Revisei os rótulos de ativação/instalação, estados de disponibilidade e mensagens de suporte a linguagens no footer, além das ações do menu LSP para abrir registros, iniciar, parar, reiniciar, remover e gerenciar servidores. O copy fixo local já está em português; não precisei alterar código.
- Preservei nomes/binários dos servidores, nomes de linguagem e arquivo, além de progresso e detalhes de erro recebidos dos próprios servidores, pois são conteúdo técnico dinâmico. A chamada da skill `/update-tab-config` já usa texto em português e permanece identificada pelo nome de comando.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`; nenhum teste foi executado.

## Tradução — componentes compartilhados da interface

- Em `app/src/view_components/markdown_toggle_view.rs`, traduzi as opções locais do alternador de Markdown para `Formatado` e `Fonte`, ampliando o espaço das opções para acomodar o rótulo sem corte. O controle é usado no editor local de arquivos e na visualização local de Markdown.
- Revisei os demais componentes compartilhados desta pasta: estados de busca, ações de toast, selo de novidade e entradas genéricas já usam português ou recebem copy do chamador. Não alterei mensagens dinâmicas, conteúdo do usuário nem alertas fornecidos por outras áreas.
- Mantive o rótulo `Beta` da variante genérica sem alteração: ela não tem uso ativo no aplicativo OSS. Toasts de agente e conteúdo ligado a IA hospedada também ficaram fora do escopo.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`; nenhum teste foi executado.

## Auditoria — gerenciamento local de workspace e sessões

- Revisei criação de abas e grupos, renomeação inline de abas/grupos/painéis, busca de abas, reabertura de sessão fechada e estado vazio do painel vertical. Os rótulos locais deste recorte já estão em português (`Renomear`, ações de criação/fechamento e `Nenhuma aba aberta`); os campos de renomeação inline não acrescentam placeholder ou erro fixo em inglês. Não precisei alterar código.
- O fluxo de restauração de workspace não adiciona copy fixo em inglês; nomes de shell, títulos de abas/painéis e caminhos são valores dinâmicos do usuário/sistema. A confirmação de fechamento de sessão em `close_session_confirmation_dialog.rs` explica o encerramento de uma sessão compartilhada, portanto permanece fora deste escopo local.
- Mantive fora desta auditoria onboarding, conversas de agente, compartilhamento/equipes, code review/PR, ações de atualização e outros fluxos hospedados. Seus textos e mensagens técnicas não são copy de gerenciamento local elegível.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`; nenhum teste foi executado.

## Tradução — editor de texto compartilhado

- Traduzi em `app/src/editor/view/element.rs` a dica para percorrer sugestões locais de comando; em `app/src/editor/view/model/mod.rs` e `view/mod.rs`, traduzi os anúncios de acessibilidade para seleção, exclusão e colagem.
- Em `accept_autosuggestion_keybinding_view.rs`, alinhei a busca do item selecionado ao nome já traduzido `Personalizado...`, preservando o comportamento de seleção do menu para atalhos customizados.
- Mantive fora deste recorte os toasts e tooltips de entrada por voz, erros de anexos e menus de busca de arquivos ligados ao contexto de modelos/IA hospedados. Conteúdo digitado/colado e nomes de teclas permanecem intactos.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`; nenhum teste foi executado.

## Auditoria — configurações locais de inicialização e abas

- Traduzi o rótulo do link de documentação no modal de salvamento de configuração, os nomes gerados para novas abas e descrições padrão do parâmetro de branch da worktree. O placeholder visual de branch agora usa `minha-feature-branch`; preservei o valor default `my-feature-branch`, pois pode ser usado como nome real da branch.
- Os demais botões, placeholders, estados de carregamento e mensagens de validação nos modais locais já estavam em português. Mantive sem tradução o conteúdo dinâmico de erros TOML/sistema, identificadores, valores de configuração e nomes técnicos dos agentes. O destino do link de documentação permanece uma URL externa.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`; nenhum teste foi executado.

## Auditoria — editor de variáveis de ambiente

- Traduzi os rótulos fixos `Save`, `Variables` e `Load`, além do título inicial `Untitled`, nos componentes do editor local de variáveis de ambiente. Os títulos, campos, validações, diálogos de comando e confirmação de alterações não salvas já estavam em português.
- Mantive os nomes de variáveis, comandos e valores do usuário. Os avisos de lixeira/restauração e ações Copy link/Duplicate/Trash pertencem à gestão de coleções sincronizadas; nomes como 1Password e LastPass são marcas externas. Ambos ficaram fora deste recorte OSS local.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`; nenhum teste foi executado.

## Auditoria — diálogos de comportamento do terminal

- Revisei `app/src/terminal/enable_auto_reload_modal.rs`: seus rótulos em inglês fazem parte do fluxo de compra/faturamento e tanto o corpo quanto as ações retornam vazio/inativas no canal OSS. Mantive esse copy hospedado fora da tradução.
- Nos diálogos e avisos de comportamento local próximos (encerramento de shell, modo Vim, expansão de alias, filtro de blocos e avisos de sistema), os textos visíveis fixos já estão em português; não precisei alterar código. Saída de processo e nomes de shell foram preservados.
- Mantive fora desta revisão prompts de agente, workflows, faturamento, AWS Bedrock, sessões compartilhadas e setup cloud, por dependerem de recursos externos ou hospedados.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`; nenhum teste foi executado.

## Auditoria — banners e notificações locais do terminal

- Traduzi em `app/src/terminal/view.rs` as mensagens e ações de bloqueio de acesso OSC 52 à área de transferência, o fallback acessível de notificação sem título, as dicas de acessibilidade das notificações e o erro de subshells do PowerShell. Em `app/src/terminal/recorder.rs`, traduzi a ação local `Open` para `Abrir`.
- Os banners de descoberta/erro de notificações, shell encerrado, expansão de aliases, Vim e abertura/edição de arquivos já estavam em português. Preservei conteúdo dinâmico do terminal e de processos.
- Mantive fora do escopo banners de login AWS, sessões compartilhadas, prompts de agente e avisos de SSH que dependem da extensão/serviço Warp; o conteúdo de `ssh_tmux_deprecation_banner.rs` também remete à extensão e documentação externas.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`; nenhum teste foi executado.

## Auditoria — busca no terminal

- Revisei o componente local de busca em `app/src/view_components/find.rs` e sua integração com `app/src/terminal/find`. Placeholder, dicas de regex/sensibilidade a maiúsculas, busca no bloco, instruções de acessibilidade, resultado atual, estado sem resultados e indicador de busca já estão em português. Não encontrei copy fixo em inglês que exigisse alteração.
- Mantive `Find` como nome interno da view e os identificadores de ações/controles, além dos valores numéricos de contagem. As mensagens `Scanning`/`Complete` e erros RegexDFAs pertencem a estados internos/logs, não à interface visível. O texto pesquisado e a saída do shell permanecem dados dinâmicos.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`; nenhum teste foi executado.

## Tradução — editor local do prompt

- Confirmei que o modal de edição do prompt é alcançável pelas configurações locais de aparência no OSS. Traduzi o título, as seções do prompt do terminal e do shell (PS1), a restauração do padrão, os botões, os controles de prompt na mesma linha e o valor `Nenhum` do seletor de separador.
- Traduzi em `app/src/context_chips/mod.rs` os nomes fixos dos chips locais de diretório, usuário, ambiente, data/hora, Git, Kubernetes, SVN e login remoto exibidos no configurador e em tooltips. Mantive intactos conteúdos dinâmicos do prompt, nomes técnicos (`Host`, `subshell`), rótulo de Pull Request que depende de integração externa e o chip de plano/lista do Agent.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`; nenhum teste foi executado.

## Auditoria — configurações locais de shell, diretório e editor

- Revisei os widgets de inicialização do shell, diretório de trabalho, reabertura de sessões fechadas e editor externo nos arquivos `app/src/settings_view/features_page.rs` e `app/src/settings_view/features/`. Os títulos, descrições, opções, placeholders e rótulos fixos visíveis já estão em português; não precisei alterar código. Os nomes reconhecidos de shells, aplicativos externos, `$EDITOR` e caminhos do usuário foram preservados.
- O editor externo aparece na página local de editor/revisão de código; mantive os nomes de editores e atalhos técnicos. Strings de busca interna, IDs de view, telemetria, comentários e mensagens de diagnóstico em inglês não são copy visível. Não incluí outros widgets da página de recursos ligados a IA, workflow ou serviços hospedados.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`; nenhum teste foi executado.

## Auditoria — automação e integração de shells nas configurações

- Em `app/src/settings_view/warpify_page.rs`, traduzi o cabeçalho fixo `Subshells` para `Shells aninhados`. Em `app/src/settings_view/mod.rs`, defini o rótulo visível da página `SettingsSection::Warpify` como `Integração de shells`, removendo o fallback de depuração `Warpify` da navegação. Os textos descritivos da página já usavam `Kikito Terminal`.
- Revisei `scripting_page.rs`: o copy visível já está em português; mantive `warpctrl` porque é o nome do comando CLI e os IDs de telemetria. Essa página só aparece quando `WarpControlCli` está habilitada. Excluí `show_blocks_view.rs`, que trata de blocos compartilhados hospedados e não aparece quando a sessão OSS está desconectada/anônima.
- Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`; nenhum teste foi executado.

## Cobertura — navegação de configurações no OSS

- Conferi o registro em `app/src/settings_view/mod.rs`: no canal OSS, `settings_pages` e a navegação são filtrados para `Appearance`, `Keybindings` e `About`. Essas três páginas, a busca e seus diálogos/dicas locais já foram auditados em lotes anteriores; o shell de configurações não expõe páginas locais adicionais no OSS.
- Páginas como recursos, integrações de shell, editor/revisão, privacidade e blocos compartilhados não entram nessa lista OSS. Não reabri esses lotes; `show_blocks_view.rs` e as páginas hospedadas permanecem excluídas por dependerem de conta/serviços externos.
- Não encontrei nova string fixa para traduzir nesta verificação de cobertura. Validação: `cargo fmt --all`, `git diff --check` e `cargo check -p warp --bin warp-oss`; nenhum teste foi executado.
