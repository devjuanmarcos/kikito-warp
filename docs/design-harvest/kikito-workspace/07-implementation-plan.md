# Ordem de implementação e validação

As etapas são sequenciais. Uma etapa seguinte só começa depois que a atual tiver interação funcional e pelo menos 98% de paridade visual na captura state-to-state comparável. Reutilizar o modelo/persistência de organizações, projetos, abas e Git; substituir a apresentação em `app/src/workspace/view/local_workspace/` sem apagar a tabela, IDs de tabs, sessões ou operações Git locais.

## Etapa 0 — Evidência e diagnóstico da interface atual

Arquivos de referência: `ui_kits/workspace/index.html`, `OrgRail.jsx`, `ProjectSidebar.jsx`, `ProjectTabBar.jsx`, `HomeScreen.jsx`, `CreateDialog.jsx`, `ImageCropper.jsx`, `GitPanel.jsx`, `GitGraph.jsx`; implementação: `app/src/workspace/view.rs`, `app/src/workspace/view/local_workspace/`, `app/src/workspace/action.rs`, `app/src/local_workspace/model.rs`.

- Servir uma cópia isolada do protótipo por HTTP local; resolver CDN/React/Babel; remover apenas o painel Tweaks da captura, não da fonte.
- Capturar 1280×800 no mesmo tema escuro, estado ativo, conteúdo de fixture e estado de ponteiro.
- Executar clique real em cada hit target e registrar eventos/ações para localizar por que o app instalado está não responsivo.
- Comparar screenshot do app atual e do protótipo, criando uma tabela de divergências por retângulo e fluxo.

**Gate:** captura íntegra, console sem erro impeditivo e causa/área do bloqueio de clique identificada. Este gate precede alteração visual para evitar esconder o defeito com mudança de layout.

## Etapa 1 — Trilho de organizações

Arquivos Rust principais: `app/src/workspace/view/local_workspace/org_rail.rs`, `app/src/workspace/view/local_workspace/mod.rs`, `app/src/workspace/view.rs` (rotas de ação/foco), testes correspondentes `org_rail_tests.rs` e/ou `mod_tests.rs`.

Recriar 64 px, cabeçalho compatível com o chrome da janela real, Início, separador, tiles de 36 px, indicador lateral, imagens/iniciais, estados hover/ativo, badge de agente, tooltips, criação e configurações. Reutilizar `Icon`, `Button`, `Tooltip`, `NotificationDot` e funções de cor. Não criar uma segunda barra de janela caso o chrome já esteja presente.

Testar clique em Home, selecionar cada organização, hover/leave, menu de botão direito, ação explícita, criação, overflow de organizações com scroll, tile com imagem, inicial Unicode, badge e janela estreita. Verificar que clique no tile não é interceptado pelo canvas de terminal e que hit target do menu não seleciona outra organização.

**Gate:** 98% visual para todos estados do rail, 100% das ações do rail operacionais e nenhum clique passa à superfície de terminal por trás. Só então iniciar Etapa 2.

## Etapa 2 — Sidebar de projetos

Arquivos: `project_sidebar.rs`, `context_menu.rs`, testes de sidebar/contexto, `mod.rs`, `workspace/view.rs` somente para ações já modeladas.

Aplicar a estrutura do documento de componentes: cabeçalho da org 34 px, busca, overline/contagem, lista virtualmente/scroll-safe, rows com branch e estados, ações no hover, empty/no-results e rodapé Novo projeto. Conservar seleção persistida e status do caminho. Fazer hit areas explícitas e separadas: linha seleciona; plus cria aba; menu abre ações sem selecionar acidentalmente.

Testar filtro por nome/caminho sem case, nenhum resultado, org vazia, path longo/Unicode, branch longa, projeto ativo/inativo, busy/error path, hover do row, menu via botão/contexto e execução de cada ação. Scroll não pode criar overlay invisível que bloqueie linhas.

**Gate:** 98% das capturas de sidebar; busca, seleção, criação e menu 100% testados. Só então iniciar Etapa 3.

## Etapa 3 — Barra de abas e canvas do projeto

Arquivos: `mod.rs`, `project_tabs.rs` (extrair de `mod.rs`), `workspace/view.rs`, testes de roteamento/seleção/fechamento.

Reproduzir 34 px, flex sizing 72–220, close control, status agent/terminal, plus e search launcher; abrir Git mantendo estado ativo. Integrar ao mecanismo de tabs/panes existente. Projeto sem tabs apresenta estado vazio e botão Nova aba. Nenhuma troca encerra ou reatribui sessão de outro projeto.

Testar select/close/new/move, active fallback, projeto vazio, tab de agente com cada estado, título longo e ativar tab de outro projeto via Home. Testar composição do canvas sob 1280×800 e janela reduzida.

**Gate:** 98% da barra/canvas nos estados cobertos e 100% dos ciclos da aba. Só então iniciar Etapa 4.

## Etapa 4 — Home e launcher

Arquivos: `home.rs`, `home_ask.rs` (extrair se necessário), testes `home_tests.rs`/`mod_tests.rs`.

Fazer header por tom aprovado, launcher com foco/atalhos, seis projetos recentes, sete abas recentes, ordenação, estados vazios e status. A cópia e os estados vêm do modelo real; sem nomes, datas, agentes ou branches fixtures hardcoded no produto. Colunas e truncamento devem seguir 960/220/40/32 descritos.

Testar busca por projetos/abas, prefixos, teclado, zero resultados, project/tab open, recent order, agent status, nenhuma organização e caminhos/títulos longos.

**Gate:** 98% de Home/launcher e 100% de navegação. Só então iniciar Etapa 5.

## Etapa 5 — Fluxos de organização/projeto e imagem

Arquivos: `create_dialog.rs`, `image_cropper.rs`, `workspace/view.rs`, `image_cropper_tests.rs` e testes do diálogo.

Preservar estrutura de diálogo 420 px e componente modal compartilhado; corrigir campos, footer, validação, foco e backdrop para alinharem com 24/12/14 px. Implementar create/edit/remove org e create/edit/remove project com persistência local e confirmação sem operações destrutivas em pastas. Crop de imagem conserva 240/128 px, proporção, zoom, pan, grid temporário e limites de decode já definidos.

Testar cada caminho com Enter, botão, Escape, X, click-outside, picker cancel, arquivo inválido/limite excedido, SVG/PNG/JPG, imagem enorme, pan nos limites, zoom 1×/4×, operação async atrasada, fechar diálogo durante processamento, falha de banco, diretório inválido e cancelamento sem salvar.

**Gate:** 98% por estado de formulário/erro/confirmação/crop e todos os fluxos completos sem perda de dados. Só então iniciar Etapa 6.

## Etapa 6 — Context menus, foco e hover

Arquivos: `context_menu.rs`, `org_rail.rs`, `project_sidebar.rs`, `workspace/view.rs`, testes de interação.

Unificar menu do trilho e menu de projeto com aparência do kit; menu de org terá as ações do contrato nativo e confirmação; menu de projeto preserva abrir Git/Nova aba/Edit/Remover conforme decisões já implementadas. Posicionar em área visível e fechar fora sem cancelar a ação acionada. Foco retorna ao acionador; setas, Enter, Espaço e Escape consistentes.

Testar mouse, teclado, fechamento fora/dentro, menu no último item da tela, menu sob sidebar rolada, clique repetido, right-click e nested buttons. Confirmar ausência de menus presos ou event handlers globais que interceptem todo o workspace.

**Gate:** 98% de todos estados de menu e nenhuma ação de teclado/mouse sem resultado esperado. Só então iniciar Etapa 7.

## Etapa 7 — Source Control visual e integração com Git existente

Arquivos: `source_control/*.rs`, `app/src/local_git/`, ações `app/src/workspace/action.rs`, testes Git e UI correspondentes. Não substituir o serviço real por `GitData.jsx`.

Alinhar largura 300–380, cabeçalho/seções, commit form, stage rows, status e grafo ao desenho: 34/26/24/28 px, lanes 12, rows 24, pills 16. Preservar operação Git local e manual, estados de conflito/stash/worktree/branch já aprovados na especificação Git, e acessibilidade dos menus. Operações de rede continuam sempre explícitas.

Testar repositório temporário real, dirty/staged/clean, conflitos, HEAD detached, sem repo, git ausente, paths com espaço/Unicode, branch longa, erros de comando, operação pendente e cancelada, além das interações visuais em cada seção.

**Gate:** 98% do layout do painel/graph e testes de operação real aprovados; não exibir nenhum fixture como status real. Este gate fecha esta fatia.

## Etapa 8 — Auditoria final integrada

Capturar 1280×800 no mesmo tema, fonte/escala e posição do ponteiro; capturar tema claro como verificação de tokens, e janelas estreitas para overflow; verificar escala e acessibilidade. Gerar image diff, relatório de percentuais por superfície, lista de estados testados, exceções mascaradas e defeitos restantes. Fazer inspeção final dos dirty changes existentes antes de editar os mesmos arquivos.

**Fechamento:** todas as superfícies ≥98%; todos os fluxos acima passam; não há hit areas inoperantes, overlays bloqueando interação, regressão de tabs/sessões, perda de dados, comentários narrativos novos, nem alterações globais de tipografia/animação para corrigir a feature.
