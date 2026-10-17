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
