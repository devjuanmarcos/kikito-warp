# Kikito Terminal: workspace local e Git — especificação de design

**Status:** aprovado pelo usuário em 18/10/2026

**Objetivo:** substituir a navegação atual por um workspace local organizado em organizações, projetos e abas, com Source Control conectado a repositórios Git reais e paridade funcional com a área Git nativa do VS Code.

**Plataforma:** aplicativo desktop Rust/WarpUI, persistência SQLite local e Git instalado no computador.

## Decisões aprovadas

- O produto continua local e sem backend Kikito.
- Organizações e projetos são metadados locais; criar, renomear ou remover esses metadados não move nem apaga pastas do usuário.
- Um projeto aponta para um diretório local. Esse diretório pode conter um repositório Git ou ainda não ter Git inicializado.
- Cada projeto tem seu conjunto de abas de terminal/sessão; alternar projeto restaura as abas e a aba ativa daquele projeto.
- O usuário inicia todas as operações Git. Fetch, pull e push são ações explícitas. Nenhuma atualização remota periódica ou disparada pela abertura do painel é permitida.
- O Source Control cobre a experiência Git nativa do VS Code, incluindo status, staging completo e parcial, commits, histórico/graph, branches, tags, comparação, cherry-pick, stash, worktrees e resolução de conflitos.
- Os itens visuais e fluxos presentes no ZIP são a referência principal de layout. A captura dentro de `uploads/` é referência complementar do painel Source Control do VS Code.
- Todo texto novo da interface será em português brasileiro. O conteúdo do terminal, nomes de branches, caminhos e mensagens produzidas pelo Git permanecem fiéis ao repositório e ao Git.
- Não adicionar comentários de código narrativos. Comentários existentes só mudam quando a lógica documentada mudar.

## O que foi analisado no ZIP

O arquivo `C:\Users\Marcos\Downloads\✅ Kikito Terminal.zip` contém o design system e protótipos em `ui_kits/workspace/`. O kit descreve a composição visual e os principais fluxos:

- `OrgRail.jsx`: trilho de organizações, seleção, criação, edição, remoção e atalhos.
- `ProjectSidebar.jsx`: cabeçalho da organização, busca, lista de projetos, branch e contagem/estado das abas, criação e menu contextual de projeto.
- `ProjectTabBar.jsx`: abas por projeto, criação/fechamento e acesso ao Source Control.
- `HomeScreen.jsx`: busca/launcher e projetos/abas recentes.
- `CreateDialog.jsx` e `ImageCropper.jsx`: fluxos de criação/edição de organização e projeto, ícone, cor, nome e diretório.
- `GitPanel.jsx`, `GitGraph.jsx` e `GitData.jsx`: painel de alterações e histórico em grafo.
- `WorkspaceData.jsx`: dados de exemplo com organizações, projetos, caminhos, branches e abas.
- `uploads/pasted-1792329106410-0.png`: referência do Source Control do VS Code, incluindo múltiplos repositórios, Changes, ações de commit e histórico.

O protótipo usa React, scripts carregados por CDN e `localStorage`; o estado inicial é demonstrativo. Git é simulado com fixtures e temporizadores: stage, commit e sincronização não executam operações Git reais. A lista de branches não tem os menus contextuais solicitados. A implementação nativa deve reproduzir o sistema de design, não copiar o armazenamento ou o modelo de simulação.

## Inventário do código atual

- A aplicação desktop e a composição principal ficam em `app/src/workspace/`, principalmente `view.rs`, `view/left_panel.rs`, `view/right_panel.rs`, `view/vertical_tabs.rs` e `tab_group.rs`.
- As abas, sessões e panes já têm modelos e restauração persistidos. As tabelas `windows` e `tabs` guardam estado de janela e diretório atual; `tab_groups` agrupa abas.
- `crates/persistence/migrations/2025-07-29-122627_add_projects_table/up.sql` cria `projects` indexada por caminho e não modela organizações, nome amigável nem associação projeto/organização. `app/src/projects.rs` e `ProjectManagementModel` registram projetos usados pelo app. Essa tabela existente não deve ser reinterpretada como cadastro de organizações.
- `app/src/code_review/git_repo_model/local.rs` fornece watcher local e status; `app/src/code_review/diff_state/local.rs` contém leitura de diffs e ações locais. `app/src/util/git.rs` e `crates/warp_util/src/git.rs` executam comandos Git e expõem helpers de branch/commit.
- O subsistema code review também contém modelos remotos. A nova superfície local não deve depender de login, APIs Warp, GraphQL ou modelos remotos.
- Existem caminhos em comparação de branches que podem buscar refs remotamente. Todo caminho chamado por visualização, seleção ou abertura de painel deve permanecer local e nunca buscar; fetch só pode ocorrer no comando manual correspondente.

## Arquitetura

### Domínio local do workspace

Introduzir um domínio persistido próprio para organizações, projetos locais, preferências de navegação e associação entre projeto e abas existentes. Manter os registros históricos da tabela `projects` para os recursos que já os usam. Chaves estrangeiras, unicidade de caminhos normalizados e ordenação explícita devem proteger a integridade. A migração não apaga nem move diretórios e preserva sessões, abas e preferências existentes.

Uma organização contém zero ou mais projetos. Um projeto contém o caminho local e metadados de apresentação. O repositório Git, quando existe, é descoberto a partir do caminho; branch é estado derivado do Git e nunca é persistido como fonte de verdade. Abas/sessões continuam usando o ciclo de vida existente, com uma relação persistida que restaura sua pertença ao projeto.

### Identidade e restauração de abas existentes

`tabs.id` (`i32`) é o identificador persistido e estável de cada aba. A linha também contém `window_id` e um `tab_group_id` opcional. Na árvore de panes, `pane_nodes.id` é a chave estrutural do nó; `tab_id` liga a raiz à aba e `parent_pane_node_id` liga os nós filhos aos pais. `pane_leaves` usa a chave composta `pane_node_id` + `kind`. Para uma folha de terminal, `terminal_panes.id` é igual ao ID do nó folha; esses inteiros organizam as linhas do banco, enquanto `terminal_panes.uuid` (`Vec<u8>`) é a identidade persistida do pane/sessão de terminal. `cwd`, `shell_launch_data` e demais campos guardam metadados necessários para recriar a sessão, não a memória de um processo shell ativo.

Na leitura, `read_root_node` parte de `tabs.id` e `read_node` percorre os nós, folhas e registros `terminal_panes` por ID estrutural, formando `TerminalPaneSnapshot`. `WorkspaceView::configure_new_workspace` recria as abas a partir desses snapshots. Durante a restauração da `PaneGroup`, o UUID salvo vira `PaneUuid` e é associado a um `TerminalPaneId`; o cwd salvo só é reutilizado quando o caminho ainda existe. A associação nova projeto/aba referencia apenas `tabs.id`: ela não possui nem remove linhas existentes de janela, aba, grupo, pane ou sessão, nem encerra processos de terminal. Remover metadados locais remove apenas associações e metadados novos.

### Casca de navegação

Compor a janela em quatro superfícies: trilho de organizações, sidebar de projetos, barra de abas do projeto e conteúdo de terminal/painel. A Home continua acessível e apresenta itens recentes. Manter dimensões, hierarquia, tipografia, cores, ícones, densidade e estados de interação do kit, adaptados aos tokens e componentes nativos do app. O protótipo menciona variantes de apresentação; a implementação inicial usa a composição padrão documentada no kit, sem transformar o painel de ajustes do protótipo em configuração de produto.

Menus contextuais devem funcionar com mouse e teclado, respeitar o alvo selecionado e desabilitar ações indisponíveis. Remoção de metadados pede confirmação e explica que arquivos permanecem no disco. Seleção de imagem valida tipo, tamanho, leitura e resultado de recorte antes de persistir o asset local.

### Serviço Git local

Criar uma fronteira por repositório que recebe o caminho do projeto e expõe snapshots imutáveis de status, refs, commits, conflitos, stashes e worktrees, mais comandos explícitos. Usar o Git instalado pelo usuário como executor de operações, preservando configuração, hooks, signing e credenciais já configuradas. Construir argumentos sem interpolação em shell, tratar caminhos com espaços/Unicode, capturar códigos de saída e erros e cancelar operações quando o dono do modelo for descartado.

Leituras de status podem ser acionadas por watcher local e por atualização explícita. Nenhuma leitura usa rede. Os comandos com rede são somente os iniciados diretamente pelo usuário (Fetch, Pull, Push e publicação/exclusão remota quando disponível) e devem indicar repositório, remote e branch antes de uma operação ambígua. Não integrar telemetria, login ou serviços Warp.

### Source Control e paridade Git

O painel deve incluir:

1. **Mudanças:** grupos de staged, unstaged, untracked e conflitos; stage/unstage de arquivo e pasta; descarte com confirmação; diff por arquivo; seleção de linhas/hunks para staging parcial; commit staged, commit all, amend, undo/revert conforme segurança e contexto.
2. **Refs e histórico:** árvore/lista local e remota de branches, tags e worktrees; grafo de commits com seleção, arquivos alterados, comparação de duas refs, merge base, branch atual, upstream e contagens conhecidas localmente.
3. **Menu de branch:** checkout; criar branch a partir da branch/ref escolhida; renomear; merge; rebase; comparar com HEAD/branch selecionada; abrir alterações da branch; publicar e apagar local/remota quando aplicável. Checkout bloqueado ou que arriscaria mudanças deve explicar o motivo e oferecer opções seguras, como cancelar ou guardar alterações.
4. **Menu de commit:** abrir alterações; copiar SHA; comparar com HEAD, branch, tag ou merge base; cherry-pick; checkout detached com aviso; criar branch a partir do commit; reverter quando permitido; inspecionar arquivos.
5. **Stash:** criar com opção de incluir untracked/staged; listar; inspecionar; aplicar; pop; excluir; tratar conflitos de aplicação.
6. **Worktrees:** listar, criar a partir de branch/ref, abrir como projeto/caminho, remover com confirmação e aviso sobre arquivos ignorados/não rastreados. Bloquear remoção quando o Git indicar risco não confirmado.
7. **Conflitos:** identificar caminhos em conflito, abrir diff/merge de três vias, oferecer ações de resolução por bloco, marcar resolvido, continuar/abortar merge, cherry-pick ou rebase e preservar o estado até uma operação Git confirmar a resolução.
8. **Linha do tempo e blame:** histórico do arquivo e autoria por linha, com carregamento sob demanda e comparação navegável.
9. **Remotes:** Fetch, Pull, Push, publicar branch, sincronizar e excluir branch remota, todos iniciados pelo usuário e com o destino exposto antes da execução.

Menus e comandos não executáveis por configuração do repositório devem ficar desabilitados com explicação. Operações destrutivas têm confirmação contextual; operações que o Git recusa não devem ser contornadas com `-f` ou descarte implícito.

## Estados visuais e comportamento

Cada superfície deve ter estados de carregamento, vazio, erro recuperável e conteúdo. Repositório ausente, Git indisponível, diretório removido, HEAD detached, conflito em andamento, sem upstream, arquivo grande/binário, permissão negada e operação cancelada precisam de tratamento explícito em português. Seleção de organização/projeto/aba permanece consistente durante refresh e após reinício. Menus se posicionam dentro da janela, suportam Escape, foco visível e navegação por teclado.

## Critérios de aceite

1. Criar, renomear, selecionar e remover organizações/projetos persiste após reiniciar o app; diretórios do usuário nunca são movidos ou apagados pelo CRUD de metadados.
2. Reabrir um projeto restaura suas abas e aba ativa, isoladas de outros projetos, sem interromper as sessões existentes.
3. Os principais fluxos e dimensões correspondem ao ZIP em screenshots comparáveis; variantes de tema/escala preservam hierarquia e legibilidade.
4. Painel, branches, grafo e menus consultam o repositório selecionado e executam Git real. Nenhum dado de fixture é exibido como estado real.
5. Abrir/trocar de painel, buscar branches e navegar no grafo não produz tráfego de rede. Fetch/Pull/Push só ocorre após ação explícita e aponta ao remote correto.
6. Testes com repositórios temporários verificam sucesso, erro, conflito, cancelamento e preservação de alterações para cada família de comandos.
7. Staging parcial preserva exatamente linhas não selecionadas e o working tree; casos CRLF, arquivo renomeado, Unicode e binário são tratados corretamente.
8. Worktrees, stash, cherry-pick, merge e rebase refletem os estados reais e permitem continuar/abortar após conflito sem perda silenciosa.
9. Acessibilidade de menus, diálogos, foco e atalhos é verificável com teclado e leitores de árvore onde suportado.

## Limites do escopo

O escopo é a integração Git nativa do VS Code, não o sistema geral de extensões SCM nem integrações hospedadas por provedores. Login/serviços Warp, sincronização automática e backend Kikito seguem excluídos. `git clone` e configuração de provedores podem ser avaliados depois; o fluxo desta feature parte de uma pasta/projeto local e dos remotes já configurados pelo usuário.

## Sequência de entregas

1. Modelo e migração local para organizações/projetos, com preservação dos dados existentes.
2. Casca de navegação, Home, criação/edição e menus de organização/projeto.
3. Associação e restauração de sessões/abas por projeto.
4. Serviço Git local, watcher/status/diffs e Source Control básico.
5. Árvore de refs, grafo e menus completos de branches/commits.
6. Staging parcial, stash, worktrees, conflitos, Timeline/blame e remotes manuais.
7. Revisão visual, acessibilidade, testes de fluxo ponta a ponta e auditoria de rede.

## Referências

- Design recebido: ZIP citado acima, especialmente `ui_kits/workspace/`, `tokens/`, `guidelines/` e `uploads/pasted-1792329106410-0.png`.
- VS Code: [branches, stash e worktrees](https://code.visualstudio.com/docs/sourcecontrol/branches-worktrees), [histórico](https://code.visualstudio.com/docs/sourcecontrol/history), [staging e commits](https://code.visualstudio.com/docs/sourcecontrol/staging-commits), [resolução de conflitos](https://code.visualstudio.com/docs/sourcecontrol/merge-conflicts), [visão geral Source Control](https://code.visualstudio.com/docs/sourcecontrol/overview).
- Código do app: `app/src/workspace/`, `app/src/projects.rs`, `app/src/code_review/`, `app/src/util/git.rs`, `crates/warp_util/src/git.rs`, `crates/persistence/`.
